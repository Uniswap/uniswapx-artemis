use crate::collectors::{
    block_collector::NewBlock, uniswapx_order_collector::UniswapXOrderResponse,
    uniswapx_route_collector::RoutedOrder,
};
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
use uniswapx_rs::order::ResolvedOrder;

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

