use core::{error::Error, fmt::Display};

/// NB: this is not in the order defined by the program
/// TODO: `seqconsts!()` this to make it so
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LidoError {
    CalculationFailure,
    ExchangeRateNotUpdatedInThisEpoch,
    InvalidAmount,
    ValidatorWithMoreStakeExists,
}

impl Display for LidoError {
    // Display=Debug, since this is just a simple str enum
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for LidoError {}
