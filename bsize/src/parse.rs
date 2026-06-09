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
use core::str::FromStr;

use crate::types::BSize;

/// The error returned when parsing a byte size fails.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    /// The input contains no number.
    Empty,
    /// The input contains an invalid byte.
    InvalidDigit,
    /// The parsed byte count is too large for the target integer type.
    PosOverflow,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Empty => "cannot parse integer from empty string",
            Self::InvalidDigit => "invalid digit found in string",
            Self::PosOverflow => "number too large to fit in target type",
        })
    }
}

impl core::error::Error for ParseError {}

impl From<ParseError> for core::num::IntErrorKind {
    fn from(error: ParseError) -> Self {
        match error {
            ParseError::Empty => Self::Empty,
            ParseError::InvalidDigit => Self::InvalidDigit,
            ParseError::PosOverflow => Self::PosOverflow,
        }
    }
}

macro_rules! impl_parse {
    ($($ty:ty),* $(,)?) => {
        $(
            impl BSize<$ty> {
                /// Parses a byte size from a byte slice.
                ///
                /// The input must end with a `B` or `b` byte suffix. Supported
                /// units are `B`, `KB`, `KiB`, `MB`, `MiB`, `GB`, `GiB`, `TB`,
                /// `TiB`, `PB`, `PiB`, `EB`, and `EiB`, case-insensitively.
                ///
                /// # Errors
                ///
                /// Returns [`ParseError`] if the input is empty, contains an
                /// invalid byte, or overflows the target integer type.
                pub fn parse(src: impl AsRef<[u8]>) -> Result<Self, ParseError> {
                    parse(src.as_ref(), <$ty>::MAX as u128).map(|value| BSize(value as $ty))
                }
            }

            impl FromStr for BSize<$ty> {
                type Err = ParseError;

                fn from_str(src: &str) -> Result<Self, Self::Err> {
                    Self::parse(src.as_bytes())
                }
            }
        )*
    };
}

impl_parse!(u8, u16, u32, u64, u128, usize);

fn parse(mut src: &[u8], max: u128) -> Result<u128, ParseError> {
    let multiply = parse_unit(&mut src)?;
    if multiply > max {
        return Err(ParseError::PosOverflow);
    }

    parse_number(src, multiply, max)
}

fn parse_unit(src: &mut &[u8]) -> Result<u128, ParseError> {
    let Some((&suffix, before_b)) = src.split_last() else {
        return if src.is_empty() {
            Err(ParseError::Empty)
        } else {
            Err(ParseError::InvalidDigit)
        };
    };
    if !suffix.eq_ignore_ascii_case(&b'B') {
        return Err(ParseError::InvalidDigit);
    }

    *src = before_b;

    if let Some((&infix, before_i)) = src.split_last() {
        if infix.eq_ignore_ascii_case(&b'i') {
            let Some((&prefix, before_prefix)) = before_i.split_last() else {
                return Err(ParseError::InvalidDigit);
            };
            let Some(factor) = binary_factor(prefix) else {
                return Err(ParseError::InvalidDigit);
            };

            *src = before_prefix;
            return Ok(factor);
        }
    }

    if let Some((&prefix, before_prefix)) = src.split_last() {
        if let Some(factor) = decimal_factor(prefix) {
            *src = before_prefix;
            return Ok(factor);
        }
    }

    Ok(1)
}

fn decimal_factor(prefix: u8) -> Option<u128> {
    Some(match prefix.to_ascii_uppercase() {
        b'K' => 1_000,
        b'M' => 1_000_000,
        b'G' => 1_000_000_000,
        b'T' => 1_000_000_000_000,
        b'P' => 1_000_000_000_000_000,
        b'E' => 1_000_000_000_000_000_000,
        _ => return None,
    })
}

fn binary_factor(prefix: u8) -> Option<u128> {
    Some(match prefix.to_ascii_uppercase() {
        b'K' => 1_u128 << 10,
        b'M' => 1_u128 << 20,
        b'G' => 1_u128 << 30,
        b'T' => 1_u128 << 40,
        b'P' => 1_u128 << 50,
        b'E' => 1_u128 << 60,
        _ => return None,
    })
}

