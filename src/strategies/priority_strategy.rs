use super::{
    shared::UniswapXStrategy,
    types::{Config, OrderStatus},
};
use crate::{
    aws_utils::cloudwatch_utils::{
        CwMetrics, DimensionName, DimensionValue, MetricBuilder, ARTEMIS_NAMESPACE,
    },
    collectors::{
        block_collector::NewBlock,
        uniswapx_order_collector::UniswapXOrder,
        uniswapx_route_collector::{OrderBatchData, OrderData, OrderRoute, RoutedOrder},
    },
    shared::RouteInfo,
    strategies::types::SubmitTxToMempoolWithExecutionMetadata,
};
use alloy::{
    hex,
    network::AnyNetwork,
    primitives::{Address, Bytes, Uint, U128, U256, U64},
    providers::{DynProvider, Provider},
    rpc::types::Filter,
};
use anyhow::Result;
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use bindings_uniswapx::basereactor::BaseReactor::SignedOrder;
use dashmap::DashMap;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    RwLock,
};
use tracing::{debug, error, info, warn};
use uniswapx_rs::order::{Order, OrderResolution, PriorityOrder, MPS};

use super::types::{Action, Event};

const DONE_EXPIRY: u64 = 300;
// Base addresses
const REACTOR_ADDRESS: &str = "0x000000001Ec5656dcdB24D90DFa42742738De729";
pub const WETH_ADDRESS: &str = "0x4200000000000000000000000000000000000006";

fn get_block_time_ms(chain_id: u64) -> u64 {
    match chain_id {
        130 => 1000,   // Unichain
        8453 => 2000,  // Base Mainnet
        _ => 2000,     // Default to 2 seconds for unknown chains
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionMetadata {
    // amount of quote token we can get
    pub quote: U256,
    // amount of quote token needed to fill the order
    pub amount_out_required: U256,
    pub order_hash: String,
    pub target_block: Option<U64>,
}

impl ExecutionMetadata {
    pub fn new(
        quote: U256,
        amount_out_required: U256,
        order_hash: &str,
        target_block: Option<U64>,
    ) -> Self {
        Self {
            quote,
            amount_out_required,
            order_hash: order_hash.to_owned(),
            target_block,
        }
    }

    pub fn calculate_priority_fee(&self, bid_percentage: U128) -> Option<U256> {
        if self.quote.le(&self.amount_out_required) {
            return None;
        }

        let profit_quote = self.quote.saturating_sub(self.amount_out_required);

        let mps_of_improvement = profit_quote
            .saturating_mul(U256::from(MPS))
            .checked_div(self.amount_out_required)?;
        let priority_fee = mps_of_improvement
            .checked_mul(U256::from(bid_percentage))?
            .checked_div(U256::from(100))?;
        Some(priority_fee)
    }
}

/// Strategy for filling UniswapX Priority Orders
/// 
/// This strategy:
/// - Tracks new orders from the UniswapX API
/// - Routes orders through Uniswap's routing API
/// - Submits fillable orders to the mempool
/// - Handles order lifecycle (new -> processing -> done)
/// - Prunes completed orders periodically
#[derive(Debug)]
#[allow(dead_code)]
pub struct UniswapXPriorityFill {
    /// Alloy DynProvider client.
    client: Arc<DynProvider<AnyNetwork>>,
    // AWS Cloudwatch CLient for metrics propagation
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
    /// executor address
    executor_address: String,
    /// Amount of profits to bid in gas
    bid_percentage: u128,
    last_block_number: RwLock<u64>,
    last_block_timestamp: RwLock<u64>,
    // map of new order hashes to order data
    new_orders: Arc<DashMap<String, OrderData>>,
    // map of order hashes that are currently being processed (routed/executed)
    processing_orders: Arc<DashMap<String, OrderData>>,
    // map of done order hashes to time at which we can safely prune them
    done_orders: Arc<DashMap<String, u64>>,
    batch_sender: Sender<Vec<OrderBatchData>>,
    route_receiver: Receiver<RoutedOrder>,
    chain_id: u64,
}

impl UniswapXPriorityFill {
    pub fn new(
        client: Arc<DynProvider<AnyNetwork>>,
        cloudwatch_client: Option<Arc<CloudWatchClient>>,
        config: Config,
        sender: Sender<Vec<OrderBatchData>>,
        receiver: Receiver<RoutedOrder>,
        chain_id: u64,
    ) -> Self {
        info!("syncing state");

        Self {
            client,
            cloudwatch_client,
            executor_address: config.executor_address,
            bid_percentage: config.bid_percentage,
            last_block_number: RwLock::new(0),
            last_block_timestamp: RwLock::new(0),
            new_orders: Arc::new(DashMap::new()),
            processing_orders: Arc::new(DashMap::new()),
            done_orders: Arc::new(DashMap::new()),
            batch_sender: sender,
            route_receiver: receiver,
            chain_id,
        }
    }
}

#[async_trait]
impl Strategy<Event, Action> for UniswapXPriorityFill {
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::UniswapXOrder(order) => self.process_order_event(&order).await,
            Event::NewBlock(block) => self.process_new_block_event(&block).await,
            Event::UniswapXRoute(route) => self.process_new_route(&route).await,
        }
    }
}

