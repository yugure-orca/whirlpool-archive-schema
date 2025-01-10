use crate::schema::ohlcv::{InitialState, WhirlpoolOhlcvDailyData};
use crate::tests::utils::jsonify;
use std::str::FromStr;

const TEST_DATA: &str = include_str!("test.json");

#[test]
fn test_deserialize() {
    let deserialized: WhirlpoolOhlcvDailyData = serde_json::from_str(TEST_DATA).unwrap();

    assert_eq!(
        deserialized.metadata.whirlpool,
        "7xuPLn8Bun4ZGHeD95xYLnPKReKtSe7zfVRzRJWJZVZW"
    );
    assert_eq!(
        deserialized.metadata.whirlpools_config,
        "2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"
    );
    assert_eq!(
        deserialized.metadata.token_a.mint,
        "So11111111111111111111111111111111111111112"
    );
    assert_eq!(deserialized.metadata.token_a.decimals, 9);
    assert_eq!(
        deserialized.metadata.token_b.mint,
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
    );
    assert_eq!(deserialized.metadata.token_b.decimals, 6);
    assert_eq!(deserialized.metadata.tick_spacing, 256);

    match &deserialized.initial_state {
        InitialState::Existing {
            previous_close_sqrt_price,
            previous_close_decimal_price,
        } => {
            assert_eq!(*previous_close_sqrt_price, 7610009059036061257u128);
            assert_eq!(
                *previous_close_decimal_price,
                bigdecimal::BigDecimal::from_str("1.701887712e2").unwrap()
            );
        }
        _ => panic!("Expected an Existing variant"),
    }

    assert_eq!(
        deserialized.estimated_fees.liquidity_provider_fee_a,
        252601194u64
    );
    assert_eq!(
        deserialized.estimated_fees.liquidity_provider_fee_b,
        43764416u64
    );
    assert_eq!(deserialized.estimated_fees.protocol_fee_a, 37744976u64);
    assert_eq!(deserialized.estimated_fees.protocol_fee_b, 6539501u64);

    assert_eq!(deserialized.daily.timestamp, 1716854400);

    assert_eq!(
        deserialized.daily.ohlc.sqrt_price.open,
        7610009059036061257u128
    );
    assert_eq!(
        deserialized.daily.ohlc.sqrt_price.high,
        8236778002618335840u128
    );
    assert_eq!(
        deserialized.daily.ohlc.sqrt_price.low,
        7556602015374160108u128
    );
    assert_eq!(
        deserialized.daily.ohlc.sqrt_price.close,
        7579124336314958608u128
    );
    assert_eq!(
        deserialized.daily.ohlc.decimal_price.open,
        bigdecimal::BigDecimal::from_str("1.701887712e2").unwrap()
    );
    assert_eq!(
        deserialized.daily.ohlc.decimal_price.high,
        bigdecimal::BigDecimal::from_str("1.993771011e2").unwrap()
    );
    assert_eq!(
        deserialized.daily.ohlc.decimal_price.low,
        bigdecimal::BigDecimal::from_str("1.678083838e2").unwrap()
    );
    assert_eq!(
        deserialized.daily.ohlc.decimal_price.close,
        bigdecimal::BigDecimal::from_str("1.688101744e2").unwrap()
    );

    assert_eq!(deserialized.daily.volume.ab.total_in, 14517309573);
    assert_eq!(deserialized.daily.volume.ab.total_out, 2583444372);
    assert_eq!(deserialized.daily.volume.ab.count, 55);

    assert_eq!(deserialized.daily.volume.ba.total_in, 2515196188);
    assert_eq!(deserialized.daily.volume.ba.total_out, 13527531206);
    assert_eq!(deserialized.daily.volume.ba.count, 15);
}

#[test]
fn test_serialize() {
    let deserialized: WhirlpoolOhlcvDailyData = serde_json::from_str(TEST_DATA).unwrap();
    let serialized = jsonify(&deserialized);
    assert_eq!(serialized, TEST_DATA.trim_end().to_string());
}
