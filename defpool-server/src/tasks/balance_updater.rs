use crate::payout::BalanceCalculator;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, error};

/// Start the background balance update task
pub fn start_balance_updater(calculator: Arc<BalanceCalculator>, coins: Vec<String>) {
    tokio::spawn(async move {
        let interval = Duration::from_secs(300); // Update every 5 minutes

        info!("Starting balance updater (interval: 300s, coins: {:?})", coins);

        loop {
            tokio::time::sleep(interval).await;

            for coin in &coins {
                match calculator.update_all_balances(coin).await {
                    Ok(_) => {
                        info!("Balance update completed for {}", coin);
                    }
                    Err(e) => {
                        error!("Failed to update balances for {}: {}", coin, e);
                    }
                }
            }
        }
    });
}