impl UniswapXStrategy for UniswapXPriorityFill {}

impl UniswapXPriorityFill {
    pub fn get_new_order(&self, hash: &str) -> Option<OrderData> {
        self.new_orders.get(hash).map(|entry| entry.value().clone())
    }

    pub fn update_new_order<F>(&self, hash: &str, f: F)
    where
        F: FnOnce(&mut OrderData),
    {
        if let Some(mut entry) = self.new_orders.get_mut(hash) {
            f(entry.value_mut());
        }
    }

    fn decode_order(&self, encoded_order: &str) -> Result<PriorityOrder, Box<dyn Error>> {
        let encoded_order = if let Some(stripped) = encoded_order.strip_prefix("0x") {
            stripped
        } else {
            encoded_order
        };
        
        let order_hex = hex::decode(encoded_order)
            .map_err(|e| format!("Failed to decode hex: {}", e))?;

        PriorityOrder::decode_inner(&order_hex, false)
            .map_err(|e| format!("Failed to decode order: {}", e).into())
    }

    async fn get_order_status(&self, order: &PriorityOrder) -> OrderStatus {
        let resolved_order = order.resolve(
            *self.last_block_number.read().await,
            *self.last_block_timestamp.read().await,
            get_block_time_ms(self.chain_id),
            Uint::from(0),
        );
        let order_status = match resolved_order {
            OrderResolution::Expired | OrderResolution::Invalid => OrderStatus::Done,
            OrderResolution::NotFillableYet(resolved) => OrderStatus::NotFillableYet(resolved),
            OrderResolution::Resolved(resolved) => OrderStatus::Open(resolved),
        };
        order_status
    }

