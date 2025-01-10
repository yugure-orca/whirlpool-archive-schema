use serde_derive::{Deserialize, Serialize};

/*

Whirlpool Token File JSON Schema

A whirlpool token file (whirlpool-token-yyyymmdd.json.gz) is GZIP compressed JSON file with the following schema:
 
{
  slot: u64,
  blockHeight: u64,
  blockTime: i64,
  tokens: [
    { mint: String(base58 encoding), decimals: u8 },
    { mint: String(base58 encoding), decimals: u8 },
    ...
  ]
}

*/

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WhirlpoolToken {
  pub slot: u64,
  pub block_height: u64,
  pub block_time: i64,
  pub tokens: Vec<TokenInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
  pub mint: String,
  pub decimals: u8,
}
