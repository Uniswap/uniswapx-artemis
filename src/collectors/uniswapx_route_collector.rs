use std::{collections::HashSet, future::Future, sync::Arc};

use alloy_primitives::Uint;
use anyhow::{anyhow, Result};
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use reqwest::{header::ORIGIN, Error, Response};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info};
use uniswapx_rs::order::{Order, ResolvedOrder};

use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use futures::lock::Mutex;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use reqwest::{Client, StatusCode};

use crate::{
    aws_utils::cloudwatch_utils::{build_metric_future, CwMetrics, DimensionValue},
    shared::send_metric_with_order_hash,
};

const ROUTING_API: &str = "https://api.uniswap.org/v1/quote";
const QUICKROUTES_API: &str = "https://trade-api.gateway.uniswap.org/v1/indicative_quote";
const SLIPPAGE_TOLERANCE: &str = "2.5";
const DEADLINE: u64 = 1000;
const ETH_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
const ONE_ETHER: u64 = 1e18 as u64;

#[derive(Debug, Clone)]
pub struct OrderData {
    pub order: Order,
    pub encoded_order: Option<String>,
    pub hash: String,
    pub signature: String,
    pub resolved: ResolvedOrder,
}

#[derive(Clone, Debug)]
pub struct OrderBatchData {
    pub orders: Vec<OrderData>,
    pub amount_in: Uint<256, 4>,
    pub amount_out_required: Uint<256, 4>,
    pub token_in: String,
    pub token_out: String,
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
enum TradeType {
    #[serde(rename = "exactIn")]
    ExactIn,
    #[serde(rename = "exactOut")]
    ExactOut,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RoutingApiQuery {
    token_in_address: String,
    token_out_address: String,
    token_in_chain_id: u64,
    token_out_chain_id: u64,
    #[serde(rename = "type")]
    trade_type: TradeType,
    amount: String,
    recipient: String,
    slippage_tolerance: String,
    deadline: u64,
    #[serde(rename = "enableUniversalRouter")]
    enable_universal_router: bool,
    protocols: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TokenInRoute {
    address: String,
    chain_id: u64,
    symbol: String,
    decimals: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct V3Route {
    address: String,
    token_in: TokenInRoute,
    token_out: TokenInRoute,
    fee: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct V2Route {
    address: String,
    token_in: TokenInRoute,
    token_out: TokenInRoute,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Route {
    #[serde(rename = "v3-pool")]
    V3(V3Route),
    #[serde(rename = "v2-pool")]
    V2(V2Route),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub struct MethodParameters {
    pub calldata: String,
    pub value: String,
    pub to: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRoute {
    pub quote: String,
    pub quote_gas_adjusted: String,
    pub gas_price_wei: String,
    pub gas_use_estimate_quote: String,
    pub gas_use_estimate: String,
    pub route: Vec<Vec<Route>>,
    pub method_parameters: MethodParameters,
}

#[derive(Clone, Debug)]
pub struct RouteOrderParams {
    pub chain_id: u64,
    pub token_in: String,
    pub token_out: String,
    pub amount: String,
    pub recipient: String,
    pub slippage_tolerance: String,
    pub deadline: u64,
}

#[derive(Clone, Debug)]
pub struct RoutedOrder {
    pub route: OrderRoute,
    pub request: OrderBatchData,
}

/// A new order event, containing the internal order.
#[derive(Debug, Clone, Deserialize)]
pub struct RouteResponse {
    pub orders: Vec<Route>,
}

/// A collector that listens for new orders on UniswapX, and generates a stream of
/// [events](Route) which contain the order.
pub struct UniswapXRouteCollector {
    pub client: Client,
    pub chain_id: u64,
    pub route_request_receiver: Mutex<Receiver<Vec<OrderBatchData>>>,
    pub route_sender: Sender<RoutedOrder>,
    pub executor_address: String,
    pub cloudwatch_client: Option<Arc<CloudWatchClient>>,
}

impl UniswapXRouteCollector {
    pub fn new(
        chain_id: u64,
        route_request_receiver: Receiver<Vec<OrderBatchData>>,
        route_sender: Sender<RoutedOrder>,
        executor_address: String,
        cloudwatch_client: Option<Arc<CloudWatchClient>>,
    ) -> Self {
        Self {
            client: Client::new(),
            chain_id,
            route_request_receiver: Mutex::new(route_request_receiver),
            route_sender,
            executor_address,
            cloudwatch_client,
        }
    }

    pub fn has_native_route(
        &self,
        chain_id: u64,
        token_in: String,
    ) -> impl Future<Output = Result<Response, Error>> + Send {
        let client = Client::new();
        async move {
            let response = client
                .post(QUICKROUTES_API)
                .header("accept", "application/json")
                .header("content-type", "application/json")
                .header("x-api-key", "JoyCGj29tT4pymvhaGciK4r1aIPvqW6W53xT1fwo")
                .json(&serde_json::json!({
                    "type": "EXACT_OUTPUT",
                    "tokenInChainId": chain_id,
                    "tokenOutChainId": chain_id,
                    "tokenIn": resolve_address(token_in),
                    "tokenOut": resolve_address(ETH_ADDRESS.to_string()),
                    "amount": ONE_ETHER
                }))
                .send()
                .await;
            response
        }
    }

    pub async fn route_order(
        &self,
        params: RouteOrderParams,
        order_hash: String,
    ) -> Result<OrderRoute> {
        let client = Client::new();

        // Original routing logic
        let query = RoutingApiQuery {
            token_in_address: resolve_address(params.token_in.clone()),
            token_in_chain_id: params.chain_id,
            token_out_address: resolve_address(params.token_out.clone()),
            token_out_chain_id: params.chain_id,
            trade_type: TradeType::ExactIn,
            amount: params.amount.clone(),
            recipient: params.recipient.clone(),
            slippage_tolerance: params.slippage_tolerance.clone(),
            deadline: params.deadline,
            enable_universal_router: true,
            protocols: "v2,v3,mixed".to_string(),
        };

        let query_string = serde_qs::to_string(&query).unwrap();
        let start = std::time::Instant::now();

        // Start all futures in parallel
        let (routing_result, native_token_in_result, native_token_out_result) = tokio::join!(
            client
                .get(format!("{}?{}", ROUTING_API, query_string))
                .header(ORIGIN, "https://app.uniswap.org")
                .header("accept", "application/json")
                .send(),
            self.has_native_route(params.chain_id, params.token_in.clone()),
            self.has_native_route(params.chain_id, params.token_out.clone())
        );

        // Check results from all dependency futures
        let dependent_futures = [
            ("token in to native request", native_token_in_result),
            ("token out to native request", native_token_out_result),
        ];

        for (request_type, result) in dependent_futures {
            result.map_err(|e| {
                anyhow!(
                    "{} - [Quickroutes] {} failed with error: {}",
                    order_hash,
                    request_type,
                    e
                )
            })?;
        }

        let routing_response = routing_result.unwrap();

        let elapsed = start.elapsed();
        let metric_future = build_metric_future(
            self.cloudwatch_client.clone(),
            DimensionValue::Router02,
            CwMetrics::RoutingMs,
            elapsed.as_millis() as f64,
        );
        if let Some(metric_future) = metric_future {
            send_metric_with_order_hash!(&Arc::new(""), metric_future);
        }

        match routing_response.status() {
            StatusCode::OK => Ok(routing_response
                .json::<OrderRoute>()
                .await
                .map_err(|e| anyhow!("{} - Failed to parse response: {}", order_hash, e))?),
            StatusCode::BAD_REQUEST => Err(anyhow!(
                "{} - Bad request: {}",
                order_hash,
                routing_response.status()
            )),
            StatusCode::NOT_FOUND => Err(anyhow!(
                "{} - Not quote found: {}",
                order_hash,
                routing_response.status()
            )),
            StatusCode::TOO_MANY_REQUESTS => Err(anyhow!(
                "{} - Too many requests: {}",
                order_hash,
                routing_response.status()
            )),
            StatusCode::INTERNAL_SERVER_ERROR => Err(anyhow!(
                "{} - Internal server error: {}",
                order_hash,
                routing_response.status()
            )),
            _ => Err(anyhow!(
                "{} - Unexpected error with status code: {}",
                order_hash,
                routing_response.status()
            )),
        }
    }
}

/// Implementation of the [Collector](Collector) trait for the
/// [UniswapXRouteCollector](UniswapXRouteCollector).
#[async_trait]
impl Collector<RoutedOrder> for UniswapXRouteCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, RoutedOrder>> {
        let stream = async_stream::stream! {
            loop {
                let mut all_requests = Vec::new();
                let mut seen = HashSet::new();
                let mut receiver = self.route_request_receiver.lock().await;

                // Collect all available messages without blocking
                while let Ok(requests) = receiver.try_recv() {
                    for request in requests {
                        if !seen.contains(&request.orders[0].hash) {
                            seen.insert(request.orders[0].hash.clone());
                            all_requests.push(request);
                        }
                    }
                }

                // If no messages were received, wait for one
                if all_requests.is_empty() {
                    if let Some(requests) = receiver.recv().await {
                        for request in requests {
                            if !seen.contains(&request.orders[0].hash) {
                                seen.insert(request.orders[0].hash.clone());
                                all_requests.push(request);
                            }
                        }
                    } else {
                        break; // Channel closed
                    }
                }

                drop(receiver); // Release the lock

                let mut tasks = FuturesUnordered::new();

                for batch in all_requests {
                    let order_hash = batch.orders[0].hash.clone();
                    let OrderBatchData { token_in, token_out, amount_in, .. } = batch.clone();
                    info!(
                        "{} - Routing order, token in: {}, token out: {}",
                        order_hash,
                        token_in, token_out
                    );

                    let future = async move {
                        let route_result = self.route_order(RouteOrderParams {
                            chain_id: self.chain_id,
                            token_in: token_in.clone(),
                            token_out: token_out.clone(),
                            amount: amount_in.to_string(),
                            recipient: self.executor_address.clone(),
                            slippage_tolerance: SLIPPAGE_TOLERANCE.to_string(),
                            deadline: DEADLINE,
                        }, order_hash).await;
                        (batch, route_result)
                    };

                    tasks.push(future);
                }

                while let Some((batch, route_result)) = tasks.next().await {
                    match route_result {
                        Ok(route) => {
                            yield RoutedOrder {
                                request: batch,
                                route,
                            };
                        }
                        Err(e) => {
                            // formatting is done in fn route_order
                            error!("{}", e);
                        }
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}

// The Uniswap routing API requires that "ETH" be used instead of the zero address
fn resolve_address(token: String) -> String {
    if token == "0x0000000000000000000000000000000000000000" {
        return "ETH".to_string();
    }
    token
}
