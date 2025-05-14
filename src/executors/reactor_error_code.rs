use alloy::{
    eips::BlockId,
    network::AnyNetwork,
    providers::{DynProvider, Provider},
    rpc::types::TransactionRequest,
    serde::WithOtherFields,
    transports::RpcError,
};
use anyhow::Result;

pub enum ReactorErrorCode {
    InvalidDeadline,
    OrderNotFillable,
    OrderAlreadyFilled,
    InsufficientETH,
    InsufficientToken,
    NativeTransferFailed,
    AllowanceExpired,
    Unknown,
}

// implements the From trait for the ReactorErrorCode enum to convert it to a string
impl From<String> for ReactorErrorCode {
    fn from(s: String) -> Self {
        // Remove quotes and whitespace before matching
        let cleaned = s.trim().trim_matches('\"');
        // Take first 10 chars (including 0x) if longer
        let code = if cleaned.len() > 10 {
            &cleaned[..10]
        } else {
            cleaned
        };

        match code {
            "0xc6035520" => ReactorErrorCode::OrderNotFillable,
            "0xee3b3d4b" => ReactorErrorCode::OrderAlreadyFilled,
            "0x769d11e4" => ReactorErrorCode::InvalidDeadline,
            "0x6a12f104" => ReactorErrorCode::InsufficientETH,
            "0x675cae38" => ReactorErrorCode::InsufficientToken,
            "0xf4b3b1bc" => ReactorErrorCode::NativeTransferFailed,
            "0xd81b2f2e" => ReactorErrorCode::AllowanceExpired,
            _ => ReactorErrorCode::Unknown,
        }
    }
}

impl std::fmt::Display for ReactorErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ReactorErrorCode::InvalidDeadline => "InvalidDeadline",
            ReactorErrorCode::OrderNotFillable => "OrderNotFillable",
            ReactorErrorCode::OrderAlreadyFilled => "OrderAlreadyFilled",
            ReactorErrorCode::InsufficientETH => "InsufficientETH",
            ReactorErrorCode::InsufficientToken => "InsufficientToken",
            ReactorErrorCode::NativeTransferFailed => "NativeTransferFailed",
            ReactorErrorCode::AllowanceExpired => "AllowanceExpired",
            ReactorErrorCode::Unknown => "Unknown",
        };
        write!(f, "{s}")
    }
}

pub async fn get_revert_reason(
    provider: &DynProvider<AnyNetwork>,
    tx: WithOtherFields<TransactionRequest>,
    block_number: u64,
) -> Result<ReactorErrorCode, Box<dyn std::error::Error>> {
    // Simulate the transaction at the block right before it was mined
    let result = provider
        .call(tx)
        .block(BlockId::Number(block_number.into()))
        .await;

    // Extract revert reason from the error
    match result {
        Ok(_) => Err("Tx succeeded in simulation".into()),
        Err(e) => {
            let err_msg = e.to_string(); // Clone the error message first
            if let RpcError::ErrorResp(err) = e {
                if let Some(data) = err.data.as_ref().map(|d| d.get()) {
                    let error_code = ReactorErrorCode::from(data.to_string());
                    if matches!(error_code, ReactorErrorCode::Unknown) {
                        Err(format!("Failed to extract revert reason from code: {data}").into())
                    } else {
                        Ok(error_code)
                    }
                } else {
                    Err(format!("Failed to extract revert reason: {err_msg}").into())
                }
            } else {
                Err(format!("Failed to extract revert reason: {err_msg}").into())
            }
        }
    }
}
