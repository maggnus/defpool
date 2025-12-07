use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Miner database model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Miner {
    pub id: i32,
    pub wallet_address: String,
    pub created_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
    pub total_shares: i64,
    pub total_valid_shares: i64,
    pub total_invalid_shares: i64,
}

/// Worker database model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Worker {
    pub id: i32,
    pub miner_id: i32,
    pub worker_name: String,
    pub created_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
    pub hashrate: f64,
    pub total_shares: i64,
}

/// Share database model
#[derive(Debug, Clone, FromRow)]
pub struct Share {
    pub id: i64,
    #[allow(dead_code)]
    pub miner_id: i32,
    #[allow(dead_code)]
    pub worker_id: Option<i32>,
    #[allow(dead_code)]
    pub target_name: String,
    #[allow(dead_code)]
    pub difficulty: f64,
    #[allow(dead_code)]
    pub valid: bool,
    #[allow(dead_code)]
    pub created_at: DateTime<Utc>,
}

/// New share submission (from proxy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareSubmission {
    pub wallet_address: String,
    pub worker_name: String,
    pub target_name: String,
    pub difficulty: f64,
    pub valid: bool,
}

/// Miner statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerStats {
    pub wallet_address: String,
    pub total_shares: i64,
    pub valid_shares: i64,
    pub invalid_shares: i64,
    pub hashrate: f64,
    pub workers_count: i32,
    pub last_seen: Option<DateTime<Utc>>,
}
