use std::{str::FromStr, sync::Arc};
use tracing::{info, warn, debug};

use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    network::{AnyNetwork, EthereumWallet, TransactionBuilder},
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
        build_metric_future, receipt_status_to_metric, CwMetrics, DimensionValue
    }, 
    executors::{
        bundle_client::BundleClient,
        transaction_utils::{poll_for_receipt, process_receipt, handle_send_error, TransactionOutcome},
    },
    shared::{get_nonce_with_retry, send_metric_with_order_hash, u256},
    strategies::{keystore::KeyStore, types::SubmitTxToMempoolWithExecutionMetadata}
};

const GAS_LIMIT: u64 = 1_000_000;
const MAX_RETRIES: u32 = 3;
const TX_BACKOFF_MS: u64 = 0; // retry immediately
static QUOTE_BASED_PRIORITY_BID_BUFFER: U256 = u256!(2);
static GWEI_PER_ETH: U256 = u256!(1_000_000_000);
const QUOTE_ETH_LOG10_THRESHOLD: usize = 8;
// The number of bps to add to the base bid for each fallback bid
const DEFAULT_FALLBACK_BID_SCALE_FACTOR: u64 = 50;
const CONFIRMATION_TIMEOUT_SEC: u64 = 10;
const RECEIPT_POLL_INTERVAL_MS: u64 = 250;
// The number of blocks past the target block the bundle is valid for
const TARGET_BLOCK_BUNDLE_WINDOW: u64 = 4;

const UNICHAIN_ID: u64 = 130;

/// An executor that sends transactions to the public mempool.
pub struct PriorityExecutor {
    client: Arc<DynProvider<AnyNetwork>>,
    sender_client: Arc<DynProvider<AnyNetwork>>,
    key_store: Arc<KeyStore>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
    bundle_client: BundleClient,
}

impl PriorityExecutor {
    pub fn new(
        client: Arc<DynProvider<AnyNetwork>>,
        sender_client: Arc<DynProvider<AnyNetwork>>,
        key_store: Arc<KeyStore>,
        cloudwatch_client: Option<Arc<CloudWatchClient>>,
    ) -> Self {
        let bundle_client = BundleClient::new(sender_client.clone());
        Self {
            client,
            sender_client,
            key_store,
            cloudwatch_client,
            bundle_client,
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
                send_metric_with_order_hash!(order_hash, metric_future);
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
        let tx_envelope = tx_request.build(wallet).await?;
        info!("{} - Sending transaction to RPC", order_hash);
        
        match self.sender_client.send_tx_envelope(tx_envelope.clone()).await {
            Ok(tx) => {
                info!("{} - Waiting for confirmations", order_hash);
                let receipt = match tokio::time::timeout(
                    std::time::Duration::from_secs(CONFIRMATION_TIMEOUT_SEC),
                    tx.with_required_confirmations(0).get_receipt()
                ).await {
                    Ok(receipt_result) => receipt_result.ok(),
                    Err(_) => {
                        warn!("{} - Timed out waiting for transaction receipt", order_hash);
                        return Ok(TransactionOutcome::Failure(None));
                    }
                };
                
                // Process receipt
                process_receipt(
                    receipt,
                    &self.client,
                    tx_request_for_revert,
                    order_hash,
                    chain_id,
                    target_block,
                    self.cloudwatch_client.clone(),
                ).await
            }
            Err(e) => {
                handle_send_error(e.into(), &self.sender_client, wallet, &tx_request_for_revert, order_hash).await
            }
        }
    }
    
    async fn send_bundle(
        &self,
        wallet: &EthereumWallet,
        tx_request: WithOtherFields<TransactionRequest>,
        order_hash: &str,
        chain_id: u64,
        target_block: u64,
    ) -> Result<TransactionOutcome> {
        let tx_request_for_revert = tx_request.clone();
        
        // Send bundle using bundle client
        let (tx_envelope, _bundle_hash) = match self.bundle_client
            .send_bundle(wallet, tx_request, target_block, TARGET_BLOCK_BUNDLE_WINDOW, order_hash)
            .await {
                Ok(result) => result,
                Err(e) => {
                    return handle_send_error(e, &self.sender_client, wallet, &tx_request_for_revert, order_hash).await;
                }
            };
        
        // Poll for transaction receipt
        let receipt = match tokio::time::timeout(
            std::time::Duration::from_secs(CONFIRMATION_TIMEOUT_SEC),
            poll_for_receipt(&self.sender_client, &self.client, &tx_envelope, target_block, order_hash, CONFIRMATION_TIMEOUT_SEC, RECEIPT_POLL_INTERVAL_MS)
        ).await {
            Ok(receipt_result) => receipt_result?,
            Err(_) => {
                warn!("{} - Timed out waiting for transaction receipt", order_hash);
                return Ok(TransactionOutcome::Failure(None));
            }
        };
        
        // Process receipt
        process_receipt(
            receipt,
            &self.client,
            tx_request_for_revert,
            order_hash,
            chain_id,
            Some(target_block),
            self.cloudwatch_client.clone(),
        ).await
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
                .context("Chain ID not found on transaction")?;

            let metric_future = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::PriorityExecutor,
                CwMetrics::ExecutionAttempted(chain_id_u64),
                1.0,
            );
            if let Some(metric_future) = metric_future {
                send_metric_with_order_hash!(&order_hash, metric_future);
            }
            // send keystore metrics
            let keys_in_use = self.key_store.get_keys_in_use();
            let keys_available = self.key_store.get_keys_available();

            let keys_in_use_future = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::PriorityExecutor,
                CwMetrics::KeysInUse(chain_id_u64),
                keys_in_use as f64,
            );
            if let Some(metric_future) = keys_in_use_future {
                send_metric_with_order_hash!(&order_hash, metric_future);
            }

