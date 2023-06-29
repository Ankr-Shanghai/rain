use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "blockHash")]
    block_hash: String,

    #[serde(rename = "blockNumber")]
    block_number: String,

    #[serde(rename = "from")]
    from: String,

    #[serde(rename = "gas")]
    gas: String,

    #[serde(rename = "gasPrice")]
    gas_price: String,

    #[serde(rename = "hash")]
    hash: String,

    #[serde(rename = "input")]
    input: String,

    #[serde(rename = "nonce")]
    nonce: String,

    #[serde(rename = "to")]
    to: String,

    #[serde(rename = "transactionIndex")]
    transaction_index: String,

    #[serde(rename = "value")]
    value: String,

    #[serde(rename = "type")]
    txr_type: String,

    #[serde(rename = "v")]
    v: String,

    #[serde(rename = "r")]
    r: String,

    #[serde(rename = "s")]
    s: String,
}
