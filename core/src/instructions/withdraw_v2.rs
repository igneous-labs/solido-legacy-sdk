use generic_array_struct::generic_array_struct;

use crate::{
    internal_utils::const_assign_byte_arr, LIDO_STATE_ADDR, STAKE_AUTH_PDA, STAKE_PROGRAM,
    STSOL_MINT_ADDR, SYSTEM_PROGRAM, SYSVAR_CLOCK, TOKENKEG_PROGRAM, VALIDATOR_LIST_ADDR,
};

#[generic_array_struct(pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WithdrawV2IxAccs<T> {
    pub lido_state: T,
    pub user: T,
    pub burn_stsol_from: T,
    pub stsol_mint: T,
    pub vote: T,

    /// PDA obtained using
    /// [`crate::Validator::withdraw_stake_acc_seeds()`]
    /// of `Validator` entry on the `ValidatorList` account
    /// for `vote = self.vote()`
    pub withdraw_stake_from: T,

    /// Must be a system account prefunded with rent-exempt lamports
    /// for a stake account. The program will call allocate() and assign()
    pub split_stake_to: T,

    pub stake_auth: T,
    pub validator_list: T,
    pub token_prog: T,
    pub sysvar_clock: T,
    pub system_prog: T,
    pub stake_prog: T,
}

pub type WithdrawV2IxKeysOwned = WithdrawV2IxAccs<[u8; 32]>;
pub type WithdrawV2IxKeys<'a> = WithdrawV2IxAccs<&'a [u8; 32]>;
pub type WithdrawV2IxAccsFlag = WithdrawV2IxAccs<bool>;

pub const WITHDRAW_V2_IX_IS_WRITER: WithdrawV2IxAccsFlag =
    WithdrawV2IxAccsFlag::new([false; WITHDRAW_V2_IX_ACCS_LEN])
        .const_with_lido_state(true)
        .const_with_burn_stsol_from(true)
        .const_with_stsol_mint(true)
        .const_with_withdraw_stake_from(true)
        .const_with_split_stake_to(true)
        .const_with_validator_list(true);

pub const WITHDRAW_V2_IX_IS_SIGNER: WithdrawV2IxAccsFlag =
    WithdrawV2IxAccsFlag::new([false; WITHDRAW_V2_IX_ACCS_LEN])
        .const_with_user(true)
        .const_with_split_stake_to(true);

impl<T> WithdrawV2IxAccs<T> {
    /// This seems redundant because `.0` is `pub`, but this is necessary for
    /// nice init syntax with type aliases.
    ///
    /// With this, you can now do
    ///
    /// ```
    /// use solido_legacy_core::WithdrawV2IxAccsFlag;
    /// let var: WithdrawV2IxAccsFlag = WithdrawV2IxAccsFlag::new(Default::default());
    /// ```
    ///
    /// instead of
    ///
    /// ```
    /// use solido_legacy_core::{WithdrawV2IxAccsFlag, WithdrawV2IxAccs};
    /// let var: WithdrawV2IxAccsFlag = WithdrawV2IxAccs(Default::default());
    /// ```
    #[inline]
    pub const fn new(arr: [T; WITHDRAW_V2_IX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl WithdrawV2IxKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> WithdrawV2IxKeys<'_> {
        WithdrawV2IxKeys::new(self.0.each_ref())
    }
}

impl WithdrawV2IxKeysOwned {
    #[inline]
    pub fn with_lido_consts(self) -> Self {
        self.as_borrowed().with_lido_consts().into_owned()
    }

    #[inline]
    pub fn with_sol_consts(self) -> Self {
        self.as_borrowed().with_sol_consts().into_owned()
    }
}

impl WithdrawV2IxKeys<'_> {
    #[inline]
    pub fn into_owned(self) -> WithdrawV2IxKeysOwned {
        WithdrawV2IxKeysOwned::new(self.0.map(|pk| *pk))
    }
}

impl WithdrawV2IxKeys<'_> {
    #[inline]
    pub const fn with_lido_consts(self) -> Self {
        self.const_with_lido_state(&LIDO_STATE_ADDR)
            .const_with_stsol_mint(&STSOL_MINT_ADDR)
            .const_with_stake_auth(&STAKE_AUTH_PDA)
            .const_with_validator_list(&VALIDATOR_LIST_ADDR)
    }

    #[inline]
    pub const fn with_sol_consts(self) -> Self {
        self.const_with_system_prog(&SYSTEM_PROGRAM)
            .const_with_stake_prog(&STAKE_PROGRAM)
            .const_with_sysvar_clock(&SYSVAR_CLOCK)
            .const_with_token_prog(&TOKENKEG_PROGRAM)
    }
}

pub const WITHDRAW_V2_IX_DISCM: u8 = 23;

pub const WITHDRAW_V2_IX_DATA_LEN: usize = 13;

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WithdrawV2IxData([u8; WITHDRAW_V2_IX_DATA_LEN]);

impl WithdrawV2IxData {
    #[inline]
    pub const fn new(amount: u64, validator_index: u32) -> Self {
        let res = [0u8; WITHDRAW_V2_IX_DATA_LEN];
        let res =
            const_assign_byte_arr::<WITHDRAW_V2_IX_DATA_LEN, 0, 1>(res, [WITHDRAW_V2_IX_DISCM]);
        let res = const_assign_byte_arr::<WITHDRAW_V2_IX_DATA_LEN, 1, 8>(res, amount.to_le_bytes());
        let res = const_assign_byte_arr::<WITHDRAW_V2_IX_DATA_LEN, 9, 4>(
            res,
            validator_index.to_le_bytes(),
        );
        Self(res)
    }

    #[inline]
    pub const fn to_buf(self) -> [u8; WITHDRAW_V2_IX_DATA_LEN] {
        self.0
    }
}
