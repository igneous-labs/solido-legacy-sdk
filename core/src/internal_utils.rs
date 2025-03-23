#![macro_use]

macro_rules! inherent_borsh_ser {
    () => {
        /// Expose borsh serialization as inherent method
        /// so that it can still be used
        /// even with conflicting versions of borsh in downstream projects.
        ///
        /// And also pass writer by value just to be more inline with rust standards
        #[inline]
        pub fn borsh_ser<W: borsh::io::Write>(&self, mut writer: W) -> borsh::io::Result<()> {
            <Self as borsh::BorshSerialize>::serialize(self, &mut writer)
        }
    };
}

macro_rules! inherent_borsh_de {
    () => {
        /// Expose borsh deserialization as inherent method
        /// so that it can still be used
        /// even with conflicting versions of borsh in downstream projects.
        ///
        /// And also pass reader by value just to be more inline with rust standards
        #[inline]
        pub fn borsh_de<R: borsh::io::Read>(mut reader: R) -> borsh::io::Result<Self> {
            <Self as borsh::BorshDeserialize>::deserialize_reader(&mut reader)
        }
    };
}

macro_rules! inherent_borsh_serde {
    () => {
        inherent_borsh_ser!();
        inherent_borsh_de!();
    };
}

macro_rules! impl_get_le {
    ($field:ident, $t:ty) => {
        #[inline]
        pub const fn $field(&self) -> $t {
            <$t>::from_le_bytes(self.$field)
        }
    };
}

macro_rules! impl_set_le {
    ($set:ident, $field:ident, $t:ty) => {
        #[inline]
        pub const fn $set(&mut self, val: $t) {
            self.$field = val.to_le_bytes();
        }
    };
}

macro_rules! impl_with_le {
    ($with:ident, $field:ident, $t:ty) => {
        #[inline]
        pub const fn $with(mut self, val: $t) -> Self {
            self.$field = val.to_le_bytes();
            self
        }
    };
}

macro_rules! impl_set_with_get_le {
    ($set:ident, $with:ident, $field:ident, $t:ty) => {
        impl_set_le!($set, $field, $t);
        impl_with_le!($with, $field, $t);
        impl_get_le!($field, $t);
    };
}

macro_rules! impl_get_ref {
    ($field:ident, $t:ty) => {
        #[inline]
        pub const fn $field(&self) -> &$t {
            &self.$field
        }
    };
}

macro_rules! impl_get {
    ($field:ident, $t:ty) => {
        #[inline]
        pub const fn $field(&self) -> $t {
            self.$field
        }
    };
}

macro_rules! impl_set {
    ($set:ident, $field:ident, $t:ty) => {
        #[inline]
        pub const fn $set(&mut self, val: $t) {
            self.$field = val;
        }
    };
}

macro_rules! impl_with {
    ($with:ident, $field:ident, $t:ty) => {
        #[inline]
        pub const fn $with(mut self, val: $t) -> Self {
            self.$field = val;
            self
        }
    };
}

macro_rules! impl_set_with_get_ref {
    ($set:ident, $with:ident, $field:ident, $t:ty) => {
        impl_set!($set, $field, $t);
        impl_with!($with, $field, $t);
        impl_get_ref!($field, $t);
    };
}

macro_rules! impl_set_with_get {
    ($set:ident, $with:ident, $field:ident, $t:ty) => {
        impl_set!($set, $field, $t);
        impl_with!($with, $field, $t);
        impl_get!($field, $t);
    };
}

pub const fn const_assign_byte_arr<const A: usize, const START: usize, const LEN: usize>(
    arr: &mut [u8; A],
    val: [u8; LEN],
) {
    const {
        assert!(START + LEN <= A);
    }
    // safety: bounds checked at comptime above
    unsafe {
        // guarantee nonoverlapping due to `&mut`
        core::ptr::copy_nonoverlapping(val.as_ptr(), arr.as_ptr().add(START).cast_mut(), LEN);
    }
}
