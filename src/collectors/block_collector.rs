use anyhow::Result;
use async_trait::async_trait;
use ethers::{
    prelude::Middleware,
    providers::JsonRpcClient,
    types::{H256, U256, U64},
};
use futures::lock::Mutex;
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use std::{sync::Arc, time::Duration};
use tokio_stream::StreamExt;
use tokio::sync::mpsc::Receiver;

use crate::strategies::{shared::{ARBITRUM_CHAIN_ID, ARBITRUM_TESTNET_CHAIN_ID}, types::{Collector, CollectorStream, OrderState}};

const BLOCK_POLLING_INTERVAL: Duration = Duration::from_millis(200);

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct BlockCollector<M> {
    provider: Arc<M>,
    chain_id: u64,
    has_yielded_block: RwLock<bool>,
    order_state_receiver: Mutex<Receiver<OrderState>>,
}

/// A new block event, containing the block number and hash.
#[derive(Debug, Clone)]
pub struct NewBlock {
    pub hash: H256,
    pub number: U64,
    pub timestamp: U256,
}

impl<M> BlockCollector<M> {
    pub fn new(provider: Arc<M>, chain_id: u64, order_state_receiver: Receiver<OrderState>) -> Self {
        Self { 
            provider, 
            chain_id,
            has_yielded_block: RwLock::new(false),
            order_state_receiver: Mutex::new(order_state_receiver),
        }
    }

    fn is_arbitrum(&self) -> bool
    {
        return self.chain_id == ARBITRUM_CHAIN_ID || self.chain_id == ARBITRUM_TESTNET_CHAIN_ID
    }
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses polling to subscribe to new blocks.
/// It handles errors by recreating the filter when necessary.
#[async_trait]
impl<M> Collector<NewBlock> for BlockCollector<M>
where
    M: Middleware + Send + Sync,
    M::Provider: JsonRpcClient + Send + Sync,
    M::Error: std::fmt::Display + 'static,
{
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewBlock>> {
        // Initial block number to start tracking from
        let start_block = match self.provider.get_block_number().await {
            Ok(num) => num.as_u64(),
            Err(e) => {
                error!("Failed to get initial block number: {}", e);
                return Err(e.into());
            }
        };

        info!("Starting BlockCollector from block number: {}", start_block);

        let provider = self.provider.clone();
        let polling_interval = BLOCK_POLLING_INTERVAL;

        let stream = async_stream::stream! {
            let mut last_block = start_block;
            loop {
                let mut receiver = self.order_state_receiver.lock().await;
                let mut order_state = OrderState::default();
                // Collect all available messages without blocking
                while let Ok(new_state) = receiver.try_recv() {
                    // if new_state.open != 0 || new_state.processing != 0 {
                    // info!("Received new state: {:?}", new_state);
                    order_state = new_state;
                }
                // only collect blocks if there are open orders
                if *self.has_yielded_block.read().await && order_state.open == 0 && order_state.processing == 0 {
                    //info!("No open orders, skipping block collection");
                } else if self.is_arbitrum() {
                    // Polling approach for Arbitrum
                    match provider.get_block_number().await {
                        Ok(block_number) => {
                            let current_block = block_number.as_u64();
                            // Process all blocks from last_block + 1 up to current_block
                            for block_num in (last_block + 1)..=current_block {
                                match provider.get_block(block_num).await {
                                    Ok(Some(block)) => {
                                        yield NewBlock {
                                            hash: block.hash.unwrap(),
                                            number: U64::from(block_num),
                                            timestamp: block.timestamp,
                                        };
                                        *self.has_yielded_block.write().await = true;
                                    },
                                    Ok(None) => {
                                        warn!("Block {} not found.", block_num);
                                    },
                                    Err(e) => {
                                        error!("Error fetching block {}: {}.", block_num, e);
                                    }
                                }
                            }
                            last_block = current_block;
                        },
                        Err(e) => {
                            error!("Error fetching latest block number: {}. Retrying...", e);
                        }
                    }
                } else {
                    // Existing watch_blocks() approach for other networks
                    // Attempt to watch new blocks
                    let mut watcher = match provider.watch_blocks().await {
                        Ok(w) => {
                            info!("Successfully created new block watcher.");
                            w.interval(polling_interval).stream()
                        },
                        Err(e) => {
                            error!("Failed to create block watcher: {}. Retrying in 5 seconds...", e);
                            tokio::time::sleep(Duration::from_millis(100)).await;
                            continue;
                        }
                    };
                    match watcher.next().await {
                        Some(block_hash) => {
                            match provider.get_block(block_hash).await {
                                Ok(Some(block)) => {
                                    let block_number = block.number.unwrap().as_u64();
                                    let block_timestamp = block.timestamp;
                                    
                                    // Update last processed block number
                                    if block_number > last_block {
                                        last_block = block_number;
                                        
                                        yield NewBlock {
                                            hash: block.hash.unwrap(),
                                            number: U64::from(block_number),
                                            timestamp: block_timestamp,
                                        };
                                        *self.has_yielded_block.write().await = true;
                                    }
                                },
                                Ok(None) => {
                                    warn!("Received block hash {} but block not found.", block_hash);
                                },
                                Err(e) => {
                                    error!("Error fetching block {}: {}.", block_hash, e);
                                }
                            }
                        },
                        None => {
                            warn!("Block watcher stream ended unexpectedly. Recreating watcher...");
                            break; // Break inner loop to recreate watcher
                        }
                    }
                }
                // Delay before attempting to recreate the watcher to prevent tight loops
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        };

        Ok(Box::pin(stream))
    }

    // async fn handle_state_change(&self, state_change: StrategyStateChange) -> Result<()> {
    //     match state_change {
    //         StrategyStateChange::OrderState(order_state) => {
    //             *self.strategy_order_state.write().await = order_state;
    //         }
    //     }
    //     Ok(())
    // }
}
