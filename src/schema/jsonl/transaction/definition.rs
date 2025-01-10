use serde_derive::{Deserialize, Serialize};
use crate::serde::{u128_as_string, u64_as_string, bool_as_u8};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedAdminIncreaseLiquidity {
  #[serde(with = "u128_as_string")]
  pub data_liquidity: u128,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCloseBundledPosition {
  pub data_bundle_index: u16,
  pub key_bundled_position: String,
  pub key_position_bundle: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_authority: String,
  pub key_receiver: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedClosePosition {
  pub key_position_authority: String,
  pub key_receiver: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  pub key_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectFees {
  pub key_whirlpool: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_owner_account_a: String,
  pub key_token_vault_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_b: String,
  pub key_token_program: String,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectProtocolFees {
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_collect_protocol_fees_authority: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_token_destination_a: String,
  pub key_token_destination_b: String,
  pub key_token_program: String,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectReward {
  pub data_reward_index: u8,
  pub key_whirlpool: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_reward_owner_account: String,
  pub key_reward_vault: String,
  pub key_token_program: String,
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
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_owner_account_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDeletePositionBundle {
  pub key_position_bundle: String,
  pub key_position_bundle_mint: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_owner: String,
  pub key_receiver: String,
  pub key_token_program: String,
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
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_owner_account_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_0: u64,
  #[serde(with = "u64_as_string")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeConfig {
  pub data_default_protocol_fee_rate: u16,
  pub data_fee_authority: String,
  pub data_collect_protocol_fees_authority: String,
  pub data_reward_emissions_super_authority: String,
  pub key_whirlpools_config: String,
  pub key_funder: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeFeeTier {
  pub data_tick_spacing: u16,
  pub data_default_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_fee_tier: String,
  pub key_funder: String,
  pub key_fee_authority: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePool {
  pub data_tick_spacing: u16,
  #[serde(with = "u128_as_string")]
  pub data_initial_sqrt_price: u128,
  pub key_whirlpools_config: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_funder: String,
  pub key_whirlpool: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_fee_tier: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  #[cfg(feature = "decimals")]
  pub decimals_token_mint_a: u8,
  #[cfg(feature = "decimals")]
  pub decimals_token_mint_b: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePositionBundle {
  pub key_position_bundle: String,
  pub key_position_bundle_mint: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_owner: String,
  pub key_funder: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePositionBundleWithMetadata {
  pub key_position_bundle: String,
  pub key_position_bundle_mint: String,
  pub key_position_bundle_metadata: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_owner: String,
  pub key_funder: String,
  pub key_metadata_update_auth: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
  pub key_metadata_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeReward {
  pub data_reward_index: u8,
  pub key_reward_authority: String,
  pub key_funder: String,
  pub key_whirlpool: String,
  pub key_reward_mint: String,
  pub key_reward_vault: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  #[cfg(feature = "decimals")]
  pub decimals_reward_mint: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeTickArray {
  pub data_start_tick_index: i32,
  pub key_whirlpool: String,
  pub key_funder: String,
  pub key_tick_array: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenBundledPosition {
  pub data_bundle_index: u16,
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_bundled_position: String,
  pub key_position_bundle: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_authority: String,
  pub key_whirlpool: String,
  pub key_funder: String,
  pub key_system_program: String,
  pub key_rent: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPosition {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_funder: String,
  pub key_owner: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPositionWithMetadata {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_funder: String,
  pub key_owner: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_metadata_account: String,
  pub key_position_token_account: String,
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
  pub key_metadata_program: String,
  pub key_metadata_update_auth: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetCollectProtocolFeesAuthority {
  pub key_whirlpools_config: String,
  pub key_collect_protocol_fees_authority: String,
  pub key_new_collect_protocol_fees_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetDefaultFeeRate {
  pub data_default_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_fee_tier: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetDefaultProtocolFeeRate {
  pub data_default_protocol_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetFeeAuthority {
  pub key_whirlpools_config: String,
  pub key_fee_authority: String,
  pub key_new_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetFeeRate {
  pub data_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetProtocolFeeRate {
  pub data_protocol_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardAuthority {
  pub data_reward_index: u8,
  pub key_whirlpool: String,
  pub key_reward_authority: String,
  pub key_new_reward_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardAuthorityBySuperAuthority {
  pub data_reward_index: u8,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_reward_emissions_super_authority: String,
  pub key_new_reward_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissions {
  pub data_reward_index: u8,
  #[serde(with = "u128_as_string")]
  pub data_emissions_per_second_x64: u128,
  pub key_whirlpool: String,
  pub key_reward_authority: String,
  pub key_reward_vault: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissionsSuperAuthority {
  pub key_whirlpools_config: String,
  pub key_reward_emissions_super_authority: String,
  pub key_new_reward_emissions_super_authority: String,
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
  pub key_token_program: String,
  pub key_token_authority: String,
  pub key_whirlpool: String,
  pub key_token_owner_account_a: String,
  pub key_vault_a: String,
  pub key_token_owner_account_b: String,
  pub key_vault_b: String,
  pub key_tick_array_0: String,
  pub key_tick_array_1: String,
  pub key_tick_array_2: String,
  pub key_oracle: String,
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
  pub key_token_program: String,
  pub key_token_authority: String,
  pub key_whirlpool_one: String,
  pub key_whirlpool_two: String,
  pub key_token_owner_account_one_a: String,
  pub key_vault_one_a: String,
  pub key_token_owner_account_one_b: String,
  pub key_vault_one_b: String,
  pub key_token_owner_account_two_a: String,
  pub key_vault_two_a: String,
  pub key_token_owner_account_two_b: String,
  pub key_vault_two_b: String,
  pub key_tick_array_one_0: String,
  pub key_tick_array_one_1: String,
  pub key_tick_array_one_2: String,
  pub key_tick_array_two_0: String,
  pub key_tick_array_two_1: String,
  pub key_tick_array_two_2: String,
  pub key_oracle_one: String,
  pub key_oracle_two: String,
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
  pub key_whirlpool: String,
  pub key_position: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectFeesV2 {
  pub key_whirlpool: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_owner_account_a: String,
  pub key_token_vault_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_b: String,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_memo_program: String,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectProtocolFeesV2 {
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_collect_protocol_fees_authority: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_token_destination_a: String,
  pub key_token_destination_b: String,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_memo_program: String,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectRewardV2 {
  pub data_reward_index: u8,
  pub key_whirlpool: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_reward_owner_account: String,
  pub key_reward_mint: String,
  pub key_reward_vault: String,
  pub key_reward_token_program: String,
  pub key_memo_program: String,
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
  pub key_whirlpool: String,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_memo_program: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_owner_account_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
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
  pub key_whirlpool: String,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_memo_program: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_owner_account_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
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
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_memo_program: String,
  pub key_token_authority: String,
  pub key_whirlpool: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_owner_account_a: String,
  pub key_vault_a: String,
  pub key_token_owner_account_b: String,
  pub key_vault_b: String,
  pub key_tick_array_0: String,
  pub key_tick_array_1: String,
  pub key_tick_array_2: String,
  pub key_oracle: String,
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
  pub key_whirlpool_one: String,
  pub key_whirlpool_two: String,
  pub key_token_mint_input: String,
  pub key_token_mint_intermediate: String,
  pub key_token_mint_output: String,
  pub key_token_program_input: String,
  pub key_token_program_intermediate: String,
  pub key_token_program_output: String,
  pub key_token_owner_account_input: String,
  pub key_vault_one_input: String,
  pub key_vault_one_intermediate: String,
  pub key_vault_two_intermediate: String,
  pub key_vault_two_output: String,
  pub key_token_owner_account_output: String,
  pub key_token_authority: String,
  pub key_tick_array_one_0: String,
  pub key_tick_array_one_1: String,
  pub key_tick_array_one_2: String,
  pub key_tick_array_two_0: String,
  pub key_tick_array_two_1: String,
  pub key_tick_array_two_2: String,
  pub key_oracle_one: String,
  pub key_oracle_two: String,
  pub key_memo_program: String,
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
  pub key_whirlpools_config: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_badge_a: String,
  pub key_token_badge_b: String,
  pub key_funder: String,
  pub key_whirlpool: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_fee_tier: String,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_system_program: String,
  pub key_rent: String,
  #[cfg(feature = "decimals")]
  pub decimals_token_mint_a: u8,
  #[cfg(feature = "decimals")]
  pub decimals_token_mint_b: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeRewardV2 {
  pub data_reward_index: u8,
  pub key_reward_authority: String,
  pub key_funder: String,
  pub key_whirlpool: String,
  pub key_reward_mint: String,
  pub key_reward_token_badge: String,
  pub key_reward_vault: String,
  pub key_reward_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  #[cfg(feature = "decimals")]
  pub decimals_reward_mint: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissionsV2 {
  pub data_reward_index: u8,
  #[serde(with = "u128_as_string")]
  pub data_emissions_per_second_x64: u128,
  pub key_whirlpool: String,
  pub key_reward_authority: String,
  pub key_reward_vault: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeConfigExtension {
  pub key_whirlpools_config: String,
  pub key_whirlpools_config_extension: String,
  pub key_funder: String,
  pub key_fee_authority: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeTokenBadge {
  pub key_whirlpools_config: String,
  pub key_whirlpools_config_extension: String,
  pub key_token_badge_authority: String,
  pub key_token_mint: String,
  pub key_token_badge: String,
  pub key_funder: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDeleteTokenBadge {
  pub key_whirlpools_config: String,
  pub key_whirlpools_config_extension: String,
  pub key_token_badge_authority: String,
  pub key_token_mint: String,
  pub key_token_badge: String,
  pub key_receiver: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetConfigExtensionAuthority {
  pub key_whirlpools_config: String,
  pub key_whirlpools_config_extension: String,
  pub key_config_extension_authority: String,
  pub key_new_config_extension_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetTokenBadgeAuthority {
  pub key_whirlpools_config: String,
  pub key_whirlpools_config_extension: String,
  pub key_config_extension_authority: String,
  pub key_new_token_badge_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPositionWithTokenExtensions {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  #[serde(with = "bool_as_u8")]
  pub data_with_token_metadata_extension: bool,
  pub key_funder: String,
  pub key_owner: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  pub key_whirlpool: String,
  // note: we can read and write "keyToken2022Program" field as expected
  pub key_token_2022_program: String,
  pub key_system_program: String,
  pub key_associated_token_program: String,
  pub key_metadata_update_auth: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedClosePositionWithTokenExtensions {
  pub key_position_authority: String,
  pub key_receiver: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  // note: we can read and write "keyToken2022Program" field as expected
  pub key_token_2022_program: String,
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
