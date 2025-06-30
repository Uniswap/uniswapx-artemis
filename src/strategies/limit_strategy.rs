use super::{
    shared::UniswapXStrategy,
    types::{Config, OrderStatus, TokenInTokenOut},
};
use crate::shared::get_reactor_address;
use crate::{
    aws_utils::cloudwatch_utils::{build_metric_future, CwMetrics, DimensionValue},
    collectors::{
        block_collector::NewBlock,
        uniswapx_order_collector::{UniswapXOrder},
        uniswapx_route_collector::{OrderBatchData, OrderData, RoutedOrder},
    },
    shared::{send_metric_with_order_hash, RouteInfo},
};
use alloy::{
    hex,
    network::AnyNetwork,
    primitives::{Address, Bytes, Uint},
    providers::{DynProvider, Provider},
    rpc::types::Filter,
};
use alloy_primitives::U128;
use anyhow::Result;
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use bindings_uniswapx::basereactor::BaseReactor::SignedOrder;
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use std::{collections::HashMap, collections::HashSet, fmt::Debug};
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::{error, info, warn};
use uniswapx_rs::order::{LimitOrder, Order, OrderResolution};

use super::types::{Action, Event};

const BLOCK_TIME: u64 = 3;
const DONE_EXPIRY: u64 = 300;

#[derive(Debug)]
#[allow(dead_code)]
pub struct LimitOrderFill {
    /// Ethers client.
    client: Arc<DynProvider<AnyNetwork>>,
    /// executor address
    executor_address: String,
    /// Amount of profits to bid in gas
    bid_percentage: u128,
    last_block_number: u64,
    last_block_timestamp: u64,
    // map of open order hashes to order data
    open_orders: HashMap<String, OrderData>,
    // map of order hashes that are currently being processed (routed/executed)
    processing_orders: HashSet<String>,
    // map of order hashes to timestamp when they were added to processing_orders
    processing_timestamps: HashMap<String, u64>,
    // map of done order hashes to time at which we can safely prune them
    done_orders: HashMap<String, u64>,
    batch_sender: Sender<Vec<OrderBatchData>>,
    route_receiver: Receiver<RoutedOrder>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
    chain_id: u64,
}

impl LimitOrderFill {
    pub fn new(
        client: Arc<DynProvider<AnyNetwork>>,
        config: Config,
        sender: Sender<Vec<OrderBatchData>>,
        receiver: Receiver<RoutedOrder>,
        cloudwatch_client: Option<Arc<CloudWatchClient>>,
        chain_id: u64,
    ) -> Self {
        info!("syncing state");

        Self {
            client,
            executor_address: config.executor_address,
            bid_percentage: config
                .bid_percentage
                .expect("Config missing bid_percentage: cannot initialize LimitOrderFill"),
            last_block_number: 0,
            last_block_timestamp: 0,
            open_orders: HashMap::new(),
            processing_orders: HashSet::new(),
            processing_timestamps: HashMap::new(),
            done_orders: HashMap::new(),
            batch_sender: sender,
            route_receiver: receiver,
            cloudwatch_client,
            chain_id,
        }
    }
}

#[async_trait]
impl Strategy<Event, Action> for LimitOrderFill {
    // In order to sync this strategy, we need to get the current bid for all Sudo pools.
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Vec<Action> {
        match event {
            Event::UniswapXOrder(order) => self.process_order_event(&order).await,
            Event::UniswapXCancelledOrder(cancelled_order) => self.process_cancelled_order_event(&cancelled_order).await,
            Event::NewBlock(block) => self.process_new_block_event(&block).await,
            Event::UniswapXRoute(route) => self.process_new_route(&route).await,
        }
    }
}

impl UniswapXStrategy for LimitOrderFill {}

impl LimitOrderFill {
    fn decode_order(&self, encoded_order: &str) -> Result<LimitOrder, Box<dyn Error>> {
        let encoded_order = if let Some(stripped) = encoded_order.strip_prefix("0x") {
            stripped
        } else {
            encoded_order
        };
        let order_hex: Vec<u8> = hex::decode(encoded_order)?;

        LimitOrder::decode_inner(&order_hex, false)
    }

    // Process new orders as they come in.
    async fn process_order_event(&mut self, event: &UniswapXOrder) -> Vec<Action> {
        if self.last_block_timestamp == 0 {
            return vec![];
        }

        let order = self
            .decode_order(&event.encoded_order)
            .map_err(|e| error!("failed to decode: {}", e))
            .ok();

        if let Some(order) = order {
            self.update_order_state(
                order,
                &event.signature,
                &event.order_hash,
                event.route.as_ref(),
            );
        }
        vec![]
    }

