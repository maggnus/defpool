use crate::db::{models::*, repository::ShareRepository};
use anyhow::Result;
use std::sync::Arc;
use tracing::{info, warn};

/// Accounting service for share tracking and miner stats
pub struct AccountingService {
    repository: Arc<ShareRepository>,
}

impl AccountingService {
    pub fn new(repository: Arc<ShareRepository>) -> Self {
        Self { repository }
    }

    /// Record a share submission
    pub async fn record_share(&self, submission: ShareSubmission) -> Result<()> {
        info!(
            "Recording share: wallet={}, worker={}, target={}, difficulty={}, valid={}",
            submission.wallet_address,
            submission.worker_name,
            submission.target_name,
            submission.difficulty,
            submission.valid
        );

        match self.repository.create_share(&submission).await {
            Ok(share) => {
                info!("Share recorded: id={}", share.id);
                Ok(())
            }
            Err(e) => {
                warn!("Failed to record share: {}", e);
                Err(e)
            }
        }
    }

    /// Get miner statistics
    pub async fn get_miner_stats(&self, wallet_address: &str) -> Result<Option<MinerStats>> {
        self.repository.get_miner_stats(wallet_address).await
    }

    /// Get miner's workers
    pub async fn get_miner_workers(&self, wallet_address: &str) -> Result<Vec<Worker>> {
        self.repository.get_miner_workers(wallet_address).await
    }

    /// Get pool-wide statistics
    pub async fn get_pool_stats(&self) -> Result<crate::api::PoolStats> {
        self.repository.get_pool_stats().await
    }
}
