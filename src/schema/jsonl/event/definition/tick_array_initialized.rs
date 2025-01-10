use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct TickArrayInitializedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: TickArrayInitializedEventOrigin,

    #[serde(rename = "w")]
    pub whirlpool: PubkeyBase58String,

    #[serde(rename = "sti")]
    pub start_tick_index: i32,

    #[serde(rename = "ta")]
    pub tick_array: PubkeyBase58String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum TickArrayInitializedEventOrigin {
    #[serde(rename = "ita")]
    InitializeTickArray,
}
