#![allow(dead_code, unused_imports, unused_variables, unused_assignments)]
use crate::constant;

use super::endpoints::Node;
use super::ethdb;
use axum::Error;
use ethers::abi::Hash;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers_core::{
    types::transaction::response::Transaction, types::Block, types::BlockId, types::H256,
};
use log::info;
use std::borrow::BorrowMut;
use std::collections::{hash_map::HashMap, BinaryHeap};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::Interval;

pub struct Service {
    db: Arc<Mutex<ethdb::store::DB>>,
}

impl Service {
    pub fn new(db: Arc<Mutex<ethdb::store::DB>>) -> Self {
        Service { db }
    }
}

impl Service {
    pub async fn sync(&mut self, heap_sort: Arc<Mutex<BinaryHeap<Node>>>) {
        let mut lst_node = Node {
            url: "https://bsc-dataseed3.ninicoin.io".to_string(),
            delay: 0,
            height: 0,
        };

        let mut interval: Interval = tokio::time::interval(Duration::from_secs(2));

        loop {
            interval.tick().await;
            // time to collect endpoint network info
            if let Some(node) = heap_sort.lock().unwrap().peek() {
                lst_node = node.clone().into();
            }
            info!("lst_node {}", lst_node);

            let provider: Provider<Http> =
                Provider::<Http>::try_from(lst_node.get_url().as_str()).unwrap();
            // first get latest block number from remote node
            let block_num = provider.get_block_number().await;
            info!("latest block number {:?}", block_num);
            // second get current and current - 24 block info
            let cur: Result<Vec<u8>, redb::Error> = self
                .db
                .lock()
                .unwrap()
                .get(constant::GLOBAL_TABLE.to_string(), constant::LATEST_BLOCK);

            if let Ok(lst_num) = cur {
                let lst_num = String::from_utf8(lst_num).unwrap().parse::<u64>().unwrap();
                let mut block_num = block_num.unwrap().as_u64();
                let latest_num = block_num;
                if block_num > lst_num {
                    let mut block_info: Vec<Block<Transaction>> = Vec::new();
                    while block_num > lst_num {
                        let block: Result<Option<Block<Transaction>>, _> = provider
                            .get_block_with_txs::<BlockId>(block_num.into())
                            .await;
                        if let Ok(Some(block)) = block {
                            block_info.push(block);
                        }
                        block_num -= 1;
                    }
                    info!("block info {}", block_info.len());
                    // save block info to db
                    for block in block_info.iter() {
                        let block_num: u64 = block.number.unwrap().as_u64();
                        // handle block hash
                        let block_with_hash: Block<TxHash> = block.clone().into();
                        let block_bytes: String = serde_json::to_string(&block_with_hash).unwrap();
                        self.db
                            .lock()
                            .unwrap()
                            .put(
                                constant::BLOCK_TABLE.to_string(),
                                block_num.to_string().as_bytes(),
                                block_bytes.as_bytes(),
                            )
                            .unwrap();
                        self.db
                            .lock()
                            .unwrap()
                            .put(
                                constant::BLOCK_TABLE.to_string(),
                                block.hash.unwrap().as_bytes(),
                                block_num.to_string().as_bytes(),
                            )
                            .unwrap();
                        // update transaction info
                        let txes = block.transactions.clone();
                        for tx in txes.iter() {
                            let tx_hash: H256 = tx.hash;
                            let tx_bytes: String = serde_json::to_string(tx).unwrap();
                            self.db
                                .lock()
                                .unwrap()
                                .put(
                                    constant::TX_TABLE.to_string(),
                                    tx_hash.as_bytes(),
                                    tx_bytes.as_bytes(),
                                )
                                .unwrap();
                        }
                    }
                    // update latest block number
                    self.db
                        .lock()
                        .unwrap()
                        .put(
                            constant::GLOBAL_TABLE.to_string(),
                            constant::LATEST_BLOCK,
                            latest_num.to_string().as_bytes(),
                        )
                        .unwrap();

                    info!("update latest block number {}", latest_num);
                }
            } else {
                // update latest block number
                self.db
                    .lock()
                    .unwrap()
                    .put(
                        constant::GLOBAL_TABLE.to_string(),
                        constant::LATEST_BLOCK,
                        block_num.unwrap().as_u64().to_string().as_bytes(),
                    )
                    .unwrap();
            }
        }
    }
}

pub async fn remote_info<'a>(uris: &'static str, heap_sort: Arc<Mutex<BinaryHeap<Node>>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    let uri: Vec<&str> = uris.split(",").collect();
    let rmuri: Arc<Mutex<HashMap<&str, ()>>> = Arc::new(Mutex::new(HashMap::new()));
    loop {
        heap_sort.lock().unwrap().clear();

        interval.tick().await;

        for url in uri.clone().into_iter() {
            if rmuri.lock().unwrap().contains_key(url) {
                continue;
            }

            let rmuri_clone = rmuri.clone();
            let provider = Provider::<Http>::try_from(url).unwrap();
            let handler = tokio::spawn(async move {
                let now = Instant::now();
                let block = provider.get_block_number().await;
                if let Ok(height) = block {
                    (height.as_u64(), now.elapsed().as_secs())
                } else {
                    rmuri_clone.lock().unwrap().insert(url.clone(), ());
                    (0, now.elapsed().as_secs())
                }
            });
            let rs = handler.await;
            if let Ok(result) = rs {
                let node = Node::new(url.to_string(), result.1, result.0);
                heap_sort.lock().unwrap().push(node);
            } else {
                continue;
            }
        }
    }
}
