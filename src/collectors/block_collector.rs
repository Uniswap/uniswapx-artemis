use alloy::{
    network::AnyNetwork,
    primitives::{BlockHash, BlockNumber, BlockTimestamp},
    providers::{DynProvider, Provider},
};
use anyhow::Result;
use artemis_core::types::{Collector, CollectorStream};
use async_trait::async_trait;
use std::sync::Arc;
use tokio_stream::StreamExt;
use tracing::{error, info};

/// A collector that listens for new blocks, and generates a stream of
/// [events](NewBlock) which contain the block number and hash.
pub struct BlockCollector {
    provider: Arc<DynProvider<AnyNetwork>>,
}

/// A new block event, containing the block number and hash.
#[derive(Debug, Clone)]
pub struct NewBlock {
    pub hash: BlockHash,
    pub number: BlockNumber,
    pub timestamp: BlockTimestamp,
}

impl BlockCollector {
    pub fn new(provider: Arc<DynProvider<AnyNetwork>>) -> Self {
        Self { provider }
    }
}

/// Implementation of the [Collector](Collector) trait for the [BlockCollector](BlockCollector).
/// This implementation uses polling to subscribe to new blocks.
#[async_trait]
impl Collector<NewBlock> for BlockCollector {
    async fn get_event_stream(&self) -> Result<CollectorStream<'_, NewBlock>> {
        // Initial block number to start tracking from
        let start_block = match self.provider.get_block_number().await {
            Ok(num) => num,
            Err(e) => {
                error!("Failed to get initial block number: {}", e);
                return Err(e.into());
            }
        };

        info!("Starting BlockCollector from block number: {}", start_block);

        let provider = self.provider.clone();

        let sub = provider.subscribe_blocks().await?;
        let stream = sub.into_stream().map(|header| NewBlock {
            hash: header.hash,
            number: header.number,
            timestamp: header.timestamp,
        });

        Ok(Box::pin(stream))
    }
}
