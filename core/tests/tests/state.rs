use solido_legacy_core::{AccountType, Lido, ValidatorList, VALIDATOR_LIST_ADDR};

use crate::common::KeyedUiAccount;

#[test]
fn lido_serde_roundtrip() {
    let account = KeyedUiAccount::from_test_fixtures_file("lido");
    let data = account.account_data();
    let deser = Lido::borsh_de(data.as_slice()).unwrap();

    eprintln!("{deser:#?}");
    // sample some fields
    assert_eq!(deser.account_type, AccountType::Lido);
    assert_eq!(deser.validator_list, VALIDATOR_LIST_ADDR);

    let mut ser = Vec::new();
    deser.borsh_ser(&mut ser).unwrap();
    assert_eq!(ser, data[..ser.len()]);
}

#[test]
fn validator_list_de() {
    let account = KeyedUiAccount::from_test_fixtures_file("validator-list");
    let data = account.account_data();
    let deser = ValidatorList::deserialize(data.as_slice()).unwrap();

    // sample some fields
    assert_eq!(deser.header.account_type, AccountType::Validator);
    assert_eq!(deser.entries.len(), 14);

    let largest_val = deser
        .entries
        .iter()
        .max_by_key(|val| val.effective_stake_balance())
        .unwrap();
    // `DdCNGDpP7qMgoAy6paFzhhak2EeyCZcgjH7ak5u5v28m`
    // check that this is indeed a vote account
    eprintln!(
        "{}",
        bs58::encode(largest_val.vote_account_address()).into_string()
    );
}
