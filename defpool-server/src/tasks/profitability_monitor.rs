use crate::state::AppState;
use crate::profitability::ProfitabilityCalculator;
use crate::profitability::providers::{PriceProvider, DifficultyProvider};
use crate::config::Config;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, error};

/// Start the background profitability monitoring task
pub fn start_profitability_monitor<P, D>(
    state: AppState,
    calculator: Arc<ProfitabilityCalculator<P, D>>,
    config: Config,
) where
    P: PriceProvider + 'static,
    D: DifficultyProvider + 'static,
{
    tokio::spawn(async move {
        let interval = Duration::from_secs(config.profitability_check_interval_secs);
        let threshold_percent = config.switch_threshold_percent;

        info!("Starting profitability monitor (interval: {}s, threshold: {}%)", 
            config.profitability_check_interval_secs, threshold_percent);

        loop {
            tokio::time::sleep(interval).await;

            match calculator.calculate_all().await {
                Ok(scores) => {
                    state.update_scores(scores.clone());

                    if let Some(best) = scores.iter()
                        .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal))
                    {
                        let current_target = state.current_target.read().unwrap().clone();
                        
                        // Check if we should switch
                        if best.target_name != current_target {
                            let current_score = scores.iter()
                                .find(|s| s.target_name == current_target)
                                .map(|s| s.score)
                                .unwrap_or(0.0);

                            let improvement_percent = ((best.score - current_score) / current_score) * 100.0;

                            if improvement_percent >= threshold_percent {
                                info!(
                                    "Switching from {} to {} (improvement: {:.2}%)",
                                    current_target, best.target_name, improvement_percent
                                );
                                state.switch_target(best.target_name.clone());
                            } else {
                                info!(
                                    "Target {} is better but below threshold ({:.2}% < {}%)",
                                    best.target_name, improvement_percent, threshold_percent
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to calculate profitability: {}", e);
                }
            }
        }
    });
}
