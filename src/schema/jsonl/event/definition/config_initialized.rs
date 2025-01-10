use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ConfigInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: ConfigInitializedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyBase58String,

    #[serde(rename = "fa")]
    pub fee_authority: PubkeyBase58String,

    #[serde(rename = "cpfa")]
    pub collect_protocol_fees_authority: PubkeyBase58String,

    #[serde(rename = "resa")]
    pub reward_emissions_super_authority: PubkeyBase58String,

    #[serde(rename = "dpfr")]
    pub default_protocol_fee_rate: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ConfigInitializedEventOrigin {
    #[serde(rename = "ic")]
    InitializeConfig,
}
