use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PoolFeeRateUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PoolFeeRateUpdatedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyBase58String,
    #[serde(rename = "w")]
    pub whirlpool: PubkeyBase58String,

    #[serde(rename = "ofr")]
    pub old_fee_rate: u16,
    #[serde(rename = "nfr")]
    pub new_fee_rate: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PoolFeeRateUpdatedEventOrigin {
    #[serde(rename = "sfr")]
    SetFeeRate,
}
