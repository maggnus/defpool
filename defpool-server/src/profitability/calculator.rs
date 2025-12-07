use super::types::ProfitabilityScore;
use super::providers::{PriceProvider, DifficultyProvider};
use crate::config::Pool;
use anyhow::Result;
use std::sync::Arc;
use tracing::{info, warn};

/// Calculator for determining pool profitability
pub struct ProfitabilityCalculator<P, D>
where
    P: PriceProvider,
    D: DifficultyProvider,
{
    price_provider: Arc<P>,
    difficulty_provider: Arc<D>,
    pools: Vec<Pool>,
}

impl<P, D> ProfitabilityCalculator<P, D>
where
    P: PriceProvider,
    D: DifficultyProvider,
{
    pub fn new(
        price_provider: Arc<P>,
        difficulty_provider: Arc<D>,
        pools: Vec<Pool>,
    ) -> Self {
        Self {
            price_provider,
            difficulty_provider,
            pools,
        }
    }

    /// Calculate profitability for all pools
    pub async fn calculate_all(&self) -> Result<Vec<ProfitabilityScore>> {
        let mut scores = Vec::new();

        for pool in &self.pools {
            match self.calculate_pool_score(pool).await {
                Ok(score) => {
                    info!(
                        "Pool {} ({}) profitability: {:.6}",
                        pool.name, pool.coin, score.score
                    );
                    scores.push(score);
                }
                Err(e) => {
                    warn!("Failed to calculate profitability for pool {}: {}", pool.name, e);
                }
            }
        }

        Ok(scores)
    }

    /// Calculate profitability score for a single pool
    async fn calculate_pool_score(&self, pool: &Pool) -> Result<ProfitabilityScore> {
        // Fetch metrics
        let price_btc = self.price_provider.get_price_btc(&pool.coin).await?;
        let difficulty = self.difficulty_provider.get_difficulty(&pool.coin).await?;
        
        // Hardcoded block reward for now (should be configurable per coin)
        let block_reward = match pool.coin.as_str() {
            "XMR" => 0.6, // Approximate XMR block reward
            _ => 1.0,
        };

        // Calculate profitability: (Block Reward * Price) / Difficulty
        let score = (block_reward * price_btc) / difficulty;

        Ok(ProfitabilityScore::new(
            pool.name.clone(),
            pool.coin.clone(),
            score,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profitability::providers::price::MockPriceProvider;
    use crate::profitability::providers::difficulty::MockDifficultyProvider;

    #[tokio::test]
    async fn test_calculate_profitability() {
        let price_provider = Arc::new(MockPriceProvider::new(0.002));
        let difficulty_provider = Arc::new(MockDifficultyProvider::new(100_000.0));
        
        let pools = vec![
            Pool {
                name: "test_pool".to_string(),
                address: "localhost:3333".to_string(),
                coin: "XMR".to_string(),
                algorithm: "RandomX".to_string(),
            },
        ];

        let calculator = ProfitabilityCalculator::new(
            price_provider,
            difficulty_provider,
            pools,
        );

        let scores = calculator.calculate_all().await.unwrap();
        assert_eq!(scores.len(), 1);
        assert!(scores[0].score > 0.0);
    }
}
