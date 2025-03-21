use borsh::{BorshDeserialize, BorshSerialize};

use crate::{AccountType, ExchangeRate, FeeRecipients, Metrics, RewardDistribution};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct Lido {
    pub account_type: AccountType,
    pub lido_version: u8,
    pub manager: [u8; 32],
    pub st_sol_mint: [u8; 32],
    pub exchange_rate: ExchangeRate,
    pub sol_reserve_account_bump_seed: u8,
    pub stake_authority_bump_seed: u8,
    pub mint_authority_bump_seed: u8,
    pub reward_distribution: RewardDistribution,
    pub fee_recipients: FeeRecipients,
    pub metrics: Metrics,
    pub validator_list: [u8; 32],
    pub maintainer_list: [u8; 32],
    pub max_commission_percentage: u8,
}

impl Lido {
    inherent_borsh_serde!();
}
