/// Given the total lamports of a validator stake account (includes rent-exemption),
/// return the maximum active stake lamports that the program will allow to be withdrawn from it.
///
/// Returns `None` on arithmetic failure.
///
/// When quoting with [`crate::ExchangeRate::quote_withdraw_stake`], make sure its returned
/// amount does not exceed this amount for the largest validator stake account in the pool.
#[inline]
pub const fn max_withdraw_lamports(stake_acc_total_lamports: u64) -> Option<u64> {
    const MINIMUM_STAKE_ACCOUNT_BALANCE: u64 = 1_002_282_880;
    const TEN_SOL: u64 = 10_000_000_000;

    let ten_pct_plus_10 = match (stake_acc_total_lamports / 10).checked_add(TEN_SOL) {
        Some(r) => r,
        None => return None,
    };
    let exhaustion = stake_acc_total_lamports.saturating_sub(MINIMUM_STAKE_ACCOUNT_BALANCE);
    Some(if ten_pct_plus_10 < exhaustion {
        ten_pct_plus_10
    } else {
        exhaustion
    })
}
