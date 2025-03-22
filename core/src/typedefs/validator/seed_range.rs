//! repr(C) & align=1, fit for pointer casting

use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, BorshDeserialize, BorshSerialize)]
pub struct SeedRange {
    begin: [u8; 8],
    end: [u8; 8],
}

impl SeedRange {
    impl_set_with_get_le!(set_begin, with_begin, begin, u64);
    impl_set_with_get_le!(set_end, with_end, end, u64);
}

impl SeedRange {
    inherent_borsh_serde!();
}

const _ASSERT_ALIGN_1: () = assert!(core::mem::align_of::<SeedRange>() == 1);
