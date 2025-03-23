use mollusk_svm::Mollusk;
use solana_pubkey::Pubkey;
use solido_legacy_core::PROGRAM_ID;

use super::{
    test_fixtures_dir, MAINNET_EXCHANGE_RATE_COMPUTED_IN_EPOCH, MIN_1_SOL_DELEGATION_FEATURE_ID,
    SOLIDO_PROG_OWNER,
};

pub fn mollusk_lido_prog() -> Mollusk {
    let mut res = Mollusk::default();
    res.add_program_with_elf_and_loader(
        &Pubkey::new_from_array(PROGRAM_ID),
        &std::fs::read(test_fixtures_dir().join("solido.so")).unwrap(),
        &Pubkey::new_from_array(SOLIDO_PROG_OWNER),
    );
    warp_to_epoch(&mut res, MAINNET_EXCHANGE_RATE_COMPUTED_IN_EPOCH);
    res.feature_set
        .deactivate(&Pubkey::new_from_array(MIN_1_SOL_DELEGATION_FEATURE_ID));
    mollusk_svm_programs_token::token::add_program(&mut res);
    res
}

pub fn warp_to_epoch(mollusk: &mut Mollusk, epoch: u64) {
    mollusk.sysvars.warp_to_slot(432_000 * epoch + 69);
}
