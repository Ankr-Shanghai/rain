#![allow(dead_code, unused_imports, unused_variables, unused_assignments)]
use crate::constant;

use super::endpoints::Node;
use super::ethdb::{self, cache};
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
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use tokio::task;
use tokio::time::Interval;

pub struct Service {
    db: Arc<RwLock<ethdb::store::DB>>,
}

impl Service {
    pub fn new(db: Arc<RwLock<ethdb::store::DB>>) -> Self {
        Service { db }
    }
}

impl Service {
    pub async fn sync(
        &mut self,
        heap_sort: Arc<Mutex<BinaryHeap<Node>>>,
        cache: Arc<cache::MemStore>,
    ) {
        let mut lst_node = Node {
            url: "https://bsc-dataseed3.ninicoin.io".to_string(),
            delay: 0,
            height: 0,
        };

        let mut interval: Interval = tokio::time::interval(Duration::from_secs(2));

        let mut cache_latest_num: u64 = self.db.read().unwrap().get_block_number().unwrap_or(0);
        cache.put(
            constant::LATEST_BLOCK,
            cache_latest_num.to_string().as_str(),
        );

        loop {
            interval.tick().await;
            // time to collect endpoint network info
            if let Some(node) = heap_sort.lock().unwrap().peek() {
                lst_node = node.clone().into();
            }
            info!("lst_node {}", lst_node);

            let provider: Provider<Http> =
                Provider::<Http>::try_from(lst_node.get_url().as_str()).unwrap();
            let arc_provider = Arc::new(provider);
            // first get latest block number from remote node
            let provider_clone = arc_provider.clone();
            let block_num = provider_clone.get_block_number().await;
            let block_num_u64 = block_num.unwrap().as_u64();
            if block_num_u64 < cache_latest_num || cache_latest_num == 0 {
                cache_latest_num = block_num_u64;
                continue;
            }

            info!("latest block number {:?}", block_num_u64);

            while cache_latest_num < block_num_u64 {
                cache_latest_num += 1;
                handler(
                    arc_provider.clone(),
                    self.db.clone(),
                    cache_latest_num.into(),
                )
                .await;
                cache.put(
                    constant::LATEST_BLOCK,
                    cache_latest_num.to_string().as_str(),
                );
            }
        }
    }
}

async fn handler(provider: Arc<Provider<Http>>, db: Arc<RwLock<ethdb::store::DB>>, num: BlockId) {
    task::spawn(async move {
        let block: Result<Option<Block<Transaction>>, _> =
            provider.get_block_with_txs::<BlockId>(num).await;
        if let Ok(Some(block)) = block {
            let block_num: u64 = block.number.unwrap().as_u64();
            let block_with_hash: Block<TxHash> = block.clone().into();
            let block_bytes: String = serde_json::to_string(&block_with_hash).unwrap();
            db.write()
                .unwrap()
                .put(
                    constant::BLOCK_TABLE.to_string(),
                    block_num.to_string().as_bytes(),
                    block_bytes.as_bytes(),
                )
                .unwrap();
            db.write()
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
                db.write()
                    .unwrap()
                    .put(
                        constant::TX_TABLE.to_string(),
                        tx_hash.as_bytes(),
                        tx_bytes.as_bytes(),
                    )
                    .unwrap();
            }
            // update latest block number
            db.write()
                .unwrap()
                .put(
                    constant::GLOBAL_TABLE.to_string(),
                    constant::LATEST_BLOCK.as_bytes(),
                    block_num.to_string().as_bytes(),
                )
                .unwrap();

            info!("update latest block number {}", block_num);
        }
    });
}

pub async fn remote_info(uris: Vec<String>, heap_sort: Arc<Mutex<BinaryHeap<Node>>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    let rmuri: Arc<Mutex<HashMap<&str, ()>>> = Arc::new(Mutex::new(HashMap::new()));
    let static_uris: &'static str = Box::leak(uris.join(",").into_boxed_str());
    loop {
        heap_sort.lock().unwrap().clear();

        interval.tick().await;

        let t_uris: Vec<&str> = static_uris.split(",").collect();

        for url in t_uris {
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
                    rmuri_clone.lock().unwrap().insert(url, ());
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
