use crate::db::models::*;
use anyhow::Result;
use sqlx::PgPool;
use tracing::{info, warn};

/// Service for managing payouts
pub struct PayoutService {
    pool: PgPool,
}

impl PayoutService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get miner's balance for a specific coin
    pub async fn get_balance(&self, wallet_address: &str, coin: &str) -> Result<Option<Balance>> {
        let balance = sqlx::query_as::<_, Balance>(
            r#"
            SELECT b.* FROM balances b
            JOIN miners m ON b.miner_id = m.id
            WHERE m.wallet_address = $1 AND b.coin = $2
            "#,
        )
        .bind(wallet_address)
        .bind(coin)
        .fetch_optional(&self.pool)
        .await?;

        Ok(balance)
    }

    /// Get all balances for a miner
    pub async fn get_all_balances(&self, wallet_address: &str) -> Result<Vec<Balance>> {
        let balances = sqlx::query_as::<_, Balance>(
            r#"
            SELECT b.* FROM balances b
            JOIN miners m ON b.miner_id = m.id
            WHERE m.wallet_address = $1
            ORDER BY b.balance DESC
            "#,
        )
        .bind(wallet_address)
        .fetch_all(&self.pool)
        .await?;

        Ok(balances)
    }

    /// Get payout settings for a miner
    pub async fn get_payout_settings(&self, wallet_address: &str) -> Result<Option<PayoutSettings>> {
        let settings = sqlx::query_as::<_, PayoutSettings>(
            r#"
            SELECT ps.* FROM payout_settings ps
            JOIN miners m ON ps.miner_id = m.id
            WHERE m.wallet_address = $1
            "#,
        )
        .bind(wallet_address)
        .fetch_optional(&self.pool)
        .await?;

        Ok(settings)
    }

    /// Update payout settings for a miner
    pub async fn update_payout_settings(
        &self,
        wallet_address: &str,
        min_threshold: f64,
        payout_coin: &str,
        auto_exchange: bool,
    ) -> Result<PayoutSettings> {
        // Get or create miner
        let miner: Miner = sqlx::query_as(
            "SELECT * FROM miners WHERE wallet_address = $1",
        )
        .bind(wallet_address)
        .fetch_one(&self.pool)
        .await?;

        let settings = sqlx::query_as::<_, PayoutSettings>(
            r#"
            INSERT INTO payout_settings (miner_id, min_payout_threshold, payout_coin, auto_exchange)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (miner_id)
            DO UPDATE SET
                min_payout_threshold = $2,
                payout_coin = $3,
                auto_exchange = $4,
                updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(miner.id)
        .bind(min_threshold)
        .bind(payout_coin)
        .bind(auto_exchange)
        .fetch_one(&self.pool)
        .await?;

        Ok(settings)
    }

    /// Request a payout
    pub async fn request_payout(&self, request: PayoutRequest) -> Result<Payout> {
        info!(
            "Payout requested: wallet={}, coin={}, amount={:?}",
            request.wallet_address, request.coin, request.amount
        );

        // Get miner
        let miner: Miner = sqlx::query_as(
            "SELECT * FROM miners WHERE wallet_address = $1",
        )
        .bind(&request.wallet_address)
        .fetch_one(&self.pool)
        .await?;

        // Get balance
        let balance = self.get_balance(&request.wallet_address, &request.coin).await?
            .ok_or_else(|| anyhow::anyhow!("No balance found for {} {}", request.wallet_address, request.coin))?;

        // Determine payout amount
        let amount = request.amount.unwrap_or(balance.balance);

        // Validate amount
        if amount <= 0.0 {
            anyhow::bail!("Payout amount must be positive");
        }

        if amount > balance.balance {
            anyhow::bail!("Insufficient balance: requested {}, available {}", amount, balance.balance);
        }

        // Check minimum threshold
        let settings = self.get_payout_settings(&request.wallet_address).await?;
        if let Some(settings) = settings {
            if amount < settings.min_payout_threshold {
                anyhow::bail!(
                    "Amount {} below minimum threshold {}",
                    amount,
                    settings.min_payout_threshold
                );
            }
        }

        // Start transaction
        let mut tx = self.pool.begin().await?;

        // Move balance to pending
        sqlx::query(
            r#"
            UPDATE balances
            SET balance = balance - $1,
                pending_balance = pending_balance + $1
            WHERE miner_id = $2 AND coin = $3
            "#,
        )
        .bind(amount)
        .bind(miner.id)
        .bind(&request.coin)
        .execute(&mut *tx)
        .await?;

        // Create payout record
        let payout = sqlx::query_as::<_, Payout>(
            r#"
            INSERT INTO payouts (miner_id, coin, amount, status, created_at)
            VALUES ($1, $2, $3, 'pending', NOW())
            RETURNING *
            "#,
        )
        .bind(miner.id)
        .bind(&request.coin)
        .bind(amount)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        info!("Payout created: id={}, amount={} {}", payout.id, amount, request.coin);
        Ok(payout)
    }

    /// Process pending payouts (called by background task)
    pub async fn process_pending_payouts(&self) -> Result<()> {
        let pending_payouts: Vec<Payout> = sqlx::query_as(
            "SELECT * FROM payouts WHERE status = 'pending' ORDER BY created_at ASC LIMIT 10",
        )
        .fetch_all(&self.pool)
        .await?;

        for payout in pending_payouts {
            info!("Processing payout: id={}, amount={} {}", payout.id, payout.amount, payout.coin);

            // Update status to processing
            sqlx::query("UPDATE payouts SET status = 'processing' WHERE id = $1")
                .bind(payout.id)
                .execute(&self.pool)
                .await?;

            // TODO: Implement actual blockchain transaction
            // For now, simulate processing
            match self.send_transaction(&payout).await {
                Ok(tx_hash) => {
                    self.complete_payout(payout.id, &tx_hash).await?;
                }
                Err(e) => {
                    self.fail_payout(payout.id, &e.to_string()).await?;
                }
            }
        }

        Ok(())
    }

    /// Send blockchain transaction (placeholder)
    async fn send_transaction(&self, payout: &Payout) -> Result<String> {
        // TODO: Implement actual blockchain transaction
        // This would involve:
        // 1. Connecting to coin daemon RPC
        // 2. Creating transaction
        // 3. Signing transaction
        // 4. Broadcasting transaction
        // 5. Returning transaction hash

        warn!("Transaction sending not implemented yet for payout {}", payout.id);
        
        // Simulate transaction
        Ok(format!("simulated_tx_hash_{}", payout.id))
    }

    /// Mark payout as completed
    async fn complete_payout(&self, payout_id: i64, tx_hash: &str) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        // Update payout status
        sqlx::query(
            r#"
            UPDATE payouts
            SET status = 'completed',
                tx_hash = $1,
                completed_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(tx_hash)
        .bind(payout_id)
        .execute(&mut *tx)
        .await?;

        // Get payout details
        let payout: Payout = sqlx::query_as("SELECT * FROM payouts WHERE id = $1")
            .bind(payout_id)
            .fetch_one(&mut *tx)
            .await?;

        // Update balance (remove from pending, add to total_paid)
        sqlx::query(
            r#"
            UPDATE balances
            SET pending_balance = pending_balance - $1,
                total_paid = total_paid + $1
            WHERE miner_id = $2 AND coin = $3
            "#,
        )
        .bind(payout.amount)
        .bind(payout.miner_id)
        .bind(&payout.coin)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        info!("Payout completed: id={}, tx_hash={}", payout_id, tx_hash);
        Ok(())
    }

    /// Mark payout as failed
    async fn fail_payout(&self, payout_id: i64, error: &str) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        // Update payout status
        sqlx::query(
            "UPDATE payouts SET status = 'failed', error_message = $1 WHERE id = $2",
        )
        .bind(error)
        .bind(payout_id)
        .execute(&mut *tx)
        .await?;

        // Get payout details
        let payout: Payout = sqlx::query_as("SELECT * FROM payouts WHERE id = $1")
            .bind(payout_id)
            .fetch_one(&mut *tx)
            .await?;

        // Return balance from pending
        sqlx::query(
            r#"
            UPDATE balances
            SET balance = balance + $1,
                pending_balance = pending_balance - $1
            WHERE miner_id = $2 AND coin = $3
            "#,
        )
        .bind(payout.amount)
        .bind(payout.miner_id)
        .bind(&payout.coin)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        warn!("Payout failed: id={}, error={}", payout_id, error);
        Ok(())
    }

    /// Get payout history for a miner
    pub async fn get_payout_history(&self, wallet_address: &str, limit: i64) -> Result<Vec<Payout>> {
        let payouts = sqlx::query_as::<_, Payout>(
            r#"
            SELECT p.* FROM payouts p
            JOIN miners m ON p.miner_id = m.id
            WHERE m.wallet_address = $1
            ORDER BY p.created_at DESC
            LIMIT $2
            "#,
        )
        .bind(wallet_address)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(payouts)
    }
}