fn parse_number(src: &[u8], multiply: u128, max: u128) -> Result<u128, ParseError> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum State {
        Empty,
        Integer,
        IntegerOverflow,
        Fraction,
        FractionOverflow,
        PosExponent,
        NegExponent,
    }

    let mut mantissa = 0_u128;
    let mut fractional_exponent = 0_i32;
    let mut exponent = 0_i32;
    let mut state = State::Empty;

    for b in src {
        match (state, *b) {
            (State::Integer | State::Empty, b'0'..=b'9') => {
                if let Some(next) = append_digit(mantissa, *b) {
                    mantissa = next;
                    state = State::Integer;
                } else {
                    mantissa = round_overflowed_mantissa(mantissa, *b);
                    fractional_exponent = fractional_exponent.saturating_add(1);
                    state = State::IntegerOverflow;
                }
            }
            (State::IntegerOverflow, b'0'..=b'9') => {
                fractional_exponent = fractional_exponent.saturating_add(1);
            }
            (State::Fraction, b'0'..=b'9') => {
                if let Some(next) = append_digit(mantissa, *b) {
                    mantissa = next;
                    fractional_exponent = fractional_exponent.saturating_sub(1);
                } else {
                    mantissa = round_overflowed_mantissa(mantissa, *b);
                    state = State::FractionOverflow;
                }
            }
            (State::PosExponent, b'0'..=b'9') => {
                exponent = append_exponent_digit(exponent, *b, true)?;
            }
            (State::NegExponent, b'0'..=b'9') => {
                exponent = append_exponent_digit(exponent, *b, false)?;
            }
            (_, b'_' | b' ')
            | (State::PosExponent, b'+')
            | (State::FractionOverflow, b'0'..=b'9') => {}
            (
                State::Integer | State::Fraction | State::IntegerOverflow | State::FractionOverflow,
                b'e' | b'E',
            ) => state = State::PosExponent,
            (State::PosExponent, b'-') => state = State::NegExponent,
            (State::Integer, b'.') => state = State::Fraction,
            (State::IntegerOverflow, b'.') => state = State::FractionOverflow,
            _ => return Err(ParseError::InvalidDigit),
        }
    }

    if state == State::Empty {
        return Err(ParseError::Empty);
    }

    let exponent = exponent.saturating_add(fractional_exponent);
    if exponent >= 0 {
        multiply_integer(mantissa, multiply, exponent.unsigned_abs(), max)
    } else {
        divide_integer(mantissa, multiply, exponent.unsigned_abs(), max)
    }
}

fn append_digit(value: u128, digit: u8) -> Option<u128> {
    value
        .checked_mul(10)
        .and_then(|value| value.checked_add(u128::from(digit - b'0')))
}

fn round_overflowed_mantissa(mantissa: u128, digit: u8) -> u128 {
    if digit >= b'5' {
        mantissa.saturating_add(1)
    } else {
        mantissa
    }
}

fn append_exponent_digit(exponent: i32, digit: u8, positive: bool) -> Result<i32, ParseError> {
    let digit = i32::from(digit - b'0');
    if positive {
        exponent
            .checked_mul(10)
            .and_then(|value| value.checked_add(digit))
            .ok_or(ParseError::PosOverflow)
    } else {
        Ok(exponent
            .checked_mul(10)
            .and_then(|value| value.checked_sub(digit))
            .unwrap_or(i32::MIN))
    }
}

fn multiply_integer(
    mantissa: u128,
    multiply: u128,
    exponent: u32,
    max: u128,
) -> Result<u128, ParseError> {
    let power = 10_u128
        .checked_pow(exponent)
        .ok_or(ParseError::PosOverflow)?;
    let multiply = multiply.checked_mul(power).ok_or(ParseError::PosOverflow)?;
    let value = mantissa
        .checked_mul(multiply)
        .ok_or(ParseError::PosOverflow)?;

    if value > max {
        Err(ParseError::PosOverflow)
    } else {
        Ok(value)
    }
}

