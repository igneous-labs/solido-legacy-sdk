use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct WithdrawMetric {
    pub total_st_sol_amount: u64,
    pub total_sol_amount: u64,
    pub count: u64,
}

impl WithdrawMetric {
    inherent_borsh_serde!();
}
