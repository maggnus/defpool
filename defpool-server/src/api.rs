use axum::{
    extract::{State, Path},
    response::Json,
    http::StatusCode,
};
use crate::state::{AppState, Target};
use crate::profitability::ProfitabilityScore;
use crate::db::models::{ShareSubmission, MinerStats, Worker, Balance, Payout, PayoutRequest};
use tracing::info;
use serde::{Deserialize, Serialize};

/// GET /api/v1/target - Get current mining target
pub async fn get_current_target(State(state): State<AppState>) -> Json<Target> {
    info!("API: Fetching current target");
    let target = state.get_current_target();
    Json(target)
}

/// GET /api/v1/targets - List all mining targets with profitability scores
pub async fn list_targets(State(state): State<AppState>) -> Json<Vec<ProfitabilityScore>> {
    info!("API: Listing all targets");
    let scores = state.profitability_scores.read().unwrap().clone();
    Json(scores)
}

/// GET /api/v1/targets/current - Get current target name
pub async fn get_current_target_name(State(state): State<AppState>) -> Json<String> {
    info!("API: Fetching current target name");
    let current_target = state.current_target.read().unwrap().clone();
    Json(current_target)
}

/// GET /api/v1/miners/{wallet}/stats - Get miner statistics
pub async fn get_miner_stats(
    State(state): State<AppState>,
    Path(wallet): Path<String>,
) -> Result<Json<MinerStats>, StatusCode> {
    info!("API: Fetching stats for miner: {}", wallet);
    
    match state.accounting_service.get_miner_stats(&wallet).await {
        Ok(Some(stats)) => Ok(Json(stats)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/v1/miners/{wallet}/workers - Get miner's workers
pub async fn get_miner_workers(
    State(state): State<AppState>,
    Path(wallet): Path<String>,
) -> Result<Json<Vec<Worker>>, StatusCode> {
    info!("API: Fetching workers for miner: {}", wallet);
    
    match state.accounting_service.get_miner_workers(&wallet).await {
        Ok(workers) => Ok(Json(workers)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// POST /api/v1/shares - Record a share (internal, from proxy)
pub async fn record_share(
    State(state): State<AppState>,
    Json(submission): Json<ShareSubmission>,
) -> StatusCode {
    match state.accounting_service.record_share(submission).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

/// GET /api/v1/stats - Get pool statistics
pub async fn get_pool_stats(State(state): State<AppState>) -> Result<Json<PoolStats>, StatusCode> {
    info!("API: Fetching pool statistics");
    
    match state.accounting_service.get_pool_stats().await {
        Ok(mut stats) => {
            // Add current target name
            stats.current_target = state.current_target.read().unwrap().clone();
            Ok(Json(stats))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Serialize)]
pub struct PoolStats {
    pub total_miners: i64,
    pub active_miners: i64,
    pub total_workers: i64,
    pub active_workers: i64,
    pub total_shares_24h: i64,
    pub pool_hashrate: f64,
    pub current_target: String,
}

/// GET /api/v1/miners/{wallet}/balances - Get miner's balances
pub async fn get_miner_balances(
    State(state): State<AppState>,
    Path(wallet): Path<String>,
) -> Result<Json<Vec<Balance>>, StatusCode> {
    info!("API: Fetching balances for miner: {}", wallet);
    
    match state.payout_service.get_all_balances(&wallet).await {
        Ok(balances) => Ok(Json(balances)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// GET /api/v1/miners/{wallet}/balance/{coin} - Get miner's balance for specific coin
pub async fn get_miner_balance(
    State(state): State<AppState>,
    Path((wallet, coin)): Path<(String, String)>,
) -> Result<Json<Balance>, StatusCode> {
    info!("API: Fetching {} balance for miner: {}", coin, wallet);
    
    match state.payout_service.get_balance(&wallet, &coin).await {
        Ok(Some(balance)) => Ok(Json(balance)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

/// POST /api/v1/miners/{wallet}/payout - Request a payout
pub async fn request_payout(
    State(state): State<AppState>,
    Path(wallet): Path<String>,
    Json(mut request): Json<PayoutRequest>,
) -> Result<Json<Payout>, StatusCode> {
    info!("API: Payout request for miner: {}", wallet);
    
    // Ensure wallet matches path
    request.wallet_address = wallet;
    
    match state.payout_service.request_payout(request).await {
        Ok(payout) => Ok(Json(payout)),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

/// GET /api/v1/miners/{wallet}/payouts - Get payout history
pub async fn get_payout_history(
    State(state): State<AppState>,
    Path(wallet): Path<String>,
) -> Result<Json<Vec<Payout>>, StatusCode> {
    info!("API: Fetching payout history for miner: {}", wallet);
    
    match state.payout_service.get_payout_history(&wallet, 50).await {
        Ok(payouts) => Ok(Json(payouts)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Deserialize)]
pub struct PayoutSettingsUpdate {
    pub min_payout_threshold: f64,
    pub payout_coin: String,
    pub auto_exchange: bool,
}

/// PUT /api/v1/miners/{wallet}/payout-settings - Update payout settings
pub async fn update_payout_settings(
    State(state): State<AppState>,
    Path(wallet): Path<String>,
    Json(settings): Json<PayoutSettingsUpdate>,
) -> Result<Json<crate::db::models::PayoutSettings>, StatusCode> {
    info!("API: Updating payout settings for miner: {}", wallet);
    
    match state.payout_service.update_payout_settings(
        &wallet,
        settings.min_payout_threshold,
        &settings.payout_coin,
        settings.auto_exchange,
    ).await {
        Ok(settings) => Ok(Json(settings)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
