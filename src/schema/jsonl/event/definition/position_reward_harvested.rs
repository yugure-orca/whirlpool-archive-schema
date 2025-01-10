use super::TransferInfo;
use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionRewardHarvestedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionRewardHarvestedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyBase58String,
    #[serde(rename = "pa")]
    pub position_authority: PubkeyBase58String,
    #[serde(rename = "p")]
    pub position: PubkeyBase58String,

    #[serde(rename = "ri")]
    pub reward_index: u8,

    // transfer info
    #[serde(rename = "tr")]
    pub transfer_reward: TransferInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionRewardHarvestedEventOrigin {
    #[serde(rename = "cr")]
    CollectReward,
    #[serde(rename = "crv2")]
    CollectRewardV2,
}
