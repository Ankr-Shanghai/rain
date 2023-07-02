#![allow(dead_code, unused_variables)]
use crate::config::AppState;
use axum::extract::State;
use std::sync::Arc;

pub async fn router(State(app_state): State<Arc<AppState>>, req: String) -> Vec<u8> {
    app_state
        .io
        .handle_request(req.as_str())
        .await
        .unwrap()
        .as_bytes()
        .to_owned()
}
