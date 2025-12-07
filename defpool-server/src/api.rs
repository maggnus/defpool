use axum::{
    extract::{State, Path},
    response::Json,
    http::StatusCode,
};
use crate::state::{AppState, Target};
use crate::profitability::ProfitabilityScore;
use crate::db::models::{ShareSubmission, MinerStats, Worker};
use tracing::info;

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
