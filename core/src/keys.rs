use const_crypto::{bs58, ed25519::derive_program_address};

pub const PROGRAM_ID_STR: &str = "CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi";
pub const PROGRAM_ID: [u8; 32] = bs58::decode_pubkey(PROGRAM_ID_STR);

pub const LIDO_STATE_ADDR_STR: &str = "49Yi1TKkNyYjPAFdR9LBvoHcUjuPX4Df5T5yv39w2XTn";
pub const LIDO_STATE_ADDR: [u8; 32] = bs58::decode_pubkey(LIDO_STATE_ADDR_STR);

pub const VALIDATOR_LIST_ADDR_STR: &str = "GL9kqRNUTUosW3RsDoXHCuXUZn73SgQQmBvtp1ng2co4";
pub const VALIDATOR_LIST_ADDR: [u8; 32] = bs58::decode_pubkey(VALIDATOR_LIST_ADDR_STR);

pub const STSOL_MINT_ADDR_STR: &str = "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj";
pub const STSOL_MINT_ADDR: [u8; 32] = bs58::decode_pubkey(STSOL_MINT_ADDR_STR);

const STAKE_AUTH_PDA_TUP: ([u8; 32], u8) = derive_program_address(
    &[b"stake_authority", LIDO_STATE_ADDR.as_slice()],
    &PROGRAM_ID,
);
pub const STAKE_AUTH_PDA: [u8; 32] = STAKE_AUTH_PDA_TUP.0;
pub const STAKE_AUTH_PDA_BUMP: u8 = STAKE_AUTH_PDA_TUP.1;
pub const STAKE_AUTH_PDA_STR: &str = bs58::encode_pubkey(&STAKE_AUTH_PDA).str();