            let keys_available_future = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::PriorityExecutor,
                CwMetrics::KeysAvailable(chain_id_u64),
                keys_available as f64,
            );
            if let Some(metric_future) = keys_available_future {
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

            let chain_id = chain_id_u64;

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
                .context("Error getting gas price")?;
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
            let mut nonce = get_nonce_with_retry(&self.client, address, &order_hash, 3).await?;
            info!("{} - Nonce: {}", order_hash, nonce);

            // Sort transactions by max_priority_fee_per_gas in descending order so that the highest bid is first
            tx_requests.sort_by(|a, b| {
                let a_fee = a.max_priority_fee_per_gas().unwrap();
                let b_fee = b.max_priority_fee_per_gas().unwrap();
                b_fee.cmp(&a_fee)
            });

            // Set unique nonces for each transaction
            for tx_request in tx_requests.iter_mut() {
                tx_request.set_nonce(nonce);
                nonce += 1;
            }

            let metric_future = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::PriorityExecutor,
                CwMetrics::OrderBid(chain_id_u64),
                1.0,
            );
            if let Some(metric_future) = metric_future {
                send_metric_with_order_hash!(&order_hash, metric_future);
            }
            info!("{} - Executing {} transactions in parallel from {:?} using {} (chain_id: {})", 
                order_hash, 
                tx_requests.len(), 
                address,
                if chain_id_u64 == UNICHAIN_ID { "bundle sending" } else { "direct transaction sending" },
                chain_id_u64
            );

            let mut attempts = 0;
            let mut success = false;
            let mut block_number = None;
            let mut retryable_failure = true;
            
            // Retry tx submission on retryable failures if none of the transactions succeeded
            while attempts < MAX_RETRIES && !success && retryable_failure {

                let metric_future = build_metric_future(
                    self.cloudwatch_client.clone(),
                    DimensionValue::PriorityExecutor,
                    CwMetrics::TxSubmitted(chain_id),
                    1.0,
                );
                if let Some(metric_future) = metric_future {
                    send_metric_with_order_hash!(&order_hash, metric_future);
                }

                // Create futures for all transactions
                let futures: Vec<_> = tx_requests.iter().map(|tx_request| {
                    // Use send_bundle for Unichain, send_transaction for Base
                    if chain_id_u64 == UNICHAIN_ID {
                        Box::pin(self.send_bundle(&wallet, tx_request.clone(), &order_hash, chain_id_u64, target_block.as_u64().unwrap()))
                            as std::pin::Pin<Box<dyn std::future::Future<Output = Result<TransactionOutcome>> + Send>>
                    } else {
                        Box::pin(self.send_transaction(&wallet, tx_request.clone(), &order_hash, chain_id_u64, target_block.as_u64()))
                            as std::pin::Pin<Box<dyn std::future::Future<Output = Result<TransactionOutcome>> + Send>>
                    }
                }).collect();

                // Wait for all transactions to complete
                let results = futures::future::join_all(futures).await;

                // Check results
                retryable_failure = false;
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
                            // Find the transaction that won the bid and compare the winning bid to our own bid
                            
                        }
                        Ok(TransactionOutcome::RetryableFailure) => {
                            retryable_failure = true;
                            // Continue to next attempt
                        }
                        Err(_) => {
                            // Continue to next attempt
                        }
                    }
                }

                if !success && attempts < MAX_RETRIES - 1 {
                    attempts += 1;
                    info!(
                        "{} - All transactions failed, retrying in {}ms (attempt {}/{})",
                        order_hash, TX_BACKOFF_MS, attempts, MAX_RETRIES
                    );
                    // Update nonces for next attempt
                    for tx_request in tx_requests.iter_mut() {
                        tx_request.set_nonce(nonce);
                        nonce += 1;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(TX_BACKOFF_MS)).await;
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
}