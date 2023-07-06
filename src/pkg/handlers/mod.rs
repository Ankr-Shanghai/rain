mod balance;
mod chainid;

mod block_num;

mod sha3;

use super::ethdb;
use std::sync::{Arc, Mutex};

use jsonrpc_core::IoHandler;
pub fn init_iohandlers(db: Arc<Mutex<ethdb::store::DB>>) -> IoHandler {
    let mut io = IoHandler::new();
    io.add_method("web3_clientVersion", chainid::ClientVersionImpl::new());
    io.add_method("web3_sha3", sha3::Web3Sha3Impl::new());
    io.add_method("eth_getBalance", balance::BalanceImpl::new());
    io.add_method("eth_chainid", chainid::ChainIDImpl::new());
    io.add_method("eth_blockNumber", block_num::BlockNumberImpl::new(db));
    io
}
