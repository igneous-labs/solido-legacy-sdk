use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub enum AccountType {
    Uninitialized,
    Lido,
    Validator,
    Maintainer,
}

impl AccountType {
    inherent_borsh_serde!();
}

impl Default for AccountType {
    #[inline]
    fn default() -> Self {
        Self::Uninitialized
    }
}
