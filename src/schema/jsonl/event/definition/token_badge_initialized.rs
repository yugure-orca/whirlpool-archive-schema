use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TokenBadgeInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: TokenBadgeInitializedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyBase58String,

    #[serde(rename = "ce")]
    pub config_extension: PubkeyBase58String,

    #[serde(rename = "tm")]
    pub token_mint: PubkeyBase58String,

    #[serde(rename = "tb")]
    pub token_badge: PubkeyBase58String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TokenBadgeInitializedEventOrigin {
    #[serde(rename = "itb")]
    InitializeTokenBadge,
}