    /// Process new order events that we fetch from UniswapX API
    /// - skip if we are already tracking this order
    /// - otherwise decode and process:
    ///     - skip if we have already processed this order
    ///     - immediately send for execution if order is fillable
    ///     - otherwise add to new_orders to be processed on new block event
    async fn process_order_event(&self, event: &UniswapXOrder) -> Vec<Action> {
        if *self.last_block_timestamp.read().await == 0 {
            info!(
                "{} - skipping processing new order event (no timestamp)",
                event.order_hash
            );
            return self.check_orders_for_submission().await;
        }
        if self.new_orders.contains_key(&event.order_hash)
            || self.processing_orders.contains_key(&event.order_hash)
        {
            debug!(
                "{} - skipping processing new order event (already tracking)",
                event.order_hash
            );
            return self.check_orders_for_submission().await;
        }

        let order = self
            .decode_order(&event.encoded_order)
            .map_err(|e| error!("failed to decode: {}", e))
            .ok()
            .unwrap();

        // Skip EXACT_OUTPUT orders for now
        if order.outputs.iter().any(|o| o.mpsPerPriorityFeeWei == U256::from(0)) {
            info!("{} - Skipping EXACT_OUTPUT order", event.order_hash);
            return self.check_orders_for_submission().await;
        }

        let order_hash = event.order_hash.clone();

        match self.get_order_status(&order).await {
            OrderStatus::Done => {
                debug!("{} - Order already done, skipping", order_hash);
            }
            OrderStatus::NotFillableYet(resolved) | OrderStatus::Open(resolved) => {
                if self.done_orders.contains_key(&order_hash) {
                    info!(
                        "{} - New order processing already done, skipping",
                        order_hash
                    );
                    return self.check_orders_for_submission().await;
                }
                let order_data = OrderData {
                    order: Order::PriorityOrder(order.clone()),
                    hash: order_hash.clone(),
                    signature: event.signature.clone(),
                    resolved,
                    encoded_order: None,
                    route: event.route.clone(),
                };
                if let Some(route) = &order_data.route {
                    if !route.method_parameters.calldata.is_empty() {
                        info!("{} - Received cached route for order", order_hash);
                    }
                }
                self.new_orders.insert(order_hash.clone(), order_data.clone());

                info!(
                    "{} - Route new order at block {}; target: {}",
                    order_hash,
                    *self.last_block_number.read().await,
                    order.cosignerData.auctionTargetBlock
                );
                let order_batch = self.get_order_batch(&order_data);
                self.try_route_order_batch(order_batch, order_hash)
                    .await;
            }
        }
        return self.check_orders_for_submission().await
    }

    async fn process_new_route(&mut self, event: &RoutedOrder) -> Vec<Action> {
        if event
            .request
            .orders
            .iter()
            .any(|o: &OrderData| self.done_orders.contains_key(&o.hash))
        {
            info!(
                "{} - Skipping route with done order",
                event.request.orders[0].hash
            );
            return vec![];
        }

        // Store route in new_orders
        for order in &event.request.orders {
            info!("{} - Received new route for order", order.hash);
            if let Some(mut entry) = self.new_orders.get_mut(&order.hash) {
                // Update the route in the existing OrderData
                entry.value_mut().route = Some(RouteInfo {
                    quote: event.route.quote.clone(),
                    quote_gas_adjusted: event.route.quote_gas_adjusted.clone(),
                    gas_use_estimate: event.route.gas_use_estimate.clone(),
                    gas_use_estimate_quote: event.route.gas_use_estimate_quote.clone(),
                    gas_price_wei: event.route.gas_price_wei.clone(),
                    method_parameters: event.route.method_parameters.clone(),
                });

                // Check if order is fillable
                let resolved_order = match &entry.order {
                    Order::PriorityOrder(order) => order,
                    _ => continue,
                };

                if let OrderStatus::NotFillableYet(_) = self.get_order_status(resolved_order).await {
                    let order_batch = self.get_order_batch(entry.value());
                    self.try_route_order_batch(order_batch, order.hash.clone())
                        .await;
                    info!(
                        "{} - Order not fillable yet, refreshing route at block {}",
                        order.hash,
                        *self.last_block_number.read().await
                    );
                }
            }
        }

        // Try to submit the order and return any actions
        return self.check_orders_for_submission().await
    }

