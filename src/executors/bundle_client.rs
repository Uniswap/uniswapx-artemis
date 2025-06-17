use std::sync::Arc;
use alloy::{
    eips::eip2718::Encodable2718,
    network::{AnyNetwork, EthereumWallet, TransactionBuilder},
    primitives::keccak256,
    providers::{DynProvider, Provider},
    rpc::types::TransactionRequest,
    serde::WithOtherFields,
    hex,
};
use anyhow::Result;
use serde_json::{json, Value};
use tracing::info;

pub struct BundleClient {
    sender_client: Arc<DynProvider<AnyNetwork>>,
}

impl BundleClient {
    pub fn new(sender_client: Arc<DynProvider<AnyNetwork>>) -> Self {
        Self { sender_client }
    }

    /// Send a single transaction as a bundle
    pub async fn send_bundle(
        &self,
        wallet: &EthereumWallet,
        tx_request: WithOtherFields<TransactionRequest>,
        target_block: u64,
        bundle_window: u64,
        order_hash: &str,
    ) -> Result<(alloy::network::AnyTxEnvelope, Option<String>)> {
        // Sign the transaction
        let tx_envelope = tx_request.build(wallet).await?;
        let raw_tx = tx_envelope.encoded_2718();
        let signed_tx = format!("0x{}", hex::encode(&raw_tx));
        
        let tx_hash = keccak256(&raw_tx);
        let tx_hash_hex = format!("0x{}", hex::encode(tx_hash));
        
        // Build bundle params (single transaction bundle that's allowed to revert)
        let params = json!({
            "txs": vec![signed_tx],
            "minBlockNumber": format!("0x{:x}", target_block),
            "maxBlockNumber": format!("0x{:x}", target_block + bundle_window),
            "revertingTxHashes": vec![tx_hash_hex],
        });
        
        info!("{} - Sending bundle for blocks {}-{}", order_hash, target_block, target_block + bundle_window);
        
        // Send bundle
        let bundle_result = self.sender_client
            .raw_request::<Vec<Value>, Value>(std::borrow::Cow::Borrowed("eth_sendBundle"), vec![params])
            .await?;
        
        // Extract bundle hash if available
        let bundle_hash = bundle_result.get("bundleHash")
            .and_then(|h| h.as_str())
            .map(|s| s.to_string());
        
        if let Some(hash) = &bundle_hash {
            info!("{} - Bundle submitted with hash: {}", order_hash, hash);
        }
        
        Ok((tx_envelope, bundle_hash))
    }
} 