use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use ethers::{
    prelude::Middleware,
    providers::JsonRpcClient,
    types::{BlockNumber, H256, U256, U64},
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tracing::{error, info, warn};

const BLOCK_POLLING_INTERVAL: Duration = Duration::from_millis(200);

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct BlockCollector<M> {
    provider: Arc<M>,
}

/// A new block event, containing the block number and hash.
#[derive(Debug, Clone)]
pub struct NewBlock {
    pub hash: H256,
    pub number: U64,
    pub timestamp: U256,
}

impl<M> BlockCollector<M> {
    pub fn new(provider: Arc<M>) -> Self {
        Self { provider }
    }
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses polling to subscribe to new blocks.
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

        let stream = async_stream::stream! {
            let mut last_block = start_block;

            loop {
                match provider.get_block(BlockNumber::Latest).await {
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
                        };
                    }
                    Ok(None) => {
                        warn!("Fetched latest block but it's empty");
                    },
                    Err(e) => {
                        error!("Error fetching block: {}.", e);
                    }
                }
                sleep(BLOCK_POLLING_INTERVAL).await;
            }
        };

        Ok(Box::pin(stream))
    }
}
