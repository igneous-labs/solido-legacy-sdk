use crate::Validator;

use super::AccountList;

pub type ValidatorList<'a> = AccountList<'a, Validator>;

impl ValidatorList<'_> {
    /// Program only allows stake to be withdrawn from the largest
    /// validator on the list
    pub fn withdrawing_validator(&self) -> Option<(usize, &Validator)> {
        self.entries
            .iter()
            .enumerate()
            .max_by_key(|(_i, v)| v.effective_stake_balance())
    }
}
