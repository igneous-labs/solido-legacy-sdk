use mollusk_svm::{
    program::{create_keyed_account_for_builtin_program, keyed_account_for_system_program},
    result::InstructionResult,
};
use solana_account::Account;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solido_legacy_core::{
    ValidatorList, WithdrawV2IxData, WithdrawV2IxKeysOwned, PROGRAM_ID, STAKE_PROGRAM,
    WITHDRAW_V2_IX_IS_SIGNER, WITHDRAW_V2_IX_IS_WRITER,
};

use crate::common::{
    lido_mainnet_withdraw_accounts, metas_from_keys_signer_writer, mollusk_lido_prog,
    payer_account, stsol_token_acc, system_transfer_ix, KeyedUiAccount,
    STAKE_ACC_RENT_EXEMPT_LAMPORTS,
};

pub fn prefund_split_stake_to_ix(payer: Pubkey, split_stake_to: Pubkey) -> Instruction {
    system_transfer_ix(payer, split_stake_to, STAKE_ACC_RENT_EXEMPT_LAMPORTS)
}

pub fn withdraw_v2_ix(
    val_list: &ValidatorList,
    user: Pubkey,
    burn_stsol_from: Pubkey,
    split_stake_to: Pubkey,
    amount: u64,
) -> Instruction {
    let (i, val) = val_list.withdrawing_validator().unwrap();
    let (s0, s1, s2, s3) = val.withdraw_stake_acc_seeds();
    let (withdraw_stake_from, _bump) = Pubkey::find_program_address(
        &[s0.as_slice(), s1.as_slice(), s2.as_slice(), s3.as_slice()],
        &Pubkey::new_from_array(PROGRAM_ID),
    );
    let keys = WithdrawV2IxKeysOwned::default()
        .with_lido_consts()
        .with_sol_consts()
        .with_vote(*val.vote_account_address())
        .with_withdraw_stake_from(withdraw_stake_from.to_bytes())
        .with_user(user.to_bytes())
        .with_burn_stsol_from(burn_stsol_from.to_bytes())
        .with_split_stake_to(split_stake_to.to_bytes());

    let metas = metas_from_keys_signer_writer(
        keys.0,
        WITHDRAW_V2_IX_IS_SIGNER.0,
        WITHDRAW_V2_IX_IS_WRITER.0,
    );
    let data = WithdrawV2IxData::new(amount, i.try_into().unwrap());

    Instruction {
        program_id: Pubkey::new_from_array(PROGRAM_ID),
        accounts: metas,
        data: data.to_buf().into(),
    }
}

#[test]
fn withdraw_v2_fixture() {
    const AMOUNT: u64 = 1_000_000_000;

    let account = KeyedUiAccount::from_test_fixtures_file("validator-list");
    let data = account.account_data();
    let val_list = ValidatorList::deserialize(data.as_slice()).unwrap();

    let [user, burn_stsol_from, split_stake_to] = core::array::from_fn(|_i| Pubkey::new_unique());
    let mollusk = mollusk_lido_prog();

    let ixs = &[
        prefund_split_stake_to_ix(user, split_stake_to),
        withdraw_v2_ix(&val_list, user, burn_stsol_from, split_stake_to, AMOUNT),
    ];
    let accounts: Vec<_> = lido_mainnet_withdraw_accounts()
        .chain([
            keyed_account_for_system_program(),
            create_keyed_account_for_builtin_program(
                &Pubkey::new_from_array(STAKE_PROGRAM),
                "solana_stake_program",
            ),
            mollusk_svm_programs_token::token::keyed_account(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (user, payer_account(1_000_000_000)),
            (burn_stsol_from, stsol_token_acc(AMOUNT, user)),
            (split_stake_to, Account::default()),
        ])
        .collect();

    let InstructionResult { raw_result, .. } = mollusk.process_instruction_chain(ixs, &accounts);

    raw_result.unwrap();
}
