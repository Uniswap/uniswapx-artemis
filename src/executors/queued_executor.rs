use crate::executors::priority_executor::PriorityExecutor;
use crate::strategies::keystore::KeyStore;
use crate::strategies::types::SubmitTxToMempoolWithExecutionMetadata;
use alloy::{network::AnyNetwork, providers::DynProvider};
use artemis_light::types::Executor;
use async_trait::async_trait;
use aws_sdk_cloudwatch::Client as CloudWatchClient;
use std::sync::Arc;

pub struct QueuedExecutor {
    provider: Arc<DynProvider<AnyNetwork>>,
    sender_client: Arc<DynProvider<AnyNetwork>>,
    key_store: Arc<KeyStore>,
    cloudwatch_client: Option<Arc<CloudWatchClient>>,
}

impl QueuedExecutor {
    pub fn new(
        provider: Arc<DynProvider<AnyNetwork>>,
        sender_client: Arc<DynProvider<AnyNetwork>>,
        key_store: Arc<KeyStore>,
        cloudwatch_client: Option<Arc<CloudWatchClient>>,
    ) -> Self {
        Self {
            provider,
            sender_client,
            key_store,
            cloudwatch_client,
        }
    }
}

#[async_trait]
impl Executor<SubmitTxToMempoolWithExecutionMetadata> for QueuedExecutor {
    async fn execute(
        &self,
        action: SubmitTxToMempoolWithExecutionMetadata,
    ) -> Result<(), anyhow::Error> {
        let public_executor = PriorityExecutor::new(
            self.provider.clone(),
            self.sender_client.clone(),
            self.key_store.clone(),
            self.cloudwatch_client.clone(),
        );

        tokio::spawn(async move { public_executor.execute(action).await });

        Ok(())
    }
}
