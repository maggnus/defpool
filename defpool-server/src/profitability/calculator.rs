use super::types::ProfitabilityScore;
use super::providers::{PriceProvider, DifficultyProvider};
use crate::config::MiningTarget;
use anyhow::Result;
use std::sync::Arc;
use tracing::{info, warn};

/// Calculator for determining mining target profitability
pub struct ProfitabilityCalculator<P, D>
where
    P: PriceProvider,
    D: DifficultyProvider,
{
    price_provider: Arc<P>,
    difficulty_provider: Arc<D>,
    targets: Vec<MiningTarget>,
}

impl<P, D> ProfitabilityCalculator<P, D>
where
    P: PriceProvider,
    D: DifficultyProvider,
{
    pub fn new(
        price_provider: Arc<P>,
        difficulty_provider: Arc<D>,
        targets: Vec<MiningTarget>,
    ) -> Self {
        Self {
            price_provider,
            difficulty_provider,
            targets,
        }
    }

    /// Calculate profitability for all mining targets
    pub async fn calculate_all(&self) -> Result<Vec<ProfitabilityScore>> {
        let mut scores = Vec::new();

        for target in &self.targets {
            match self.calculate_target_score(target).await {
                Ok(score) => {
                    info!(
                        "Target {} ({}) profitability: {:.6}",
                        target.name, target.coin, score.score
                    );
                    scores.push(score);
                }
                Err(e) => {
                    warn!("Failed to calculate profitability for target {}: {}", target.name, e);
                }
            }
        }

        Ok(scores)
    }

    /// Calculate profitability score for a single mining target
    async fn calculate_target_score(&self, target: &MiningTarget) -> Result<ProfitabilityScore> {
        // Fetch metrics
        let price_btc = self.price_provider.get_price_btc(&target.coin).await?;
        let difficulty = self.difficulty_provider.get_difficulty(&target.coin).await?;
        
        // Hardcoded block reward for now (should be configurable per coin)
        let block_reward = match target.coin.as_str() {
            "XMR" => 0.6, // Approximate XMR block reward
            _ => 1.0,
        };

        // Calculate profitability: (Block Reward * Price) / Difficulty
        let score = (block_reward * price_btc) / difficulty;

        Ok(ProfitabilityScore::new(
            target.name.clone(),
            target.coin.clone(),
            score,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profitability::providers::price::MockPriceProvider;
    use crate::profitability::providers::difficulty::MockDifficultyProvider;
    use crate::config::TargetType;

    #[tokio::test]
    async fn test_calculate_profitability() {
        let price_provider = Arc::new(MockPriceProvider::new(0.002));
        let difficulty_provider = Arc::new(MockDifficultyProvider::new(100_000.0));
        
        let targets = vec![
            MiningTarget {
                name: "test_target".to_string(),
                target_type: TargetType::Pool,
                address: "localhost:3333".to_string(),
                coin: "XMR".to_string(),
                algorithm: "RandomX".to_string(),
                daemon_rpc_url: None,
            },
        ];

        let calculator = ProfitabilityCalculator::new(
            price_provider,
            difficulty_provider,
            targets,
        );

        let scores = calculator.calculate_all().await.unwrap();
        assert_eq!(scores.len(), 1);
        assert!(scores[0].score > 0.0);
    }
}
