use serde_derive::{Deserialize, Serialize};
use crate::types::{Slot, BlockHeight, BlockTime, PubkeyString, TokenDecimals};

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
  pub slot: Slot,
  pub block_height: BlockHeight,
  pub block_time: BlockTime,
  pub tokens: Vec<TokenInfo>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
  pub mint: PubkeyString,
  pub decimals: TokenDecimals,
}