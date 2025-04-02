use std::{cmp::max, str::FromStr, sync::Arc};
use tracing::{info, warn, debug};

use alloy::{
    eips::{BlockId, BlockNumberOrTag}, network::{
        AnyNetwork, EthereumWallet, ReceiptResponse, TransactionBuilder
    }, primitives::{utils::format_units, Address, U128, U256}, providers::{DynProvider, Provider}, rpc::types::TransactionRequest, serde::WithOtherFields, signers::{local::PrivateKeySigner, Signer}
};
use anyhow::{Context, Result};
use artemis_core::types::Executor;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;

use crate::{
    aws_utils::cloudwatch_utils::{
        build_metric_future, receipt_status_to_metric, CwMetrics, DimensionValue,
    }, executors::reactor_error_code::ReactorErrorCode, shared::send_metric_with_order_hash, strategies::{keystore::KeyStore, types::SubmitTxToMempoolWithExecutionMetadata}
};
use crate::executors::reactor_error_code::get_revert_reason;
use alloy_primitives::Uint;

const GAS_LIMIT: u64 = 1_000_000;
const MAX_RETRIES: u32 = 3;
const TX_BACKOFF_MS: u64 = 0; // retry immediately
static QUOTE_BASED_PRIORITY_BID_BUFFER: U256 = Uint::from_limbs([2, 0, 0, 0]);
static GWEI_PER_ETH: U256 = Uint::from_limbs([1_000_000_000, 0, 0, 0]);
const QUOTE_ETH_LOG10_THRESHOLD: usize = 6;
// The number of bps to add to the base bid for each fallback bid
const BID_SCALE_FACTOR: u64 = 50;

/// An executor that sends transactions to the public mempool.
pub struct Public1559Executor {
    client: Arc<DynProvider<AnyNetwork>>,
    sender_client: Arc<DynProvider<AnyNetwork>>,
    key_store: Arc<KeyStore>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
}

#[derive(Debug)]
enum TransactionOutcome {
    Success(Option<u64>),
    Failure(Option<u64>),
    RetryableFailure,
}

impl Public1559Executor {
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

