use serde::Deserialize;

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

pub(crate) use send_metric_with_order_hash;

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
