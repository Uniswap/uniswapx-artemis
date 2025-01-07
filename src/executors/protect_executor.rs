use serde_json::Value;
use std::{
    ops::{Div, Mul},
    sync::Arc,
};
use tracing::{info, warn};

use anyhow::Result;
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
use artemis_core::types::Executor;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use ethers::{
    middleware::MiddlewareBuilder,
    providers::{Middleware, MiddlewareError},
    signers::{LocalWallet, Signer},
    types::{TransactionReceipt, U256},
    utils::format_units,
};

use crate::{
    aws_utils::cloudwatch_utils::{
        build_metric_future, receipt_status_to_metric, CwMetrics, DimensionValue,
    },
    executors::reactor_error_code::ReactorErrorCode,
    send_metric,
    strategies::keystore::KeyStore,
};

/// An executor that sends transactions to the mempool.
pub struct ProtectExecutor<M, N> {
    client: Arc<M>,
    sender_client: Arc<N>,
    key_store: Arc<KeyStore>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
}

impl<M: Middleware, N: Middleware> ProtectExecutor<M, N> {
    pub fn new(
        client: Arc<M>,
        sender_client: Arc<N>,
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
impl<M, N> Executor<SubmitTxToMempool> for ProtectExecutor<M, N>
where
    M: Middleware,
    M::Error: 'static,
    N: Middleware + 'static,
    N::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempool) -> Result<()> {
        let metric_future = build_metric_future(
            self.cloudwatch_client.clone(),
            DimensionValue::V3Executor,
            CwMetrics::ExecutionAttempted,
            1.0,
        );
        if let Some(metric_future) = metric_future {
            send_metric!(metric_future);
        }

        // Acquire a key from the key store
        let (public_address, private_key) = self
            .key_store
            .acquire_key()
            .await
            .expect("Failed to acquire key");
        info!("Acquired key: {}", public_address);

        let chain_id = u64::from_str_radix(
            &action
                .tx
                .chain_id()
                .expect("Chain ID not found on transaction")
                .to_string(),
            10,
        )
        .expect("Failed to parse chain ID");

        let wallet: LocalWallet = private_key
            .as_str()
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id);
        let address = wallet.address();
        action.tx.set_from(address);
        let gas_usage_result = self
            .client
            .estimate_gas(&action.tx, None)
            .await
            .or_else(|err| {
                if let Some(Value::String(four_byte)) =
                    err.as_error_response().unwrap().data.clone()
                {
                    let error_code = ReactorErrorCode::from(four_byte.clone());
                    match error_code {
                        ReactorErrorCode::OrderAlreadyFilled => {
                            info!("Order already filled, skipping execution");
                            let metric_future = build_metric_future(
                                self.cloudwatch_client.clone(),
                                DimensionValue::V3Executor,
                                CwMetrics::ExecutionSkippedAlreadyFilled,
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
                                CwMetrics::ExecutionSkippedPastDeadline,
                                1.0,
                            );
                            if let Some(metric_future) = metric_future {
                                send_metric!(metric_future);
                            }
                            Err(anyhow::anyhow!("Order Past Deadline"))
                        }
                        _ => Ok(U256::from(1_000_000)),
                    }
                } else {
                    warn!("Error estimating gas: {:?}", err);
                    Ok(U256::from(1_000_000))
                }
            });
        info!("Gas Usage {:?}", gas_usage_result);
        let gas_usage = gas_usage_result;

        let bid_gas_price;
        if let Some(gas_bid_info) = action.gas_bid_info {
            // gas price at which we'd break even, meaning 100% of profit goes to validator
            let breakeven_gas_price = gas_bid_info.total_profit / gas_usage?;
            // gas price corresponding to bid percentage
            bid_gas_price = breakeven_gas_price
                .mul(gas_bid_info.bid_percentage)
                .div(100);
        } else {
            bid_gas_price = self
                .client
                .get_gas_price()
                .await
                .map_err(|err| anyhow::anyhow!("Error getting gas price: {}", err))?;
        }
        info!("bid_gas_price: {}", bid_gas_price);
        action.tx.set_gas_price(bid_gas_price);

        let sender_client = self.sender_client.clone();
        let nonce_manager = sender_client.nonce_manager(address);
        let signer = nonce_manager.with_signer(wallet);

        info!("Executing tx {:?}", action.tx);
        let metric_future = build_metric_future(
            self.cloudwatch_client.clone(),
            DimensionValue::V3Executor,
            CwMetrics::TxSubmitted,
            1.0,
        );
        if let Some(metric_future) = metric_future {
            // do not block current thread by awaiting in the background
            send_metric!(metric_future);
        }
        let result = signer.send_transaction(action.tx, None).await;

        // Block on pending transaction getting confirmations
        let (receipt, status) = match result {
            Ok(tx) => {
                let receipt = tx
                    .confirmations(1)
                    .await
                    .map_err(|e| anyhow::anyhow!("Error waiting for confirmations: {}", e));
                match receipt {
                    Ok(Some(receipt)) => {
                        let status = receipt.status.unwrap_or_default();
                        info!(
                            "receipt: tx_hash: {:?}, status: {}",
                            receipt.transaction_hash, status,
                        );
                        (Some(receipt), status)
                    }
                    Ok(None) => {
                        warn!("No receipt after confirmation");
                        (None, ethers::types::U64::zero())
                    }
                    Err(e) => {
                        warn!("Error waiting for confirmations: {}", e);
                        (None, ethers::types::U64::zero())
                    }
                }
            }
            Err(e) => {
                warn!("Error sending transaction: {}", e);
                (None, ethers::types::U64::zero())
            }
        };

        match self.key_store.release_key(public_address.clone()).await {
            Ok(_) => {
                info!("Released key: {}", public_address);
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
                receipt_status_to_metric(status.as_u64()),
                1.0,
            );
            if let Some(metric_future) = metric_future {
                // do not block current thread by awaiting in the background
                send_metric!(metric_future);
            }
        }

        if let Some(TransactionReceipt {
            block_number: Some(block_number),
            ..
        }) = receipt
        {
            let balance_eth = self
                .client
                .get_balance(address, Some(block_number.into()))
                .await
                .map_or_else(|_| None, |v| Some(format_units(v, "ether").unwrap()));

            // TODO: use if-let chains when it becomes stable https://github.com/rust-lang/rust/issues/53667
            // if let Some(balance_eth) = balance_eth && let Some(cw) = &self.cloudwatch_client {
            if let Some(balance_eth) = balance_eth {
                info!(
                    "balance: {} at block {}",
                    balance_eth.clone(),
                    block_number.as_u64()
                );
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
