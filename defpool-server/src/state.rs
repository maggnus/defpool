use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Instant;
use crate::config::{Config, Pool};
use crate::profitability::ProfitabilityScore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub address: String,
    pub pubkey: Option<String>, // Optional for SV1
    pub protocol: String, // "sv1" or "sv2"
}

#[derive(Clone)]
pub struct AppState {
    pub current_pool: Arc<RwLock<String>>,
    pub last_switch_time: Arc<RwLock<Instant>>,
    pub profitability_scores: Arc<RwLock<Vec<ProfitabilityScore>>>,
    pub pools: Vec<Pool>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        let initial_pool = config.pools.first()
            .expect("At least one pool must be configured")
            .name.clone();

        Self {
            current_pool: Arc::new(RwLock::new(initial_pool)),
            last_switch_time: Arc::new(RwLock::new(Instant::now())),
            profitability_scores: Arc::new(RwLock::new(Vec::new())),
            pools: config.pools,
        }
    }

    pub fn get_current_target(&self) -> Target {
        let current_pool_name = self.current_pool.read().unwrap().clone();
        let pool = self.pools.iter()
            .find(|p| p.name == current_pool_name)
            .expect("Current pool not found in configuration");

        Target {
            address: pool.address.clone(),
            pubkey: None, // V1 doesn't need pubkey
            protocol: "sv1".to_string(),
        }
    }

    pub fn switch_pool(&self, new_pool: String) {
        let mut current = self.current_pool.write().unwrap();
        *current = new_pool;
        *self.last_switch_time.write().unwrap() = Instant::now();
    }

    pub fn update_scores(&self, scores: Vec<ProfitabilityScore>) {
        *self.profitability_scores.write().unwrap() = scores;
    }
}
