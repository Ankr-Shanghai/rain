#![allow(dead_code, unused_imports, unused_variables)]
use super::super::constant;
use super::super::ethdb::cache;
use jsonrpc_core::{params::Params, BoxFuture, Error, RpcMethodSimple, Value};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockNumberImpl {
    result: String,
    #[serde(skip)]
    cache: Arc<cache::MemStore>,
}

impl BlockNumberImpl {
    pub fn new(cache: Arc<cache::MemStore>) -> Self {
        BlockNumberImpl {
            result: "0x5200".to_string(),
            cache: cache,
        }
    }
}

impl RpcMethodSimple for BlockNumberImpl {
    type Out = BoxFuture<Result<Value, Error>>;
    fn call(&self, params: Params) -> Self::Out {
        info!("BlockNumberImpl::call");
        let block_num: Option<String> = self.cache.get(constant::LATEST_BLOCK);
        Box::pin(async move { Ok(Value::String(block_num.unwrap())) })
    }
}
