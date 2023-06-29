use super::logs::Log;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Receipt {
    #[serde(rename = "root")]
    root: String,

    #[serde(rename = "status")]
    status: String,

    #[serde(rename = "cumulativeGasUsed")]
    cumulative_gas_used: String,

    #[serde(rename = "logsBloom")]
    logs_bloom: String,

    #[serde(rename = "transactionHash")]
    transaction_hash: String,

    #[serde(rename = "contractAddress")]
    contract_address: String,

    #[serde(rename = "gasUsed")]
    gas_used: String,

    #[serde(rename = "blockHash")]
    block_hash: String,

    #[serde(rename = "blockNumber")]
    block_number: String,

    #[serde(rename = "transactionIndex")]
    transaction_index: String,

    #[serde(rename = "logs")]
    logs: Vec<Log>,
}
