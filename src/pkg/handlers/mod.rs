mod balance;
mod chainid;

use jsonrpc_core::IoHandler;
pub fn init_iohandlers() -> IoHandler {
    let mut io = IoHandler::new();
    io.add_method("eth_getBalance", balance::BalanceImpl::new());
    io.add_method("eth_chainid", chainid::ChainIDImpl::new());
    io
}
