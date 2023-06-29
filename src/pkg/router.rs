#![allow(dead_code, unused_variables)]
use crate::models::AppState;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

impl Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {} message: {}", self.code, self.message)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub jsonrpc: String,
    pub error: RpcError,
    pub id: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "jsonrpc: {} error: {} id: {}",
            self.jsonrpc, self.error, self.id
        )
    }
}
