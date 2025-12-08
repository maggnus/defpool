use super::models::*;
use sqlx::PgPool;
use anyhow::Result;
use chrono::Utc;

/// Repository for database operations
pub struct ShareRepository {
    pool: PgPool,
}

impl ShareRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get or create a miner by wallet address
    pub async fn get_or_create_miner(&self, wallet_address: &str) -> Result<Miner> {
        // Try to get existing miner
        let miner = sqlx::query_as::<_, Miner>(
            "SELECT * FROM miners WHERE wallet_address = $1"
        )
        .bind(wallet_address)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(miner) = miner {
            return Ok(miner);
        }

        // Create new miner
        let miner = sqlx::query_as::<_, Miner>(
            r#"
            INSERT INTO miners (wallet_address, created_at)
            VALUES ($1, $2)
            RETURNING *
            "#
        )
        .bind(wallet_address)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        Ok(miner)
    }

    /// Get or create a worker
    pub async fn get_or_create_worker(&self, miner_id: i32, worker_name: &str) -> Result<Worker> {
        // Try to get existing worker
        let worker = sqlx::query_as::<_, Worker>(
            "SELECT * FROM workers WHERE miner_id = $1 AND worker_name = $2"
        )
        .bind(miner_id)
        .bind(worker_name)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(worker) = worker {
            return Ok(worker);
        }

        // Create new worker
        let worker = sqlx::query_as::<_, Worker>(
            r#"
            INSERT INTO workers (miner_id, worker_name, created_at)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(miner_id)
        .bind(worker_name)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        Ok(worker)
    }

    /// Record a share
    pub async fn create_share(&self, submission: &ShareSubmission) -> Result<Share> {
        // Get or create miner
        let miner = self.get_or_create_miner(&submission.wallet_address).await?;

        // Get or create worker
        let worker = self.get_or_create_worker(miner.id, &submission.worker_name).await?;

        // Insert share
        let share = sqlx::query_as::<_, Share>(
            r#"
            INSERT INTO shares (miner_id, worker_id, target_name, difficulty, valid, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#
        )
        .bind(miner.id)
        .bind(worker.id)
        .bind(&submission.target_name)
        .bind(submission.difficulty)
        .bind(submission.valid)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;

        Ok(share)
    }

    /// Get miner statistics
    pub async fn get_miner_stats(&self, wallet_address: &str) -> Result<Option<MinerStats>> {
        let miner = sqlx::query_as::<_, Miner>(
            "SELECT * FROM miners WHERE wallet_address = $1"
        )
        .bind(wallet_address)
        .fetch_optional(&self.pool)
        .await?;

        let Some(miner) = miner else {
            return Ok(None);
        };

        // Get workers count
        let workers_count: i32 = sqlx::query_scalar(
            "SELECT COUNT(*)::int FROM workers WHERE miner_id = $1"
        )
        .bind(miner.id)
        .fetch_one(&self.pool)
        .await?;

        // Calculate hashrate (shares in last 10 minutes)
        let hashrate: f64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(SUM(difficulty), 0.0)::float8 / 600.0
            FROM shares
            WHERE miner_id = $1
              AND created_at > NOW() - INTERVAL '10 minutes'
              AND valid = true
            "#
        )
        .bind(miner.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Some(MinerStats {
            wallet_address: miner.wallet_address,
            total_shares: miner.total_shares,
            valid_shares: miner.total_valid_shares,
            invalid_shares: miner.total_invalid_shares,
            hashrate,
            workers_count,
            last_seen: miner.last_seen,
        }))
    }

    /// Get miner's workers
    pub async fn get_miner_workers(&self, wallet_address: &str) -> Result<Vec<Worker>> {
        let workers = sqlx::query_as::<_, Worker>(
            r#"
            SELECT w.* FROM workers w
            JOIN miners m ON w.miner_id = m.id
            WHERE m.wallet_address = $1
            ORDER BY w.last_seen DESC NULLS LAST
            "#
        )
        .bind(wallet_address)
        .fetch_all(&self.pool)
        .await?;

        Ok(workers)
    }

    /// Get pool-wide statistics
    pub async fn get_pool_stats(&self) -> Result<crate::api::PoolStats> {
        // Total miners
        let total_miners: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)::bigint FROM miners"
        )
        .fetch_one(&self.pool)
        .await?;

        // Active miners (seen in last hour)
        let active_miners: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)::bigint FROM miners WHERE last_seen > NOW() - INTERVAL '1 hour'"
        )
        .fetch_one(&self.pool)
        .await?;

        // Total workers
        let total_workers: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)::bigint FROM workers"
        )
        .fetch_one(&self.pool)
        .await?;

        // Active workers (seen in last hour)
        let active_workers: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)::bigint FROM workers WHERE last_seen > NOW() - INTERVAL '1 hour'"
        )
        .fetch_one(&self.pool)
        .await?;

        // Total shares in last 24 hours
        let total_shares_24h: i64 = sqlx::query_scalar(
            "SELECT COUNT(*)::bigint FROM shares WHERE created_at > NOW() - INTERVAL '24 hours'"
        )
        .fetch_one(&self.pool)
        .await?;

        // Pool hashrate (last 10 minutes)
        let pool_hashrate: f64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(SUM(difficulty), 0.0)::float8 / 600.0
            FROM shares
            WHERE created_at > NOW() - INTERVAL '10 minutes'
              AND valid = true
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(crate::api::PoolStats {
            total_miners,
            active_miners,
            total_workers,
            active_workers,
            total_shares_24h,
            pool_hashrate,
            current_target: "unknown".to_string(), // Will be filled by API handler
        })
    }
}
