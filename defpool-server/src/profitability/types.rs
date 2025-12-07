use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Metrics for a specific coin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinMetrics {
    pub coin: String,
    pub price_btc: f64,
    pub difficulty: f64,
    pub block_reward: f64,
}

/// Profitability score for a mining target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitabilityScore {
    pub target_name: String,
    pub coin: String,
    pub score: f64,
    pub timestamp: SystemTime,
}

impl ProfitabilityScore {
    pub fn new(target_name: String, coin: String, score: f64) -> Self {
        Self {
            target_name,
            coin,
            score,
            timestamp: SystemTime::now(),
        }
    }
}
