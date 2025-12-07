use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub listen_address: SocketAddr,
    pub initial_target_address: String,
    pub initial_target_pubkey: Option<String>,
    pub initial_target_protocol: String,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;

        // Environment overrides
        if let Ok(addr) = std::env::var("DEFPOOL_SERVER_LISTEN_ADDRESS") {
            if let Ok(socket_addr) = addr.parse() {
                config.listen_address = socket_addr;
            }
        }
        if let Ok(target) = std::env::var("DEFPOOL_SERVER_INITIAL_TARGET_ADDRESS") {
            config.initial_target_address = target;
        }
        if let Ok(pubkey) = std::env::var("DEFPOOL_SERVER_INITIAL_TARGET_PUBKEY") {
            config.initial_target_pubkey = Some(pubkey);
        }
        if let Ok(protocol) = std::env::var("DEFPOOL_SERVER_INITIAL_TARGET_PROTOCOL") {
            config.initial_target_protocol = protocol;
        }

        Ok(config)
    }
}
