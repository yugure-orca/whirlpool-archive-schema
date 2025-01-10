use crate::schema::account_delta::{AccountType, WhirlpoolAccountDeltaSet, AccountDataDelta};
use crate::tests::utils::{from_base64, jsonify};

const TEST_DATA: &str = include_str!("test.json");

#[test]
fn test_deserialize() {
    let deserialized: WhirlpoolAccountDeltaSet = serde_json::from_str(TEST_DATA).unwrap();

    assert_eq!(deserialized.slot, 313051640);
    assert_eq!(deserialized.block_height, 291364715);
    assert_eq!(deserialized.block_time, 1736497399);

    assert_eq!(deserialized.account_deltas.len(), 14);

    let deltas_0 = &deserialized.account_deltas[0];
    assert_eq!(deltas_0.pubkey, "BtCDvZXqLLJyzh8bJWYW4hLPdJZi7cQWJ1CWwU9sA2G6");
    assert_eq!(deltas_0.account_type, AccountType::TickArray);
    match &deltas_0.delta {
        AccountDataDelta::Updated { segments } => {
          assert_eq!(segments.len(), 1);
          assert_eq!(segments[0].offset, 4194);
          assert_eq!(segments[0].data, from_base64("zLpqZ4EBAAAAAAAAAAAAADaOhTuD"));
        }
        _ => panic!("Expected an Updated variant"),
    }

    let deltas_2 = &deserialized.account_deltas[2];
    assert_eq!(deltas_2.pubkey, "8bRkqN6w7EREgJ58BqNEXBa2uUBpvHD79WiwXhLAUkb5");
    assert_eq!(deltas_2.account_type, AccountType::Position);
    match &deltas_2.delta {
        AccountDataDelta::Initialized { length, segments } => {
          assert_eq!(*length, 216);
          assert_eq!(segments.len(), 1);
          assert_eq!(segments[0].offset, 0);
          assert_eq!(segments[0].data, from_base64("qryP5HpA99CyNpDX0HWNHV2LiVDOx6m018ea6P+1xroNvWKhmDeTW4ZXj4W+EZIFQxaKpAe5ya1xfs3kw4gaIH6Rrf5S/CncFrUQ4hgAAAAAAAAAAAAAAJS+//+YwP//h5NLfEz1YAcAAAAAAAAAAAAAAAAAAAAAoqW8aCH4nAE="));
        }
        _ => panic!("Expected an Initialized variant"),
    }

    let deltas_13 = &deserialized.account_deltas[13];
    assert_eq!(deltas_13.pubkey, "9vNKzrrHAjqjuTGLjCBo9Ai4edMYgP9dsG4tFZ2hF251");
    assert_eq!(deltas_13.account_type, AccountType::Whirlpool);
    match &deltas_13.delta {
        AccountDataDelta::Updated { segments } => {
          assert_eq!(segments.len(), 3);
          assert_eq!(segments[0].offset, 65);
          assert_eq!(segments[0].data, from_base64("1zjrb4n02QIAAAAAAAAAAKCg/v/QtPcx"));
          assert_eq!(segments[1].offset, 165);
          assert_eq!(segments[1].data, from_base64("B3zHPhiAcAAz"));
          assert_eq!(segments[2].offset, 261);
          assert_eq!(segments[2].data, from_base64("99g="));
        }
        _ => panic!("Expected an Updated variant"),
    }
}

#[test]
fn test_serialize() {
    let deserialized: WhirlpoolAccountDeltaSet = serde_json::from_str(TEST_DATA).unwrap();
    let serialized = jsonify(&deserialized);
    assert_eq!(serialized, TEST_DATA.trim_end().to_string());
}