    /// Process new block events
    /// - update the block number and timestamp
    /// - check for fills from block logs and remove from processing_orders
    /// - check new_orders for orders that are now fillable and send for execution
    /// - prune done orders
    async fn process_new_block_event(&mut self, event: &NewBlock) -> Vec<Action> {
        *self.last_block_number.write().await = event.number;
        *self.last_block_timestamp.write().await = event.timestamp;

        info!(
            "Processing block {} at {}, Order set sizes -- open: {}, processing: {}, done: {}",
            event.number,
            event.timestamp,
            self.new_orders.len(),
            self.processing_orders.len(),
            self.done_orders.len()
        );

        // check fills from block logs and remove from processing_orders
        if let Err(e) = self.handle_fills().await {
            error!("Error handling fills: {}", e);
        }

        let actions = self.check_new_orders_for_processing().await;

        if *self.last_block_number.read().await % 100 == 0 {
            self.prune_done_orders().await;
            if let Some(cw) = &self.cloudwatch_client {
                let metric_future = cw
                    .put_metric_data()
                    .namespace(ARTEMIS_NAMESPACE)
                    .metric_data(
                        MetricBuilder::new(CwMetrics::LatestBlock(self.chain_id))
                            .add_dimension(
                                DimensionName::Service.as_ref(),
                                DimensionValue::PriorityExecutor.as_ref(),
                            )
                            .with_value(event.number as f64)
                            .build(),
                    )
                    .send();
                tokio::spawn(async move {
                    if let Err(e) = metric_future.await {
                        warn!("Error sending block metric: {:?}", e);
                    }
                });
            }
        }

        actions
    }

    /// encode orders into generic signed orders
    fn get_signed_orders(&self, orders: Vec<OrderData>) -> Result<Vec<SignedOrder>> {
        let mut signed_orders: Vec<SignedOrder> = Vec::new();
        for batch in orders.iter() {
            match &batch.order {
                Order::PriorityOrder(order) => {
                    signed_orders.push(SignedOrder {
                        order: Bytes::from(order.encode_inner()),
                        sig: Bytes::from_str(&batch.signature)?,
                    });
                }
                _ => {
                    return Err(anyhow::anyhow!("Invalid order type"));
                }
            }
        }
        Ok(signed_orders)
    }

    fn get_order_batch(&self, order_data: &OrderData) -> OrderBatchData {
        let amount_in: Uint<256, 4> = order_data.resolved.input.amount;
        let amount_out = order_data
            .resolved
            .outputs
            .iter()
            .fold(Uint::from(0), |sum, output| sum.wrapping_add(output.amount));

        OrderBatchData {
            orders: vec![order_data.clone()],
            amount_in,
            amount_out_required: amount_out,
            token_in: order_data.resolved.input.token.clone(),
            token_out: order_data.resolved.outputs[0].token.clone(),
            chain_id: self.chain_id,
        }
    }

    async fn handle_fills(&self) -> Result<()> {
        let reactor_address = REACTOR_ADDRESS.parse::<Address>().unwrap();
        let filter = Filter::new()
            .select(*self.last_block_number.read().await)
            .address(reactor_address)
            .event("Fill(bytes32,address,address,uint256)");

        let logs = self.client.get_logs(&filter).await.unwrap_or_else(|e| {
            error!("Failed to get logs: {}", e);
            Vec::new()
        });

        for log in logs {
            let order_hash = format!("0x{:x}", log.topics()[1]);
            info!(
                "{} - Removing filled order from processing_orders",
                order_hash
            );
            self.new_orders.remove(&order_hash);
            self.processing_orders.remove(&order_hash);
            self.done_orders.insert(
                order_hash.to_string(),
                self.current_timestamp()? + DONE_EXPIRY,
            );
        }
        Ok(())
    }

    /// The profit of a priority order is calculated a bit differently
    /// Rationale:
    ///     - we will always bid the base fee
    ///     - since we have to provide 1 MP (1/1000th of a bp) for every wei of priority fee
    ///     - we return the data needed to calculate the maximum MPS of improvement we can offer from our quote and the order specs
    fn get_execution_metadata(
        &self,
        RoutedOrder {
            request,
            route,
            target_block,
            ..
        }: &RoutedOrder,
    ) -> Option<ExecutionMetadata> {
        let quote = U256::from_str_radix(&route.quote, 10).ok()?;
        let amount_out_required =
            U256::from_str_radix(&request.amount_out_required.to_string(), 10).ok()?;
        if quote.le(&amount_out_required) {
            info!("{} - Quote is less than amount out required", request.orders[0].hash);
            return None;
        }

        Some({
            ExecutionMetadata {
                quote,
                amount_out_required,
                order_hash: request.orders[0].hash.clone(),
                target_block: target_block.map(|b| U64::from(b)),
            }
        })
    }

