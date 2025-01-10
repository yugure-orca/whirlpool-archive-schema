use serde_derive::{Deserialize, Serialize};
use crate::serde::vec_u8_as_base64_string;

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
  pub slot: u64,
  pub block_height: u64,
  pub block_time: i64,
  pub accounts: Vec<WhirlpoolStateAccount>,
  #[serde(with = "vec_u8_as_base64_string")]
  pub program_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WhirlpoolStateAccount {
  pub pubkey: String,
  #[serde(with = "vec_u8_as_base64_string")]
  pub data: Vec<u8>,
}