fn divide_integer(
    mantissa: u128,
    multiply: u128,
    exponent: u32,
    max: u128,
) -> Result<u128, ParseError> {
    if exponent >= 58 {
        return Ok(0);
    }

    let mut value = U256::multiply(mantissa, multiply);
    let mut round = false;

    for _ in 0..exponent {
        let (quotient, remainder) = value.div_rem_10();
        value = quotient;
        round = remainder >= 5;
    }

    if round {
        value = value.checked_add_one().ok_or(ParseError::PosOverflow)?;
    }

    let value = value.try_into_u128().ok_or(ParseError::PosOverflow)?;
    if value > max {
        Err(ParseError::PosOverflow)
    } else {
        Ok(value)
    }
}

#[derive(Clone, Copy)]
struct U256 {
    hi: u128,
    lo: u128,
}

impl U256 {
    fn multiply(lhs: u128, rhs: u128) -> Self {
        let mask = u128::from(u64::MAX);
        let lhs_lo = lhs & mask;
        let lhs_hi = lhs >> 64;
        let rhs_lo = rhs & mask;
        let rhs_hi = rhs >> 64;

        let low = lhs_lo * rhs_lo;
        let mid_left = lhs_lo * rhs_hi;
        let mid_right = lhs_hi * rhs_lo;
        let high = lhs_hi * rhs_hi;

        let carry = (low >> 64) + (mid_left & mask) + (mid_right & mask);
        let lo = (low & mask) | ((carry & mask) << 64);
        let hi = high + (mid_left >> 64) + (mid_right >> 64) + (carry >> 64);

        Self { hi, lo }
    }

    fn div_rem_10(self) -> (Self, u8) {
        let mut remainder = 0_u128;

        let (hi_hi, next) = div_limb(self.hi >> 64, remainder);
        remainder = next;
        let (hi_lo, next) = div_limb(self.hi as u64 as u128, remainder);
        remainder = next;
        let (lo_hi, next) = div_limb(self.lo >> 64, remainder);
        remainder = next;
        let (lo_lo, remainder) = div_limb(self.lo as u64 as u128, remainder);

        (
            Self {
                hi: (hi_hi << 64) | hi_lo,
                lo: (lo_hi << 64) | lo_lo,
            },
            remainder as u8,
        )
    }

    fn checked_add_one(self) -> Option<Self> {
        let (lo, carry) = self.lo.overflowing_add(1);
        let hi = if carry {
            self.hi.checked_add(1)?
        } else {
            self.hi
        };

        Some(Self { hi, lo })
    }

    fn try_into_u128(self) -> Option<u128> {
        if self.hi == 0 {
            Some(self.lo)
        } else {
            None
        }
    }
}

fn div_limb(limb: u128, remainder: u128) -> (u128, u128) {
    let value = (remainder << 64) | limb;
    (value / 10, value % 10)
}

#[cfg(test)]
mod tests {
    use super::{ParseError, U256};
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
    }

    #[test]
    fn rejects_invalid_units() {
        assert_eq!(BSize::<u64>::parse(b""), Err(ParseError::Empty));
        assert_eq!(BSize::<u64>::parse(b"1"), Err(ParseError::InvalidDigit));
        assert_eq!(BSize::<u64>::parse(b"1K"), Err(ParseError::InvalidDigit));
        assert_eq!(BSize::<u64>::parse(b"1XB"), Err(ParseError::InvalidDigit));
        assert_eq!(BSize::<u64>::parse(b"1iB"), Err(ParseError::InvalidDigit));
    }

    #[test]
    fn rejects_overflow() {
        assert_eq!(BSize::<u8>::parse(b"256B"), Err(ParseError::PosOverflow));
        assert_eq!(BSize::<u8>::parse(b"1KB"), Err(ParseError::PosOverflow));
        assert_eq!(BSize::<u8>::parse(b"0.001KB"), Err(ParseError::PosOverflow));
        assert_eq!(
            BSize::<u16>::parse(b"65.536KB"),
            Err(ParseError::PosOverflow)
        );
    }

    #[test]
    fn parses_u128_max() {
        assert_eq!(
            BSize::<u128>::parse(b"340282366920938463463374607431768211455B"),
            Ok(BSize(u128::MAX)),
        );
        assert_eq!(
            BSize::<u128>::parse(b"340282366920938463463374607431768211456B"),
            Err(ParseError::PosOverflow),
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
