#![allow(dead_code, unused_variables)]
use jsonrpc_core::{params::Params, BoxFuture, Error, RpcMethodSimple, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SignImpl {
    result: String,
}

impl SignImpl {
    pub fn new() -> Self {
        SignImpl {
            result: "0x38".to_string(),
        }
    }
}

impl RpcMethodSimple for SignImpl {
    type Out = BoxFuture<Result<Value, Error>>;
    fn call(&self, params: Params) -> Self::Out {
        info!("menthod: eth_sign params: {:?}", params);
        let header = "\x19Ethereum Signed Message:\n";
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
