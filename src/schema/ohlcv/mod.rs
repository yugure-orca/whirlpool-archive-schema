use crate::serde::{big_decimal_as_string, u128_as_string, u64_as_string};
use crate::types::{BlockTime, DecimalPrice, PubkeyString, Slot, TokenDecimals};
use serde_derive::{Deserialize, Serialize};

/*

Whirlpool OHLCV Daily JSON Lines Format

To reduce data size, we use short field names.

Each line is a JSON object with the following schema:

{
  whirlpool(w): String(base58 encoding),
  whirlpoolsConfig(wc): String(base58 encoding),
  tokenA(ta): { mint(m): String(base58 encoding), decimals(d): u8 },
  tokenB(tb): { mint(m): String(base58 encoding), decimals(d): u8 },
  tickSpacing(ts): u16,
  initialState(is):
    { t: "existing(e)", p: { previousCloseSqrtPrice(pcsp): String, previousCloseDecimalPrice(pcdp): String } } |
    { t: "new(n)", p: { initialSqrtPrice(isp): String, initialDecimalPrice(idp): String, initializedSlot: u64, initializedBlockTime(ibt): i64 } },
  estimatedFees(ef): {
    liquidityProviderFeeA(lpfa): u64,
    liquidityProviderFeeB(lpfb): u64,
    protocolFeeA(pfa): u64,
    protocolFeeB(pfb): u64,
  },
  daily(d): {
    timestamp(t): i64(UTC, UNIX timestamp in seconds, first second of the day),
    ohlc(p): { sqrtPrice(sp): { open(o): String, high(h): String, low(l): String, close(c): String }, decimalPrice(dp): { open(o): String, high(h): String, low(l): String, close(c): String } },
    volume(v): {
      ab: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
      ba: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
    },
  },
}

Whirlpool OHLCV Minutely JSON Lines Format

To reduce data size, we use short field names.
Also, data for minutes with no trades at all will be omitted.

Each line is a JSON object with the following schema:

{
  whirlpool(w): String(base58 encoding),
  whirlpoolsConfig(wc): String(base58 encoding),
  tokenA(ta): { mint(m): String(base58 encoding), decimals(d): u8 },
  tokenB(tb): { mint(m): String(base58 encoding), decimals(d): u8 },
  tickSpacing(ts): u16,
  initialState(is):
    { t: "existing(e)", p: { previousCloseSqrtPrice(pcsp): String, previousCloseDecimalPrice(pcdp): String } } |
    { t: "new(n)", p: { initialSqrtPrice(isp): String, initialDecimalPrice(idp): String, initializedSlot: u64, initializedBlockTime(ibt): i64 } },
  estimatedFees(ef): {
    liquidityProviderFeeA(lpfa): u64,
    liquidityProviderFeeB(lpfb): u64,
    protocolFeeA(pfa): u64,
    protocolFeeB(pfb): u64,
  },
  daily(d): {
    timestamp(t): i64(UTC, UNIX timestamp in seconds, first second of the day),
    ohlc(p): { sqrtPrice(sp): { open(o): String, high(h): String, low(l): String, close(c): String }, decimalPrice(dp): { open(o): String, high(h): String, low(l): String, close(c): String } },
    volume(v): {
      ab: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
      ba: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
    },
  },
  minutely(m): [
    {
      timestamp(t): i64(UTC, UNIX timestamp in seconds, first second of the minute),
      ohlc(p): { sqrtPrice(sp): { open(o): String, high(h): String, low(l): String, close(c): String }, decimalPrice(dp): { open(o): String, high(h): String, low(l): String, close(c): String } },
      volume(v): {
        ab: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
        ba: { totalIn(ti): String, totalOut(to): String, count(c): u64 },
      },
    },
    ...
  ],
}

*/

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct WhirlpoolOhlcvDailyData {
    #[serde(flatten)]
    pub metadata: OhlcvMetadata,
    #[serde(rename = "is")]
    pub initial_state: InitialState,
    #[serde(rename = "ef")]
    pub estimated_fees: EstimatedFees,
    #[serde(rename = "d")]
    pub daily: OhlcvData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct WhirlpoolOhlcvMinutelyData {
    #[serde(flatten)]
    pub metadata: OhlcvMetadata,
    #[serde(rename = "is")]
    pub initial_state: InitialState,
    #[serde(rename = "ef")]
    pub estimated_fees: EstimatedFees,
    #[serde(rename = "d")]
    pub daily: OhlcvData,
    #[serde(rename = "m")]
    pub minutely: Vec<OhlcvData>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct OhlcvMetadata {
    #[serde(rename = "w")]
    pub whirlpool: PubkeyString,
    #[serde(rename = "wc")]
    pub whirlpools_config: PubkeyString,
    #[serde(rename = "ta")]
    pub token_a: TokenData,
    #[serde(rename = "tb")]
    pub token_b: TokenData,
    #[serde(rename = "ts")]
    pub tick_spacing: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TokenData {
    #[serde(rename = "m")]
    pub mint: PubkeyString,
    #[serde(rename = "d")]
    pub decimals: TokenDecimals,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(tag = "t", content = "p")]
pub enum InitialState {
    #[serde(rename = "e")]
    Existing {
        #[serde(rename = "pcsp", with = "u128_as_string")]
        previous_close_sqrt_price: u128,
        #[serde(rename = "pcdp", with = "big_decimal_as_string")]
        previous_close_decimal_price: DecimalPrice,
    },
    #[serde(rename = "n")]
    New {
        #[serde(rename = "isp", with = "u128_as_string")]
        initial_sqrt_price: u128,
        #[serde(rename = "idp", with = "big_decimal_as_string")]
        initial_decimal_price: DecimalPrice,
        #[serde(rename = "is")]
        initialized_slot: Slot,
        #[serde(rename = "ibt")]
        initialized_block_time: BlockTime,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct EstimatedFees {
    #[serde(rename = "lpfa", with = "u64_as_string")]
    pub liquidity_provider_fee_a: u64,
    #[serde(rename = "lpfb", with = "u64_as_string")]
    pub liquidity_provider_fee_b: u64,
    #[serde(rename = "pfa", with = "u64_as_string")]
    pub protocol_fee_a: u64,
    #[serde(rename = "pfb", with = "u64_as_string")]
    pub protocol_fee_b: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct OhlcvData {
    #[serde(rename = "t")]
    pub timestamp: BlockTime,
    #[serde(rename = "p")]
    pub ohlc: PriceData,
    #[serde(rename = "v")]
    pub volume: VolumeData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PriceData {
    #[serde(rename = "sp")]
    pub sqrt_price: SqrtPriceData,
    #[serde(rename = "dp")]
    pub decimal_price: DecimalPriceData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SqrtPriceData {
    #[serde(rename = "o", with = "u128_as_string")]
    pub open: u128,
    #[serde(rename = "h", with = "u128_as_string")]
    pub high: u128,
    #[serde(rename = "l", with = "u128_as_string")]
    pub low: u128,
    #[serde(rename = "c", with = "u128_as_string")]
    pub close: u128,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct DecimalPriceData {
    #[serde(rename = "o", with = "big_decimal_as_string")]
    pub open: DecimalPrice,
    #[serde(rename = "h", with = "big_decimal_as_string")]
    pub high: DecimalPrice,
    #[serde(rename = "l", with = "big_decimal_as_string")]
    pub low: DecimalPrice,
    #[serde(rename = "c", with = "big_decimal_as_string")]
    pub close: DecimalPrice,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct VolumeData {
    pub ab: VolumeDirectionData,
    pub ba: VolumeDirectionData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct VolumeDirectionData {
    #[serde(rename = "ti", with = "u128_as_string")]
    pub total_in: u128,
    #[serde(rename = "to", with = "u128_as_string")]
    pub total_out: u128,
    #[serde(rename = "c")]
    pub count: u64,
}
