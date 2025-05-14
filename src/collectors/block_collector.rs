use alloy::{
    network::AnyNetwork,
    primitives::{BlockHash, BlockNumber, BlockTimestamp},
    providers::{DynProvider, Provider},
    rpc::types::eth::BlockNumberOrTag,
};
use anyhow::Result;
use artemis_light::types::{Collector, CollectorStream};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::time::Duration;
use tokio_stream::StreamExt;

const BLOCK_POLLING_INTERVAL: Duration = Duration::from_secs(1);

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
        let provider = self.provider.clone();

        // First try to subscribe to blocks
        match provider.subscribe_blocks().await {
            Ok(sub) => {
                let stream = sub.into_stream().map(|header| NewBlock {
                    hash: header.hash,
                    number: header.number,
                    timestamp: header.timestamp,
                });
                Ok(Box::pin(stream))
            }
            Err(_) => {
                // Fallback to polling
                let stream = tokio_stream::wrappers::IntervalStream::new(tokio::time::interval(
                    BLOCK_POLLING_INTERVAL,
                ))
                .then(move |_| {
                    let provider = provider.clone();
                    async move {
                        match provider.get_block_by_number(BlockNumberOrTag::Latest).await {
                            Ok(Some(block)) => {
                                let header = &block.header;
                                Some(NewBlock {
                                    hash: header.hash,
                                    number: header.number,
                                    timestamp: header.timestamp,
                                })
                            }
                            Ok(None) => None,
                            Err(_) => None,
                        }
                    }
                })
                .filter_map(|x| x);
                Ok(Box::pin(stream))
            }
        }
    }
}
