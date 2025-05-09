use std::{str::FromStr, sync::Arc};
use tracing::{info, warn, debug};

use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    network::{AnyNetwork, EthereumWallet, ReceiptResponse, TransactionBuilder},
    primitives::{utils::format_units, Address, U128, U256},
    providers::{DynProvider, Provider},
    rpc::types::TransactionRequest,
    serde::WithOtherFields,
    signers::{local::PrivateKeySigner, Signer},
};
use anyhow::{Context, Result};
use artemis_core::types::Executor;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use uniswapx_rs::order::BPS;

use crate::{
    aws_utils::cloudwatch_utils::{
        build_metric_future, receipt_status_to_metric, revert_code_to_metric, CwMetrics, DimensionValue
    }, 
    executors::reactor_error_code::ReactorErrorCode, 
    shared::{burn_nonce, get_nonce_with_retry, send_metric_with_order_hash, u256},
    strategies::{keystore::KeyStore, types::SubmitTxToMempoolWithExecutionMetadata}
};
use crate::executors::reactor_error_code::get_revert_reason;

const GAS_LIMIT: u64 = 1_000_000;
static QUOTE_BASED_PRIORITY_BID_BUFFER: U256 = u256!(2);
static GWEI_PER_ETH: U256 = u256!(1_000_000_000);
const QUOTE_ETH_LOG10_THRESHOLD: usize = 8;
// The number of bps to add to the base bid for each fallback bid
const DEFAULT_FALLBACK_BID_SCALE_FACTOR: u64 = 50;
const CONFIRMATION_TIMEOUT_SEC: u64 = 10;

/// An executor that sends transactions to the public mempool.
pub struct PriorityExecutor {
    client: Arc<DynProvider<AnyNetwork>>,
    sender_client: Arc<DynProvider<AnyNetwork>>,
    key_store: Arc<KeyStore>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
}

#[derive(Debug)]
enum TransactionOutcome {
    Success(Option<u64>),
    Failure(Option<u64>),
    TimedOut(Option<u64>),
    RetryableFailure,
}

impl PriorityExecutor {
    pub fn new(
        client: Arc<DynProvider<AnyNetwork>>,
        sender_client: Arc<DynProvider<AnyNetwork>>,
        key_store: Arc<KeyStore>,
        cloudwatch_client: Option<Arc<CloudWatchClient>>,
    ) -> Self {
        Self {
            client,
            sender_client,
            key_store,
            cloudwatch_client,
        }
    }

    fn increment_tx_metric(
        &self,
        order_hash: &Arc<String>,
        chain_id: u64,
        outcome: &Result<TransactionOutcome, anyhow::Error>,
    ) {
        if let Some(cloudwatch_client) = &self.cloudwatch_client {
            let metric = match outcome {
                Ok(TransactionOutcome::Success(_)) => CwMetrics::TxSucceeded(chain_id),
                Ok(TransactionOutcome::Failure(_)) | Ok(TransactionOutcome::RetryableFailure) => CwMetrics::TxReverted(chain_id),
                Err(_) => CwMetrics::TxStatusUnknown(chain_id),
            };
            
            let metric_future = build_metric_future(
                Some(cloudwatch_client.clone()),
                DimensionValue::PriorityExecutor,
                metric,
                1.0,
            );
            if let Some(metric_future) = metric_future {
                send_metric_with_order_hash!(&Arc::new(order_hash.to_string()), metric_future);
            }
        }
    }

