use crate::config::AppState;
use axum::extract::State;
use log::info;
use std::sync::Arc;

pub async fn health() -> &'static str {
    "OK"
}

pub async fn config(State(app_state): State<Arc<AppState>>) -> String {
    info!("config: {}", app_state.config);
    format!("config: {}", app_state.config)
}
