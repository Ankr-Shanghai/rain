use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Log {
    #[serde(rename = "address")]
    address: String,

    #[serde(rename = "topics")]
    topics: Vec<String>,

    #[serde(rename = "data")]
    data: String,

    #[serde(rename = "blockHash")]
    block_hash: String,

    #[serde(rename = "blockNumber")]
    block_number: String,

    #[serde(rename = "transactionHash")]
    transaction_hash: String,

    #[serde(rename = "transactionIndex")]
    transaction_index: String,

    #[serde(rename = "logIndex")]
    log_index: String,

    #[serde(rename = "removed")]
    removed: bool,
}
