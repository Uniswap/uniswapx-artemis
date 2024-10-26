use super::{
    shared::{ReactorAddress, UniswapXStrategy},
    types::{Config, OrderStatus},
};
use crate::{
    aws_utils::cloudwatch_utils::{
        CwMetrics, DimensionName, DimensionValue, MetricBuilder, ARTEMIS_NAMESPACE,
    },
    collectors::{
        block_collector::NewBlock,
        uniswapx_order_collector::{OrderType, UniswapXOrder},
        uniswapx_route_collector::{OrderBatchData, OrderData, RoutedOrder},
    },
    strategies::types::SubmitTxToMempoolWithExecutionMetadata,
};
use alloy_primitives::Uint;
use anyhow::Result;
use artemis_core::executors::mempool_executor::{GasBidInfo, SubmitTxToMempool};
use artemis_core::types::Strategy;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use bindings_uniswapx::shared_types::SignedOrder;
use dashmap::DashMap;
use ethers::{
    providers::Middleware,
    types::{Address, Bytes, Filter, U256},
    utils::hex,
};
use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    RwLock,
};
use tracing::{error, info, warn};
use uniswapx_rs::order::{Order, OrderResolution, ResolutionContext, MPS, ResolvableOrder};

use super::types::{Action, Event};

const DONE_EXPIRY: u64 = 300;
// Base addresses
pub const WETH_ADDRESS: &str = "0x4200000000000000000000000000000000000006";

#[derive(Debug, Clone)]
pub struct ExecutionMetadata {
    // amount of quote token we can get
    pub quote: U256,
    // amount of quote token needed to fill the order
    pub amount_out_required: U256,
    pub order_hash: String,
}

impl ExecutionMetadata {
    pub fn new(quote: U256, amount_out_required: U256, order_hash: &str) -> Self {
        Self {
            quote,
            amount_out_required,
            order_hash: order_hash.to_owned(),
        }
    }

    pub fn calculate_priority_fee(&self, bid_percentage: u64) -> Option<U256> {
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

#[derive(Debug)]
#[allow(dead_code)]
pub struct UniswapXUniswapFill<M> {
    /// Ethers client.
    client: Arc<M>,
    // AWS Cloudwatch CLient for metrics propagation
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
    /// executor address
    executor_address: String,
    /// Amount of profits to bid in gas
    bid_percentage: u64,
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
    reactor_address: ReactorAddress,
    order_type: OrderType,
    block_time: u64,
}

impl<M: Middleware + 'static> UniswapXUniswapFill<M> {
    pub fn new(
        client: Arc<M>,
        cloudwatch_client: Option<Arc<CloudWatchClient>>,
        config: Config,
        sender: Sender<Vec<OrderBatchData>>,
        receiver: Receiver<RoutedOrder>,
        order_type: OrderType,
        block_time: u64,
    ) -> Self {
        info!("syncing state");

        let reactor_address = match order_type {
            OrderType::DutchV2 => ReactorAddress::DutchV2,
            OrderType::DutchV3 => ReactorAddress::DutchV3,
            OrderType::Priority => ReactorAddress::Priority,
        };
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
            order_type,
            reactor_address,
            block_time,
        }
    }
}

#[async_trait]
impl<M: Middleware + 'static> Strategy<Event, Action> for UniswapXUniswapFill<M> {
    async fn sync_state(&mut self) -> Result<()> {
        info!("syncing state");

        Ok(())
    }

    // Process incoming events, seeing if we can arb new orders, and updating the internal state on new blocks.
    async fn process_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::UniswapXOrder(order) => self.process_order_event(&order).await,
            Event::NewBlock(block) => self.process_new_block_event(&block).await,
            Event::UniswapXRoute(route) => self.process_new_route(&route).await,
        }
    }
}

impl<M: Middleware + 'static> UniswapXStrategy<M> for UniswapXUniswapFill<M> {}