    // Process cancelled orders as they come in.
    async fn process_cancelled_order_event(&mut self, event: &UniswapXOrder) -> Vec<Action> {
        info!("Processing cancelled order: {}", event.order_hash);
        
        // Immediately remove the cancelled order from all states
        self.handle_order_failure(&event.order_hash, "OrderCancelled");
        
        vec![]
    }

    async fn process_new_route(&mut self, event: &RoutedOrder) -> Vec<Action> {
        if event
            .request
            .orders
            .iter()
            .any(|o| self.done_orders.contains_key(&o.hash))
        {
            return vec![];
        }

        let OrderBatchData {
            orders,
            amount_required: amount_out_required,
            ..
        } = &event.request;

        // Filter out orders that are already being processed
        let filtered_orders: Vec<OrderData> = orders
            .iter()
            .filter(|o| !self.processing_orders.contains(&o.hash))
            .cloned()
            .collect();
        if filtered_orders.is_empty() {
            return vec![];
        }

        if let Some(profit) = self.get_profit_eth(event) {
            info!("profit: {}", profit);
            info!(
                "Sending trade: num trades: {} routed quote: {}, batch needs: {}, profit: {} wei",
                filtered_orders.len(),
                event.route.quote_gas_adjusted,
                amount_out_required,
                profit
            );
            let signed_orders = self
                .get_signed_orders(filtered_orders.clone())
                .unwrap_or_else(|e| {
                    error!("Error getting signed orders: {}", e);
                    vec![]
                });

            let fill_tx_request = self
                .build_fill(
                    self.client.clone(),
                    &self.executor_address,
                    signed_orders,
                    event,
                )
                .await;
            match fill_tx_request {
                Ok(fill_tx_request) => {
                    // Mark orders as processing to prevent duplicate execution
                    for order in filtered_orders.iter() {
                        self.add_to_processing(&order.hash);
                    }
                    
                    // Create a custom action that includes order hashes for failure handling
                    return vec![Action::SubmitTx(SubmitTxToMempool {
                        tx: fill_tx_request,
                        gas_bid_info: Some(GasBidInfo {
                            bid_percentage: U128::from(self.bid_percentage),
                            total_profit: U128::from(profit),
                        }),
                    })];
                }
                Err(e) => {
                    warn!(
                        "{} - Error building fill: {}",
                        event.request.orders[0].hash, e
                    );
                    // If we can't build the fill, mark orders as done to prevent infinite retries
                    for order in filtered_orders.iter() {
                        self.mark_as_done(&order.hash);
                    }
                    return vec![];
                }
            }
        } else {
            let metric_future = build_metric_future(
                self.cloudwatch_client.clone(),
                DimensionValue::Router02,
                CwMetrics::Unprofitable(event.request.chain_id),
                1.0,
            );
            if let Some(metric_future) = metric_future {
                send_metric_with_order_hash!(
                    &Arc::new(event.request.orders[0].hash.clone()),
                    metric_future
                );
            }
        }

        vec![]
    }

    /// Process new block events, updating the internal state.
    async fn process_new_block_event(&mut self, event: &NewBlock) -> Vec<Action> {
        self.last_block_number = event.number;
        self.last_block_timestamp = event.timestamp;

        info!(
            "Processing block {} at {}, Order set sizes -- open: {}, processing: {}, done: {}",
            event.number,
            event.timestamp,
            self.open_orders.len(),
            self.processing_orders.len(),
            self.done_orders.len()
        );
        self.handle_fills().await.unwrap_or_else(|e| {
            error!("{} - Error handling fills: {}", event.number, e);
        });
        self.update_open_orders();
        self.prune_done_orders();
        
        // Clean up stale processing orders every 10 blocks (approximately 30 seconds)
        if event.number % 10 == 0 {
            self.cleanup_stale_processing_orders();
        }

        self.batch_sender
            .send(self.get_order_batches().values().cloned().collect())
            .await
            .unwrap_or_else(|e| {
                error!("{} - Error sending order batches: {}", event.number, e);
            });

        vec![]
    }

    /// encode orders into generic signed orders
    fn get_signed_orders(&self, orders: Vec<OrderData>) -> Result<Vec<SignedOrder>> {
        let mut signed_orders: Vec<SignedOrder> = Vec::new();
        for batch in orders.iter() {
            match &batch.order {
                Order::LimitOrder(order) => {
                    signed_orders.push(SignedOrder {
                        order: Bytes::from(order.encode_inner()),
                        sig: Bytes::from_str(&batch.signature).unwrap_or_else(|e| {
                            error!("Error encoding signature: {}", e);
                            Bytes::new()
                        }),
                    });
                }
                _ => {
                    return Err(anyhow::anyhow!("Invalid order type"));
                }
            }
        }
        Ok(signed_orders)
    }

