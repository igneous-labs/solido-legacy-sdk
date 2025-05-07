use generic_array_struct::generic_array_struct;

use crate::{
    LIDO_STATE_ADDR, RESERVE_PDA, STSOL_MINT_ADDR, SYSTEM_PROGRAM, SYSVAR_CLOCK, SYSVAR_RENT,
    VALIDATOR_LIST_ADDR,
};

#[generic_array_struct(builder pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateExchangeRateV2IxAccs<T> {
    pub lido_state: T,
    pub reserve: T,
    pub stsol_mint: T,
    pub validator_list: T,
    pub sysvar_clock: T,
    pub sysvar_rent: T,
}

pub type UpdateExchangeRateV2IxKeysOwned = UpdateExchangeRateV2IxAccs<[u8; 32]>;
pub type UpdateExchangeRateV2IxKeys<'a> = UpdateExchangeRateV2IxAccs<&'a [u8; 32]>;
pub type UpdateExchangeRateV2IxAccsFlag = UpdateExchangeRateV2IxAccs<bool>;

pub const UPDATE_EXCHANGE_RATE_V2_IX_IS_WRITER: UpdateExchangeRateV2IxAccsFlag =
    UpdateExchangeRateV2IxAccsFlag::new([false; UPDATE_EXCHANGE_RATE_V2_IX_ACCS_LEN])
        .const_with_lido_state(true);

pub const UPDATE_EXCHANGE_RATE_V2_IX_IS_SIGNER: UpdateExchangeRateV2IxAccsFlag =
    UpdateExchangeRateV2IxAccsFlag::new([false; UPDATE_EXCHANGE_RATE_V2_IX_ACCS_LEN]);

impl<T> UpdateExchangeRateV2IxAccs<T> {
    #[inline]
    pub const fn new(arr: [T; UPDATE_EXCHANGE_RATE_V2_IX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl UpdateExchangeRateV2IxKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> UpdateExchangeRateV2IxKeys<'_> {
        UpdateExchangeRateV2IxKeys::new(self.0.each_ref())
    }
}

impl UpdateExchangeRateV2IxKeys<'_> {
    #[inline]
    pub fn into_owned(self) -> UpdateExchangeRateV2IxKeysOwned {
        UpdateExchangeRateV2IxKeysOwned::new(self.0.map(|pk| *pk))
    }
}

impl UpdateExchangeRateV2IxKeys<'_> {
    /// The account keys input to this ix should never change
    pub const CONST: Self = Self::new([&SYSTEM_PROGRAM; UPDATE_EXCHANGE_RATE_V2_IX_ACCS_LEN])
        .with_lido_consts()
        .with_sol_consts();

    #[inline]
    pub const fn with_lido_consts(self) -> Self {
        self.const_with_lido_state(&LIDO_STATE_ADDR)
            .const_with_stsol_mint(&STSOL_MINT_ADDR)
            .const_with_validator_list(&VALIDATOR_LIST_ADDR)
            .const_with_reserve(&RESERVE_PDA)
    }

    #[inline]
    pub const fn with_sol_consts(self) -> Self {
        self.const_with_sysvar_clock(&SYSVAR_CLOCK)
            .const_with_sysvar_rent(&SYSVAR_RENT)
    }
}

pub const UPDATE_EXCHANGE_RATE_V2_IX_DISCM: u8 = 22;

pub const UPDATE_EXCHANGE_RATE_V2_IX_DATA_LEN: usize = 1;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpdateExchangeRateV2IxData([u8; UPDATE_EXCHANGE_RATE_V2_IX_DATA_LEN]);

impl UpdateExchangeRateV2IxData {
    pub const CONST: Self = Self([UPDATE_EXCHANGE_RATE_V2_IX_DISCM]);

    #[inline]
    pub const fn new() -> Self {
        Self::CONST
    }

    #[inline]
    pub const fn to_buf(self) -> [u8; UPDATE_EXCHANGE_RATE_V2_IX_DATA_LEN] {
        self.0
    }
}
