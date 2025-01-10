use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ConfigExtensionUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: ConfigExtensionUpdatedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyBase58String,

    #[serde(rename = "ce")]
    pub config_extension: PubkeyBase58String,

    #[serde(rename = "ocea")]
    pub old_config_extension_authority: PubkeyBase58String,
    #[serde(rename = "ncea")]
    pub new_config_extension_authority: PubkeyBase58String,

    #[serde(rename = "otba")]
    pub old_token_badge_authority: PubkeyBase58String,
    #[serde(rename = "ntba")]
    pub new_token_badge_authority: PubkeyBase58String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ConfigExtensionUpdatedEventOrigin {
    #[serde(rename = "scea")]
    SetConfigExtensionAuthority,
    #[serde(rename = "stba")]
    SetTokenBadgeAuthority,
}
