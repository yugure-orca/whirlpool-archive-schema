use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ConfigExtensionInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: ConfigExtensionInitializedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyBase58String,

    #[serde(rename = "ce")]
    pub config_extension: PubkeyBase58String,

    #[serde(rename = "cea")]
    pub config_extension_authority: PubkeyBase58String,

    #[serde(rename = "tba")]
    pub token_badge_authority: PubkeyBase58String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ConfigExtensionInitializedEventOrigin {
    #[serde(rename = "ice")]
    InitializeConfigExtension,
}
