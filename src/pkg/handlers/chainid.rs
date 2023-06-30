#![allow(dead_code, unused_variables)]
use jsonrpc_core::{params::Params, BoxFuture, Error, RpcMethodSimple, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChainIDImpl {
    result: String,
}

impl ChainIDImpl {
    pub fn new() -> Self {
        ChainIDImpl {
            result: "0x38".to_string(),
        }
    }
}

impl RpcMethodSimple for ChainIDImpl {
    type Out = BoxFuture<Result<Value, Error>>;
    fn call(&self, params: Params) -> Self::Out {
        Box::pin(async move { Ok(Value::String("0x38".to_string())) })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientVersionImpl {
    result: String,
}

impl ClientVersionImpl {
    pub fn new() -> Self {
        ClientVersionImpl {
            result: "Mis/v1.0.0/Linux/Rust v1.70.0".to_string(),
        }
    }
}

impl RpcMethodSimple for ClientVersionImpl {
    type Out = BoxFuture<Result<Value, Error>>;
    fn call(&self, params: Params) -> Self::Out {
        Box::pin(async move { Ok(Value::String("Mis/v1.0.0/Linux/Rust v1.70.0".to_string())) })
    }
}