impl<M: Middleware + 'static> UniswapXUniswapFill<M> {
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

    fn decode_order(&self, encoded_order: &str) -> Result<Order, Box<dyn Error>> {
        let encoded_order = if let Some(stripped) = encoded_order.strip_prefix("0x") {
            stripped
        } else {
            encoded_order
        };
        let order_hex = hex::decode(encoded_order)?;
        if let Ok(order) = Order::decode_inner(&order_hex, false) {
            return Ok(order);
        }

        Err("Failed to decode order".into())
    }

    /// Process new order events that we fetch from UniswapX API
    /// - skip if we are already tracking this order
    /// - otherwise decode and process:
    ///     - skip if we have already processed this order
    ///     - immediately send for execution if order is fillable
    ///     - otherwise add to new_orders to be processed on new block event
    async fn process_order_event(&self, event: &UniswapXOrder) -> Option<Action> {
        if *self.last_block_timestamp.read().await == 0 {
            info!(
                "{} - skipping processing new order event (no timestamp)",
                event.order_hash
            );
            return None;
        }
        if self.new_orders.contains_key(&event.order_hash)
            || self.processing_orders.contains_key(&event.order_hash)
        {
            info!(
                "{} - skipping processing new order event (already tracking)",
                event.order_hash
            );
            return None;
        }

        let order = self
            .decode_order(&event.encoded_order)
            .map_err(|e| error!("failed to decode: {}", e))
            .ok()?;

        let order_hash = event.order_hash.clone();
        let resolved_order = order.resolve(ResolutionContext {
            block_number: *self.last_block_number.read().await,
            timestamp: *self.last_block_timestamp.read().await + self.block_time,
            priority_fee: Uint::from(0),
        });

        let order_status = match resolved_order {
            OrderResolution::Expired | OrderResolution::Invalid => OrderStatus::Done,
            OrderResolution::NotFillableYet(resolved) => OrderStatus::NotFillableYet(resolved),
            OrderResolution::Resolved(resolved) => OrderStatus::Open(resolved),
        };

        match order_status {
            OrderStatus::Open(resolved) => {
                if self.done_orders.contains_key(&order_hash) {
                    info!(
                        "{} - New order processing already done, skipping",
                        order_hash
                    );
                    return None;
                }
                let order_data = OrderData {
                    order,
                    hash: order_hash.clone(),
                    signature: event.signature.clone(),
                    resolved,
                };
                self.processing_orders
                    .insert(order_hash.clone(), order_data.clone());

                info!(
                    "{} - Sending incoming order immediately for routing and execution at block {}",
                    order_hash,
                    *self.last_block_number.read().await
                );
                let order_batch = self.get_order_batch(&order_data);
                self.try_send_order_batch(order_batch, order_hash, order_data)
                    .await;
            }
            OrderStatus::NotFillableYet(resolved) => {
                if let Order::PriorityOrder(priority_order) = &order {
                    info!(
                        "{} - Priority order not fillable yet - last block: {}, target: {}",
                        order_hash,
                        *self.last_block_number.read().await,
                        priority_order.cosignerData.auctionTargetBlock
                    );
                } else {
                    info!(
                        "{} - order not fillable yet",
                        order_hash
                    );
                }
                self.new_orders.insert(
                    order_hash.clone(),
                    OrderData {
                        order,
                        hash: order_hash.clone(),
                        signature: event.signature.clone(),
                        resolved,
                    },
                );
            }
            OrderStatus::Done => {
                // info!("{} - Order already done, skipping", order_hash);
            }
        }

        None
    }

    async fn process_new_route(&mut self, event: &RoutedOrder) -> Option<Action> {
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
            return None;
        }

        let OrderBatchData {
            // orders,
            orders,
            amount_out_required,
            ..
        } = &event.request;

        match &orders[0].order {
            Order::V2DutchOrder(_) | Order::V3DutchOrder(_) => {
                if let Some(profit) = self.get_profit_eth(&RoutedOrder {
                    request: OrderBatchData {
                        orders: orders.clone(),
                        amount_out_required: *amount_out_required,
                        ..event.request.clone()
                    },
                    route: event.route.clone(),
                }) {
                    info!(
                        "Sending trade: num trades: {} routed quote: {}, batch needs: {}, profit: {} wei",
                        orders.len(),
                        event.route.quote_gas_adjusted,
                        amount_out_required,
                        profit
                    );

                    let signed_orders = self.get_signed_orders(orders.clone()).ok()?;
                    return Some(Action::SubmitTx(SubmitTxToMempool {
                        tx: self
                            .build_fill(
                                self.client.clone(),
                                &self.executor_address,
                                signed_orders,
                                self.reactor_address.clone(),
                                event,
                            )
                            .await
                            .ok()?,
                        gas_bid_info: Some(GasBidInfo {
                            bid_percentage: self.bid_percentage,
                            total_profit: profit,
                        }),
                    }));
                }

                return None
            },
            Order::PriorityOrder(_) => {
            if let Some(metadata) = self.get_execution_metadata(event) {
                info!(
                    "{} - Sending trade: num trades: {} routed quote: {}, batch needs: {}",
                    metadata.order_hash,
                    orders.len(),
                    event.route.quote,
                    amount_out_required,
                );

                let signed_orders = self.get_signed_orders(orders.clone()).ok()?;
                return Some(Action::SubmitPublicTx(
                    SubmitTxToMempoolWithExecutionMetadata {
                        execution: SubmitTxToMempool {
                            tx: self
                                .build_fill(
                                    self.client.clone(),
                                    &self.executor_address,
                                    signed_orders,
                                    self.reactor_address.clone(),
                                    event,
                                )
                                .await
                                .ok()?,
                            gas_bid_info: Some(GasBidInfo {
                                bid_percentage: self.bid_percentage,
                                total_profit: U256::from(0),
                            }),
                        },
                        metadata,
                    },
                    ));
                }
                return None
            }
        }
    }

    /// Process new block events
    /// - update the block number and timestamp
    /// - check for fills from block logs and remove from processing_orders
    /// - check new_orders for orders that are now fillable and send for execution
    /// - prune done orders
    async fn process_new_block_event(&mut self, event: &NewBlock) -> Option<Action> {
        *self.last_block_number.write().await = event.number.as_u64();
        *self.last_block_timestamp.write().await = event.timestamp.as_u64();

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

        self.check_new_orders_for_processing().await;

        if *self.last_block_number.read().await % 100 == 0 {
            self.prune_done_orders().await;
            if let Some(cw) = &self.cloudwatch_client {
                let metric_future = cw
                    .put_metric_data()
                    .namespace(ARTEMIS_NAMESPACE)
                    .metric_data(
                        MetricBuilder::new(CwMetrics::LatestBlock)
                            .add_dimension(
                                DimensionName::Service.as_ref(),
                                DimensionValue::PriorityExecutor.as_ref(),
                            )
                            .with_value(event.number.as_u64() as f64)
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

        None
    }

    /// encode orders into generic signed orders
    fn get_signed_orders(&self, orders: Vec<OrderData>) -> Result<Vec<SignedOrder>> {
        let mut signed_orders: Vec<SignedOrder> = Vec::new();
        for batch in orders.iter() {
            signed_orders.push(SignedOrder {
                order: Bytes::from(batch.order.encode_inner()),
                sig: Bytes::from_str(&batch.signature)?,
            });
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
        }
    }

    async fn handle_fills(&self) -> Result<()> {
        let reactor_address = self.reactor_address.as_str().parse::<Address>().unwrap();
        let filter = Filter::new()
            .select(*self.last_block_number.read().await)
            .address(reactor_address)
            .event("Fill(bytes32,address,address,uint256)");

        let logs = self.client.get_logs(&filter).await
            .unwrap_or_else(|e| {
                error!("Failed to get logs: {}", e);
                Vec::new()
        });

        for log in logs {
            let order_hash = format!("0x{:x}", log.topics[1]);
            info!(
                "{} - Removing filled order from processing_orders",
                order_hash
            );
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
        RoutedOrder { request, route }: &RoutedOrder,
    ) -> Option<ExecutionMetadata> {
        let quote = U256::from_str_radix(&route.quote, 10).ok()?;
        let amount_out_required =
            U256::from_str_radix(&request.amount_out_required.to_string(), 10).ok()?;
        if quote.le(&amount_out_required) {
            return None;
        }

        Some(ExecutionMetadata {
            quote,
            amount_out_required,
            order_hash: request.orders[0].hash.clone()
        })
    }

    /// process an order status and the associated resolved order
    /// if order is done, mark as done
    /// if order is not fillable yet, do nothing
    /// if order is open, send for execution
    async fn process_new_order(
        &mut self,
        order: Order,
        order_hash: String,
        signature: &str,
    ) -> Result<()> {
        let resolved = order.resolve(ResolutionContext {
            block_number: *self.last_block_number.read().await,
            timestamp: *self.last_block_timestamp.read().await + self.block_time,
            priority_fee: Uint::from(0),
        });
        let order_status = match resolved {
            OrderResolution::Expired => OrderStatus::Done,
            OrderResolution::Invalid => OrderStatus::Done,
            OrderResolution::NotFillableYet(resolved_order) => {
                OrderStatus::NotFillableYet(resolved_order)
            }
            OrderResolution::Resolved(resolved_order) => OrderStatus::Open(resolved_order),
        };

        match order_status {
            OrderStatus::Done => {
                self.new_orders.remove(&order_hash);
                self.done_orders
                    .insert(order_hash, self.current_timestamp()? + DONE_EXPIRY);
            }
            OrderStatus::NotFillableYet(_) => {
                if let Order::PriorityOrder(priority_order) = &order {
                    info!(
                        "{} - Order not fillable yet at latest block: {}; target: {}",
                        order_hash,
                        *self.last_block_number.read().await,
                        priority_order.cosignerData.auctionTargetBlock
                    );
                } else {
                    info!(
                        "{} - order not fillable yet",
                        order_hash
                    );
                }
            }
            OrderStatus::Open(resolved_order) => {
                let order_data = OrderData {
                    order,
                    hash: order_hash.to_string(),
                    signature: signature.to_string(),
                    resolved: resolved_order,
                };
                self.new_orders.remove(&order_hash);
                self.processing_orders
                    .insert(order_hash.to_string(), order_data.clone());
                info!(
                    "{} - Sending order for routing and execution at latest block {}",
                    order_hash,
                    *self.last_block_number.read().await
                );
                let order_batch = self.get_order_batch(&order_data);
                self.try_send_order_batch(order_batch, order_hash, order_data)
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
    async fn check_new_orders_for_processing(&mut self) {
        let order_hashes = self
            .new_orders
            .iter()
            .map(|entry| entry.key().clone())
            .collect::<Vec<String>>();

        for order_hash in order_hashes {
            if let Some(order_data) = self.get_new_order(&order_hash) {
                // Check if the order type matches self.order_type
                let order_matches = match (&order_data.order, &self.order_type) {
                    (Order::PriorityOrder(_), OrderType::Priority) => true,
                    (Order::V2DutchOrder(_), OrderType::DutchV2) => true,
                    (Order::V3DutchOrder(_), OrderType::DutchV3) => true,
                    _ => false,
                };

                if !order_matches {
                    error!("{} - Order type doesn't match the expected type for this strategy", order_hash);
                    continue;
                }

                if let Err(e) = self
                    .process_new_order(
                        order_data.order.clone(),
                        order_hash.clone(),
                        &order_data.signature,
                    )
                    .await
                {
                    error!("Error processing new order: {}", e);
                }
            }
        }
    }

    async fn try_send_order_batch(
        &self,
        order_batch: OrderBatchData,
        order_hash: String,
        order_data: OrderData,
    ) {
        match self.batch_sender.send(vec![order_batch]).await {
            Ok(_) => (),
            Err(e) => {
                error!(
                    "{} - Failed to send batch: {}; moving order back to new_orders",
                    order_hash, e
                );
                self.processing_orders.remove(&order_hash);
                self.new_orders.insert(order_hash.clone(), order_data);
            }
        }
    }

    fn get_profit_eth(&self, RoutedOrder { request, route }: &RoutedOrder) -> Option<U256> {
        let quote = U256::from_str_radix(&route.quote, 10).ok()?;
        let amount_out_required =
            U256::from_str_radix(&request.amount_out_required.to_string(), 10).ok()?;
        if quote.le(&amount_out_required) {
            return None;
        }
        let profit_quote = quote.saturating_sub(amount_out_required);

        if request.token_out.to_lowercase() == WETH_ADDRESS.to_lowercase() {
            return Some(profit_quote);
        }

        let gas_use_eth = U256::from_str_radix(&route.gas_use_estimate, 10)
            .ok()?
            .saturating_mul(U256::from_str_radix(&route.gas_price_wei, 10).ok()?);
        profit_quote
            .saturating_mul(gas_use_eth)
            .checked_div(U256::from_str_radix(&route.gas_use_estimate_quote, 10).ok()?)
    }
}

