use crate::serde::{u64_as_string, vec_u8_as_base64_string};
use crate::types::{BlockHeight, BlockTime, Bytes, PubkeyString, SignatureString, Slot};
use serde_derive::{Deserialize, Serialize};
mod definition;
pub use definition::*;

/*

Whirlpool Transaction File JSON Lines Format

A whirlpool transaction file (whirlpool-transaction-yyyymmdd.json.gz) is GZIP compressed text file.
Each line is a JSON object with the following schema:

{
  slot: u64,
  blockHeight: u64,
  blockTime: i64,
  transactions: [
    {
      index: u32,
      signature: String(base58 encoding),
      payer: String(base58 encoding),
      balances: [
        { account: String(base58 encoding), pre: String(u64 as string), post: String(u64 as string) },
        { account: String(base58 encoding), pre: String(u64 as string), post: String(u64 as string) },
        ...
      ],
      instructions: [
        { name: String, payload: Value },
        { name: String, payload: Value },
        ...
      ],
    },
    ...
  ]
}

*/

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BlockWhirlpoolTransaction {
    pub slot: Slot,
    pub block_height: BlockHeight,
    pub block_time: BlockTime,
    pub transactions: Vec<WhirlpoolTransaction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WhirlpoolTransaction {
    pub index: u32,
    pub signature: SignatureString,
    pub payer: PubkeyString,
    pub balances: Vec<TokenBalanceChange>,
    pub instructions: Vec<ProgramDeployOrWhirlpoolInstruction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceChange {
    pub account: PubkeyString,
    #[serde(with = "u64_as_string")]
    pub pre: u64,
    #[serde(with = "u64_as_string")]
    pub post: u64,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ProgramDeployOrWhirlpoolInstruction {
    ProgramDeployInstruction(ProgramDeployInstruction),
    WhirlpoolInstruction(WhirlpoolInstruction),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProgramDeployInstruction {
    #[serde(with = "vec_u8_as_base64_string")]
    pub program_data: Bytes,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase", tag = "name", content = "payload")]
pub enum WhirlpoolInstruction {
    AdminIncreaseLiquidity(DecodedAdminIncreaseLiquidity),
    CloseBundledPosition(DecodedCloseBundledPosition),
    ClosePosition(DecodedClosePosition),
    CollectFees(DecodedCollectFees),
    CollectProtocolFees(DecodedCollectProtocolFees),
    CollectReward(DecodedCollectReward),
    DecreaseLiquidity(DecodedDecreaseLiquidity),
    DeletePositionBundle(DecodedDeletePositionBundle),
    IncreaseLiquidity(DecodedIncreaseLiquidity),
    InitializeConfig(DecodedInitializeConfig),
    InitializeFeeTier(DecodedInitializeFeeTier),
    InitializePool(DecodedInitializePool),
    InitializePositionBundle(DecodedInitializePositionBundle),
    InitializePositionBundleWithMetadata(DecodedInitializePositionBundleWithMetadata),
    InitializeReward(DecodedInitializeReward),
    InitializeTickArray(DecodedInitializeTickArray),
    OpenBundledPosition(DecodedOpenBundledPosition),
    OpenPosition(DecodedOpenPosition),
    OpenPositionWithMetadata(DecodedOpenPositionWithMetadata),
    SetCollectProtocolFeesAuthority(DecodedSetCollectProtocolFeesAuthority),
    SetDefaultFeeRate(DecodedSetDefaultFeeRate),
    SetDefaultProtocolFeeRate(DecodedSetDefaultProtocolFeeRate),
    SetFeeAuthority(DecodedSetFeeAuthority),
    SetFeeRate(DecodedSetFeeRate),
    SetProtocolFeeRate(DecodedSetProtocolFeeRate),
    SetRewardAuthority(DecodedSetRewardAuthority),
    SetRewardAuthorityBySuperAuthority(DecodedSetRewardAuthorityBySuperAuthority),
    SetRewardEmissions(DecodedSetRewardEmissions),
    SetRewardEmissionsSuperAuthority(DecodedSetRewardEmissionsSuperAuthority),
    Swap(DecodedSwap),
    TwoHopSwap(DecodedTwoHopSwap),
    UpdateFeesAndRewards(DecodedUpdateFeesAndRewards),
    CollectFeesV2(DecodedCollectFeesV2),
    CollectProtocolFeesV2(DecodedCollectProtocolFeesV2),
    CollectRewardV2(DecodedCollectRewardV2),
    DecreaseLiquidityV2(DecodedDecreaseLiquidityV2),
    IncreaseLiquidityV2(DecodedIncreaseLiquidityV2),
    SwapV2(DecodedSwapV2),
    TwoHopSwapV2(DecodedTwoHopSwapV2),
    InitializePoolV2(DecodedInitializePoolV2),
    InitializeRewardV2(DecodedInitializeRewardV2),
    SetRewardEmissionsV2(DecodedSetRewardEmissionsV2),
    InitializeConfigExtension(DecodedInitializeConfigExtension),
    InitializeTokenBadge(DecodedInitializeTokenBadge),
    DeleteTokenBadge(DecodedDeleteTokenBadge),
    SetConfigExtensionAuthority(DecodedSetConfigExtensionAuthority),
    SetTokenBadgeAuthority(DecodedSetTokenBadgeAuthority),
    OpenPositionWithTokenExtensions(DecodedOpenPositionWithTokenExtensions),
    ClosePositionWithTokenExtensions(DecodedClosePositionWithTokenExtensions),
}

impl serde::Serialize for ProgramDeployOrWhirlpoolInstruction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        match self {
            ProgramDeployOrWhirlpoolInstruction::ProgramDeployInstruction(instruction) => {
                let mut state = serializer.serialize_struct("TransactionInstruction", 2)?;
                state.serialize_field("name", "programDeploy")?;
                state.serialize_field("payload", instruction)?;
                state.end()
            }
            ProgramDeployOrWhirlpoolInstruction::WhirlpoolInstruction(instruction) => {
                instruction.serialize(serializer)
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for ProgramDeployOrWhirlpoolInstruction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let name = value
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| serde::de::Error::missing_field("name"))?;

        if name == "programDeploy" {
            let payload = value
                .get("payload")
                .ok_or_else(|| serde::de::Error::missing_field("payload"))?;
            let instruction = ProgramDeployInstruction::deserialize(payload)
                .map_err(serde::de::Error::custom)?;
            Ok(ProgramDeployOrWhirlpoolInstruction::ProgramDeployInstruction(instruction))
        } else {
            let instruction = WhirlpoolInstruction::deserialize(value)
                .map_err(serde::de::Error::custom)?;
            Ok(ProgramDeployOrWhirlpoolInstruction::WhirlpoolInstruction(instruction))
        }
    }
}
