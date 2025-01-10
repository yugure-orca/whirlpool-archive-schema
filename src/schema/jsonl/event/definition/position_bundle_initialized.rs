use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionBundleInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionBundleInitializedEventOrigin,

    #[serde(rename = "pb")]
    pub position_bundle: PubkeyBase58String,

    #[serde(rename = "pbm")]
    pub position_bundle_mint: PubkeyBase58String,

    #[serde(rename = "pbo")]
    pub position_bundle_owner: PubkeyBase58String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionBundleInitializedEventOrigin {
    #[serde(rename = "ipb")]
    InitializePositionBundle,
    #[serde(rename = "ipbwm")]
    InitializePositionBundleWithMetadata,
}
