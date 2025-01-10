use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FeeTierInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: FeeTierInitializedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyBase58String,

    #[serde(rename = "ft")]
    pub fee_tier: PubkeyBase58String,

    #[serde(rename = "ts")]
    pub tick_spacing: u16,

    #[serde(rename = "dfr")]
    pub default_fee_rate: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum FeeTierInitializedEventOrigin {
    #[serde(rename = "ift")]
    InitializeFeeTier,
}
