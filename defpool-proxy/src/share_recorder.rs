use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

/// Share submission to record on server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareSubmission {
    pub wallet_address: String,
    pub worker_name: String,
    pub target_name: String,
    pub difficulty: f64,
    pub valid: bool,
}

/// Client for recording shares to the server
pub struct ShareRecorder {
    client: reqwest::Client,
    server_url: String,
}

impl ShareRecorder {
    pub fn new(server_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            server_url,
        }
    }

    /// Record a share submission
    pub async fn record_share(&self, submission: ShareSubmission) -> Result<()> {
        let url = format!("{}/api/v1/shares", self.server_url);
        
        debug!(
            "Recording share: wallet={}, worker={}, target={}, difficulty={}, valid={}",
            submission.wallet_address,
            submission.worker_name,
            submission.target_name,
            submission.difficulty,
            submission.valid
        );

        let response = self.client
            .post(&url)
            .json(&submission)
            .send()
            .await?;

        if !response.status().is_success() {
            warn!("Failed to record share: {}", response.status());
            anyhow::bail!("Server returned error: {}", response.status());
        }

        debug!("Share recorded successfully");
        Ok(())
    }

    /// Get current target name from server
    pub async fn get_current_target(&self) -> Result<String> {
        let url = format!("{}/api/v1/targets/current", self.server_url);
        
        let target_name = self.client
            .get(&url)
            .send()
            .await?
            .json::<String>()
            .await?;

        Ok(target_name)
    }
}
