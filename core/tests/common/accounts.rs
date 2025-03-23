use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};
use solana_account::Account;
use solana_account_decoder_client_types::UiAccount;
use solana_pubkey::Pubkey;
use solido_legacy_core::{STAKE_AUTH_PDA, STSOL_MINT_ADDR, SYSTEM_PROGRAM, TOKENKEG_PROGRAM};

use super::test_fixtures_dir;

/// This is the json format of
/// `solana account -o <FILENAME>.json --output json <ACCOUNT-PUBKEY>`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyedUiAccount {
    pub pubkey: String,
    pub account: UiAccount,
}

impl KeyedUiAccount {
    fn from_file<P: AsRef<Path>>(json_file_path: P) -> Self {
        let mut file = File::open(json_file_path).unwrap();
        serde_json::from_reader(&mut file).unwrap()
    }

    /// Loads a KeyedUiAccount from `<test_fixtures_dir()>/relative_json_file_path.json`
    pub fn from_test_fixtures_file<P: AsRef<Path>>(relative_json_file_path: P) -> Self {
        Self::from_file(
            test_fixtures_dir()
                .join(relative_json_file_path)
                .with_extension("json"),
        )
    }

    /// Assumes data is not `UiAccountData::Json`
    pub fn account_data(&self) -> Vec<u8> {
        self.account.data.decode().unwrap()
    }
}

pub fn lido_mainnet_withdraw_accounts() -> impl Iterator<Item = (Pubkey, Account)> {
    [
        "largest-vsa",
        "lido",
        "stsol",
        "validator-list",
        "largest-vote",
    ]
    .into_iter()
    .map(|fname| {
        let KeyedUiAccount { pubkey, account } = KeyedUiAccount::from_test_fixtures_file(fname);
        (pubkey.parse().unwrap(), account.decode().unwrap())
    })
    .chain([(Pubkey::new_from_array(STAKE_AUTH_PDA), Account::default())])
}

pub fn payer_account(lamports: u64) -> Account {
    Account::new(lamports, 0, &Pubkey::new_from_array(SYSTEM_PROGRAM))
}

pub fn stsol_token_acc(amt: u64, owner: Pubkey) -> Account {
    let mut data = vec![0; 165];
    data[0..32].copy_from_slice(&STSOL_MINT_ADDR);
    data[32..64].copy_from_slice(owner.as_array());
    data[64..72].copy_from_slice(&amt.to_le_bytes());
    data[108] = 1; // AccountState
    Account {
        lamports: 2_039_280,
        data,
        owner: Pubkey::new_from_array(TOKENKEG_PROGRAM),
        executable: false,
        rent_epoch: u64::MAX,
    }
}
