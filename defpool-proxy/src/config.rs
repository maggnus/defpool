use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_endpoint: String,
    pub listen_address: SocketAddr,
    pub default_wallet: Option<String>,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;
        
        // Override with environment variable if present
        if let Ok(endpoint) = std::env::var("DEFPOOL_SERVER_ENDPOINT") {
            config.server_endpoint = endpoint;
        }
        
        Ok(config)
    }
}
