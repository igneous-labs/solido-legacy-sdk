use solana_pubkey::Pubkey;
use solido_legacy_core::{Validator, PROGRAM_ID};

pub fn find_withdraw_stake_acc(validator: &Validator) -> (Pubkey, u8) {
    let (s0, s1, s2, s3) = validator.withdraw_stake_acc_seeds();
    Pubkey::find_program_address(
        &[s0.as_slice(), s1.as_slice(), s2.as_slice(), s3.as_slice()],
        &Pubkey::new_from_array(PROGRAM_ID),
    )
}
