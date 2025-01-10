use crate::serde::{big_decimal_as_string, u128_as_string};
use crate::types::{PubkeyBase58String, DecimalPrice, Decimals};
use super::TokenProgram;

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PoolInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PoolInitializedEventOrigin,

    #[serde(rename = "ts")]
    pub tick_spacing: u16,
    #[serde(rename = "sp", with = "u128_as_string")]
    pub sqrt_price: u128,
    #[serde(rename = "dp", with = "big_decimal_as_string")]
    pub decimal_price: DecimalPrice,

    #[serde(rename = "c")]
    pub config: PubkeyBase58String,
    #[serde(rename = "tma")]
    pub token_mint_a: PubkeyBase58String,
    #[serde(rename = "tmb")]
    pub token_mint_b: PubkeyBase58String,
    #[serde(rename = "f")]
    pub funder: PubkeyBase58String,
    #[serde(rename = "w")]
    pub whirlpool: PubkeyBase58String,
    #[serde(rename = "ft")]
    pub fee_tier: PubkeyBase58String,

    #[serde(rename = "tpa")]
    pub token_program_a: TokenProgram,
    #[serde(rename = "tpb")]
    pub token_program_b: TokenProgram,

    // decimals
    #[serde(rename = "tda")]
    pub token_decimals_a: Decimals,
    #[serde(rename = "tdb")]
    pub token_decimals_b: Decimals,

    // pool state
    #[serde(rename = "cti")]
    pub current_tick_index: i32,
    #[serde(rename = "fr")]
    pub fee_rate: u16,
    #[serde(rename = "pfr")]
    pub protocol_fee_rate: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PoolInitializedEventOrigin {
    #[serde(rename = "ip")]
    InitializePool,
    #[serde(rename = "ipv2")]
    InitializePoolV2,
}
