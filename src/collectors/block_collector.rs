use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use ethers::{
    prelude::Middleware,
    providers::JsonRpcClient,
    types::{H256, U256, U64},
};
use tracing::{error, info, warn};
use std::{sync::Arc, time::Duration};
use tokio_stream::StreamExt;

const BLOCK_POLLING_INTERVAL: Duration = Duration::from_millis(200);

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct BlockCollector<M> {
    provider: Arc<M>,
    chain_id: u64,
}

/// A new block event, containing the block number and hash.
#[derive(Debug, Clone)]
pub struct NewBlock {
    pub hash: H256,
    pub number: U64,
    pub timestamp: U256,
}

impl<M> BlockCollector<M> {
    pub fn new(provider: Arc<M>, chain_id: u64) -> Self {
        Self { provider, chain_id }
    }

    // Add this new method to check if the network is Arbitrum
    fn is_arbitrum(&self) -> bool
    {
        return self.chain_id == 42161 || self.chain_id == 421613
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
                if self.is_arbitrum() {
                    // Polling approach for Arbitrum
                    loop {
                        match provider.get_block_number().await {
                            Ok(block_number) => {
                                if block_number.as_u64() > last_block {
                                    match provider.get_block(block_number).await {
                                        Ok(Some(block)) => {
                                            last_block = block_number.as_u64();
                                            yield NewBlock {
                                                hash: block.hash.unwrap(),
                                                number: block_number,
                                                timestamp: block.timestamp,
                                            };
                                        },
                                        Ok(None) => {
                                            warn!("Block {} not found.", block_number);
                                        },
                                        Err(e) => {
                                            error!("Error fetching block {}: {}.", block_number, e);
                                        }
                                    }
                                }
                            },
                            Err(e) => {
                                error!("Error fetching latest block number: {}. Retrying...", e);
                            }
                        }
                        tokio::time::sleep(polling_interval).await;
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

                    // Iterate over incoming block hashes
                    loop {
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
                // Delay before attempting to recreate the watcher to prevent tight loops
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        };

        Ok(Box::pin(stream))
    }
}
