use crate::schema::transaction::{WhirlpoolTransactionSet, DecodedInstruction, DecodedWhirlpoolInstruction};
use crate::tests::utils::{from_base64, jsonify};

const TEST_DATA: &str = include_str!("test.json");

#[test]
fn test_deserialize() {
    let deserialized: WhirlpoolTransactionSet = serde_json::from_str(TEST_DATA).unwrap();

    assert_eq!(deserialized.slot, 268396603);
    assert_eq!(deserialized.block_height, 248114148);
    assert_eq!(deserialized.block_time, 1716880096);

    assert_eq!(deserialized.transactions.len(), 5);

    assert_eq!(deserialized.transactions[0].index, 14);
    assert_eq!(deserialized.transactions[0].signature, "P9LKsZpwCjx5AXeptM8H8ydkdQVqwCP263arQ6YubAHi1GEJDKxpvCaKcuHKBLLsEp1q8xfF2LYepxiVCGJRzqr");
    assert_eq!(deserialized.transactions[0].payer, "GuiU6MpLahPHSHYcsfSRjwLUm1AtZ9zP2eiLAkJMBjg");
    assert_eq!(deserialized.transactions[0].balances.len(), 2);
    assert_eq!(deserialized.transactions[0].balances[1].account, "GnKhBGLtubsgAPYgLUX2hDiBYij9hH5Gboss3JF7w5RW");
    assert_eq!(deserialized.transactions[0].balances[1].pre, 27768587731);
    assert_eq!(deserialized.transactions[0].balances[1].post, 28741366678);
    assert_eq!(deserialized.transactions[0].instructions.len(), 1);
    match &deserialized.transactions[0].instructions[0] {
        DecodedInstruction::WhirlpoolInstruction(instruction) => {
            match instruction {
                DecodedWhirlpoolInstruction::Swap(data) => {
                    assert_eq!(data.data_amount, 972778947);
                    assert_eq!(data.key_whirlpool, "8SwU6NXuWbpuUR9FgpuAHFNjUpF28QyF7im3ZyKq7Ni4");
                }
                _ => panic!("Expected a Swap variant"),
            }
        }
        _ => panic!("Expected a WhirlpoolInstruction variant"),
    }

    assert_eq!(deserialized.transactions[4].index, 642);
    assert_eq!(deserialized.transactions[4].signature, "5Xpk6Uo412FK35oeQUQ3Suc6ZMmR9Wo48LvNfk3B3FQWMEuZfbGY5NfCL9cVPnY7pqGsWkrPzSBU2XszTpHBVZyS");
    assert_eq!(deserialized.transactions[4].payer, "23zF9Azpe9CN4iPeTsQndD1mQpcb5Gz1qFREL5gPTZvG");
    assert_eq!(deserialized.transactions[4].balances.len(), 0);
    assert_eq!(deserialized.transactions[4].instructions.len(), 1);
    match &deserialized.transactions[4].instructions[0] {
        DecodedInstruction::ProgramDeployInstruction(data) => {
            assert_eq!(data.program_data, from_base64("f0VMRgIBAQAAAAAAAAAAAAMA9wABAAAAuEYLAAAA"));
        }
        _ => panic!("Expected a ProgramDeployInstruction variant"),
    }
}

#[test]
fn test_serialize() {
    let deserialized: WhirlpoolTransactionSet = serde_json::from_str(TEST_DATA).unwrap();
    let serialized = jsonify(&deserialized);
    assert_eq!(serialized, TEST_DATA.trim_end().to_string());
}
