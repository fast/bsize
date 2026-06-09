mod private {
    pub trait Sealed {}

    impl Sealed for usize {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}
}

/// A marker trait for all unsigned integers.
pub trait Unsigned: private::Sealed {}

impl Unsigned for usize {}
impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
