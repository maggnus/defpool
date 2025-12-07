use async_trait::async_trait;
use anyhow::Result;
use serde::Deserialize;
use tracing::{debug, warn};

/// Abstract interface for fetching network difficulty
#[async_trait]
pub trait DifficultyProvider: Send + Sync {
    /// Get the current network difficulty for a coin
    async fn get_difficulty(&self, coin: &str) -> Result<f64>;
}

/// MoneroBlocks API difficulty provider
pub struct PoolApiProvider {
    client: reqwest::Client,
}

impl PoolApiProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn fetch_xmr_difficulty(&self) -> Result<f64> {
        // MoneroBlocks.info API
        let url = "https://moneroblocks.info/api/get_stats";
        
        debug!("Fetching XMR difficulty from MoneroBlocks");

        let response = self.client
            .get(url)
            .header("User-Agent", "DefPool/1.0")
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            warn!("MoneroBlocks API returned status: {}", response.status());
            anyhow::bail!("MoneroBlocks API error: {}", response.status());
        }

        let data: MoneroBlocksResponse = response.json().await?;
        
        let difficulty = data.difficulty as f64;
        debug!("XMR network difficulty: {}", difficulty);
        Ok(difficulty)
    }
}

#[derive(Deserialize)]
struct MoneroBlocksResponse {
    difficulty: u64,
}

impl Default for PoolApiProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DifficultyProvider for PoolApiProvider {
    async fn get_difficulty(&self, coin: &str) -> Result<f64> {
        match coin {
            "XMR" => self.fetch_xmr_difficulty().await,
            _ => {
                warn!("No difficulty provider for coin: {}, using default", coin);
                Ok(100_000.0) // Fallback
            }
        }
    }
}

/// Mock difficulty provider for testing
#[allow(dead_code)]
pub struct MockDifficultyProvider {
    default_difficulty: f64,
}

#[allow(dead_code)]
impl MockDifficultyProvider {
    pub fn new(default_difficulty: f64) -> Self {
        Self { default_difficulty }
    }
}

#[async_trait]
impl DifficultyProvider for MockDifficultyProvider {
    async fn get_difficulty(&self, _coin: &str) -> Result<f64> {
        Ok(self.default_difficulty)
    }
}

