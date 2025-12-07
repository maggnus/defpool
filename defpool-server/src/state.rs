use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub address: String,
    pub pubkey: Option<String>, // Optional for SV1
    pub protocol: String, // "sv1" or "sv2"
}

#[derive(Clone)]
pub struct AppState {
    pub current_target: Arc<RwLock<Target>>,
}

use crate::config::Config;

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            current_target: Arc::new(RwLock::new(Target {
                address: config.initial_target_address,
                pubkey: config.initial_target_pubkey,
                protocol: config.initial_target_protocol,
            })),
        }
    }
}
