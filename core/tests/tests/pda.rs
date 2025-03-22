use solana_pubkey::Pubkey;
use solido_legacy_core::ValidatorList;

use crate::common::{find_withdraw_stake_acc, KeyedUiAccount};

const LARGEST_VAL_VOTE: [u8; 32] =
    const_crypto::bs58::decode_pubkey("DdCNGDpP7qMgoAy6paFzhhak2EeyCZcgjH7ak5u5v28m");

#[test]
fn withdraw_stake_acc_sanity() {
    let account = KeyedUiAccount::from_test_fixtures_file("validator-list");
    let data = account.account_data();
    let deser = ValidatorList::deserialize(data.as_slice()).unwrap();

    let val = deser
        .entries
        .iter()
        .find(|v| *v.vote_account_address() == LARGEST_VAL_VOTE)
        .unwrap();

    let (pda, _bump) = find_withdraw_stake_acc(val);

    assert_eq!(
        pda,
        Pubkey::from_str_const("58EuDup5Gg9nEqDoBBGEDnZ5fMc1u7fQi6kHmEESSUkh")
    );
}
