use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct RewardAuthorityUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: RewardAuthorityUpdatedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyBase58String,

    #[serde(rename = "ri")]
    pub reward_index: u8,

    #[serde(rename = "ora")]
    pub old_reward_authority: PubkeyBase58String,
    #[serde(rename = "nra")]
    pub new_reward_authority: PubkeyBase58String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum RewardAuthorityUpdatedEventOrigin {
    #[serde(rename = "sra")]
    SetRewardAuthority,
    #[serde(rename = "srabsa")]
    SetRewardAuthorityBySuperAuthority,
}