    fn get_order_batches(&self) -> HashMap<TokenInTokenOut, OrderBatchData> {
        let mut order_batches: HashMap<TokenInTokenOut, OrderBatchData> = HashMap::new();

        // group orders by token in and token out, excluding orders being processed
        self.open_orders
            .iter()
            .filter(|(_, order_data)| !self.processing_orders.contains(&order_data.hash))
            .for_each(|(_, order_data)| {
                let token_in_token_out = TokenInTokenOut {
                    token_in: order_data.resolved.input.token.clone(),
                    token_out: order_data.resolved.outputs[0].token.clone(),
                };

                let amount_in = order_data.resolved.input.amount;
                let amount_out = order_data
                    .resolved
                    .outputs
                    .iter()
                    .fold(Uint::from(0), |sum, output| sum.wrapping_add(output.amount));

                let amount_required = if order_data.order.is_exact_output() {
                    amount_in
                } else {
                    amount_out
                };
                // insert new order and update total amount out
                if let std::collections::hash_map::Entry::Vacant(e) =
                    order_batches.entry(token_in_token_out.clone())
                {
                    e.insert(OrderBatchData {
                        orders: vec![order_data.clone()],
                        amount_in,
                        amount_out,
                        amount_required,
                        token_in: order_data.resolved.input.token.clone(),
                        token_out: order_data.resolved.outputs[0].token.clone(),
                        chain_id: self.chain_id,
                    });
                } else {
                    let order_batch_data = order_batches.get_mut(&token_in_token_out).unwrap();
                    order_batch_data.orders.push(order_data.clone());
                    order_batch_data.amount_in = order_batch_data.amount_in.wrapping_add(amount_in);
                    order_batch_data.amount_required = order_batch_data
                        .amount_required
                        .wrapping_add(amount_required);
                }
            });
        order_batches
    }

    async fn handle_fills(&mut self) -> Result<()> {
        let reactor_address = get_reactor_address("limit").parse::<Address>().unwrap();
        // Look at last 10 blocks to catch any recent fills
        let from_block = if self.last_block_number > 10 { 
            self.last_block_number - 10 
        } else { 
            0 
        };
        let filter = Filter::new()
            .from_block(from_block)
            .to_block(self.last_block_number)
            .address(reactor_address)
            .event("Fill(bytes32,address,address,uint256)");

        // early return on error
        let logs = self.client.get_logs(&filter).await?;
        for log in logs {
            let order_hash = format!("0x{:x}", log.topics()[1]);
            // remove from open and processing
            info!("{} - Removing filled order", order_hash);
            self.open_orders.remove(&order_hash);
            self.remove_from_processing(&order_hash);
            // add to done
            self.done_orders.insert(
                order_hash.to_string(),
                self.current_timestamp()? + DONE_EXPIRY,
            );
        }

        Ok(())
    }

    fn prune_done_orders(&mut self) {
        let mut to_remove = Vec::new();
        for (order_hash, deadline) in self.done_orders.iter() {
            if *deadline < self.last_block_timestamp {
                to_remove.push(order_hash.clone());
            }
        }
        for order_hash in to_remove {
            self.done_orders.remove(&order_hash);
        }
    }

    fn update_open_orders(&mut self) {
        // TODO: this is nasty, plz cleanup
        let binding = self.open_orders.clone();
        let order_hashes: Vec<(&String, &OrderData)> = binding.iter().collect();
        for (order_hash, order_data) in order_hashes {
            match &order_data.order {
                Order::LimitOrder(order) => {
                    self.update_order_state(
                        order.clone(),
                        &order_data.signature,
                        &order_hash.to_string(),
                        order_data.route.as_ref(),
                    );
                }
                _ => {
                    error!("Invalid order type");
                }
            }
        }
    }

    fn mark_as_done(&mut self, order: &str) {
        if self.open_orders.contains_key(order) {
            self.open_orders.remove(order);
        }
        self.remove_from_processing(order);
        if !self.done_orders.contains_key(order) {
            self.done_orders
                .insert(order.to_string(), self.last_block_timestamp + DONE_EXPIRY);
        }
    }

