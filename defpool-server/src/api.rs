use axum::{
    extract::State,
    response::Json,
};
use crate::state::{AppState, Target};
use crate::profitability::ProfitabilityScore;
use tracing::info;

pub async fn get_target(State(state): State<AppState>) -> Json<Target> {
    info!("Proxy connected, fetching target");
    let target = state.get_current_target();
    Json(target)
}

pub async fn get_pools(State(state): State<AppState>) -> Json<Vec<ProfitabilityScore>> {
    let scores = state.profitability_scores.read().unwrap().clone();
    Json(scores)
}

pub async fn get_current_pool(State(state): State<AppState>) -> Json<String> {
    let current_pool = state.current_pool.read().unwrap().clone();
    Json(current_pool)
}

