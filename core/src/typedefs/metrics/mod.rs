use borsh::{BorshDeserialize, BorshSerialize};

mod lamports_histogram;
mod withdraw_metric;

pub use lamports_histogram::*;
pub use withdraw_metric::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct Metrics {
    pub fee_treasury_sol_total: u64,
    pub fee_validation_sol_total: u64,
    pub fee_developer_sol_total: u64,
    pub st_sol_appreciation_sol_total: u64,
    pub fee_treasury_st_sol_total: u64,
    pub fee_validation_st_sol_total: u64,
    pub fee_developer_st_sol_total: u64,
    pub deposit_amount: LamportsHistogram,
    pub withdraw_amount: WithdrawMetric,
}

impl Metrics {
    inherent_borsh_serde!();
}
