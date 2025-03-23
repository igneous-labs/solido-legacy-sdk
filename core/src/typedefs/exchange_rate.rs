use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct ExchangeRate {
    pub computed_in_epoch: u64,
    pub st_sol_supply: u64,
    pub sol_balance: u64,
}

impl ExchangeRate {
    /// Returns the staked lamports that will be split
    /// from the active stake account upon burning `stsol_amount` stSOL
    ///
    /// Returns None on arithmetic failure
    #[inline]
    pub fn quote_withdraw_stake(&self, stsol_amount: u64) -> Option<u64> {
        if self.st_sol_supply == 0 {
            return None;
        }
        // unchecked mul: 2 u64s will not overflow u128
        // unchecked div: nonzero denom checked above
        let res = (u128::from(stsol_amount) * u128::from(self.sol_balance))
            / u128::from(self.st_sol_supply);
        res.try_into().ok()
    }
}

impl ExchangeRate {
    inherent_borsh_serde!();
}
