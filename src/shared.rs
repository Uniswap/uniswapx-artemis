use std::{str::FromStr, sync::Arc};

use alloy::{network::{AnyNetwork, EthereumWallet, TransactionBuilder}, providers::{DynProvider, Provider}, rpc::types::TransactionRequest, serde::WithOtherFields};
use alloy_primitives::{Address, U256};
use serde::Deserialize;

const NONCE_BURN_GAS_MULTIPLIER: u128 = 10;
const NONCE_BURN_PRIORITY_FEE: u128 = 1e7 as u128; // 0.01 gwei (max priority bid possible)
const ETH_TRANSFER_GAS: u64 = 21000;

/// ERC20ETH address - same across all chains
/// ERC20ETH is an ERC20 wrapper for native ETH that uses ERC-7914 for smart wallet compatibility
pub const ERC20ETH_ADDRESS: &str = "0x00000000e20E49e6dCeE6e8283A0C090578F0fb9";

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

/// Normalizes ERC20ETH to native ETH (zero address) for internal processing.
/// ERC20ETH is an ERC20 wrapper for native ETH, so we treat it as native ETH
/// since that's what we'll receive during the callback.
pub fn normalize_erc20eth_to_native(token: &str) -> String {
    if token.eq_ignore_ascii_case(ERC20ETH_ADDRESS) {
        "0x0000000000000000000000000000000000000000".to_string()
    } else {
        token.to_string()
    }
}

/// Normalizes ERC20ETH to native ETH (Address::ZERO) for internal processing.
pub fn normalize_erc20eth_to_native_address(token: Address) -> Address {
    if let Ok(erc20eth_addr) = Address::from_str(ERC20ETH_ADDRESS) {
        if token == erc20eth_addr {
            return Address::ZERO;
        }
    }
    token
}

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
    let base_fee = provider
        .get_gas_price()
        .await?;

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
            // Don't wait for confirmations
            tracing::info!("{} - Nonce burn transaction sent: {:?}", order_hash, tx.tx_hash());
            Ok(())
        }
        Err(e) => {
            tracing::error!("{} - Error sending nonce burn transaction: {}", order_hash, e);
            return Err(anyhow::anyhow!("{} - Error sending nonce burn transaction: {}", order_hash, e));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_erc20eth_to_native_lowercase() {
        let erc20eth = "0x00000000e20e49e6dcee6e8283a0c090578f0fb9";
        let result = normalize_erc20eth_to_native(erc20eth);
        assert_eq!(result, "0x0000000000000000000000000000000000000000");
    }

    #[test]
    fn test_normalize_erc20eth_to_native_uppercase() {
        let erc20eth = "0x00000000E20E49E6DCEE6E8283A0C090578F0FB9";
        let result = normalize_erc20eth_to_native(erc20eth);
        assert_eq!(result, "0x0000000000000000000000000000000000000000");
    }

    #[test]
    fn test_normalize_erc20eth_to_native_mixed_case() {
        let erc20eth = ERC20ETH_ADDRESS; // Already mixed case
        let result = normalize_erc20eth_to_native(erc20eth);
        assert_eq!(result, "0x0000000000000000000000000000000000000000");
    }

    #[test]
    fn test_normalize_erc20eth_to_native_regular_token() {
        let token = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
        let result = normalize_erc20eth_to_native(token);
        assert_eq!(result, token);
    }

    #[test]
    fn test_normalize_erc20eth_to_native_zero_address() {
        let zero = "0x0000000000000000000000000000000000000000";
        let result = normalize_erc20eth_to_native(zero);
        assert_eq!(result, zero); // Should remain unchanged
    }

    #[test]
    fn test_normalize_erc20eth_to_native_address_erc20eth() {
        let erc20eth = Address::from_str(ERC20ETH_ADDRESS).unwrap();
        let result = normalize_erc20eth_to_native_address(erc20eth);
        assert_eq!(result, Address::ZERO);
    }

    #[test]
    fn test_normalize_erc20eth_to_native_address_regular_token() {
        let token = Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();
        let result = normalize_erc20eth_to_native_address(token);
        assert_eq!(result, token);
    }

    #[test]
    fn test_normalize_erc20eth_to_native_address_zero() {
        let zero = Address::ZERO;
        let result = normalize_erc20eth_to_native_address(zero);
        assert_eq!(result, Address::ZERO);
    }
}