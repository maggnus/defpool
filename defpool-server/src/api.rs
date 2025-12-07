use axum::{
    extract::State,
    response::Json,
};
use crate::state::{AppState, Target};

use tracing::info;

pub async fn get_target(State(state): State<AppState>) -> Json<Target> {
    info!("Proxy connected, fetching target");
    let target = state.current_target.read().unwrap().clone();
    Json(target)
}
