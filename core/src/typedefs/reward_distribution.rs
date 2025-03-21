use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct RewardDistribution {
    pub treasury_fee: u32,
    pub developer_fee: u32,
    pub st_sol_appreciation: u32,
}

impl RewardDistribution {
    inherent_borsh_serde!();
}
