use anyhow::Result;
use crate::collectors::{
    block_collector::NewBlock, uniswapx_order_collector::UniswapXOrderResponse,
    uniswapx_route_collector::RoutedOrder,
};
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
use async_trait::async_trait;
use uniswapx_rs::order::ResolvedOrder;
use std::pin::Pin;
use tokio_stream::Stream;
use tokio_stream::StreamExt;

use super::uniswapx_strategy::ExecutionMetadata;

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    NewBlock(NewBlock),
    UniswapXOrderResponse(Box<UniswapXOrderResponse>),
    UniswapXRoute(Box<RoutedOrder>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrderState {
    pub open: usize,
    pub processing: usize,
    pub done: usize,
}

impl Default for OrderState {
    fn default() -> Self {
        Self {
            open: 0,
            processing: 0,
            done: 0,
        }
    }
}

/// Core state change enum for the current strategy.
#[derive(Debug, Clone)]
pub enum StrategyStateChange {
    OrderState(OrderState),
}

#[derive(Debug, Clone)]
pub struct SubmitTxToMempoolWithExecutionMetadata {
    pub execution: SubmitTxToMempool,
    pub metadata: ExecutionMetadata,
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitTx(SubmitTxToMempool),
    SubmitPublicTx(SubmitTxToMempoolWithExecutionMetadata),
}

/// Configuration for variables we need to pass to the strategy.
#[derive(Debug, Clone)]
pub struct Config {
    pub bid_percentage: u64,
    pub executor_address: String,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TokenInTokenOut {
    pub token_in: String,
    pub token_out: String,
}

#[derive(Debug, Clone)]
pub enum OrderStatus {
    Open(ResolvedOrder),
    NotFillableYet(ResolvedOrder),
    Done,
}

/// Strategy trait, which defines the core logic for each opportunity.
#[async_trait]
pub trait StatefulStrategy<E, A>: Send + Sync {
    /// Sync the initial state of the strategy if needed, usually by fetching
    /// onchain data.
    async fn sync_state(&mut self) -> Result<()>;

    /// Process an event, and return an action if needed.
    async fn process_event(&mut self, event: E) -> Option<A>;
    
    /// Get any state changes since the last check
    async fn get_state_change(&self) -> Option<StrategyStateChange>;
}

/// Collector trait, which defines a source of events.
#[async_trait]
pub trait Collector<E>: Send + Sync {
    /// Returns the core event stream for the collector.
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, E>>;

    async fn handle_state_change(&self, state_change: StrategyStateChange) -> Result<()>;
}

/// A stream of events emitted by a [Collector](Collector).
pub type CollectorStream<'a, E> = Pin<Box<dyn Stream<Item = E> + Send + 'a>>;

/// CollectorMap is a wrapper around a [Collector](Collector) that maps outgoing
/// events to a different type.
pub struct CollectorMap<E, F> {
    collector: Box<dyn Collector<E>>,
    f: F,
}
impl<E, F> CollectorMap<E, F> {
    pub fn new(collector: Box<dyn Collector<E>>, f: F) -> Self {
        Self { collector, f }
    }
}

#[async_trait]
impl<E1, E2, F> Collector<E2> for CollectorMap<E1, F>
where
    E1: Send + Sync + 'static,
    E2: Send + Sync + 'static,
    F: Fn(E1) -> E2 + Send + Sync + Clone + 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, E2>> {
        let stream = self.collector.get_event_stream().await?;
        let f = self.f.clone();
        let stream = stream.map(f);
        Ok(Box::pin(stream))
    }

    async fn handle_state_change(&self, state_change: StrategyStateChange) -> Result<()> {
        self.collector.handle_state_change(state_change).await
    }
}
