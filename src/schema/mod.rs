pub mod jsonl;
pub mod json;

// JSON
pub type WhirlpoolStateJson = json::state::WhirlpoolState;
pub type WhirlpoolTokenJson = json::token::WhirlpoolToken;

// JSONL
pub type WhirlpoolTransactionJsonlUnit = jsonl::transaction::WhirlpoolTransactionBlock;
pub type WhirlpoolEventJsonlUnit = jsonl::event::WhirlpoolEventBlock;
pub type WhirlpoolAccountDataDeltaJsonlUnit = jsonl::account_data_delta::WhirlpoolAccountDataDelta;
pub type WhirlpoolOhlcvDailyJsonlUnit = jsonl::ohlcv::WhirlpoolOhlcvDailyData;
pub type WhirlpoolOhlcvMinutelyJsonlUnit = jsonl::ohlcv::WhirlpoolOhlcvMinutelyData;
