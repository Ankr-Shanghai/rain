#![allow(dead_code, unused_imports)]
use std::cmp::{Ordering, PartialOrd};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Node<'a> {
    url: &'a str,
    delay: u64,
    height: u64,
}

impl<'a> Node<'a> {
    pub fn new(url: &'a str, delay: u64, height: u64) -> Self {
        Self { url, delay, height }
    }
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height
            .cmp(&other.delay)
            .then_with(|| self.delay.cmp(&other.delay))
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
