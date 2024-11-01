use serde_json::Value;
use std::sync::Arc;
use tracing::{info, warn};

use anyhow::{Context, Result};
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
        receipt_status_to_metric, CwMetrics, DimensionName, DimensionValue, MetricBuilder,
        ARTEMIS_NAMESPACE,
    },
    executors::reactor_error_code::ReactorErrorCode,
    strategies::{keystore::KeyStore, types::SubmitTxToMempoolWithExecutionMetadata},
};

macro_rules! send_metric_with_order_hash {
    ($order_hash: expr, $future: expr) => {
        let hash = Arc::clone($order_hash);
        tokio::spawn(async move {
            if let Err(e) = $future.await {
                warn!("{} - error sending metric: {:?}", hash, e);
            }
        })
    };
}

/// An executor that sends transactions to the public mempool.
pub struct Public1559Executor<M, N> {
    client: Arc<M>,
    sender_client: Arc<N>,
    key_store: Arc<KeyStore>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
}

impl<M: Middleware, N: Middleware> Public1559Executor<M, N> {
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
impl<M, N> Executor<SubmitTxToMempoolWithExecutionMetadata> for Public1559Executor<M, N>
where
    M: Middleware + 'static,
    M::Error: 'static,
    N: Middleware + 'static,
    N::Error: 'static,
{
    /// Send a transaction to the mempool.
    async fn execute(&self, mut action: SubmitTxToMempoolWithExecutionMetadata) -> Result<()> {
        let order_hash = Arc::new(action.metadata.order_hash.clone());
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

        let wallet: LocalWallet = private_key
            .as_str()
            .parse::<LocalWallet>()
            .unwrap()
            .with_chain_id(chain_id);
        let address = wallet.address();

        action.execution.tx.set_from(address);

        // early return on OrderAlready filled
        // always use 1_000_000 gas for now
        let gas_usage_result: Result<U256, anyhow::Error> = self
            .client
            .estimate_gas(&action.execution.tx, None)
            .await
            .or_else(|err| {
                if let Some(Value::String(four_byte)) =
                    err.as_error_response().unwrap().data.clone()
                {
                    let error_code = ReactorErrorCode::from(four_byte.clone());
                    match error_code {
                        ReactorErrorCode::OrderAlreadyFilled => {
                            info!("{} - Order already filled, skipping execution", order_hash);
                            Err(anyhow::anyhow!("Order Already Filled"))
                        }
                        ReactorErrorCode::InvalidDeadline => {
                            info!("{} - Order past deadline, skipping execution", order_hash);
                            Err(anyhow::anyhow!("Order Past Deadline"))
                        }
                        _ => Ok(U256::from(1_000_000)),
                    }
                } else {
                    warn!("Error estimating gas: {:?}", err);
                    Ok(U256::from(1_000_000))
                }
            });

        let gas_usage = match gas_usage_result {
            Ok(gas) => gas,
            Err(e) => {
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

        action.execution.tx.set_gas(gas_usage);

        let bid_priority_fee;
        let base_fee: U256 = self
            .client
            .get_gas_price()
            .await
            .context("Error getting gas price: {}")?;
        
        // if let Some(gas_bid_info) = action.execution.gas_bid_info {
        //     // priority fee at which we'd break even, meaning 100% of profit goes to user in the form of price improvement
        //     // TODO: use gas estimate here
        //     bid_priority_fee = action
        //         .metadata
        //         .calculate_priority_fee(gas_bid_info.bid_percentage)
        // } else {
            bid_priority_fee = Some(U256::from(50));
        // }

        let eip1559_tx = action.execution.tx.as_eip1559_mut();
        if let Some(eip1559_tx) = eip1559_tx {
            eip1559_tx.max_fee_per_gas = Some(base_fee);
            eip1559_tx.max_priority_fee_per_gas = bid_priority_fee;
        } else {
            return Err(anyhow::anyhow!("Transaction is not EIP1559"));
        }

        action.execution.tx.set_gas(gas_usage);

        let sender_client = self.sender_client.clone();
        let nonce_manager = sender_client.nonce_manager(address);
        let signer = nonce_manager.with_signer(wallet);

        info!("{} - Executing tx from {:?}", order_hash, address);
        if let Some(cw) = &self.cloudwatch_client {
            let metric_future = cw
                .put_metric_data()
                .namespace(ARTEMIS_NAMESPACE)
                .metric_data(
                    MetricBuilder::new(CwMetrics::TxSubmitted)
                        .add_dimension(
                            DimensionName::Service.as_ref(),
                            DimensionValue::PriorityExecutor.as_ref(),
                        )
                        .with_value(1.0)
                        .build(),
                )
                .send();

            // do not block current thread by awaiting in the background
            send_metric_with_order_hash!(&order_hash, metric_future);
        }
        let result = signer.send_transaction(action.execution.tx, None).await;

        // Block on pending transaction getting confirmations
        let (receipt, status) = match result {
            Ok(tx) => {
                let receipt = tx.confirmations(1).await.map_err(|e| {
                    anyhow::anyhow!("{} - Error waiting for confirmations: {}", order_hash, e)
                });
                match receipt {
                    Ok(Some(receipt)) => {
                        let status = receipt.status.unwrap_or_default();
                        info!(
                            "{} - receipt: tx_hash: {:?}, status: {}",
                            order_hash, receipt.transaction_hash, status,
                        );
                        (Some(receipt), status)
                    }
                    Ok(None) => {
                        warn!("{} - No receipt after confirmation", order_hash);
                        (None, ethers::types::U64::zero())
                    }
                    Err(e) => {
                        warn!("{} - Error waiting for confirmations: {}", order_hash, e);
                        (None, ethers::types::U64::zero())
                    }
                }
            }
            Err(e) => {
                warn!("{} - Error sending transaction: {}", order_hash, e);
                (None, ethers::types::U64::zero())
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
        if let Some(cw) = &self.cloudwatch_client {
            let metric_future = cw
                .put_metric_data()
                .namespace(ARTEMIS_NAMESPACE)
                .metric_data(
                    MetricBuilder::new(receipt_status_to_metric(status.as_u64()))
                        .add_dimension(
                            DimensionName::Service.as_ref(),
                            DimensionValue::PriorityExecutor.as_ref(),
                        )
                        .with_value(1.0)
                        .build(),
                )
                .send();

            send_metric_with_order_hash!(&order_hash, metric_future);
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
                    "{}- balance: {} at block {}",
                    order_hash,
                    balance_eth.clone(),
                    block_number.as_u64()
                );
                if let Some(cw) = &self.cloudwatch_client {
                    let metric_future = cw
                        .put_metric_data()
                        .namespace(ARTEMIS_NAMESPACE)
                        .metric_data(
                            MetricBuilder::new(CwMetrics::Balance(format!("{:?}", address))) // {:?} gives the full 0x-prefixed address
                                .add_dimension(
                                    DimensionName::Service.as_ref(),
                                    DimensionValue::PriorityExecutor.as_ref(),
                                )
                                .with_value(balance_eth.parse::<f64>().unwrap_or(0.0))
                                .build(),
                        )
                        .send();
                    tokio::spawn(async move {
                        if let Err(e) = metric_future.await {
                            warn!("{} - error sending metric: {:?}", order_hash, e);
                        }
                    });
                }
            }
        }

        Ok(())
    }
}
