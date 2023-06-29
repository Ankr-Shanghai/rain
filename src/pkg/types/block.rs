use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Block {
    #[serde(rename = "difficulty")]
    difficulty: String,

    #[serde(rename = "extraData")]
    extra_data: String,

    #[serde(rename = "gasLimit")]
    gas_limit: String,

    #[serde(rename = "gasUsed")]
    gas_used: String,

    #[serde(rename = "hash")]
    hash: String,

    #[serde(rename = "logsBloom")]
    logs_bloom: String,

    #[serde(rename = "miner")]
    miner: String,

    #[serde(rename = "mixHash")]
    mix_hash: String,

    #[serde(rename = "nonce")]
    nonce: String,

    #[serde(rename = "number")]
    number: String,

    #[serde(rename = "parentHash")]
    parent_hash: String,

    #[serde(rename = "receiptsRoot")]
    receipts_root: String,

    #[serde(rename = "sha3Uncles")]
    sha3_uncles: String,

    #[serde(rename = "size")]
    size: String,

    #[serde(rename = "stateRoot")]
    state_root: String,

    #[serde(rename = "timestamp")]
    timestamp: String,

    #[serde(rename = "totalDifficulty")]
    total_difficulty: i64,

    #[serde(rename = "transactions")]
    transactions: Vec<String>,

    #[serde(rename = "transactionsRoot")]
    transactions_root: String,

    #[serde(rename = "uncles")]
    uncles: Vec<String>,
}