    /// process an order status and the associated resolved order
    /// if order is done, mark as done
    /// if order is not fillable yet, do nothing
    /// if order is open, send for execution
    async fn process_new_order(
        &mut self,
        order: PriorityOrder,
        order_hash: String,
        signature: &str,
        route: Option<RouteInfo>,
    ) -> Result<()> {
        let order_status = self.get_order_status(&order).await;

        match order_status {
            OrderStatus::Done => {
                info!("{} - Order is done, removing from new_orders and processing_orders", order_hash);
                self.new_orders.remove(&order_hash);
                self.processing_orders.remove(&order_hash);
                self.done_orders
                    .insert(order_hash, self.current_timestamp()? + DONE_EXPIRY);
            }
            OrderStatus::NotFillableYet(resolved_order) | OrderStatus::Open(resolved_order) => {
                let order_data = OrderData {
                    order: Order::PriorityOrder(order),
                    hash: order_hash.to_string(),
                    signature: signature.to_string(),
                    resolved: resolved_order,
                    encoded_order: None,
                    route: route,
                };
                info!(
                    "{} - Requesting fresh route for order",
                    order_hash
                );
                let order_batch = self.get_order_batch(&order_data);
                self.try_route_order_batch(order_batch, order_hash)
                    .await;
            }
        }

        Ok(())
    }

    async fn prune_done_orders(&mut self) {
        info!("Pruning done orders");
        let mut to_remove = Vec::new();
        for item in self.done_orders.iter() {
            if *item.value() < *self.last_block_timestamp.read().await {
                to_remove.push(item.key().clone());
            }
        }
        for order_hash in to_remove {
            self.done_orders.remove(&order_hash);
        }
    }

    /// check all new orders we are tracking
    /// if they are now fillable at the latest block, move then to self.processing_orders and send for execution
    async fn check_new_orders_for_processing(&mut self) -> Vec<Action> {
        let order_hashes = self
            .new_orders
            .iter()
            .map(|entry| entry.key().clone())
            .collect::<Vec<String>>();

        for order_hash in order_hashes {
            if let Some(order_data) = self.get_new_order(&order_hash) {
                match &order_data.order {
                    Order::PriorityOrder(order) => {
                        if let Err(e) = self
                            .process_new_order(
                                order.clone(),
                                order_hash.clone(),
                                &order_data.signature,
                                order_data.route.clone(),
                            )
                            .await
                        {
                            error!("Error processing new order: {}", e);
                        }
                    }
                    _ => {
                        error!("Invalid order type");
                    }
                }
            }
        }

        // After processing orders, check if any can be submitted
        return self.check_orders_for_submission().await
    }

