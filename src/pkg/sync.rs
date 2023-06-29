#![allow(dead_code)]

struct Service {
    db: super::ethdb::store::DB,
}

impl Service {
    pub fn new(db: super::ethdb::store::DB) -> Self {
        Service { db }
    }
}
