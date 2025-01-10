use crate::schema::token::WhirlpoolToken;
use crate::tests::utils::jsonify;

const TEST_DATA: &str = include_str!("test.json");

#[test]
fn test_deserialize() {
    let deserialized: WhirlpoolToken = serde_json::from_str(TEST_DATA).unwrap();

    assert_eq!(deserialized.slot, 298625934);
    assert_eq!(deserialized.block_height, 277241214);
    assert_eq!(deserialized.block_time, 1730332799);

    let expected_tokens_0_mint = "124gpW1VL9PfAFzdGDqtbLspBKXDp2G5MaXNdJMwpump";
    let expected_tokens_0_decimals = 6;
    let expected_tokens_4_mint = "zjah1kuEDSU2LZMsgj3akPSzjVFaPVubZfwzM9tH5cp";
    let expected_tokens_4_decimals = 12;
    assert_eq!(deserialized.tokens.len(), 5);
    assert_eq!(deserialized.tokens[0].mint, expected_tokens_0_mint);
    assert_eq!(deserialized.tokens[0].decimals, expected_tokens_0_decimals);
    assert_eq!(deserialized.tokens[4].mint, expected_tokens_4_mint);
    assert_eq!(deserialized.tokens[4].decimals, expected_tokens_4_decimals);
}

#[test]
fn test_serialize() {
    let deserialized: WhirlpoolToken = serde_json::from_str(TEST_DATA).unwrap();
    let serialized = jsonify(&deserialized);
    assert_eq!(serialized, TEST_DATA.trim_end().to_string());
}
