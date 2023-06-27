use crate::handlers::balance::get_balance;
use axum::extract::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Once;

type MethodType = dyn Fn(JsonRPC) -> String;

static mut METHODMAP: Option<HashMap<&str, Box<MethodType>>> = None;
static INIT: Once = Once::new();

pub fn init() {
    unsafe {
        INIT.call_once(|| {
            METHODMAP = Some(HashMap::new());
            METHODMAP
                .as_mut()
                .unwrap()
                .insert("eth_getBalance", Box::new(get_balance));
        });
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRPC {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<String>,
    pub id: String,
}

impl Display for JsonRPC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "jsonrpc: {} method: {} params: {:?} id: {}",
            self.jsonrpc, self.method, self.params, self.id
        )
    }
}

pub async fn router(Json(req): Json<JsonRPC>) -> String {
    let method_name = req.method.clone();
    let call: Option<&Box<MethodType>>;
    unsafe {
        let methods = METHODMAP.as_ref().unwrap();
        call = methods.get(method_name.as_str());
    }
    if let Some(executor) = call {
        executor(req)
    } else {
        format!("method {} not support yet", method_name)
    }
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
