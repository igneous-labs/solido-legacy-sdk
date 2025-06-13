use borsh::{BorshDeserialize, BorshSerialize};
use sanctum_u64_ratio::{Floor, Ratio};

use crate::LidoError;

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
    pub const fn quote_withdraw_stake(&self, stsol_amount: u64) -> Result<u64, LidoError> {
        let ratio = self.sol_balance_over_st_sol_supply();
        if ratio.0.is_zero() {
            return Err(LidoError::CalculationFailure);
        }
        match ratio.apply(stsol_amount) {
            Some(v) => Ok(v),
            None => Err(LidoError::CalculationFailure),
        }
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