    async fn send_transaction(
        &self,
        wallet: &EthereumWallet,
        tx_request: WithOtherFields<TransactionRequest>,
        order_hash: &str,
        chain_id: u64,
        target_block: Option<u64>,
    ) -> Result<TransactionOutcome> {
        let tx_request_for_revert = tx_request.clone();
        let tx = tx_request.build(wallet).await?;
        info!("{} - Sending transaction to RPC", order_hash);
        let result = self.sender_client.send_tx_envelope(tx).await;

        match result {
            Ok(tx) => {
                info!("{} - Waiting for confirmations", order_hash);
                let receipt = match tokio::time::timeout(
                    std::time::Duration::from_secs(CONFIRMATION_TIMEOUT_SEC),
                    tx.with_required_confirmations(0).get_receipt()
                ).await {
                    Ok(receipt_result) => receipt_result.map_err(|e| {
                        anyhow::anyhow!("{} - Error waiting for confirmations: {}", order_hash, e)
                    }),
                    Err(_) => {
                        warn!("{} - Timed out waiting for transaction receipt attempting to burn nonce", order_hash);
                        return Ok(TransactionOutcome::TimedOut(None));
                    }
                };
                

                match receipt {
                    Ok(receipt) => {
                        let target_block_delta: f64 = receipt.block_number.unwrap() as f64 - target_block.unwrap() as f64;
                        if let Some(target_block) = target_block {
                            info!("{} - target block delta: {}, target_block: {}, actual_block: {}", order_hash, target_block_delta, target_block, receipt.block_number.unwrap());
                        }
                        let metric_future = build_metric_future(
                            self.cloudwatch_client.clone(),
                            DimensionValue::PriorityExecutor,
                            CwMetrics::TargetBlockDelta(chain_id),
                            target_block_delta as f64,
                        );
                        if let Some(metric_future) = metric_future {
                            send_metric_with_order_hash!(&Arc::new(order_hash.to_string()), metric_future);
                        }
                        let status = receipt.status();
                        info!(
                            "{} - receipt: tx_hash: {:?}, status: {}",
                            order_hash, receipt.transaction_hash, status,
                        );
                        
                        if !status && receipt.block_number.is_some() {
                            info!("{} - Attempting to get revert reason", order_hash);
                            // Parse revert reason
                            match get_revert_reason(&self.client, tx_request_for_revert, receipt.block_number.unwrap()).await {
                            
                                Ok(reason) => {
                                    info!("{} - Revert reason: {}", order_hash, reason);
                                    let metric_future = build_metric_future(
                                        self.cloudwatch_client.clone(),
                                        DimensionValue::PriorityExecutor,
                                        revert_code_to_metric(chain_id, reason.to_string()),
                                        1.0,
                                    );
                                    if let Some(metric_future) = metric_future {
                                        // do not block current thread by awaiting in the background
                                        send_metric_with_order_hash!(&Arc::new(order_hash.to_string()), metric_future);
                                    }
                                    // Retry if the order isn't yet fillable
                                    if matches!(reason, ReactorErrorCode::OrderNotFillable) {
                                        return Ok(TransactionOutcome::RetryableFailure);
                                    }
                                    else {
                                        info!("{} - Order not fillable, returning failure", order_hash);
                                        return Ok(TransactionOutcome::Failure(receipt.block_number));
                                    }
                                }
                                Err(e) => {
                                    info!("{} - Failed to get revert reason - error: {:?}", order_hash, e);
                                    Ok(TransactionOutcome::Failure(None))
                                }
                            }
                        } else {
                            Ok(TransactionOutcome::Success(receipt.block_number))
                        }
                    }
                    Err(e) => {
                        warn!("{} - Error waiting for confirmations: {}", order_hash, e);
                        Ok(TransactionOutcome::Failure(None))
                    }
                }
            }
            Err(e) => {
                warn!("{} - Error sending transaction: {}", order_hash, e);
                // If the nonce is already used, burn the nonce for the next transaction
                if e.to_string().contains("replacement transaction underpriced") {
                    info!("{} - Nonce already used, burning nonce for next transaction", order_hash);
                    burn_nonce(
                        &self.sender_client,
                        wallet,
                        tx_request_for_revert.from.unwrap(),
                        tx_request_for_revert.nonce.unwrap(),
                        order_hash
                    ).await?;
                }
                Ok(TransactionOutcome::Failure(None))
            }
        }
    }

