use core::ops;

use crate::types::BSize;

macro_rules! impl_ops {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ops::Add<BSize<$ty>> for BSize<$ty> {
                type Output = Self;

                #[inline(always)]
                fn add(self, rhs: BSize<$ty>) -> Self::Output {
                    BSize(self.0 + rhs.0)
                }
            }

            impl ops::AddAssign<BSize<$ty>> for BSize<$ty> {
                #[inline(always)]
                fn add_assign(&mut self, rhs: BSize<$ty>) {
                    self.0 += rhs.0;
                }
            }

            impl ops::Sub<BSize<$ty>> for BSize<$ty> {
                type Output = Self;

                #[inline(always)]
                fn sub(self, rhs: BSize<$ty>) -> Self::Output {
                    BSize(self.0 - rhs.0)
                }
            }

            impl ops::SubAssign<BSize<$ty>> for BSize<$ty> {
                #[inline(always)]
                fn sub_assign(&mut self, rhs: BSize<$ty>) {
                    self.0 -= rhs.0;
                }
            }
        )*
    };
}

impl_ops!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::BSize;

    #[test]
    fn adds_byte_sizes() {
        assert_eq!((BSize::<u8>(3) + BSize(5)).0, 8);
        assert_eq!((BSize::<u16>(3) + BSize(5)).0, 8);
        assert_eq!((BSize::<u32>(3) + BSize(5)).0, 8);
        assert_eq!((BSize::<u64>(3) + BSize(5)).0, 8);
        assert_eq!((BSize::<u128>(3) + BSize(5)).0, 8);
        assert_eq!((BSize::<usize>(3) + BSize(5)).0, 8);
    }

    #[test]
    fn add_assigns_byte_sizes() {
        let mut size = BSize::<usize>(3);
        size += BSize(5);
        assert_eq!(size.0, 8);
    }

    #[test]
    fn subtracts_byte_sizes() {
        assert_eq!((BSize::<u8>(8) - BSize(5)).0, 3);
        assert_eq!((BSize::<u16>(8) - BSize(5)).0, 3);
        assert_eq!((BSize::<u32>(8) - BSize(5)).0, 3);
        assert_eq!((BSize::<u64>(8) - BSize(5)).0, 3);
        assert_eq!((BSize::<u128>(8) - BSize(5)).0, 3);
        assert_eq!((BSize::<usize>(8) - BSize(5)).0, 3);
    }

    #[test]
    fn sub_assigns_byte_sizes() {
        let mut size = BSize::<usize>(8);
        size -= BSize(5);
        assert_eq!(size.0, 3);
    }
}