    async fn send_transaction(
        &self,
        wallet: &EthereumWallet,
        tx_request: WithOtherFields<TransactionRequest>,
        order_hash: &str,
    ) -> Result<TransactionOutcome> {
        let tx_request_for_revert = tx_request.clone();
        let tx = tx_request.build(wallet).await?;
        let result = self.sender_client.send_tx_envelope(tx).await;

        match result {
            Ok(tx) => {
                info!("{} - Waiting for confirmations", order_hash);
                let receipt = tx
                    .with_required_confirmations(0)
                    .get_receipt()
                    .await
                    .map_err(|e| {
                        anyhow::anyhow!("{} - Error waiting for confirmations: {}", order_hash, e)
                    });
                
                match receipt {
                    Ok(receipt) => {
                        let status = receipt.status();
                        info!(
                            "{} - receipt: tx_hash: {:?}, status: {}",
                            order_hash, receipt.transaction_hash, status,
                        );
                        
                        if !status {
                            info!("{} - Attempting to get revert reason", order_hash);
                            // Parse revert reason
                            let revert_reason = get_revert_reason(
                                &self.sender_client,
                                tx_request_for_revert,
                                receipt.block_number.unwrap()
                            ).await;
                            
                            if let Ok(reason) = revert_reason {
                                info!("{} - Revert reason: {}", order_hash, reason);
                                // Retry if the order isn't yet fillable
                                if matches!(reason, ReactorErrorCode::OrderNotFillable) {
                                    return Ok(TransactionOutcome::RetryableFailure);
                                }
                                else {
                                    info!("{} - Order not fillable, returning failure", order_hash);
                                    return Ok(TransactionOutcome::Failure(receipt.block_number));
                                }
                            }
                            info!("{} - Failed to get revert reason - error: {:?}", order_hash, revert_reason.err().unwrap());
                            Ok(TransactionOutcome::Failure(None))
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
                Ok(TransactionOutcome::Failure(None))
            }
        }
    }
}

#[async_trait]
impl Executor<SubmitTxToMempoolWithExecutionMetadata> for Public1559Executor {
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempoolWithExecutionMetadata) -> Result<()> {
        let order_hash = Arc::new(action.metadata.order_hash.clone());
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
        let (public_address, private_key) = self
            .key_store
            .acquire_key()
            .await
            .expect("Failed to acquire key");
        info!("{} - Acquired key: {}", order_hash, public_address);

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
        let address = Address::from_str(&public_address).unwrap();

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

        // estimate_gas always fails because of target block being a future block
        /*
        let gas_usage_result = self
            .client
            .estimate_gas(&action.execution.tx)
            .await
            .or_else(|err| {
                if let Some(raw) = &err.as_error_resp().unwrap().data {
                    if let Ok(serde_value) = serde_json::from_str::<serde_json::Value>(raw.get()) {
                        if let serde_json::Value::String(four_byte) = serde_value {
                            let error_code = ReactorErrorCode::from(four_byte.clone());
                            match error_code {
                                ReactorErrorCode::OrderAlreadyFilled => {
                                    info!(
                                        "{} - Order already filled, skipping execution",
                                        order_hash
                                    );
                                    let metric_future = build_metric_future(
                                        self.cloudwatch_client.clone(),
                                        DimensionValue::PriorityExecutor,
                                        CwMetrics::ExecutionSkippedAlreadyFilled(chain_id_u64),
                                        1.0,
                                    );
                                    if let Some(metric_future) = metric_future {
                                        send_metric_with_order_hash!(&order_hash, metric_future);
                                    }
                                    Err(anyhow::anyhow!("Order Already Filled"))
                                }
                                ReactorErrorCode::InvalidDeadline => {
                                    info!(
                                        "{} - Order past deadline, skipping execution",
                                        order_hash
                                    );
                                    let metric_future = build_metric_future(
                                        self.cloudwatch_client.clone(),
                                        DimensionValue::PriorityExecutor,
                                        CwMetrics::ExecutionSkippedPastDeadline(chain_id_u64),
                                        1.0,
                                    );
                                    if let Some(metric_future) = metric_future {
                                        send_metric_with_order_hash!(&order_hash, metric_future);
                                    }
                                    Err(anyhow::anyhow!("Order Past Deadline"))
                                }
                                _ => Ok(GAS_LIMIT),
                            }
                        } else {
                            warn!("{} - Unexpected error data: {:?}", order_hash, serde_value);
                            Ok(GAS_LIMIT)
                        }
                    } else {
                        warn!("{} - Failed to parse error data: {:?}", order_hash, err);
                        Ok(GAS_LIMIT)
                    }
                } else {
                    warn!("{} - Error estimating gas: {:?}", order_hash, err);
                    Ok(GAS_LIMIT)
                }
            });

        let gas_usage = match gas_usage_result {
            Ok(gas) => gas,
            Err(e) => {
                warn!("{} - Error getting gas usage: {}", order_hash, e);
                // Release the key before returning
                match self.key_store.release_key(public_address.clone()).await {
                    Ok(_) => {
                        info!("{} - Released key: {}", order_hash, public_address);
                    }
                    Err(release_err) => {
                        warn!("{} - Failed to release key: {}", order_hash, release_err);
                    }
                }
                return Err(e);
            }
        };
        */

        let mut bid_priority_fees = vec![];
        let base_fee = self
            .client
            .get_gas_price()
            .await
            .context("Error getting gas price: {}")?;

        if let Some(gas_bid_info) = action.execution.gas_bid_info {
            // priority fee at which we'd break even, meaning 100% of profit goes to user in the form of price improvement
            if action.metadata.gas_use_estimate_quote > U256::from(0) {
                let quote_based_priority_bid = action
                    .metadata
                    .calculate_priority_fee_from_gas_use_estimate(QUOTE_BASED_PRIORITY_BID_BUFFER);
                bid_priority_fees.push(quote_based_priority_bid);
                info!("{} - quote_based_priority_bid: {:?}", order_hash, quote_based_priority_bid);
            }
            // If the quote is large in ETH, add more bids
            // < 1e7 gwei = 1 fallback bid, 1e8 = 2 fallback bids, 1e9 = 3 fallback bids, etc.
            let mut num_fallback_bids = 1;
            if action.metadata.quote_eth > U256::from(0) {
                debug!("{} - Adding fallback bids based on quote size", order_hash);
                let quote_in_gwei = &action.metadata.quote_eth / GWEI_PER_ETH;
                info!("{} - quote_eth_gwei: {:?}", order_hash, quote_in_gwei);
                
                if quote_in_gwei > U256::from(0) {
                    let quote_gwei_log10 = quote_in_gwei.log10();
                    info!("{} - quote_gwei_log10: {:?}", order_hash, quote_gwei_log10);
                    if quote_gwei_log10 > QUOTE_ETH_LOG10_THRESHOLD {
                        num_fallback_bids = quote_gwei_log10 - QUOTE_ETH_LOG10_THRESHOLD;
                    }
                }
            }
            for i in 0..num_fallback_bids {
                let gas_bid_bps = gas_bid_info.bid_percentage;
                let bid_bps = gas_bid_bps - U128::from(BID_SCALE_FACTOR * i as u64);
                let fallback_bid = action
                    .metadata
                    .calculate_priority_fee(bid_bps);
                bid_priority_fees.push(fallback_bid);
                info!("{} - fallback_bid_{}: {:?}", order_hash, i, fallback_bid);
            }
        } else {
            bid_priority_fees.push(Some(U256::from(50)));
        }

        if bid_priority_fees.len() == 0 {
            info!(
                "{} - No bid priority fees, indicating quote < amount_out_required; skipping",
                order_hash
            );
            // Release the key before returning
            match self.key_store.release_key(public_address.clone()).await {
                Ok(_) => {
                    info!("{} - Released key: {}", order_hash, public_address);
                }
                Err(release_err) => {
                    warn!("{} - Failed to release key: {}", order_hash, release_err);
                }
            }
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

        let sender_client = self.sender_client.clone();

        // Retry up to 3 times to get the nonce.
        let mut nonce = {
            let mut attempts = 0;
            loop {
                match sender_client.get_transaction_count(address).await {
                    Ok(nonce) => break nonce,
                    Err(e) => {
                        if attempts < 2 {
                            attempts += 1;
                        } else {
                            return Err(anyhow::anyhow!(
                                "{} - Failed to get nonce after 3 attempts: {}",
                                order_hash,
                                e
                            ));
                        }
                    }
                }
            }
        };

        // Set unique nonces for each transaction
        for tx_request in tx_requests.iter_mut() {
            tx_request.set_nonce(nonce);
            nonce += 1;
        }

        info!("{} - Executing {} transactions in parallel from {:?}", order_hash, tx_requests.len(), address);

        let mut attempts = 0;
        let mut success = false;
        let mut block_number = None;
        let mut retryable_failure = true;
        
        // Retry tx submission on retryable failures if none of the transactions succeeded
        while attempts < MAX_RETRIES && !success && retryable_failure {
            // Create futures for all transactions
            let futures: Vec<_> = tx_requests.iter().map(|tx_request| {
                self.send_transaction(&wallet, tx_request.clone(), &order_hash)
            }).collect();

            // Wait for all transactions to complete
            let results = futures::future::join_all(futures).await;

            // Check results
            retryable_failure = false;
            for (i, result) in results.iter().enumerate() {
                match result {
                    Ok(TransactionOutcome::Success(result)) => {
                        success = true;
                        block_number = *result;
                        info!("{} - Transaction {} succeeded at block {}", order_hash, i, block_number.unwrap());
                        break;
                    }
                    Ok(TransactionOutcome::Failure(result)) => {
                        if i == results.len() - 1 {
                            block_number = *result;
                        }
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

        // regardless of outcome, ensure we release the key
        match self.key_store.release_key(public_address.clone()).await {
            Ok(_) => {
                info!("{} - Released key: {}", order_hash, public_address);
            }
            Err(e) => {
                info!("{} - Failed to release key: {}", order_hash, e);
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
    }
}