use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;
use solido_legacy_core::SYSTEM_PROGRAM;

pub fn metas_from_keys_signer_writer<const N: usize>(
    keys: [[u8; 32]; N],
    is_signer: [bool; N],
    is_writer: [bool; N],
) -> Vec<AccountMeta> {
    keys.into_iter()
        .zip(is_signer)
        .zip(is_writer)
        .map(|((pubkey, is_signer), is_writable)| AccountMeta {
            pubkey: Pubkey::new_from_array(pubkey),
            is_signer,
            is_writable,
        })
        .collect()
}

pub fn system_transfer_ix(from: Pubkey, to: Pubkey, lamports: u64) -> Instruction {
    let mut data = vec![0; 12];
    data[0] = 2;
    data[4..].copy_from_slice(&lamports.to_le_bytes());
    Instruction {
        program_id: Pubkey::new_from_array(SYSTEM_PROGRAM),
        accounts: vec![
            AccountMeta {
                pubkey: from,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: to,
                is_signer: false,
                is_writable: true,
            },
        ],
        data,
    }
}
