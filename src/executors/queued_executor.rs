use crate::executors::public_1559_executor::Public1559Executor;
use crate::strategies::keystore::KeyStore;
use crate::strategies::types::SubmitTxToMempoolWithExecutionMetadata;
use artemis_core::types::Executor;
use async_trait::async_trait;
use ethers::providers::Middleware;
use std::sync::Arc;

pub struct QueuedExecutor<M: Middleware + 'static, N: Middleware + 'static> {
    provider: Arc<M>,
    sender_client: Arc<N>,
    key_store: Arc<KeyStore>,
}

impl<M: Middleware + 'static, N: Middleware + 'static> QueuedExecutor<M, N> {
    pub fn new(provider: Arc<M>, sender_client: Arc<N>, key_store: Arc<KeyStore>) -> Self {
        Self {
            provider,
            sender_client,
            key_store,
        }
    }
}

#[async_trait]
impl<M, N> Executor<SubmitTxToMempoolWithExecutionMetadata> for QueuedExecutor<M, N>
where
    M: Middleware + 'static,
    M::Error: 'static,
    N: Middleware + 'static,
    N::Error: 'static,
{
    async fn execute(
        &self,
        action: SubmitTxToMempoolWithExecutionMetadata,
    ) -> Result<(), anyhow::Error> {
        let public_executor = Public1559Executor::<M, N>::new(
            self.provider.clone(),
            self.sender_client.clone(),
            self.key_store.clone(),
        );

        tokio::spawn(async move { public_executor.execute(action).await });

        Ok(())
    }
}
