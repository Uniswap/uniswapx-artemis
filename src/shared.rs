use std::sync::Arc;

use alloy::{
    network::{AnyNetwork, EthereumWallet, ReceiptResponse, TransactionBuilder},
    providers::{DynProvider, Provider},
    rpc::types::TransactionRequest,
    serde::WithOtherFields,
};
use alloy_primitives::{Address, U256};
use serde::Deserialize;

// Centralized environment variable functions
pub fn get_reactor_address(strategy_type: &str) -> String {
    let env_var = format!("REACTOR_ADDRESS_{}", strategy_type.to_uppercase());
    std::env::var(&env_var)
        .or_else(|_| std::env::var("REACTOR_ADDRESS"))
        .unwrap_or_else(|_| {
            match strategy_type {
                "limit" => "0x5F88087fbc0c47e9aC7Dbda8Bb561127735EEC87",
                "dutchv3" => "0xB274d5F4b833b61B340b654d600A864fB604a87c",
                "uniswap" => "0x00000011F84B9aa48e5f8aA8B9897600006289Be",
                "priority" => "0x000000001Ec5656dcdB24D90DFa42742738De729",
                _ => "0x5F88087fbc0c47e9aC7Dbda8Bb561127735EEC87", // default
            }
            .to_string()
        })
}

pub fn get_routing_api() -> String {
    std::env::var("ROUTING_API")
        .unwrap_or_else(|_| "https://router-dev.mimboku.com/quote".to_string())
}

pub fn get_uniswapx_api_url() -> String {
    std::env::var("MIMBOKU_API_URL").unwrap_or_else(|_| {
        "https://j6nfzv9j4e.execute-api.us-east-1.amazonaws.com/prod/limit".to_string()
    })
}

const NONCE_BURN_GAS_MULTIPLIER: u128 = 10;
const NONCE_BURN_PRIORITY_FEE: u128 = 1e7 as u128; // 0.01 gwei (max priority bid possible)
const ETH_TRANSFER_GAS: u64 = 21000;

macro_rules! send_metric_with_order_hash {
    ($order_hash: expr, $future: expr) => {
        let hash = Arc::clone($order_hash);
        tokio::spawn(async move {
            if let Err(e) = $future.await {
                tracing::warn!("{} - error sending metric: {:?}", hash, e);
            }
        })
    };
}

macro_rules! u256 {
    ($($limb:expr),*) => {
        alloy_primitives::Uint::from_limbs([$($limb, 0, 0, 0),*])
    };
}

pub(crate) use send_metric_with_order_hash;
pub(crate) use u256;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub struct MethodParameters {
    pub calldata: String,
    pub value: String,
    pub to: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RouteInfo {
    pub quote: String,
    #[serde(rename = "quoteGasAdjusted")]
    pub quote_gas_adjusted: String,
    #[serde(rename = "gasUseEstimate")]
    pub gas_use_estimate: String,
    #[serde(rename = "gasUseEstimateQuote")]
    pub gas_use_estimate_quote: String,
    #[serde(rename = "gasPriceWei")]
    pub gas_price_wei: String,
    #[serde(rename = "methodParameters")]
    pub method_parameters: MethodParameters,
}

pub async fn get_nonce_with_retry(
    sender_client: &Arc<DynProvider<AnyNetwork>>,
    address: Address,
    order_hash: &str,
    max_attempts: u32,
) -> Result<u64, anyhow::Error> {
    let mut attempts = 0;
    loop {
        match sender_client.get_transaction_count(address).await {
            Ok(nonce) => break Ok(nonce),
            Err(e) => {
                if attempts < max_attempts - 1 {
                    attempts += 1;
                } else {
                    return Err(anyhow::anyhow!(
                        "{} - Failed to get nonce after {} attempts: {}",
                        order_hash,
                        max_attempts,
                        e
                    ));
                }
            }
        }
    }
}

/// @notice Burns a specific nonce by sending a 0 ETH transaction to self with a high gas price.
/// @dev This function is used to invalidate a nonce by creating a dummy transaction.
/// @param provider The Ethereum provider used to send the transaction.
/// @param wallet The wallet used to sign the transaction.
/// @param address The address whose nonce will be burned.
/// @param nonce The specific nonce to burn.
/// @param order_hash A string identifier for logging and tracing purposes.
/// @return Returns Ok(()) if the transaction is sent and confirmed, or an error otherwise.
pub async fn burn_nonce(
    provider: &Arc<DynProvider<AnyNetwork>>,
    wallet: &EthereumWallet,
    address: Address,
    nonce: u64,
    order_hash: &str,
) -> Result<(), anyhow::Error> {
    let base_fee = provider.get_gas_price().await?;

    // Create a dummy transaction that sends 0 ETH to self with high gas price
    let tx_request = WithOtherFields::new(TransactionRequest {
        from: Some(address),
        to: Some(address.into()),
        value: Some(U256::ZERO),
        nonce: Some(nonce),
        gas: Some(ETH_TRANSFER_GAS), // Standard ETH transfer gas
        gas_price: Some(base_fee * NONCE_BURN_GAS_MULTIPLIER),
        max_fee_per_gas: Some(base_fee * NONCE_BURN_GAS_MULTIPLIER),
        max_priority_fee_per_gas: Some(NONCE_BURN_PRIORITY_FEE),
        ..Default::default()
    });

    // Sign and send the transaction
    let tx = tx_request.build(wallet).await?;
    let result = provider.send_tx_envelope(tx).await;

    match result {
        Ok(tx) => {
            tracing::info!("{} - Waiting for confirmations", order_hash);
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
                    tracing::info!(
                        "{} - Nonce burn: tx_hash: {:?}, status: {}",
                        order_hash,
                        receipt.transaction_hash,
                        status,
                    );
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("{} - Error burning nonce: {}", order_hash, e);
                    return Err(anyhow::anyhow!(
                        "{} - Error burning nonce: {}",
                        order_hash,
                        e
                    ));
                }
            }
        }
        Err(e) => {
            tracing::error!(
                "{} - Error sending nonce burn transaction: {}",
                order_hash,
                e
            );
            return Err(anyhow::anyhow!(
                "{} - Error sending nonce burn transaction: {}",
                order_hash,
                e
            ));
        }
    }
}
