use borsh::{BorshDeserialize, BorshSerialize};
use sanctum_u64_ratio::{Floor, Ratio};

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
    pub const fn quote_withdraw_stake(&self, stsol_amount: u64) -> Option<u64> {
        let ratio = self.sol_balance_over_st_sol_supply();
        if ratio.0.is_zero() {
            return None;
        }
        ratio.apply(stsol_amount)
    }

    #[inline]
    pub const fn sol_balance_over_st_sol_supply(&self) -> Floor<Ratio<u64, u64>> {
        Floor(Ratio {
            n: self.sol_balance,
            d: self.st_sol_supply,
        })
    }
}

impl ExchangeRate {
    inherent_borsh_serde!();
}
