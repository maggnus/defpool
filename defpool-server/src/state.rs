use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Instant;
use crate::config::{Config, MiningTarget};
use crate::profitability::ProfitabilityScore;
use crate::accounting::AccountingService;
use crate::payout::PayoutService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub address: String,
    pub pubkey: Option<String>, // Optional for SV1
    pub protocol: String, // "sv1" or "sv2"
}

#[derive(Clone)]
pub struct AppState {
    pub current_target: Arc<RwLock<String>>,
    pub last_switch_time: Arc<RwLock<Instant>>,
    pub profitability_scores: Arc<RwLock<Vec<ProfitabilityScore>>>,
    pub targets: Vec<MiningTarget>,
    pub accounting_service: Arc<AccountingService>,
    pub payout_service: Arc<PayoutService>,
}

impl AppState {
    pub fn new(
        config: Config,
        accounting_service: Arc<AccountingService>,
        payout_service: Arc<PayoutService>,
    ) -> Self {
        let initial_target = config.targets.first()
            .expect("At least one mining target must be configured")
            .name.clone();

        Self {
            current_target: Arc::new(RwLock::new(initial_target)),
            last_switch_time: Arc::new(RwLock::new(Instant::now())),
            profitability_scores: Arc::new(RwLock::new(Vec::new())),
            targets: config.targets,
            accounting_service,
            payout_service,
        }
    }

    pub fn get_current_target(&self) -> Target {
        let current_target_name = self.current_target.read().unwrap().clone();
        let mining_target = self.targets.iter()
            .find(|t| t.name == current_target_name)
            .expect("Current target not found in configuration");

        Target {
            address: mining_target.address.clone(),
            pubkey: None, // V1 doesn't need pubkey
            protocol: "sv1".to_string(),
        }
    }

    pub fn switch_target(&self, new_target: String) {
        let mut current = self.current_target.write().unwrap();
        *current = new_target;
        *self.last_switch_time.write().unwrap() = Instant::now();
    }

    pub fn update_scores(&self, scores: Vec<ProfitabilityScore>) {
        *self.profitability_scores.write().unwrap() = scores;
    }
}