    fn get_bids_for_order(
        &self,
        action: &SubmitTxToMempoolWithExecutionMetadata,
        order_hash: &str,
    ) -> Vec<Option<U256>> {
        let mut bid_priority_fees: Vec<Option<U256>> = vec![];

        // priority fee at which we'd break even, meaning 100% of profit goes to user in the form of price improvement
        if action.metadata.gas_use_estimate_quote > U256::from(0) {
            let quote_based_priority_bid = action
                .metadata
                .calculate_priority_fee_from_gas_use_estimate(QUOTE_BASED_PRIORITY_BID_BUFFER);
            if let Some(bid) = quote_based_priority_bid {
                bid_priority_fees.push(Some(bid));
                debug!("{} - quote_based_priority_bid: {:?}", order_hash, bid);
            }
        }

        // If the quote is large in ETH, add more bids
        // < 1e5 gwei = 1 fallback bid, 1e6 = 2 fallback bids, 1e7 = 3 fallback bids, etc.
        let mut num_fallback_bids = 3;
        if let Some(quote_eth) = action.metadata.quote_eth {
            if quote_eth > U256::from(0) {
                debug!("{} - Adding fallback bids based on quote size", order_hash);
                let quote_in_gwei = &quote_eth / GWEI_PER_ETH;
                debug!("{} - quote_eth_gwei: {:?}", order_hash, quote_in_gwei);
                
                if quote_in_gwei > U256::from(0) {
                    let quote_gwei_log10 = quote_in_gwei.log10();
                    debug!("{} - quote_gwei_log10: {:?}", order_hash, quote_gwei_log10);
                    if quote_gwei_log10 > QUOTE_ETH_LOG10_THRESHOLD {
                        num_fallback_bids += (quote_gwei_log10 - QUOTE_ETH_LOG10_THRESHOLD) as u64;
                    }
                }
            }
        }

        // Each fallback bid is 10000 - BID_SCALE_FACTOR * 2^i
        // If BID_SCALE_FACTOR = 50, then the bids are:
        // 9950, 9900, 9800, 9600, 9200, ...
        for i in 0..num_fallback_bids {
            // Check if the shift would cause overflow or if the result would be negative
            let bid_scale_factor = action.metadata.fallback_bid_scale_factor.unwrap_or(DEFAULT_FALLBACK_BID_SCALE_FACTOR);
            let bid_reduction = U128::from(bid_scale_factor * (1 << i));
            if bid_reduction >= U128::from(BPS) {
                // Stop generating more fallback bids
                break;
            }
            
            let bid_bps = U128::from(BPS) - bid_reduction;
            let fallback_bid = action
                .metadata
                .calculate_priority_fee(bid_bps);
            if let Some(bid) = fallback_bid {
                bid_priority_fees.push(Some(bid));
                debug!("{} - fallback_bid_{}: {:?}", order_hash, i, bid);
            }
        }

        bid_priority_fees
    }

    /// @notice Assigns nonces to transaction requests
    /// @param tx_requests Mutable slice of transaction requests to assign nonces to
    /// @param address Ethereum address to get nonce for
    /// @param order_hash Order hash for logging
    /// @param revert_protection Whether to use same nonce for all transactions
    async fn assign_nonces(
        &self,
        tx_requests: &mut [WithOtherFields<TransactionRequest>],
        nonce: u64,
        revert_protection: bool,
    ) {
        // Sort transactions by max_priority_fee_per_gas in descending order so that the highest bid is first
        tx_requests.sort_by(|a, b| {
            let a_fee = a.max_priority_fee_per_gas().unwrap();
            let b_fee = b.max_priority_fee_per_gas().unwrap();
            b_fee.cmp(&a_fee)
        });

        // With revert protection, we must use the same nonce for all transactions,
        // otherwise the reverted txs will sit in the mempool and block all nonces above it
        let mut current_nonce = nonce;
        for tx_request in tx_requests.iter_mut() {
            tx_request.set_nonce(current_nonce);
            if !revert_protection {
                current_nonce += 1;
            }
        }
    }
}

