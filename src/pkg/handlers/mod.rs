mod balance;
mod chainid;

mod sha3;

use jsonrpc_core::IoHandler;
pub fn init_iohandlers() -> IoHandler {
    let mut io = IoHandler::new();
    io.add_method("eth_getBalance", balance::BalanceImpl::new());
    io.add_method("eth_chainid", chainid::ChainIDImpl::new());
    io.add_method("web3_clientVersion", chainid::ClientVersionImpl::new());
    io.add_method("web3_sha3", sha3::Web3Sha3Impl::new());
    io
}
