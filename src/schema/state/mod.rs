use crate::{
    serde::vec_u8_as_base64_string,
    types::{BlockHeight, BlockTime, Bytes, PubkeyString, Slot},
};
use serde_derive::{Deserialize, Serialize};

/*

Whirlpool State File JSON Schema

A whirlpool state file (whirlpool-state-yyyymmdd.json.gz) is GZIP compressed JSON file with the following schema:

{
  slot: u64,
  blockHeight: u64,
  blockTime: i64,
  accounts: [
    { pubkey: String(base58 encoding), data: String(base64 encoding) },
    { pubkey: String(base58 encoding), data: String(base64 encoding) },
    ...
  ],
  programData: String(base64 encoding)
}

*/

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WhirlpoolState {
    pub slot: Slot,
    pub block_height: BlockHeight,
    pub block_time: BlockTime,
    pub accounts: Vec<Account>,
    #[serde(with = "vec_u8_as_base64_string")]
    pub program_data: Bytes,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub pubkey: PubkeyString,
    #[serde(with = "vec_u8_as_base64_string")]
    pub data: Bytes,
}
