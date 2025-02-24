use std::{str::FromStr, sync::Arc};
use tracing::{info, warn};

use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    network::{
        AnyNetwork, EthereumWallet, ReceiptResponse, TransactionBuilder,
    },
    primitives::{utils::format_units, Address, U256},
    providers::{DynProvider, Provider},
    rpc::types::TransactionReceipt,
    serde::WithOtherFields,
    signers::{local::PrivateKeySigner, Signer},
};
use anyhow::{Context, Result};
use artemis_core::types::Executor;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;

use crate::{
    aws_utils::cloudwatch_utils::{
        build_metric_future, receipt_status_to_metric, CwMetrics, DimensionValue,
    },
    shared::send_metric_with_order_hash,
    strategies::{keystore::KeyStore, types::SubmitTxToMempoolWithExecutionMetadata},
};

const GAS_LIMIT: u64 = 1_000_000;

/// An executor that sends transactions to the public mempool.
pub struct Public1559Executor {
    client: Arc<DynProvider<AnyNetwork>>,
    sender_client: Arc<DynProvider<AnyNetwork>>,
    key_store: Arc<KeyStore>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
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

        let bid_priority_fee;
        let base_fee = self
            .client
            .get_gas_price()
            .await
            .context("Error getting gas price: {}")?;

        if let Some(gas_bid_info) = action.execution.gas_bid_info {
            // priority fee at which we'd break even, meaning 100% of profit goes to user in the form of price improvement
            // TODO: use gas estimate here
            bid_priority_fee = action
                .metadata
                .calculate_priority_fee(gas_bid_info.bid_percentage)
        } else {
            bid_priority_fee = Some(U256::from(50));
        }

        if bid_priority_fee.is_none() {
            info!(
                "{} - No bid priority fee, indicating quote < amount_out_required; skipping",
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

        let mut tx_request = action.execution.tx.clone();
        tx_request.set_gas_limit(GAS_LIMIT);
        tx_request.set_max_fee_per_gas(base_fee);
        tx_request.set_max_priority_fee_per_gas(bid_priority_fee.unwrap().to());

        let sender_client = self.sender_client.clone();

        // Retry up to 3 times to get the nonce.
        let nonce = {
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
        tx_request.set_nonce(nonce);
        info!("{} - Executing tx from {:?}", order_hash, address);
        let tx = tx_request.build(&wallet).await?;
        let result = sender_client.send_tx_envelope(tx).await;

        //let metric_future = build_metric_future(
        //    self.cloudwatch_client.clone(),
        //    DimensionValue::PriorityExecutor,
        //    CwMetrics::TxSubmitted(chain_id_u64),
        //    1.0,
        //);
        //if let Some(metric_future) = metric_future {
        //    // do not block current thread by awaiting in the background
        //    send_metric_with_order_hash!(&order_hash, metric_future);
        //}

        // Block on pending transaction getting confirmations
        let (receipt, status) = match result {
            Ok(tx) => {
                info!("{} - Waiting for confirmations", order_hash);
                let receipt = tx
                    .with_required_confirmations(1)
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
                        (Some(receipt), status)
                    }
                    Err(e) => {
                        warn!("{} - Error waiting for confirmations: {}", order_hash, e);
                        (None, false)
                    }
                }
            }
            Err(e) => {
                warn!("{} - Error sending transaction: {}", order_hash, e);
                (None, false)
            }
        };

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
        // TODO: parse revert reason
        if let Some(_) = &self.cloudwatch_client {
            let metric_future = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::PriorityExecutor,
                receipt_status_to_metric(status, chain_id_u64),
                1.0,
            );
            if let Some(metric_future) = metric_future {
                // do not block current thread by awaiting in the background
                send_metric_with_order_hash!(&order_hash, metric_future);
            }
        }

        if let Some(WithOtherFields {
            inner:
                TransactionReceipt::<_> {
                    block_number: Some(block_number),
                    ..
                },
            ..
        }) = receipt
        {
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
                    block_number
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
