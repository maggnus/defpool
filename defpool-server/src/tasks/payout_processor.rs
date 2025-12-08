use crate::payout::PayoutService;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, error};

/// Start the background payout processing task
pub fn start_payout_processor(payout_service: Arc<PayoutService>) {
    tokio::spawn(async move {
        let interval = Duration::from_secs(60); // Check every minute

        info!("Starting payout processor (interval: 60s)");

        loop {
            tokio::time::sleep(interval).await;

            match payout_service.process_pending_payouts().await {
                Ok(_) => {
                    // Success - no need to log unless there were payouts
                }
                Err(e) => {
                    error!("Failed to process payouts: {}", e);
                }
            }
        }
    });
}
