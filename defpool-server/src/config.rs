use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TargetType {
    Pool,
    Daemon,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MiningTarget {
    pub name: String,
    #[serde(rename = "type")]
    #[allow(dead_code)] // Will be used for daemon vs pool logic
    pub target_type: TargetType,
    pub address: String,
    pub coin: String,
    #[allow(dead_code)] // Will be used for multi-algorithm support
    pub algorithm: String,
    #[allow(dead_code)] // Will be used for daemon RPC calls
    pub daemon_rpc_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub listen_address: SocketAddr,
    pub targets: Vec<MiningTarget>,
    pub profitability_check_interval_secs: u64,
    pub switch_threshold_percent: f64,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        
        // Validate targets
        if config.targets.is_empty() {
            anyhow::bail!("At least one mining target must be configured");
        }
        
        Ok(config)
    }
}

