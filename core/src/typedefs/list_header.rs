use borsh::{BorshDeserialize, BorshSerialize};

use super::AccountType;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct ListHeader {
    pub account_type: AccountType,
    pub lido_version: u8,
    pub max_entries: u32,
}

impl ListHeader {
    inherent_borsh_serde!();
}
