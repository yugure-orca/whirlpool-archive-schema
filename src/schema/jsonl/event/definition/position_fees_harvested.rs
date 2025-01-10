use super::TransferInfo;
use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionFeesHarvestedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionFeesHarvestedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyBase58String,
    #[serde(rename = "pa")]
    pub position_authority: PubkeyBase58String,
    #[serde(rename = "p")]
    pub position: PubkeyBase58String,

    // transfer info
    #[serde(rename = "ta")]
    pub transfer_a: TransferInfo,
    #[serde(rename = "tb")]
    pub transfer_b: TransferInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionFeesHarvestedEventOrigin {
    #[serde(rename = "cf")]
    CollectFees,
    #[serde(rename = "cfv2")]
    CollectFeesV2,
}
