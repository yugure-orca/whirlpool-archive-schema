pub mod jsonl;
pub mod json;

pub type WhirlpoolTransactionJsonlUnit = jsonl::transaction::WhirlpoolTransactionBlock;
pub type WhirlpoolEventJsonlUnit = jsonl::event::WhirlpoolEventBlock;
pub type WhirlpoolOhlcvDailyJsonlUnit = jsonl::ohlcv::WhirlpoolOhlcvDailyData;
pub type WhirlpoolOhlcvMinutelyJsonlUnit = jsonl::ohlcv::WhirlpoolOhlcvMinutelyData;
