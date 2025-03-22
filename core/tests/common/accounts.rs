use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};
use solana_account_decoder_client_types::UiAccount;

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
