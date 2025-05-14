use crate::collectors::{
    block_collector::NewBlock, uniswapx_order_collector::UniswapXOrder,
    uniswapx_route_collector::RoutedOrder,
};
use artemis_light::executors::mempool_executor::SubmitTxToMempool;
use uniswapx_rs::order::ResolvedOrder;

use super::priority_strategy::ExecutionMetadata;

/// Core Event enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Event {
    NewBlock(NewBlock),
    UniswapXOrder(Box<UniswapXOrder>),
    UniswapXRoute(Box<RoutedOrder>),
}

#[derive(Debug, Clone)]
pub struct SubmitTxToMempoolWithExecutionMetadata {
    pub execution: SubmitTxToMempool,
    pub metadata: ExecutionMetadata,
}

/// Core Action enum for the current strategy.
#[derive(Debug, Clone)]
pub enum Action {
    SubmitTx(Box<SubmitTxToMempool>),
    SubmitPublicTx(Box<SubmitTxToMempoolWithExecutionMetadata>),
}

/// Configuration for variables we need to pass to the strategy.
#[derive(Debug, Clone)]
pub struct Config {
    pub bid_percentage: Option<u64>,
    pub executor_address: String,
    pub min_block_percentage_buffer: Option<u64>,
    pub fallback_bid_scale_factor: Option<u64>,
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
