use crate::LIDO_STATE_ADDR;

pub const VALIDATOR_STAKE_ACCOUNT_SEED: [u8; 23] = *b"validator_stake_account";

pub type ValidatorStakeSeeds<'a> = (&'a [u8; 32], &'a [u8; 32], &'a [u8; 23], [u8; 8]);

#[inline]
pub const fn validator_stake_seeds(vote_acc_addr: &[u8; 32], seed: u64) -> ValidatorStakeSeeds<'_> {
    (
        &LIDO_STATE_ADDR,
        vote_acc_addr,
        &VALIDATOR_STAKE_ACCOUNT_SEED,
        seed.to_le_bytes(),
    )
}
