use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct LamportsHistogram {
    pub counts: [u64; 12],
    pub total: u64,
}

impl LamportsHistogram {
    inherent_borsh_serde!();
}
