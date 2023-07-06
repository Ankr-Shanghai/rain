#![allow(dead_code, unused_imports)]
use std::{
    cmp::{Ordering, PartialOrd},
    fmt::Display,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub url: String,
    pub delay: u64,
    pub height: u64,
}

impl Node {
    pub fn new(url: String, delay: u64, height: u64) -> Self {
        Self { url, delay, height }
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_height(&self) -> u64 {
        self.height
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "url: {} delay: {} height: {}",
            self.url, self.delay, self.height
        )
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height
            .cmp(&other.height)
            .then_with(|| other.delay.cmp(&self.delay))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
