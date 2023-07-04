#![allow(dead_code, unused_imports, unused_variables)]
use super::endpoints::Node;
use ethers::abi::Hash;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use log::info;
use std::collections::{hash_map::HashMap, BinaryHeap};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::Interval;

struct Service {
    db: super::ethdb::store::DB,
}

impl Service {
    pub fn new(db: super::ethdb::store::DB) -> Self {
        Service { db }
    }
}

impl Service {
    pub fn sync() {
        // time to collect endpoint network info
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
