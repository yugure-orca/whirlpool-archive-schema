use crate::schema::event::{WhirlpoolEvent, BlockWhirlpoolEvent};
use crate::tests::utils::jsonify;

const TEST_DATA: &str = include_str!("test.json");

#[test]
fn test_deserialize() {
    let deserialized: BlockWhirlpoolEvent = serde_json::from_str(TEST_DATA).unwrap();

    assert_eq!(deserialized.slot, 268396603);
    assert_eq!(deserialized.block_height, 248114148);
    assert_eq!(deserialized.block_time, 1716880096);

    assert_eq!(deserialized.transactions.len(), 5);

    assert_eq!(
        deserialized.transactions[0].signature,
        "P9LKsZpwCjx5AXeptM8H8ydkdQVqwCP263arQ6YubAHi1GEJDKxpvCaKcuHKBLLsEp1q8xfF2LYepxiVCGJRzqr"
    );
    assert_eq!(
        deserialized.transactions[0].payer,
        "GuiU6MpLahPHSHYcsfSRjwLUm1AtZ9zP2eiLAkJMBjg"
    );
    assert_eq!(deserialized.transactions[0].events.len(), 1);
    match &deserialized.transactions[0].events[0] {
        WhirlpoolEvent::Traded(payload) => {
            assert_eq!(payload.transfer_in.amount, 972778947);
            assert_eq!(
                payload.whirlpool,
                "8SwU6NXuWbpuUR9FgpuAHFNjUpF28QyF7im3ZyKq7Ni4"
            );
        }
        _ => panic!("Expected a Traded variant"),
    }

    assert_eq!(
        deserialized.transactions[4].signature,
        "5Xpk6Uo412FK35oeQUQ3Suc6ZMmR9Wo48LvNfk3B3FQWMEuZfbGY5NfCL9cVPnY7pqGsWkrPzSBU2XszTpHBVZyS"
    );
    assert_eq!(
        deserialized.transactions[4].payer,
        "23zF9Azpe9CN4iPeTsQndD1mQpcb5Gz1qFREL5gPTZvG"
    );
    assert_eq!(deserialized.transactions[4].events.len(), 1);
    match &deserialized.transactions[4].events[0] {
        WhirlpoolEvent::ProgramDeployed(_) => {}
        _ => panic!("Expected a ProgramDeployed variant"),
    }
}

#[test]
fn test_serialize() {
    let deserialized: BlockWhirlpoolEvent = serde_json::from_str(TEST_DATA).unwrap();
    let serialized = jsonify(&deserialized);
    assert_eq!(serialized, TEST_DATA.trim_end().to_string());
}
