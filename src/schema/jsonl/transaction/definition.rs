use serde_derive::{Deserialize, Serialize};
use crate::types::PubkeyString;
use crate::serde::{u128_as_string, u64_as_string, bool_as_u8};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedAdminIncreaseLiquidity {
  #[serde(with = "u128_as_string")]
  pub data_liquidity: u128,
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCloseBundledPosition {
  pub data_bundle_index: u16,
  pub key_bundled_position: PubkeyString,
  pub key_position_bundle: PubkeyString,
  pub key_position_bundle_token_account: PubkeyString,
  pub key_position_bundle_authority: PubkeyString,
  pub key_receiver: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedClosePosition {
  pub key_position_authority: PubkeyString,
  pub key_receiver: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_mint: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_token_program: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectFees {
  pub key_whirlpool: PubkeyString,
  pub key_position_authority: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_token_owner_account_a: PubkeyString,
  pub key_token_vault_a: PubkeyString,
  pub key_token_owner_account_b: PubkeyString,
  pub key_token_vault_b: PubkeyString,
  pub key_token_program: PubkeyString,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectProtocolFees {
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_collect_protocol_fees_authority: PubkeyString,
  pub key_token_vault_a: PubkeyString,
  pub key_token_vault_b: PubkeyString,
  pub key_token_destination_a: PubkeyString,
  pub key_token_destination_b: PubkeyString,
  pub key_token_program: PubkeyString,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectReward {
  pub data_reward_index: u8,
  pub key_whirlpool: PubkeyString,
  pub key_position_authority: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_reward_owner_account: PubkeyString,
  pub key_reward_vault: PubkeyString,
  pub key_token_program: PubkeyString,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDecreaseLiquidity {
  #[serde(with = "u128_as_string")]
  pub data_liquidity_amount: u128,
  #[serde(with = "u64_as_string")]
  pub data_token_amount_min_a: u64,
  #[serde(with = "u64_as_string")]
  pub data_token_amount_min_b: u64,
  pub key_whirlpool: PubkeyString,
  pub key_token_program: PubkeyString,
  pub key_position_authority: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_token_owner_account_a: PubkeyString,
  pub key_token_owner_account_b: PubkeyString,
  pub key_token_vault_a: PubkeyString,
  pub key_token_vault_b: PubkeyString,
  pub key_tick_array_lower: PubkeyString,
  pub key_tick_array_upper: PubkeyString,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDeletePositionBundle {
  pub key_position_bundle: PubkeyString,
  pub key_position_bundle_mint: PubkeyString,
  pub key_position_bundle_token_account: PubkeyString,
  pub key_position_bundle_owner: PubkeyString,
  pub key_receiver: PubkeyString,
  pub key_token_program: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedIncreaseLiquidity {
  #[serde(with = "u128_as_string")]
  pub data_liquidity_amount: u128,
  #[serde(with = "u64_as_string")]
  pub data_token_amount_max_a: u64,
  #[serde(with = "u64_as_string")]
  pub data_token_amount_max_b: u64,
  pub key_whirlpool: PubkeyString,
  pub key_token_program: PubkeyString,
  pub key_position_authority: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_token_owner_account_a: PubkeyString,
  pub key_token_owner_account_b: PubkeyString,
  pub key_token_vault_a: PubkeyString,
  pub key_token_vault_b: PubkeyString,
  pub key_tick_array_lower: PubkeyString,
  pub key_tick_array_upper: PubkeyString,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeConfig {
  pub data_default_protocol_fee_rate: u16,
  pub data_fee_authority: PubkeyString,
  pub data_collect_protocol_fees_authority: PubkeyString,
  pub data_reward_emissions_super_authority: PubkeyString,
  pub key_whirlpools_config: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_system_program: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeFeeTier {
  pub data_tick_spacing: u16,
  pub data_default_fee_rate: u16,
  pub key_whirlpools_config: PubkeyString,
  pub key_fee_tier: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_fee_authority: PubkeyString,
  pub key_system_program: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePool {
  pub data_tick_spacing: u16,
  #[serde(with = "u128_as_string")]
  pub data_initial_sqrt_price: u128,
  pub key_whirlpools_config: PubkeyString,
  pub key_token_mint_a: PubkeyString,
  pub key_token_mint_b: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_token_vault_a: PubkeyString,
  pub key_token_vault_b: PubkeyString,
  pub key_fee_tier: PubkeyString,
  pub key_token_program: PubkeyString,
  pub key_system_program: PubkeyString,
  pub key_rent: PubkeyString,
  #[cfg(feature = "transaction_decimals")]
  pub decimals_token_mint_a: u8,
  #[cfg(feature = "transaction_decimals")]
  pub decimals_token_mint_b: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePositionBundle {
  pub key_position_bundle: PubkeyString,
  pub key_position_bundle_mint: PubkeyString,
  pub key_position_bundle_token_account: PubkeyString,
  pub key_position_bundle_owner: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_token_program: PubkeyString,
  pub key_system_program: PubkeyString,
  pub key_rent: PubkeyString,
  pub key_associated_token_program: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePositionBundleWithMetadata {
  pub key_position_bundle: PubkeyString,
  pub key_position_bundle_mint: PubkeyString,
  pub key_position_bundle_metadata: PubkeyString,
  pub key_position_bundle_token_account: PubkeyString,
  pub key_position_bundle_owner: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_metadata_update_auth: PubkeyString,
  pub key_token_program: PubkeyString,
  pub key_system_program: PubkeyString,
  pub key_rent: PubkeyString,
  pub key_associated_token_program: PubkeyString,
  pub key_metadata_program: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeReward {
  pub data_reward_index: u8,
  pub key_reward_authority: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_reward_mint: PubkeyString,
  pub key_reward_vault: PubkeyString,
  pub key_token_program: PubkeyString,
  pub key_system_program: PubkeyString,
  pub key_rent: PubkeyString,
  #[cfg(feature = "transaction_decimals")]
  pub decimals_reward_mint: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeTickArray {
  pub data_start_tick_index: i32,
  pub key_whirlpool: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_tick_array: PubkeyString,
  pub key_system_program: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenBundledPosition {
  pub data_bundle_index: u16,
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_bundled_position: PubkeyString,
  pub key_position_bundle: PubkeyString,
  pub key_position_bundle_token_account: PubkeyString,
  pub key_position_bundle_authority: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_system_program: PubkeyString,
  pub key_rent: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPosition {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_funder: PubkeyString,
  pub key_owner: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_mint: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_token_program: PubkeyString,
  pub key_system_program: PubkeyString,
  pub key_rent: PubkeyString,
  pub key_associated_token_program: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPositionWithMetadata {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_funder: PubkeyString,
  pub key_owner: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_mint: PubkeyString,
  pub key_position_metadata_account: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_token_program: PubkeyString,
  pub key_system_program: PubkeyString,
  pub key_rent: PubkeyString,
  pub key_associated_token_program: PubkeyString,
  pub key_metadata_program: PubkeyString,
  pub key_metadata_update_auth: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetCollectProtocolFeesAuthority {
  pub key_whirlpools_config: PubkeyString,
  pub key_collect_protocol_fees_authority: PubkeyString,
  pub key_new_collect_protocol_fees_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetDefaultFeeRate {
  pub data_default_fee_rate: u16,
  pub key_whirlpools_config: PubkeyString,
  pub key_fee_tier: PubkeyString,
  pub key_fee_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetDefaultProtocolFeeRate {
  pub data_default_protocol_fee_rate: u16,
  pub key_whirlpools_config: PubkeyString,
  pub key_fee_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetFeeAuthority {
  pub key_whirlpools_config: PubkeyString,
  pub key_fee_authority: PubkeyString,
  pub key_new_fee_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetFeeRate {
  pub data_fee_rate: u16,
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_fee_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetProtocolFeeRate {
  pub data_protocol_fee_rate: u16,
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_fee_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardAuthority {
  pub data_reward_index: u8,
  pub key_whirlpool: PubkeyString,
  pub key_reward_authority: PubkeyString,
  pub key_new_reward_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardAuthorityBySuperAuthority {
  pub data_reward_index: u8,
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_reward_emissions_super_authority: PubkeyString,
  pub key_new_reward_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissions {
  pub data_reward_index: u8,
  #[serde(with = "u128_as_string")]
  pub data_emissions_per_second_x64: u128,
  pub key_whirlpool: PubkeyString,
  pub key_reward_authority: PubkeyString,
  pub key_reward_vault: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissionsSuperAuthority {
  pub key_whirlpools_config: PubkeyString,
  pub key_reward_emissions_super_authority: PubkeyString,
  pub key_new_reward_emissions_super_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSwap {
  #[serde(with = "u64_as_string")]
  pub data_amount: u64,
  #[serde(with = "u64_as_string")]
  pub data_other_amount_threshold: u64,
  #[serde(with = "u128_as_string")]
  pub data_sqrt_price_limit: u128,
  #[serde(with = "bool_as_u8")]
  pub data_amount_specified_is_input: bool,
  #[serde(with = "bool_as_u8")]
  pub data_a_to_b: bool,
  pub key_token_program: PubkeyString,
  pub key_token_authority: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_token_owner_account_a: PubkeyString,
  pub key_vault_a: PubkeyString,
  pub key_token_owner_account_b: PubkeyString,
  pub key_vault_b: PubkeyString,
  pub key_tick_array_0: PubkeyString,
  pub key_tick_array_1: PubkeyString,
  pub key_tick_array_2: PubkeyString,
  pub key_oracle: PubkeyString,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedTwoHopSwap {
  #[serde(with = "u64_as_string")]
  pub data_amount: u64,
  #[serde(with = "u64_as_string")]
  pub data_other_amount_threshold: u64,
  #[serde(with = "bool_as_u8")]
  pub data_amount_specified_is_input: bool,
  #[serde(with = "bool_as_u8")]
  pub data_a_to_b_one: bool,
  #[serde(with = "bool_as_u8")]
  pub data_a_to_b_two: bool,
  #[serde(with = "u128_as_string")]
  pub data_sqrt_price_limit_one: u128,
  #[serde(with = "u128_as_string")]
  pub data_sqrt_price_limit_two: u128,
  pub key_token_program: PubkeyString,
  pub key_token_authority: PubkeyString,
  pub key_whirlpool_one: PubkeyString,
  pub key_whirlpool_two: PubkeyString,
  pub key_token_owner_account_one_a: PubkeyString,
  pub key_vault_one_a: PubkeyString,
  pub key_token_owner_account_one_b: PubkeyString,
  pub key_vault_one_b: PubkeyString,
  pub key_token_owner_account_two_a: PubkeyString,
  pub key_vault_two_a: PubkeyString,
  pub key_token_owner_account_two_b: PubkeyString,
  pub key_vault_two_b: PubkeyString,
  pub key_tick_array_one_0: PubkeyString,
  pub key_tick_array_one_1: PubkeyString,
  pub key_tick_array_one_2: PubkeyString,
  pub key_tick_array_two_0: PubkeyString,
  pub key_tick_array_two_1: PubkeyString,
  pub key_tick_array_two_2: PubkeyString,
  pub key_oracle_one: PubkeyString,
  pub key_oracle_two: PubkeyString,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_1: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_2: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_3: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedUpdateFeesAndRewards {
  pub key_whirlpool: PubkeyString,
  pub key_position: PubkeyString,
  pub key_tick_array_lower: PubkeyString,
  pub key_tick_array_upper: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectFeesV2 {
  pub key_whirlpool: PubkeyString,
  pub key_position_authority: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_token_mint_a: PubkeyString,
  pub key_token_mint_b: PubkeyString,
  pub key_token_owner_account_a: PubkeyString,
  pub key_token_vault_a: PubkeyString,
  pub key_token_owner_account_b: PubkeyString,
  pub key_token_vault_b: PubkeyString,
  pub key_token_program_a: PubkeyString,
  pub key_token_program_b: PubkeyString,
  pub key_memo_program: PubkeyString,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectProtocolFeesV2 {
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_collect_protocol_fees_authority: PubkeyString,
  pub key_token_mint_a: PubkeyString,
  pub key_token_mint_b: PubkeyString,
  pub key_token_vault_a: PubkeyString,
  pub key_token_vault_b: PubkeyString,
  pub key_token_destination_a: PubkeyString,
  pub key_token_destination_b: PubkeyString,
  pub key_token_program_a: PubkeyString,
  pub key_token_program_b: PubkeyString,
  pub key_memo_program: PubkeyString,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectRewardV2 {
  pub data_reward_index: u8,
  pub key_whirlpool: PubkeyString,
  pub key_position_authority: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_reward_owner_account: PubkeyString,
  pub key_reward_mint: PubkeyString,
  pub key_reward_vault: PubkeyString,
  pub key_reward_token_program: PubkeyString,
  pub key_memo_program: PubkeyString,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDecreaseLiquidityV2 {
  #[serde(with = "u128_as_string")]
  pub data_liquidity_amount: u128,
  #[serde(with = "u64_as_string")]
  pub data_token_amount_min_a: u64,
  #[serde(with = "u64_as_string")]
  pub data_token_amount_min_b: u64,
  pub key_whirlpool: PubkeyString,
  pub key_token_program_a: PubkeyString,
  pub key_token_program_b: PubkeyString,
  pub key_memo_program: PubkeyString,
  pub key_position_authority: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_token_mint_a: PubkeyString,
  pub key_token_mint_b: PubkeyString,
  pub key_token_owner_account_a: PubkeyString,
  pub key_token_owner_account_b: PubkeyString,
  pub key_token_vault_a: PubkeyString,
  pub key_token_vault_b: PubkeyString,
  pub key_tick_array_lower: PubkeyString,
  pub key_tick_array_upper: PubkeyString,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedIncreaseLiquidityV2 {
  #[serde(with = "u128_as_string")]
  pub data_liquidity_amount: u128,
  #[serde(with = "u64_as_string")]
  pub data_token_amount_max_a: u64,
  #[serde(with = "u64_as_string")]
  pub data_token_amount_max_b: u64,
  pub key_whirlpool: PubkeyString,
  pub key_token_program_a: PubkeyString,
  pub key_token_program_b: PubkeyString,
  pub key_memo_program: PubkeyString,
  pub key_position_authority: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_token_mint_a: PubkeyString,
  pub key_token_mint_b: PubkeyString,
  pub key_token_owner_account_a: PubkeyString,
  pub key_token_owner_account_b: PubkeyString,
  pub key_token_vault_a: PubkeyString,
  pub key_token_vault_b: PubkeyString,
  pub key_tick_array_lower: PubkeyString,
  pub key_tick_array_upper: PubkeyString,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSwapV2 {
  #[serde(with = "u64_as_string")]
  pub data_amount: u64,
  #[serde(with = "u64_as_string")]
  pub data_other_amount_threshold: u64,
  #[serde(with = "u128_as_string")]
  pub data_sqrt_price_limit: u128,
  #[serde(with = "bool_as_u8")]
  pub data_amount_specified_is_input: bool,
  #[serde(with = "bool_as_u8")]
  pub data_a_to_b: bool,
  pub key_token_program_a: PubkeyString,
  pub key_token_program_b: PubkeyString,
  pub key_memo_program: PubkeyString,
  pub key_token_authority: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_token_mint_a: PubkeyString,
  pub key_token_mint_b: PubkeyString,
  pub key_token_owner_account_a: PubkeyString,
  pub key_vault_a: PubkeyString,
  pub key_token_owner_account_b: PubkeyString,
  pub key_vault_b: PubkeyString,
  pub key_tick_array_0: PubkeyString,
  pub key_tick_array_1: PubkeyString,
  pub key_tick_array_2: PubkeyString,
  pub key_oracle: PubkeyString,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedTwoHopSwapV2 {
  #[serde(with = "u64_as_string")]
  pub data_amount: u64,
  #[serde(with = "u64_as_string")]
  pub data_other_amount_threshold: u64,
  #[serde(with = "bool_as_u8")]
  pub data_amount_specified_is_input: bool,
  #[serde(with = "bool_as_u8")]
  pub data_a_to_b_one: bool,
  #[serde(with = "bool_as_u8")]
  pub data_a_to_b_two: bool,
  #[serde(with = "u128_as_string")]
  pub data_sqrt_price_limit_one: u128,
  #[serde(with = "u128_as_string")]
  pub data_sqrt_price_limit_two: u128,
  pub key_whirlpool_one: PubkeyString,
  pub key_whirlpool_two: PubkeyString,
  pub key_token_mint_input: PubkeyString,
  pub key_token_mint_intermediate: PubkeyString,
  pub key_token_mint_output: PubkeyString,
  pub key_token_program_input: PubkeyString,
  pub key_token_program_intermediate: PubkeyString,
  pub key_token_program_output: PubkeyString,
  pub key_token_owner_account_input: PubkeyString,
  pub key_vault_one_input: PubkeyString,
  pub key_vault_one_intermediate: PubkeyString,
  pub key_vault_two_intermediate: PubkeyString,
  pub key_vault_two_output: PubkeyString,
  pub key_token_owner_account_output: PubkeyString,
  pub key_token_authority: PubkeyString,
  pub key_tick_array_one_0: PubkeyString,
  pub key_tick_array_one_1: PubkeyString,
  pub key_tick_array_one_2: PubkeyString,
  pub key_tick_array_two_0: PubkeyString,
  pub key_tick_array_two_1: PubkeyString,
  pub key_tick_array_two_2: PubkeyString,
  pub key_oracle_one: PubkeyString,
  pub key_oracle_two: PubkeyString,
  pub key_memo_program: PubkeyString,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
  pub transfer_2: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePoolV2 {
  pub data_tick_spacing: u16,
  #[serde(with = "u128_as_string")]
  pub data_initial_sqrt_price: u128,
  pub key_whirlpools_config: PubkeyString,
  pub key_token_mint_a: PubkeyString,
  pub key_token_mint_b: PubkeyString,
  pub key_token_badge_a: PubkeyString,
  pub key_token_badge_b: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_token_vault_a: PubkeyString,
  pub key_token_vault_b: PubkeyString,
  pub key_fee_tier: PubkeyString,
  pub key_token_program_a: PubkeyString,
  pub key_token_program_b: PubkeyString,
  pub key_system_program: PubkeyString,
  pub key_rent: PubkeyString,
  #[cfg(feature = "transaction_decimals")]
  pub decimals_token_mint_a: u8,
  #[cfg(feature = "transaction_decimals")]
  pub decimals_token_mint_b: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeRewardV2 {
  pub data_reward_index: u8,
  pub key_reward_authority: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_whirlpool: PubkeyString,
  pub key_reward_mint: PubkeyString,
  pub key_reward_token_badge: PubkeyString,
  pub key_reward_vault: PubkeyString,
  pub key_reward_token_program: PubkeyString,
  pub key_system_program: PubkeyString,
  pub key_rent: PubkeyString,
  #[cfg(feature = "transaction_decimals")]
  pub decimals_reward_mint: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissionsV2 {
  pub data_reward_index: u8,
  #[serde(with = "u128_as_string")]
  pub data_emissions_per_second_x64: u128,
  pub key_whirlpool: PubkeyString,
  pub key_reward_authority: PubkeyString,
  pub key_reward_vault: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeConfigExtension {
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpools_config_extension: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_fee_authority: PubkeyString,
  pub key_system_program: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeTokenBadge {
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpools_config_extension: PubkeyString,
  pub key_token_badge_authority: PubkeyString,
  pub key_token_mint: PubkeyString,
  pub key_token_badge: PubkeyString,
  pub key_funder: PubkeyString,
  pub key_system_program: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDeleteTokenBadge {
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpools_config_extension: PubkeyString,
  pub key_token_badge_authority: PubkeyString,
  pub key_token_mint: PubkeyString,
  pub key_token_badge: PubkeyString,
  pub key_receiver: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetConfigExtensionAuthority {
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpools_config_extension: PubkeyString,
  pub key_config_extension_authority: PubkeyString,
  pub key_new_config_extension_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetTokenBadgeAuthority {
  pub key_whirlpools_config: PubkeyString,
  pub key_whirlpools_config_extension: PubkeyString,
  pub key_config_extension_authority: PubkeyString,
  pub key_new_token_badge_authority: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPositionWithTokenExtensions {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  #[serde(with = "bool_as_u8")]
  pub data_with_token_metadata_extension: bool,
  pub key_funder: PubkeyString,
  pub key_owner: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_mint: PubkeyString,
  pub key_position_token_account: PubkeyString,
  pub key_whirlpool: PubkeyString,
  // note: we can read and write "keyToken2022Program" field as expected
  pub key_token_2022_program: PubkeyString,
  pub key_system_program: PubkeyString,
  pub key_associated_token_program: PubkeyString,
  pub key_metadata_update_auth: PubkeyString,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedClosePositionWithTokenExtensions {
  pub key_position_authority: PubkeyString,
  pub key_receiver: PubkeyString,
  pub key_position: PubkeyString,
  pub key_position_mint: PubkeyString,
  pub key_position_token_account: PubkeyString,
  // note: we can read and write "keyToken2022Program" field as expected
  pub key_token_2022_program: PubkeyString,
}


pub type RemainingAccountsInfo = Vec<[u8; 2]>;
pub type RemainingAccountsKeys = Vec<String>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferAmountWithTransferFeeConfig {
  #[serde(with = "u64_as_string")]
  pub amount: u64,
  #[serde(with = "bool_as_u8")]
  pub transfer_fee_config_opt: bool,
  pub transfer_fee_config_bps: u16,
  #[serde(with = "u64_as_string")]
  pub transfer_fee_config_max: u64,
}
