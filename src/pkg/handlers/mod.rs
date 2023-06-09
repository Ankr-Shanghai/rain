#![allow(dead_code, unused_variables)]
mod balance;
mod chainid;

mod block_num;

mod sha3;

use super::ethdb::{self, cache};
use std::sync::{Arc, RwLock};

use jsonrpc_core::IoHandler;
pub fn init_iohandlers(
    db: Arc<RwLock<ethdb::store::DB>>,
    cache: Arc<cache::MemStore>,
) -> IoHandler {
    let mut io = IoHandler::new();
    io.add_method("web3_clientVersion", chainid::ClientVersionImpl::new());
    io.add_method("web3_sha3", sha3::Web3Sha3Impl::new());
    io.add_method("eth_getBalance", balance::BalanceImpl::new());
    io.add_method("eth_chainid", chainid::ChainIDImpl::new());
    io.add_method("eth_blockNumber", block_num::BlockNumberImpl::new(cache));
    io
}
