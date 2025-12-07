use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct Pool {
    pub name: String,
    pub address: String,
    pub coin: String,
    #[allow(dead_code)] // Will be used for multi-algorithm support
    pub algorithm: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub listen_address: SocketAddr,
    pub pools: Vec<Pool>,
    pub profitability_check_interval_secs: u64,
    pub switch_threshold_percent: f64,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        
        // Validate pools
        if config.pools.is_empty() {
            anyhow::bail!("At least one pool must be configured");
        }
        
        Ok(config)
    }
}
