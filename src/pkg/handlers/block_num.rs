#![allow(dead_code, unused_imports, unused_variables)]
use super::super::constant;
use super::super::ethdb;
use jsonrpc_core::{params::Params, BoxFuture, Error, RpcMethodSimple, Value};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockNumberImpl {
    result: String,
    #[serde(skip)]
    db: Arc<Mutex<ethdb::store::DB>>,
}

impl BlockNumberImpl {
    pub fn new(db: Arc<Mutex<ethdb::store::DB>>) -> Self {
        BlockNumberImpl {
            result: "0x5200".to_string(),
            db: db,
        }
    }
}

impl RpcMethodSimple for BlockNumberImpl {
    type Out = BoxFuture<Result<Value, Error>>;
    fn call(&self, params: Params) -> Self::Out {
        let block_num = self
            .db
            .lock()
            .unwrap()
            .get(constant::GLOBAL_TABLE.to_string(), constant::LATEST_BLOCK);
        Box::pin(async move {
            Ok(Value::String(
                String::from_utf8(block_num.unwrap()).unwrap(),
            ))
        })
    }
}
