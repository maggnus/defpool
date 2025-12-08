use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tracing::{debug, info};

/// Calculator for miner balances based on shares
pub struct BalanceCalculator {
    pool: PgPool,
}

impl BalanceCalculator {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Calculate balance for a miner based on shares since last calculation
    /// Uses PPLNS (Pay Per Last N Shares) method
    pub async fn calculate_miner_balance(
        &self,
        miner_id: i32,
        coin: &str,
        since: DateTime<Utc>,
    ) -> Result<f64> {
        // Get total valid shares for this coin since last calculation
        let miner_shares: f64 = sqlx::query_scalar(
            r#"
            SELECT COALESCE(SUM(difficulty), 0.0)::float8
            FROM shares
            WHERE miner_id = $1
              AND target_name IN (
                  SELECT name FROM unnest($2::text[]) AS name
              )
              AND valid = true
              AND created_at > $3
            "#,
        )
        .bind(miner_id)
        .bind(&[coin]) // In production, map coin to target names
        .bind(since)
        .fetch_one(&self.pool)
        .await?;

        debug!(
            "Miner {} has {} difficulty shares for {} since {}",
            miner_id, miner_shares, coin, since
        );

        // TODO: Calculate actual earnings based on:
        // 1. Block rewards found
        // 2. Pool fees
        // 3. Share of total pool hashrate
        // For now, return a placeholder
        Ok(miner_shares * 0.0001) // Placeholder calculation
    }

    /// Update all miner balances for a specific coin
    pub async fn update_all_balances(&self, coin: &str) -> Result<()> {
        info!("Updating balances for coin: {}", coin);

        // Get all miners with shares for this coin
        let miners: Vec<(i32, DateTime<Utc>)> = sqlx::query_as(
            r#"
            SELECT DISTINCT m.id, COALESCE(b.updated_at, m.created_at) as last_update
            FROM miners m
            LEFT JOIN balances b ON b.miner_id = m.id AND b.coin = $1
            WHERE EXISTS (
                SELECT 1 FROM shares s
                WHERE s.miner_id = m.id
                  AND s.valid = true
                  AND s.created_at > COALESCE(b.updated_at, m.created_at)
            )
            "#,
        )
        .bind(coin)
        .fetch_all(&self.pool)
        .await?;

        for (miner_id, last_update) in miners {
            let earned = self.calculate_miner_balance(miner_id, coin, last_update).await?;

            if earned > 0.0 {
                // Update or insert balance
                sqlx::query(
                    r#"
                    INSERT INTO balances (miner_id, coin, balance, updated_at)
                    VALUES ($1, $2, $3, NOW())
                    ON CONFLICT (miner_id, coin)
                    DO UPDATE SET
                        balance = balances.balance + $3,
                        updated_at = NOW()
                    "#,
                )
                .bind(miner_id)
                .bind(coin)
                .bind(earned)
                .execute(&self.pool)
                .await?;

                debug!("Updated balance for miner {}: +{} {}", miner_id, earned, coin);
            }
        }

        info!("Balance update complete for {}", coin);
        Ok(())
    }
}
