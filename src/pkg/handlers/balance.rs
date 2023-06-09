#![allow(dead_code, unused_imports)]
use jsonrpc_core::{params::Params, BoxFuture, Error, RpcMethodSimple, Value};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceImpl {
    result: String,
}

impl BalanceImpl {
    pub fn new() -> Self {
        BalanceImpl {
            result: "0x5200".to_string(),
        }
    }
}

impl RpcMethodSimple for BalanceImpl {
    type Out = BoxFuture<Result<Value, Error>>;
    fn call(&self, params: Params) -> Self::Out {
        info!("params: {:?}", params);
        Box::pin(async move { Ok(Value::String("0x1fa0".to_string())) })
    }
}