    async fn try_route_order_batch(
        &self,
        order_batch: OrderBatchData,
        order_hash: String,
    ) {
        match self.batch_sender.send(vec![order_batch]).await {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "{} - Failed to send batch: {}",
                    order_hash, e
                );
            }
        }
    }

    async fn check_orders_for_submission(&self) -> Vec<Action> {
        let order_hashes: Vec<String> = self.new_orders
            .iter()
            .map(|entry| entry.key().clone())
            .collect();

        let mut actions = Vec::new();

        for order_hash in order_hashes {
            if let Some(mut order_data) = self.new_orders.get_mut(&order_hash) {
                // Skip if no route available
                if order_data.route.as_ref().map_or(true, |r| r.method_parameters.calldata.is_empty()) {
                    debug!("{} - No route available, skipping", order_hash);
                    continue;
                }
                // skip if order is already in processing_orders
                if self.processing_orders.contains_key(&order_hash) {
                    debug!("{} - Order is already in processing_orders, skipping", order_hash);
                    continue;
                }

                // Check if order is now fillable
                let order = match &order_data.order {
                    Order::PriorityOrder(order) => order,
                    _ => continue,
                };

                match self.get_order_status(order).await {
                    OrderStatus::Done => {
                        info!("{} - Order is done, removing from new_orders and processing_orders", order_hash);
                        self.new_orders.remove(&order_hash);
                        self.processing_orders.remove(&order_hash);
                        self.done_orders.insert(
                            order_hash,
                            self.current_timestamp().unwrap_or(0) + DONE_EXPIRY,
                        );
                        continue;
                    }
                    OrderStatus::NotFillableYet(_) => {
                        debug!("{} - Order is not fillable yet, skipping", order_hash);
                        continue;
                    }
                    OrderStatus::Open(_) => {
                        debug!("{} - Order is open, adding to processing_orders", order_hash);
                        // if already in processing_orders, skip (prevent race condition)
                        if self.processing_orders.contains_key(&order_hash) {
                            continue;
                        }
                        else {
                            self.processing_orders.insert(order_hash.clone(), order_data.value().clone());
                        }

                        let routed_order = RoutedOrder {
                            request: OrderBatchData {
                                orders: vec![order_data.value().clone()],
                                amount_in: order_data.resolved.input.amount,
                                amount_out_required: order_data.resolved.outputs.iter()
                                    .fold(Uint::from(0), |sum, output| sum.wrapping_add(output.amount)),
                                token_in: order_data.resolved.input.token.clone(),
                                token_out: order_data.resolved.outputs[0].token.clone(),
                                chain_id: self.chain_id,
                            },
                            route: OrderRoute {
                                quote: order_data.route.as_ref().unwrap().quote.clone(),
                                quote_gas_adjusted: order_data.route.as_ref().unwrap().quote_gas_adjusted.clone(),
                                gas_price_wei: order_data.route.as_ref().unwrap().gas_price_wei.clone(),
                                gas_use_estimate_quote: order_data.route.as_ref().unwrap().gas_use_estimate_quote.clone(),
                                gas_use_estimate: order_data.route.as_ref().unwrap().gas_use_estimate.clone(),
                                route: vec![],
                                method_parameters: order_data.route.as_ref().unwrap().method_parameters.clone(),
                            },
                            target_block: Some(order.cosignerData.auctionTargetBlock),
                        };

                        info!(
                            "{} - Order is now fillable, attempting submission with existing route",
                            order_hash
                        );

                        match self.build_fill(
                            self.client.clone(),
                            &self.executor_address,
                            self.get_signed_orders(vec![order_data.value().clone()]).unwrap(),
                            &routed_order,
                        ).await {
                            Ok(fill_tx_request) => {
                                debug!("{} - Successfully built fill transaction", order_hash);
                                let metadata = self.get_execution_metadata(&routed_order);
                                match metadata {
                                    Some(metadata) => {
                                        let action = Action::SubmitPublicTx(
                                            SubmitTxToMempoolWithExecutionMetadata {
                                                execution: SubmitTxToMempool {
                                                    tx: fill_tx_request.clone(),
                                                    gas_bid_info: Some(GasBidInfo {
                                                        bid_percentage: U128::from(self.bid_percentage),
                                                        // this field is not used for priority orders
                                                        total_profit: U128::from(0),
                                                    }),
                                                },
                                                metadata: metadata.clone(),
                                            },
                                        );
                                        actions.push(action);
                                        info!("{} - Successfully queued transaction for submission", order_hash);
                                    }
                                    None => {
                                        // Clear the route and refresh
                                        order_data.value_mut().route = None;
                                        // Refresh route and try again
                                        let order_batch = self.get_order_batch(&order_data.value());
                                        self.try_route_order_batch(order_batch, order_hash.clone())
                                            .await;
                                        info!(
                                            "{} - Order not fillable yet, refreshing route at block {}",
                                            order_hash,
                                            *self.last_block_number.read().await
                                        );
                                        self.processing_orders.remove(&order_hash);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("{} - Error building fill transaction: {}", order_hash, e);
                                continue;
                            }
                        }
                    }
                }
            }
        }

        actions
    }
}
