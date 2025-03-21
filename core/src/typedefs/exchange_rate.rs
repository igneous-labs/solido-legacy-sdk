use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct ExchangeRate {
    pub computed_in_epoch: u64,
    pub st_sol_supply: u64,
    pub sol_balance: u64,
}

impl ExchangeRate {
    inherent_borsh_serde!();
}
