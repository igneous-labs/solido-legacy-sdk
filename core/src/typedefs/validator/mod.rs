//! repr(C) & align=1, fit for pointer casting

use borsh::{BorshDeserialize, BorshSerialize};

mod seed_range;

pub use seed_range::*;

use crate::{validator_stake_seeds, ValidatorStakeSeeds};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct Validator {
    vote_account_address: [u8; 32],
    stake_seeds: SeedRange,
    unstake_seeds: SeedRange,
    stake_accounts_balance: [u8; 8],
    unstake_accounts_balance: [u8; 8],
    effective_stake_balance: [u8; 8],
    active: bool,
}

impl Validator {
    /// Returns the seeds to find the PDA of the validator
    /// stake account to service stsol withdrawals from
    #[inline]
    pub const fn withdraw_stake_acc_seeds(&self) -> ValidatorStakeSeeds<'_> {
        validator_stake_seeds(&self.vote_account_address, self.stake_seeds.begin())
    }
}

/// Getters and setters
impl Validator {
    impl_set_with_get_ref!(
        set_vote_account_address,
        with_vote_account_address,
        vote_account_address,
        [u8; 32]
    );

    impl_set_with_get_ref!(set_stake_seeds, with_stake_seeds, stake_seeds, SeedRange);

    impl_set_with_get_ref!(
        set_unstake_seeds,
        with_unstake_seeds,
        unstake_seeds,
        SeedRange
    );

    impl_set_with_get_le!(
        set_stake_accounts_balance,
        with_stake_accounts_balance,
        stake_accounts_balance,
        u64
    );

    impl_set_with_get_le!(
        set_unstake_accounts_balance,
        with_unstake_accounts_balance,
        unstake_accounts_balance,
        u64
    );

    impl_set_with_get_le!(
        set_effective_stake_balance,
        with_effective_stake_balance,
        effective_stake_balance,
        u64
    );

    impl_set_with_get!(set_active, with_active, active, bool);
}

impl Validator {
    inherent_borsh_serde!();
}

const _ASSERT_ALIGN_1: () = assert!(core::mem::align_of::<Validator>() == 1);
