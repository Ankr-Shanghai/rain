#![allow(dead_code)]
use crate::router::JsonRPC;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct BalanceResponse {
    jsonrpc: String,
    result: String,
    id: String,
}

pub fn get_balance(req: JsonRPC) -> String {
    info!("get_balance: {}", req);
    let rsp = BalanceResponse {
        jsonrpc: "2.0".to_string(),
        result: "0x0".to_string(),
        id: req.id,
    };

    serde_json::to_string(&rsp).unwrap()
}