#[async_trait]
impl Executor<SubmitTxToMempoolWithExecutionMetadata> for PriorityExecutor {
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempoolWithExecutionMetadata) -> Result<()> {
        info!("{} - Executing transaction", action.metadata.order_hash);
        let order_hash = Arc::new(action.metadata.order_hash.clone());
        
        // Initialize this variable outside the main logic so we can access it in the cleanup section
        let mut public_address = None;
        
        // Use a closure to handle the main logic with ? operator for early returns
        let result = async {
            let chain_id_u64 = action
                .execution
                .tx
                .chain_id()
                .expect("Chain ID not found on transaction")
                .to_string()
                .parse::<u64>()
                .unwrap();

            let metric_future = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::PriorityExecutor,
                CwMetrics::ExecutionAttempted(chain_id_u64),
                1.0,
            );
            if let Some(metric_future) = metric_future {
                send_metric_with_order_hash!(&order_hash, metric_future);
            }

            // Acquire a key from the key store
            let (addr, private_key) = self
                .key_store
                .acquire_key()
                .await
                .context("Failed to acquire key")?;
            
            // Store the address for cleanup
            public_address = Some(addr.clone());
            
            info!("{} - Acquired key: {}", order_hash, addr);

            let chain_id = u64::from_str_radix(
                &action
                    .execution
                    .tx
                    .chain_id()
                    .expect("Chain ID not found on transaction")
                    .to_string(),
                10,
            )
            .expect("Failed to parse chain ID");

            let wallet = EthereumWallet::from(
                private_key
                    .as_str()
                    .parse::<PrivateKeySigner>()
                    .unwrap()
                    .with_chain_id(Some(chain_id)),
            );
            let address = Address::from_str(&addr).unwrap();

            action.execution.tx.set_from(address);

            // early return on OrderAlready filled
            // always use 1_000_000 gas for now
            let target_block = match action.metadata.target_block {
                Some(b) => BlockId::Number(b.into()),
                _ => BlockId::Number(BlockNumberOrTag::Latest),
            };

            info!(
                "{} - target_block: {}",
                order_hash,
                target_block.as_u64().unwrap()
            );

            let base_fee = self
                .client
                .get_gas_price()
                .await
                .context("Error getting gas price: {}")?;
            let bid_priority_fees = self.get_bids_for_order(&action, &order_hash);

            if bid_priority_fees.len() == 0 {
                info!(
                    "{} - No bid priority fees, indicating quote < amount_out_required; skipping",
                    order_hash
                );
                info!("{} - Quote < amount_out_required; skipping", order_hash);
                return Err(anyhow::anyhow!("Quote < amount_out_required"));
            }

            // Create a tx for each bid
            let mut tx_requests: Vec<WithOtherFields<TransactionRequest>> = Vec::new();
            for bid_priority_fee in bid_priority_fees.iter() {
                if let Some(bid) = bid_priority_fee {
                    let mut tx_request = action.execution.tx.clone();
                    let bid_priority_fee_128 = bid.to::<u128>();
                    tx_request.set_gas_limit(GAS_LIMIT);
                    tx_request.set_max_fee_per_gas(base_fee + bid_priority_fee_128);
                    tx_request.set_max_priority_fee_per_gas(bid_priority_fee_128);
                    tx_requests.push(tx_request);
                }
            }

            // Retry up to 3 times to get the nonce.
            let nonce = get_nonce_with_retry(&self.client, address, &order_hash, 3).await
                .context("Failed to get nonce after retries")?;
            info!("{} - Nonce: {}", order_hash, nonce);

            self.assign_nonces(&mut tx_requests, nonce, action.metadata.revert_protection).await;

            let metric_future = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::PriorityExecutor,
                CwMetrics::OrderBid(chain_id_u64),
                1.0,
            );
            if let Some(metric_future) = metric_future {
                send_metric_with_order_hash!(&order_hash, metric_future);
            }
            info!("{} - Executing {} transactions in parallel from {:?}", order_hash, tx_requests.len(), address);

            let mut success = false;
            let mut block_number = None;

            let metric_future = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::PriorityExecutor,
                CwMetrics::TxSubmitted(chain_id),
                1.0,
            );
            if let Some(metric_future) = metric_future {
                send_metric_with_order_hash!(&Arc::new(order_hash.to_string()), metric_future);
            }

            // Create futures for all transactions
            let futures: Vec<_> = tx_requests.iter().map(|tx_request| {
                self.send_transaction(&wallet, tx_request.clone(), &order_hash, chain_id_u64, target_block.as_u64())
            }).collect();

            // Wait for all transactions to complete
            let results = futures::future::join_all(futures).await;

            // Check results
            for (i, result) in results.iter().enumerate() {
                self.increment_tx_metric(&order_hash, chain_id, result);
                match result {
                    Ok(TransactionOutcome::Success(result)) => {
                        success = true;
                        block_number = *result;

                        let metric_future = build_metric_future(
                            self.cloudwatch_client.clone(),
                            DimensionValue::PriorityExecutor,
                            CwMetrics::OrderFilled(chain_id_u64),
                            1.0,
                        );
                        if let Some(metric_future) = metric_future {
                            send_metric_with_order_hash!(&order_hash, metric_future);
                        }
                        info!("{} - Transaction {} succeeded at block {}", order_hash, i, block_number.unwrap());
                        break;
                    }
                    Ok(TransactionOutcome::Failure(result)) => {
                        if i == results.len() - 1 {
                            block_number = *result;
                        }
                    }
                    Ok(TransactionOutcome::RetryableFailure) | Err(_) => {
                        // Skip the order for now
                    }
                    Ok(TransactionOutcome::TimedOut(_result)) => {
                        // no successes and revert protection is enabled, burn nonce
                        // this is to allow the next tx from this wallet to be submitted
                        // it may be that one order succeeded, but the others timed out
                        // however, we only want to burn the nonce if all txs timed out
                        if i == results.len() - 1 && action.metadata.revert_protection {
                            burn_nonce(
                                &self.sender_client,
                                &wallet,
                                address,
                                nonce,
                                &order_hash
                            ).await?;
                        }
                    }   
                }
            }

            // post key-release processing
            if let Some(_) = &self.cloudwatch_client {
                let metric_future = build_metric_future(
                    self.cloudwatch_client.clone(),
                    DimensionValue::PriorityExecutor,
                    receipt_status_to_metric(success, chain_id_u64),
                    1.0,
                );
                if let Some(metric_future) = metric_future {
                    // do not block current thread by awaiting in the background
                    send_metric_with_order_hash!(&order_hash, metric_future);
                }
            }

            if success {
                let balance_eth = self
                    .client
                    .get_balance(address)
                    .await
                    .map_or_else(|_| None, |v| Some(format_units(v, "ether").unwrap()));

                // TODO: use if-let chains when it becomes stable https://github.com/rust-lang/rust/issues/53667
                // if let Some(balance_eth) = balance_eth && let Some(cw) = &self.cloudwatch_client {
                if let Some(balance_eth) = balance_eth {
                    info!(
                        "{}- balance: {} at block {}",
                        order_hash,
                        balance_eth.clone(),
                        block_number.unwrap()
                    );
                    let metric_future = build_metric_future(
                        self.cloudwatch_client.clone(),
                        DimensionValue::PriorityExecutor,
                        CwMetrics::Balance(format!("{:?}", address)),
                        balance_eth.parse::<f64>().unwrap_or(0.0),
                    );
                    if let Some(metric_future) = metric_future {
                        send_metric_with_order_hash!(&order_hash, metric_future);
                    }
                }
            }

            Ok(())
        }.await;
        
        // Ensure key is released if it was acquired
        if let Some(addr) = public_address {
            match self.key_store.release_key(addr.clone()).await {
                Ok(_) => {
                    info!("{} - Released key: {}", order_hash, addr);
                }
                Err(e) => {
                    warn!("{} - Failed to release key: {}", order_hash, e);
                }
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{U256, U128, U64};
    use alloy::network::AnyNetwork;
    use alloy::providers::{DynProvider, Provider, RootProvider};
    use alloy::rpc::types::TransactionRequest;
    use crate::strategies::types::SubmitTxToMempoolWithExecutionMetadata;
    use crate::strategies::priority_strategy::ExecutionMetadata;
    use artemis_core::executors::mempool_executor::{SubmitTxToMempool, GasBidInfo};
    use std::sync::Arc;

    // Mock provider that implements the Provider trait
    #[derive(Clone)]
    struct MockProvider;
    impl Provider<AnyNetwork> for MockProvider {
        fn root(&self) -> &RootProvider<AnyNetwork> {
            unimplemented!("Mock provider does not support root provider")
        }
    }

    // Helper function to create a test action
    fn create_test_action(
        quote_size: U256,
        amount_required: U256,
        gas_estimate: U256,
        is_exact_output: bool,
        target_block: Option<u64>,
    ) -> SubmitTxToMempoolWithExecutionMetadata {
        let action = SubmitTxToMempoolWithExecutionMetadata {
            execution: SubmitTxToMempool {
                tx: WithOtherFields::new(TransactionRequest::default()),
                gas_bid_info: Some(GasBidInfo {
                    bid_percentage: U128::from(0),
                    total_profit: U128::from(0),
                }),
            },
            metadata: ExecutionMetadata {
                quote: quote_size,
                quote_eth: Some(quote_size),
                exact_output: is_exact_output,
                amount_required: amount_required,
                gas_use_estimate_quote: gas_estimate,
                order_hash: "test_hash".to_string(),
                target_block: target_block.map(|b| U64::from(b)),
                fallback_bid_scale_factor: Some(DEFAULT_FALLBACK_BID_SCALE_FACTOR),
                revert_protection: false,
            },
        };
        action
    }

    #[tokio::test]
    async fn test_get_bids_for_order_small_quote() {
        let executor = PriorityExecutor::new(
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(KeyStore::new()),
            None,
        );

        let action = create_test_action(
            U256::from(9e17), // quote: 0.9 ETH
            U256::from(8e17), // amount_required: 0.8 ETH
            U256::from(100000), // gas_estimate: 100k gas
            false,
            None,
        );

        let bids = executor.get_bids_for_order(&action, "test_hash");
        assert_eq!(bids.len(), 1 + 3); // 1 quote-based bid + minimum 3 fallback bids
        assert!(bids[0].is_some());
    }

    #[tokio::test]
    async fn test_get_bids_for_order_large_quote() {
        let executor = PriorityExecutor::new(
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(KeyStore::new()),
            None,
        );

        let action = create_test_action(
            U256::from(1000e18), // quote: 1000 ETH
            U256::from(900e18), // amount_required: 900 ETH
            U256::from(100000), // gas_estimate: 100k gas
            false,
            None,
        );

        let bids = executor.get_bids_for_order(&action, "test_hash");
        assert_eq!(bids.len(), 1 + 3 + 4); // 1 quote-based bid + 3 fallback bids + 4 additional fallback bids
    }

    #[tokio::test]
    async fn test_get_bids_for_order_exact_output() {
        let executor = PriorityExecutor::new(
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(KeyStore::new()),
            None,
        );

        let action = create_test_action(
            U256::from(7e18), // quote: 7 ETH
            U256::from(8e18), // amount_required: 8 ETH
            U256::from(1000), // gas_estimate: 1k gas
            true,
            None,
        );

        let bids = executor.get_bids_for_order(&action, "test_hash");
        assert_eq!(bids.len(), 1 + 3 + 1); // 1 quote-based bid + minimum 3 fallback bids + 1 additional fallback bid
        assert!(bids[0].is_some());
    }

    #[tokio::test]
    async fn test_get_bids_for_order_no_gas_estimate() {
        let executor = PriorityExecutor::new(
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(KeyStore::new()),
            None,
        );

        let action = create_test_action(
            U256::from(2e17), // quote: 0.2 ETH
            U256::from(1e17), // amount_required: 0.1 ETH
            U256::from(0), // gas_estimate: 0 gas
            false,
            None,
        );

        let bids = executor.get_bids_for_order(&action, "test_hash");
        assert_eq!(bids.len(), 3); // 3 fallback bids should still be generated
    }

    #[tokio::test]
    async fn test_get_bids_for_order_too_little_quote() {
        let executor = PriorityExecutor::new(
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(KeyStore::new()),
            None,
        );

        let action = create_test_action(
            U256::from(2e17), // quote: 0.2 ETH
            U256::from(3e17), // amount_required: 0.3 ETH
            U256::from(10000), // gas_estimate: 10000 gas
            false,
            None,
        );

        let bids = executor.get_bids_for_order(&action, "test_hash");
        assert_eq!(bids.len(), 0);
    }

    #[tokio::test]
    async fn test_extremely_large_quote() {
        let executor = PriorityExecutor::new(
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(KeyStore::new()),
            None,
        );

        let action = create_test_action(
            U256::from(1e30), // quote: 1e12 ETH
            U256::from(1e29), // amount_required: 1e11 ETH
            U256::from(100000), // gas_estimate: 100k gas
            false,
            None,
        );

        let bids = executor.get_bids_for_order(&action, "test_hash");
        assert_eq!(bids.len(), 1 + 8); // 1 quote-based bid + max of 8 additional fallback bids (based on underflow check)
    }

    #[tokio::test]
    async fn test_assign_nonces_with_revert_protection() {
        let executor = PriorityExecutor::new(
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(KeyStore::new()),
            None,
        );

        let mut tx_requests = vec![
            WithOtherFields::new(TransactionRequest::default()),
            WithOtherFields::new(TransactionRequest::default()),
            WithOtherFields::new(TransactionRequest::default()),
        ];

        // Set different priority fees to test sorting
        tx_requests[0].set_max_priority_fee_per_gas(100);
        tx_requests[1].set_max_priority_fee_per_gas(300);
        tx_requests[2].set_max_priority_fee_per_gas(200);

        executor.assign_nonces(&mut tx_requests, 42, true).await;

        // Verify all transactions have the same nonce (42)
        assert_eq!(tx_requests[0].nonce().unwrap(), 42);
        assert_eq!(tx_requests[1].nonce().unwrap(), 42);
        assert_eq!(tx_requests[2].nonce().unwrap(), 42);

        // Verify transactions are sorted by priority fee
        assert_eq!(tx_requests[0].max_priority_fee_per_gas().unwrap(), 300);
        assert_eq!(tx_requests[1].max_priority_fee_per_gas().unwrap(), 200);
        assert_eq!(tx_requests[2].max_priority_fee_per_gas().unwrap(), 100);
    }

    #[tokio::test]
    async fn test_assign_nonces_without_revert_protection() {
        let executor = PriorityExecutor::new(
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(KeyStore::new()),
            None,
        );

        let mut tx_requests = vec![
            WithOtherFields::new(TransactionRequest::default()),
            WithOtherFields::new(TransactionRequest::default()),
            WithOtherFields::new(TransactionRequest::default()),
        ];

        // Set different priority fees to test sorting
        tx_requests[0].set_max_priority_fee_per_gas(100);
        tx_requests[1].set_max_priority_fee_per_gas(300);
        tx_requests[2].set_max_priority_fee_per_gas(200);


        executor.assign_nonces(&mut tx_requests, 42, false).await;

        // Verify transactions have incrementing nonces starting from 42
        assert_eq!(tx_requests[0].nonce().unwrap(), 42);
        assert_eq!(tx_requests[1].nonce().unwrap(), 43);
        assert_eq!(tx_requests[2].nonce().unwrap(), 44);

        // Verify transactions are sorted by priority fee
        assert_eq!(tx_requests[0].max_priority_fee_per_gas().unwrap(), 300);
        assert_eq!(tx_requests[1].max_priority_fee_per_gas().unwrap(), 200);
        assert_eq!(tx_requests[2].max_priority_fee_per_gas().unwrap(), 100);
    }

    #[tokio::test]
    async fn test_assign_nonces_empty_requests() {
        let executor = PriorityExecutor::new(
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(DynProvider::new(MockProvider)),
            Arc::new(KeyStore::new()),
            None,
        );

        let mut tx_requests: Vec<WithOtherFields<TransactionRequest>> = vec![];

        // Should not panic with empty requests
        executor.assign_nonces(&mut tx_requests, 42, true).await;
        assert!(tx_requests.is_empty());
    }
}