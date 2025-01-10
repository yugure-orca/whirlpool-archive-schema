use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionHarvestUpdatedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionHarvestUpdatedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyBase58String,
    #[serde(rename = "p")]
    pub position: PubkeyBase58String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionHarvestUpdatedEventOrigin {
    #[serde(rename = "ufar")]
    UpdateFeesAndRewards,
}