    fn update_order_state(
        &mut self,
        order: LimitOrder,
        signature: &str,
        order_hash: &String,
        route: Option<&RouteInfo>,
    ) {
        let resolved = order.resolve(self.last_block_timestamp + BLOCK_TIME);
        let order_status: OrderStatus = match resolved {
            OrderResolution::Expired => OrderStatus::Done,
            OrderResolution::Invalid => OrderStatus::Done,
            OrderResolution::Resolved(resolved_order) => OrderStatus::Open(resolved_order),
            _ => OrderStatus::Done,
        };

        match order_status {
            OrderStatus::Done => {
                self.mark_as_done(order_hash);
            }
            OrderStatus::Open(resolved_order) => {
                if self.done_orders.contains_key(order_hash) {
                    info!("{} - Order already done, skipping", order_hash);
                    return;
                }
                if !self.open_orders.contains_key(order_hash) {
                    info!("{} - Adding new order", order_hash);
                }
                self.open_orders.insert(
                    order_hash.clone(),
                    OrderData {
                        order: Order::LimitOrder(order),
                        hash: order_hash.clone(),
                        signature: signature.to_string(),
                        resolved: resolved_order,
                        encoded_order: None,
                        route: route.cloned(),
                    },
                );
            }
            // Noop
            _ => {}
        }
    }

    /// Get order data from hash - helper method for batch failure handling
    fn get_order_data_from_hash(&self, order_hash: &str) -> Option<OrderData> {
        // First check if it's in open_orders (shouldn't be, but just in case)
        if let Some(order_data) = self.open_orders.get(order_hash) {
            return Some(order_data.clone());
        }
        
        // Check if we can reconstruct from done_orders (for debugging)
        if self.done_orders.contains_key(order_hash) {
            warn!("{} - Order found in done_orders during batch failure handling", order_hash);
            return None;
        }
        
        None
    }

    /// Handle specific order failure (e.g., cancelled order, insufficient funds)
    /// This method can be called when we know specific orders failed
    pub fn handle_order_failure(&mut self, order_hash: &str, failure_reason: &str) {
        info!("{} - Handling order failure: {}", order_hash, failure_reason);
        
        // Remove from processing_orders
        if self.processing_orders.contains(order_hash) {
            self.remove_from_processing(order_hash);
            info!("{} - Removed from processing_orders due to failure", order_hash);
            
            // Determine if order should be retried or marked as done based on failure reason
            match failure_reason {
                "OrderAlreadyFilled" | "OrderNotFillable" | "InvalidDeadline" | "OrderCancelled" => {
                    // These are permanent failures, mark as done
                    info!("{} - Permanent failure detected, marking as done", order_hash);
                    self.mark_as_done(order_hash);
                }
                "InsufficientETH" | "InsufficientToken" | "NativeTransferFailed" => {
                    // These are temporary failures, add back to open_orders for retry
                    if let Some(order_data) = self.get_order_data_from_hash(order_hash) {
                        if !self.open_orders.contains_key(order_hash) {
                            self.open_orders.insert(order_hash.to_string(), order_data);
                            info!("{} - Added back to open_orders for retry after temporary failure", order_hash);
                        }
                    }
                }
                _ => {
                    // Unknown failure reason, be conservative and mark as done
                    warn!("{} - Unknown failure reason '{}', marking as done", order_hash, failure_reason);
                    self.mark_as_done(order_hash);
                }
            }
        } else {
            warn!("{} - Order not found in processing_orders during failure handling", order_hash);
        }
    }

    /// Clean up orders that have been processing for too long
    /// This handles cases where transactions fail silently or get stuck
    fn cleanup_stale_processing_orders(&mut self) {
        let current_time = self.last_block_timestamp;
        let max_processing_time = 60; // 60 seconds max processing time
        
        let mut to_remove = Vec::new();
        
        for (order_hash, start_time) in self.processing_timestamps.iter() {
            if current_time - start_time > max_processing_time {
                to_remove.push(order_hash.clone());
            }
        }
        
        for order_hash in to_remove.clone() {
            if self.processing_orders.remove(&order_hash) {
                self.processing_timestamps.remove(&order_hash);
                info!("{} - Moved from processing_orders back to open_orders due to timeout", order_hash);
                
                // Try to reconstruct order data for retry
                if let Some(order_data) = self.get_order_data_from_hash(&order_hash) {
                    if !self.open_orders.contains_key(&order_hash) {
                        self.open_orders.insert(order_hash.clone(), order_data);
                    }
                } else {
                    warn!("{} - Could not reconstruct order data, marking as done", order_hash);
                    self.mark_as_done(&order_hash);
                }
            }
        }
        
        if !to_remove.is_empty() {
            info!("Cleaned up {} stale processing orders", to_remove.len());
        }
    }

    /// Add order to processing_orders with timestamp tracking
    fn add_to_processing(&mut self, order_hash: &str) {
        self.processing_orders.insert(order_hash.to_string());
        self.processing_timestamps.insert(order_hash.to_string(), self.last_block_timestamp);
    }

    /// Remove order from processing_orders and clear timestamp
    fn remove_from_processing(&mut self, order_hash: &str) {
        self.processing_orders.remove(order_hash);
        self.processing_timestamps.remove(order_hash);
    }
}
