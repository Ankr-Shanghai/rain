#![allow(dead_code, unused_imports)]
use std::collections::BinaryHeap;

struct Service {
    db: super::ethdb::store::DB,
}

impl Service {
    pub fn new(db: super::ethdb::store::DB) -> Self {
        Service { db }
    }
}
