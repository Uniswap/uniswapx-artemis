use alloy_primitives::{utils::format_units, Address, U128};
use std::{str::FromStr, sync::Arc};
use tracing::{info, warn};

use alloy::{
    network::{AnyNetwork, EthereumWallet, ReceiptResponse, TransactionBuilder},
    providers::{DynProvider, Provider},
    rpc::types::TransactionReceipt,
    serde::WithOtherFields,
    signers::{local::PrivateKeySigner, Signer},
};
use anyhow::Result;
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
use artemis_core::types::Executor;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;

use crate::{
    aws_utils::cloudwatch_utils::{
        build_metric_future, receipt_status_to_metric, revert_code_to_metric, CwMetrics, DimensionValue
    }, executors::reactor_error_code::{get_revert_reason, ReactorErrorCode}, send_metric, shared::get_nonce_with_retry, strategies::keystore::KeyStore
};

const GAS_LIMIT: u64 = 1_000_000;

/// An executor that sends transactions to the mempool.
pub struct DutchExecutor {
    client: Arc<DynProvider<AnyNetwork>>,
    sender_client: Arc<DynProvider<AnyNetwork>>,
    key_store: Arc<KeyStore>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
}

impl DutchExecutor {
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
impl Executor<SubmitTxToMempool> for DutchExecutor {
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempool) -> Result<()> {
        let chain_id = action
            .tx
            .chain_id()
            .expect("Chain ID not found on transaction")
            .to_string()
            .parse::<u64>()
            .unwrap();
        let metric_future = build_metric_future(
            self.cloudwatch_client.clone(),
            DimensionValue::V3Executor,
            CwMetrics::ExecutionAttempted(chain_id),
            1.0,
        );
        if let Some(metric_future) = metric_future {
            send_metric!(metric_future);
        }

        // send keystore metrics
        let keys_in_use = self.key_store.get_keys_in_use();
        let keys_available = self.key_store.get_keys_available();

        let keys_in_use_future = build_metric_future(
            self.cloudwatch_client.clone(),
            DimensionValue::V3Executor,
            CwMetrics::KeysInUse(chain_id),
            keys_in_use as f64,
        );
        if let Some(metric_future) = keys_in_use_future {
            send_metric!(metric_future);
        }

        let keys_available_future = build_metric_future(
            self.cloudwatch_client.clone(),
            DimensionValue::V3Executor,
            CwMetrics::KeysAvailable(chain_id),
            keys_available as f64,
        );
        if let Some(metric_future) = keys_available_future {
            send_metric!(metric_future);
        }

        // Acquire a key from the key store
        let (addr, private_key) = self
            .key_store
            .acquire_key()
            .await
            .expect("Failed to acquire key");
        info!("Acquired key: {}", addr);

        let chain_id = u64::from_str_radix(
            &action
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
        action.tx.set_from(address);


        // Retry up to 3 times to get the nonce.
        let nonce = get_nonce_with_retry(&self.client, address, "", 3).await?;
        action.tx.set_nonce(nonce);
        action.tx.set_gas_limit(GAS_LIMIT);

        let gas_usage_result = self.client.estimate_gas(&action.tx).await.or_else(|err| {
            if let Some(raw) = &err.as_error_resp().unwrap().data {
                if let Ok(serde_value) = serde_json::from_str::<serde_json::Value>(raw.get()) {
                    if let serde_json::Value::String(four_byte) = serde_value {
                        let error_code = ReactorErrorCode::from(four_byte.clone());
                        match error_code {
                            ReactorErrorCode::OrderAlreadyFilled => {
                                info!("Order already filled, skipping execution");
                                let metric_future = build_metric_future(
                                    self.cloudwatch_client.clone(),
                                    DimensionValue::V3Executor,
                                    CwMetrics::ExecutionSkippedAlreadyFilled(chain_id),
                                    1.0,
                                );
                                if let Some(metric_future) = metric_future {
                                    send_metric!(metric_future);
                                }
                                Err(anyhow::anyhow!("Order Already Filled"))
                            }
                            ReactorErrorCode::InvalidDeadline => {
                                info!("Order past deadline, skipping execution");
                                let metric_future = build_metric_future(
                                    self.cloudwatch_client.clone(),
                                    DimensionValue::V3Executor,
                                    CwMetrics::ExecutionSkippedPastDeadline(chain_id),
                                    1.0,
                                );
                                if let Some(metric_future) = metric_future {
                                    send_metric!(metric_future);
                                }
                                Err(anyhow::anyhow!("Order Past Deadline"))
                            }
                            _ => Ok(1_000_000),
                        }
                    } else {
                        warn!("Unexpected error data: {:?}", serde_value);
                        Ok(1_000_000)
                    }
                } else {
                    warn!("Error estimating gas: {:?}", err);
                    Ok(1_000_000)
                }
            } else {
                warn!("Error estimating gas: {:?}", err);
                Ok(1_000_000)
            }
        });
        info!("Gas Usage {:?}", gas_usage_result);
        let gas_usage = gas_usage_result.unwrap_or(1_000_000);

        let bid_gas_price;
        if let Some(gas_bid_info) = action.gas_bid_info {
            // gas price at which we'd break even, meaning 100% of profit goes to validator
            let breakeven_gas_price = gas_bid_info.total_profit / U128::from(gas_usage);
            // gas price corresponding to bid percentage
            bid_gas_price = breakeven_gas_price * gas_bid_info.bid_percentage / U128::from(100);
        } else {
            bid_gas_price = self
                .client
                .get_gas_price()
                .await
                .map_or_else(|_| U128::from(1), |v| U128::from(v));
        }
        info!("bid_gas_price: {}", bid_gas_price);
        action.tx.set_gas_price(bid_gas_price.to());

        info!("Executing tx {:?}", action.tx);
        let chain_id = action
            .tx
            .chain_id()
            .expect("Chain ID not found on transaction")
            .to_string()
            .parse::<u64>()
            .unwrap();

        let send_metric_if_some = |metric| {
            if let Some(metric_future) = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::V3Executor,
                metric,
                1.0,
            ) {
                send_metric!(metric_future);
            }
        };

        send_metric_if_some(CwMetrics::OrderBid(chain_id));
        send_metric_if_some(CwMetrics::TxSubmitted(chain_id));

        let tx_request_for_revert = action.tx.clone();
        let tx = action.tx.build(&wallet).await?;
        let result = self.sender_client.send_tx_envelope(tx).await;

        // Block on pending transaction getting confirmations
        let (receipt, status) = match result {
            Ok(tx) => {
                let receipt = tx
                    .with_required_confirmations(1)
                    .get_receipt()
                    .await
                    .map_err(|e| anyhow::anyhow!("Error waiting for confirmations: {}", e));
                match receipt {
                    Ok(receipt) => {
                        let status = receipt.status();
                        info!(
                            "receipt: tx_hash: {:?}, status: {}",
                            receipt.transaction_hash, status,
                        );

                        if !status && receipt.block_number.is_some() {
                            info!("Attempting to get revert reason");
                            // Parse revert reason
                            match get_revert_reason(&self.client, tx_request_for_revert, receipt.block_number.unwrap()).await {
                                Ok(reason) => {
                                    info!("Revert reason: {}", reason);
                                    let metric_future = build_metric_future(
                                        self.cloudwatch_client.clone(),
                                        DimensionValue::V3Executor,
                                        revert_code_to_metric(chain_id, reason.to_string()),
                                        1.0,
                                    );
                                    if let Some(metric_future) = metric_future {
                                        // do not block current thread by awaiting in the background
                                        send_metric!(metric_future);
                                    }
                                }
                                Err(e) => {
                                    info!("Failed to get revert reason - error: {:?}", e);
                                }
                            }
                        }
                        else {
                            let send_metric_if_some = |metric| {
                                if let Some(metric_future) = build_metric_future(
                                    self.cloudwatch_client.clone(),
                                    DimensionValue::V3Executor,
                                    metric,
                                    1.0,
                                ) {
                                    send_metric!(metric_future);
                                }
                            };

                            send_metric_if_some(CwMetrics::OrderFilled(chain_id));
                            send_metric_if_some(CwMetrics::TxSucceeded(chain_id));
                            info!("Transaction succeeded");
                        }

                        (Some(receipt), status)
                    }
                    Err(e) => {
                        warn!("Error waiting for confirmations: {}", e);
                        (None, false)
                    }
                }
            }
            Err(e) => {
                warn!("Error sending transaction: {}", e);
                (None, false)
            }
        };

        match self.key_store.release_key(addr.clone()).await {
            Ok(_) => {
                info!("Released key: {}", addr);
            }
            Err(e) => {
                info!("Failed to release key: {}", e);
            }
        }

        // post key-release processing
        // TODO: parse revert reason
        if let Some(_) = &self.cloudwatch_client {
            let metric_future = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::V3Executor,
                receipt_status_to_metric(status, chain_id),
                1.0,
            );
            if let Some(metric_future) = metric_future {
                // do not block current thread by awaiting in the background
                send_metric!(metric_future);
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
                info!("balance: {} at block {}", balance_eth.clone(), block_number);
                let metric_future = build_metric_future(
                    self.cloudwatch_client.clone(),
                    DimensionValue::V3Executor,
                    CwMetrics::Balance(format!("{:?}", address)),
                    balance_eth.parse::<f64>().unwrap_or(0.0),
                );
                if let Some(metric_future) = metric_future {
                    send_metric!(metric_future);
                }
            }
        }

        Ok(())
    }
}
