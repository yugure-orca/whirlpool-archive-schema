use super::TransferInfo;
use crate::types::PubkeyBase58String;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ProtocolFeesCollectedEventPayload {
    // origin
    #[serde(rename = "o")]
    pub origin: ProtocolFeesCollectedEventOrigin,

    #[serde(rename = "c")]
    pub config: PubkeyBase58String,
    #[serde(rename = "w")]
    pub whirlpool: PubkeyBase58String,
    #[serde(rename = "cpfa")]
    pub collect_protocol_fees_authority: PubkeyBase58String,

    // transfer info
    #[serde(rename = "ta")]
    pub transfer_a: TransferInfo,
    #[serde(rename = "tb")]
    pub transfer_b: TransferInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum ProtocolFeesCollectedEventOrigin {
    #[serde(rename = "cpf")]
    CollectProtocolFees,
    #[serde(rename = "cpfv2")]
    CollectProtocolFeesV2,
}
