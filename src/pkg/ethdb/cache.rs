use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct MemStore {
    cache: Arc<Mutex<HashMap<String, String>>>,
}

impl MemStore {
    pub fn new() -> Self {
        let cache: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
        MemStore { cache }
    }
}

impl MemStore {
    pub fn get(&self, key: &str) -> Option<String> {
        self.cache.lock().unwrap().get(key).cloned()
    }

    pub fn put(&self, key: &str, value: &str) {
        self.cache
            .lock()
            .unwrap()
            .insert(key.to_string(), value.to_string());
    }
}

impl std::default::Default for MemStore {
    fn default() -> Self {
        let cache: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
        MemStore { cache }
    }
}

impl std::fmt::Debug for MemStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemStore")
            .field("cache", &self.cache)
            .finish()
    }
}
