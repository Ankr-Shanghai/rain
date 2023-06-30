#![allow(dead_code, unused_variables)]

use jsonrpc_core::{params::Params, BoxFuture, Error, RpcMethodSimple, Value};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Web3Sha3Impl {
    result: String,
}

impl Web3Sha3Impl {
    pub fn new() -> Self {
        Web3Sha3Impl {
            result: "0x5200".to_string(),
        }
    }
}

impl RpcMethodSimple for Web3Sha3Impl {
    type Out = BoxFuture<Result<Value, Error>>;
    fn call(&self, params: Params) -> Self::Out {
        info!("menthod: web3_sha3 params: {:?}", params);
        if let Ok(data) = params.parse::<Vec<String>>() {
            if let Some(ds) = data.get(0) {
                let da = &ds[2..ds.len()];
                let d = hex::decode(da).unwrap();
                let rs = keccak_hash::keccak(d);
                Box::pin(async move { Ok(Value::String(format!("0x{}", hex::encode(rs)))) })
            } else {
                Box::pin(async move { Err(Error::invalid_params("Invalid params")) })
            }
        } else {
            Box::pin(async move { Err(Error::invalid_params("Invalid params")) })
        }
    }
}
