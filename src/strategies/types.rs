use crate::collectors::{
    block_collector::NewBlock, uniswapx_order_collector::UniswapXOrder,
    uniswapx_route_collector::RoutedOrder,
};
use artemis_core::executors::mempool_executor::SubmitTxToMempool;
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
    SubmitTx(SubmitTxToMempool),
    SubmitPublicTx(SubmitTxToMempoolWithExecutionMetadata),
}

/// Configuration for variables we need to pass to the strategy.
#[derive(Debug, Clone)]
pub struct Config {
    pub bid_bps: Option<u128>,
    pub executor_address: String,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TokenInTokenOut {
    pub token_in: String,
    pub token_out: String,
    pub exact_out: bool,
}

#[derive(Debug, Clone)]
pub enum OrderStatus {
    Open(ResolvedOrder),
    NotFillableYet(ResolvedOrder),
    Done,
}
