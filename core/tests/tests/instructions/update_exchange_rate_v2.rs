use mollusk_svm::result::InstructionResult;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solido_legacy_core::{
    UpdateExchangeRateV2IxData, UpdateExchangeRateV2IxKeys, PROGRAM_ID,
    UPDATE_EXCHANGE_RATE_V2_IX_IS_SIGNER, UPDATE_EXCHANGE_RATE_V2_IX_IS_WRITER,
};

use crate::common::{
    lido_mainnet_update_exchange_rate_accounts, metas_from_keys_signer_writer, mollusk_lido_prog,
    warp_to_epoch, MAINNET_EXCHANGE_RATE_COMPUTED_IN_EPOCH,
};

pub fn update_exchange_rate_v2_ix() -> Instruction {
    let metas = metas_from_keys_signer_writer(
        UpdateExchangeRateV2IxKeys::CONST.into_owned().0,
        UPDATE_EXCHANGE_RATE_V2_IX_IS_SIGNER.0,
        UPDATE_EXCHANGE_RATE_V2_IX_IS_WRITER.0,
    );
    let data = UpdateExchangeRateV2IxData::CONST;
    Instruction {
        program_id: Pubkey::new_from_array(PROGRAM_ID),
        accounts: metas,
        data: data.to_buf().into(),
    }
}

#[test]
fn update_exchange_rate_v2_fixture() {
    let mut mollusk = mollusk_lido_prog();
    warp_to_epoch(&mut mollusk, MAINNET_EXCHANGE_RATE_COMPUTED_IN_EPOCH + 1);

    let ixs = &[update_exchange_rate_v2_ix()];
    let accounts: Vec<_> = lido_mainnet_update_exchange_rate_accounts()
        .chain([
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
        ])
        .collect();

    let InstructionResult { raw_result, .. } = mollusk.process_instruction_chain(ixs, &accounts);

    raw_result.unwrap();
}
