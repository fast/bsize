use crate::unsigned::Unsigned;
use core::any::type_name;
use core::fmt;

/// Byte size representation.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BSize<T: Unsigned>(pub T);

impl<T: Unsigned + fmt::Debug> fmt::Debug for BSize<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BSize<{}>({:?})", type_name::<T>(), self.0)
    }
}

impl<T: Unsigned + fmt::Display> fmt::Display for BSize<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bytes", self.0)
    }
}

impl<T: Unsigned> BSize<T> {
    /// Calculate a new byte size with the provided function, returning a new struct.
    pub fn with(self, f: impl FnOnce(T) -> T) -> Self {
        BSize(f(self.0))
    }

    /// Constructs a byte size wrapper from a quantity of bytes.
    #[inline(always)]
    pub const fn b(size: T) -> Self {
        BSize(size)
    }
}

macro_rules! impl_constructors {
    ($ty:ty => { $($name:ident = $size:literal),* $(,)? }) => {
        impl BSize<$ty> {
            $(
                #[doc = concat!(
                    "Constructs a byte size wrapper from a quantity of `",
                    stringify!($name),
                    "` units."
                )]
                #[inline(always)]
                pub const fn $name(size: $ty) -> Self {
                    BSize(size * $size)
                }
            )*
        }
    };
}

impl_constructors!(u16 => {
    kb = 1_000,
    kib = 1_024,
});

impl_constructors!(u32 => {
    kb = 1_000,
    kib = 1_024,
    mb = 1_000_000,
    mib = 1_048_576,
    gb = 1_000_000_000,
    gib = 1_073_741_824,
});

impl_constructors!(u64 => {
    kb = 1_000,
    kib = 1_024,
    mb = 1_000_000,
    mib = 1_048_576,
    gb = 1_000_000_000,
    gib = 1_073_741_824,
    tb = 1_000_000_000_000,
    tib = 1_099_511_627_776,
    pb = 1_000_000_000_000_000,
    pib = 1_125_899_906_842_624,
    eb = 1_000_000_000_000_000_000,
    eib = 1_152_921_504_606_846_976,
});

impl_constructors!(u128 => {
    kb = 1_000,
    kib = 1_024,
    mb = 1_000_000,
    mib = 1_048_576,
    gb = 1_000_000_000,
    gib = 1_073_741_824,
    tb = 1_000_000_000_000,
    tib = 1_099_511_627_776,
    pb = 1_000_000_000_000_000,
    pib = 1_125_899_906_842_624,
    eb = 1_000_000_000_000_000_000,
    eib = 1_152_921_504_606_846_976,
});

impl_constructors!(usize => {
    kb = 1_000,
    kib = 1_024,
});

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl_constructors!(usize => {
    mb = 1_000_000,
    mib = 1_048_576,
    gb = 1_000_000_000,
    gib = 1_073_741_824,
});

#[cfg(target_pointer_width = "64")]
impl_constructors!(usize => {
    tb = 1_000_000_000_000,
    tib = 1_099_511_627_776,
    pb = 1_000_000_000_000_000,
    pib = 1_125_899_906_842_624,
    eb = 1_000_000_000_000_000_000,
    eib = 1_152_921_504_606_846_976,
});

#[cfg(test)]
mod tests {
    use super::BSize;

    #[test]
    fn defaults() {
        assert_eq!(BSize::<u8>::default(), BSize::b(0));
        assert_eq!(BSize::<u16>::default(), BSize::b(0));
        assert_eq!(BSize::<u32>::default(), BSize::b(0));
        assert_eq!(BSize::<u64>::default(), BSize::b(0));
        assert_eq!(BSize::<u128>::default(), BSize::b(0));
        assert_eq!(BSize::<usize>::default(), BSize::b(0));
    }

    #[test]
    fn constructs_u8_units() {
        assert_eq!(BSize::<u8>::b(2).0, 2);
    }

    #[test]
    fn constructs_u16_units() {
        assert_eq!(BSize::<u16>::kb(2).0, 2_000);
        assert_eq!(BSize::<u16>::kib(2).0, 2_048);
    }

    #[test]
    fn constructs_u32_units() {
        assert_eq!(BSize::<u32>::gb(2).0, 2_000_000_000);
        assert_eq!(BSize::<u32>::gib(2).0, 2_147_483_648);
    }

    #[test]
    fn constructs_u64_units() {
        assert_eq!(BSize::<u64>::eb(2).0, 2_000_000_000_000_000_000);
        assert_eq!(BSize::<u64>::eib(2).0, 2_305_843_009_213_693_952);
    }

    #[test]
    fn constructs_u128_units() {
        assert_eq!(BSize::<u128>::eb(20).0, 20_000_000_000_000_000_000);
        assert_eq!(BSize::<u128>::eib(20).0, 23_058_430_092_136_939_520);
    }

    #[test]
    fn constructs_usize_units() {
        assert_eq!(BSize::<usize>::kb(2).0, 2_000);
        assert_eq!(BSize::<usize>::kib(2).0, 2_048);
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    #[test]
    fn constructs_32_bit_usize_units() {
        assert_eq!(BSize::<usize>::gb(2).0, 2_000_000_000);
        assert_eq!(BSize::<usize>::gib(2).0, 2_147_483_648);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn constructs_64_bit_usize_units() {
        assert_eq!(BSize::<usize>::eb(2).0, 2_000_000_000_000_000_000);
        assert_eq!(BSize::<usize>::eib(2).0, 2_305_843_009_213_693_952);
    }
}
