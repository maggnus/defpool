use axum::{
    extract::State,
    response::Json,
};
use crate::state::{AppState, Target};
use crate::profitability::ProfitabilityScore;
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

