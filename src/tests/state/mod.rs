use crate::schema::state::WhirlpoolState;
use crate::tests::utils::{from_base64, jsonify};

const TEST_DATA: &str = include_str!("test.json");

#[test]
fn test_deserialize() {
    let deserialized: WhirlpoolState = serde_json::from_str(TEST_DATA).unwrap();

    assert_eq!(deserialized.slot, 129171690);
    assert_eq!(deserialized.block_height, 117105530);
    assert_eq!(deserialized.block_time, 1649635197);

    let expected_accounts_0_pubkey = "95Lj8Zywc2Y3pemspLyqsvU3Z9JrHd2xpNsk7zACQPV2";
    let expected_accounts_0_data = from_base64("qryP5HpA99DyL5MQp9RFVK38fpgddAeEPkj5Nf7TAl/qe/yaQ/yJNcCfGyvWDeNmBf3jp3z0bjWzIOFoWqCSr6l59RVZmvkzp+AcjAAAAAAAAAAAAAAAAMCm///AsP//RJk5K+X6IQAAAAAAAAAAAAAAAAAAAAAAjbUt6f9vBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    let expected_accounts_4_pubkey = "7JdDKvF7KP6TFEVMuUH2wDosBT4iL6s4tKz52xFDyUkS";
    let expected_accounts_4_data = from_base64("qryP5HpA99DyL5MQp9RFVK38fpgddAeEPkj5Nf7TAl/qe/yaQ/yJNcWWJRE3K4W3XBfKkN2BpjdE9BmM1LrKx5IVFG3SMRuEg2GDV24AAAAAAAAAAAAAAMCo//9Arf//5C0tLpJ22P7//////////wAAAAAAAAAA9QDoGPYCBwAAAAAAAAAAAAAAAAAAAAAAcxVEClOuBQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA");
    assert_eq!(deserialized.accounts.len(), 5);
    assert_eq!(deserialized.accounts[0].pubkey, expected_accounts_0_pubkey);
    assert_eq!(deserialized.accounts[0].data, expected_accounts_0_data);
    assert_eq!(deserialized.accounts[4].pubkey, expected_accounts_4_pubkey);
    assert_eq!(deserialized.accounts[4].data, expected_accounts_4_data);

    let expected_program_data = from_base64("f0VMRgIBAQAAAAAAAAAAAAMA9wABAAAAwEAE");
    assert_eq!(deserialized.program_data, expected_program_data);
}

#[test]
fn test_serialize() {
    let deserialized: WhirlpoolState = serde_json::from_str(TEST_DATA).unwrap();
    let serialized = jsonify(&deserialized);
    assert_eq!(serialized, TEST_DATA.trim_end().to_string());
}
