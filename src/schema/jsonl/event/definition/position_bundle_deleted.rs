use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PositionBundleDeletedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: PositionBundleDeletedEventOrigin,

    #[serde(rename = "pb")]
    pub position_bundle: PubkeyBase58String,

    #[serde(rename = "pbm")]
    pub position_bundle_mint: PubkeyBase58String,

    #[serde(rename = "pbo")]
    pub position_bundle_owner: PubkeyBase58String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum PositionBundleDeletedEventOrigin {
    #[serde(rename = "dpb")]
    DeletePositionBundle,
}
