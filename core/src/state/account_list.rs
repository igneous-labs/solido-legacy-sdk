use core::num::TryFromIntError;

use borsh::{BorshDeserialize, BorshSerialize};

use crate::{ListHeader, Validator};

pub type ValidatorList<'a> = AccountList<'a, Validator>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AccountList<'a, T> {
    pub header: ListHeader,
    pub entries: &'a [T],
}

/// Includes `entries.len()`. Used for borsh ser/de only
#[derive(BorshDeserialize, BorshSerialize)]
struct AccountListHeaderExt {
    header: ListHeader,
    entries_len: u32,
}

impl<'a, T> AccountList<'a, T> {
    #[inline]
    pub fn deserialize(data: &'a [u8]) -> borsh::io::Result<Self> {
        let mut remaining = data;
        let AccountListHeaderExt {
            header,
            entries_len,
        } = AccountListHeaderExt::deserialize(&mut remaining)?;

        let entries_len = usize::try_from(entries_len).map_err(u32_usize_try_from_err)?;
        let t_size = core::mem::size_of::<T>();
        let min_expected_len = entries_len * t_size;
        if remaining.len() < min_expected_len {
            return Err(borsh::io::Error::new(
                borsh::io::ErrorKind::InvalidData,
                "data too small for entries",
            ));
        }

        const {
            assert!(core::mem::align_of::<T>() == 1);
        };
        // SAFETY: align = 1 checked at compile time above
        let entries =
            unsafe { core::slice::from_raw_parts(remaining.as_ptr().cast(), entries_len) };
        Ok(Self { header, entries })
    }
}

fn u32_usize_try_from_err(_e: TryFromIntError) -> borsh::io::Error {
    borsh::io::Error::new(
        borsh::io::ErrorKind::Other,
        "could not convert u32 <-> usize",
    )
}
