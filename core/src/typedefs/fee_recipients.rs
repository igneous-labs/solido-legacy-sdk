use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct FeeRecipients {
    pub treasury_account: [u8; 32],
    pub developer_account: [u8; 32],
}

impl FeeRecipients {
    inherent_borsh_serde!();
}
