// Copyright 2026 FastLabs Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::fmt;

/// The error returned when parsing a byte size fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid byte size")
    }
}

impl core::error::Error for ParseError {}

macro_rules! impl_parse {
    ($ty:ty, $parse:path) => {
        impl crate::types::BSize<$ty> {
            /// Parses a byte size from a byte slice.
            ///
            /// The input must end with a `B` or `b` byte suffix. Supported
            /// units depend on the target integer type.
            ///
            /// # Errors
            ///
            /// Returns [`ParseError`] if the input cannot be parsed as a byte
            /// size for the target integer type.
            pub fn parse(src: impl AsRef<[u8]>) -> Result<Self, crate::parse::ParseError> {
                $parse(src.as_ref()).map(crate::types::BSize)
            }
        }

        impl core::str::FromStr for crate::types::BSize<$ty> {
            type Err = crate::parse::ParseError;

            fn from_str(src: &str) -> Result<Self, Self::Err> {
                Self::parse(src.as_bytes())
            }
        }
    };
}

macro_rules! impl_parse_number {
    ($ty:ty, $multiply:path, $divide:path) => {
        impl crate::parse::number::ParseNumber for $ty {
            const ZERO: Self = 0;

            fn append_digit(self, digit: u8) -> Option<Self> {
                self.checked_mul(10)
                    .and_then(|value| value.checked_add((digit - b'0') as $ty))
            }

            fn round_overflowed(self, digit: u8) -> Self {
                if digit >= b'5' {
                    self.saturating_add(1)
                } else {
                    self
                }
            }

            fn multiply_integer(
                self,
                multiply: Self,
                exponent: u32,
                max: Self,
            ) -> Result<Self, crate::parse::ParseError> {
                $multiply(self, multiply, exponent, max)
            }

            fn divide_integer(
                self,
                multiply: Self,
                exponent: u32,
                max: Self,
            ) -> Result<Self, crate::parse::ParseError> {
                $divide(self, multiply, exponent, max)
            }
        }
    };
}

mod number;
mod parse_u128;
mod parse_u16;
mod parse_u32;
mod parse_u64;
mod parse_u8;
mod u256;

fn strip_b_suffix(src: &mut &[u8]) -> Result<(), ParseError> {
    let Some((&suffix, before_b)) = src.split_last() else {
        return Err(ParseError);
    };
    if !suffix.eq_ignore_ascii_case(&b'B') {
        return Err(ParseError);
    }

    *src = before_b;
    Ok(())
}

fn is_ascii_unit(byte: u8) -> bool {
    matches!(
        byte.to_ascii_uppercase(),
        b'K' | b'M' | b'G' | b'T' | b'P' | b'E'
    )
}

#[cfg(test)]
mod tests {
    use super::ParseError;
    use super::u256::U256;
    use crate::types::BSize;

    #[test]
    fn parses_bytes() {
        assert_eq!(BSize::<u8>::parse(b"255B").unwrap(), BSize(255));
        assert_eq!(BSize::<u16>::parse(b"1 B").unwrap(), BSize(1));
        assert_eq!(BSize::<usize>::parse(b"1_234B").unwrap(), BSize(1_234));
    }

    #[test]
    fn parses_units() {
        assert_eq!(BSize::<u16>::parse(b"1KB").unwrap(), BSize(1_000));
        assert_eq!(BSize::<u16>::parse(b"1KiB").unwrap(), BSize(1_024));
        assert_eq!(BSize::<u16>::parse(b"1kb").unwrap(), BSize(1_000));
        assert_eq!(BSize::<u16>::parse(b"1kib").unwrap(), BSize(1_024));
        assert_eq!(BSize::<u16>::parse(b"1KIB").unwrap(), BSize(1_024));
        assert_eq!(BSize::<u32>::parse(b"2MB").unwrap(), BSize(2_000_000));
        assert_eq!(BSize::<u32>::parse(b"2MiB").unwrap(), BSize(2_097_152));
    }

    #[test]
    fn parses_fractional_units() {
        assert_eq!(BSize::<u16>::parse(b"65.535KB").unwrap(), BSize(u16::MAX));
        assert_eq!(BSize::<u16>::parse(b"0.5B").unwrap(), BSize(1));
        assert_eq!(BSize::<u16>::parse(b"0.4B").unwrap(), BSize(0));
        assert_eq!(BSize::<u32>::parse(b"1.5e3B").unwrap(), BSize(1_500));
        assert_eq!(BSize::<u8>::parse(b"25.55B").unwrap(), BSize(26));
        assert_eq!(BSize::<u8>::parse(b"255.4B").unwrap(), BSize(u8::MAX));
        assert_eq!(BSize::<u16>::parse(b"65535.4B").unwrap(), BSize(u16::MAX));
        assert_eq!(
            BSize::<u32>::parse(b"4294967295.4B").unwrap(),
            BSize(u32::MAX)
        );
    }

    #[test]
    fn rejects_invalid_units() {
        assert_eq!(BSize::<u64>::parse(b""), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1"), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1K"), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1XB"), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1iB"), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1e+B"), Err(ParseError));
        assert_eq!(BSize::<u8>::parse(b"1eB"), Err(ParseError));
        assert_eq!(BSize::<u8>::parse(b"1EB"), Err(ParseError));
    }

    #[test]
    fn rejects_overflow() {
        assert_eq!(BSize::<u8>::parse(b"256B"), Err(ParseError));
        assert_eq!(BSize::<u8>::parse(b"1KB"), Err(ParseError));
        assert_eq!(BSize::<u8>::parse(b"0.001KB"), Err(ParseError));
        assert_eq!(BSize::<u8>::parse(b"255.5B"), Err(ParseError));
        assert_eq!(BSize::<u16>::parse(b"65535.5B"), Err(ParseError));
        assert_eq!(BSize::<u32>::parse(b"4294967295.5B"), Err(ParseError));
        assert_eq!(BSize::<u16>::parse(b"65.536KB"), Err(ParseError));
    }

    #[test]
    fn parses_u128_max() {
        assert_eq!(
            BSize::<u128>::parse(b"340282366920938463463374607431768211455B"),
            Ok(BSize(u128::MAX)),
        );
        assert_eq!(
            BSize::<u128>::parse(b"340282366920938463463374607431768211456B"),
            Err(ParseError),
        );
    }

    #[test]
    fn parses_u128_max_with_decimal_unit() {
        assert_eq!(
            BSize::<u128>::parse(b"340282366920938463463.374607431768211455EB"),
            Ok(BSize(u128::MAX)),
        );
    }

    #[test]
    fn multiplies_u128_into_u256() {
        let value = U256::multiply(u128::MAX, 1_000_000_000_000_000_000);

        assert_eq!(value.hi, 999_999_999_999_999_999);
        assert_eq!(
            value.lo,
            340_282_366_920_938_463_462_374_607_431_768_211_456
        );
    }
}
