use std::sync::Arc;
use alloy::{
    eips::eip2718::Encodable2718,
    network::{AnyNetwork, EthereumWallet, ReceiptResponse},
    primitives::keccak256,
    providers::{DynProvider, Provider},
    rpc::types::TransactionRequest,
    serde::WithOtherFields,
};
use anyhow::Result;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use tracing::{debug, info, warn};

use crate::{
    aws_utils::cloudwatch_utils::{
        build_metric_future, revert_code_to_metric, CwMetrics, DimensionValue
    },
    executors::reactor_error_code::{get_revert_reason, ReactorErrorCode},
    shared::{burn_nonce, send_metric_with_order_hash},
};

#[derive(Debug)]
pub enum TransactionOutcome {
    Success(Option<u64>),
    Failure(Option<u64>),
    RetryableFailure,
}

/// Poll for transaction receipt with timeout
pub async fn poll_for_receipt(
    sender_client: &Arc<DynProvider<AnyNetwork>>,
    client: &Arc<DynProvider<AnyNetwork>>,
    tx_envelope: &alloy::network::AnyTxEnvelope,
    target_block: u64,
    order_hash: &str,
    timeout_sec: u64,
    poll_interval_ms: u64,
) -> Result<Option<alloy::serde::WithOtherFields<alloy::rpc::types::TransactionReceipt<alloy::network::AnyReceiptEnvelope<alloy::rpc::types::Log>>>>> {
    // Compute the transaction hash
    let tx_bytes = tx_envelope.encoded_2718();
    let tx_hash = keccak256(&tx_bytes);
    
    let start_time = std::time::Instant::now();
    while start_time.elapsed() < std::time::Duration::from_secs(timeout_sec) {
        // Use sender_client to get receipt from the same endpoint
        match sender_client.get_transaction_receipt(tx_hash).await {
            Ok(Some(receipt)) => return Ok(Some(receipt)),
            Ok(None) => {
                // Check if we've passed the target block
                let current_block = client.get_block_number().await?;
                if current_block > target_block + 1 {
                    info!("{} - Transaction not included, target block passed", order_hash);
                    return Ok(None);
                }
            }
            Err(e) => {
                debug!("{} - Error getting receipt: {}", order_hash, e);
            }
        }
        
        tokio::time::sleep(std::time::Duration::from_millis(poll_interval_ms)).await;
    }
    
    Ok(None)
}

/// Process transaction receipt and determine outcome
pub async fn process_receipt(
    receipt: Option<impl ReceiptResponse>,
    client: &Arc<DynProvider<AnyNetwork>>,
    tx_request_for_revert: WithOtherFields<TransactionRequest>,
    order_hash: &str,
    chain_id: u64,
    target_block: Option<u64>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
) -> Result<TransactionOutcome> {
    match receipt {
        Some(receipt) => {
            // Handle metrics for target block delta
            if let (Some(block_num), Some(target)) = (receipt.block_number(), target_block) {
                let target_block_delta = block_num as f64 - target as f64;
                info!("{} - target block delta: {}, target_block: {}, actual_block: {}", 
                      order_hash, target_block_delta, target, block_num);
                
                let metric_future = build_metric_future(
                    cloudwatch_client.clone(),
                    DimensionValue::PriorityExecutor,
                    CwMetrics::TargetBlockDelta(chain_id),
                    target_block_delta,
                );
                if let Some(metric_future) = metric_future {
                    send_metric_with_order_hash!(&Arc::new(order_hash.to_string()), metric_future);
                }
            }
            
            let status = receipt.status();
            info!("{} - receipt: tx_hash: {:?}, status: {}", 
                  order_hash, receipt.transaction_hash(), status);
            
            if !status && receipt.block_number().is_some() {
                // Get revert reason
                info!("{} - Attempting to get revert reason", order_hash);
                match get_revert_reason(client, tx_request_for_revert, receipt.block_number().unwrap()).await {
                    Ok(reason) => {
                        info!("{} - Revert reason: {}", order_hash, reason);
                        let metric_future = build_metric_future(
                            cloudwatch_client,
                            DimensionValue::PriorityExecutor,
                            revert_code_to_metric(chain_id, reason.to_string()),
                            1.0,
                        );
                        if let Some(metric_future) = metric_future {
                            send_metric_with_order_hash!(&Arc::new(order_hash.to_string()), metric_future);
                        }
                        
                        if matches!(reason, ReactorErrorCode::OrderNotFillable) {
                            Ok(TransactionOutcome::RetryableFailure)
                        } else {
                            Ok(TransactionOutcome::Failure(receipt.block_number()))
                        }
                    }
                    Err(e) => {
                        info!("{} - Failed to get revert reason: {:?}", order_hash, e);
                        Ok(TransactionOutcome::Failure(receipt.block_number()))
                    }
                }
            } else {
                Ok(TransactionOutcome::Success(receipt.block_number()))
            }
        }
        None => {
            warn!("{} - No receipt found", order_hash);
            Ok(TransactionOutcome::Failure(None))
        }
    }
}

/// Handle transaction sending errors
pub async fn handle_send_error(
    error: anyhow::Error,
    sender_client: &Arc<DynProvider<AnyNetwork>>,
    wallet: &EthereumWallet,
    tx_request: &WithOtherFields<TransactionRequest>,
    order_hash: &str,
) -> Result<TransactionOutcome> {
    warn!("{} - Error sending transaction: {}", order_hash, error);
    
    // Handle nonce issues
    if error.to_string().contains("replacement transaction underpriced") {
        info!("{} - Nonce already used, burning nonce for next transaction", order_hash);
        burn_nonce(
            sender_client,
            wallet,
            tx_request.from.unwrap(),
            tx_request.nonce.unwrap(),
            order_hash
        ).await?;
    }
    
    Ok(TransactionOutcome::Failure(None))
} 