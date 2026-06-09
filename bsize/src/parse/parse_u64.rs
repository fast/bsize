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

use super::ParseError;
use crate::types::BSize;
use core::str::FromStr;

impl BSize<u64> {
    /// Parses a byte size from a byte slice.
    ///
    /// The input must end with a `B` or `b` byte suffix. Supported units are
    /// `B`, `KB`, `KiB`, `MB`, `MiB`, `GB`, `GiB`, `TB`, `TiB`, `PB`, `PiB`,
    /// `EB`, and `EiB`, case-insensitively.
    ///
    /// The numeric part may be an integer or an ordinary decimal number.
    /// Scientific notation is not supported.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError`] if the input cannot be parsed as a `u64` byte
    /// size.
    pub fn parse(src: impl AsRef<[u8]>) -> Result<Self, ParseError> {
        parse_u64(src.as_ref()).map(BSize)
    }
}

impl FromStr for BSize<u64> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::parse(src.as_bytes())
    }
}

#[cfg(target_pointer_width = "64")]
impl BSize<usize> {
    /// Parses a byte size from a byte slice.
    ///
    /// The input must end with a `B` or `b` byte suffix. On 64-bit targets,
    /// supported units are `B`, `KB`, `KiB`, `MB`, `MiB`, `GB`, `GiB`, `TB`,
    /// `TiB`, `PB`, `PiB`, `EB`, and `EiB`, case-insensitively.
    ///
    /// The numeric part may be an integer or an ordinary decimal number.
    /// Scientific notation is not supported.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError`] if the input cannot be parsed as a `usize` byte
    /// size.
    pub fn parse(src: impl AsRef<[u8]>) -> Result<Self, ParseError> {
        parse_usize(src.as_ref()).map(BSize)
    }
}

#[cfg(target_pointer_width = "64")]
impl FromStr for BSize<usize> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::parse(src.as_bytes())
    }
}

fn parse_u64(mut src: &[u8]) -> Result<u64, ParseError> {
    let multiply = parse_unit(&mut src)?;

    parse_number(src, multiply)
}

#[cfg(target_pointer_width = "64")]
fn parse_usize(src: &[u8]) -> Result<usize, ParseError> {
    parse_u64(src).map(|value| value as usize)
}

fn parse_unit(src: &mut &[u8]) -> Result<u64, ParseError> {
    super::strip_b_suffix(src)?;

    if let Some((&infix, before_i)) = src.split_last() {
        if infix.eq_ignore_ascii_case(&b'i') {
            let Some((&prefix, before_prefix)) = before_i.split_last() else {
                return Err(ParseError);
            };
            let Some(factor) = binary_factor(prefix) else {
                return Err(ParseError);
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

fn decimal_factor(prefix: u8) -> Option<u64> {
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

fn binary_factor(prefix: u8) -> Option<u64> {
    Some(match prefix.to_ascii_uppercase() {
        b'K' => 1_u64 << 10,
        b'M' => 1_u64 << 20,
        b'G' => 1_u64 << 30,
        b'T' => 1_u64 << 40,
        b'P' => 1_u64 << 50,
        b'E' => 1_u64 << 60,
        _ => return None,
    })
}

fn parse_number(src: &[u8], multiply: u64) -> Result<u64, ParseError> {
    let mut value = 0_u64;
    let mut has_digit = false;
    let mut idx = 0;

    while idx < src.len() {
        match src[idx] {
            digit @ b'0'..=b'9' => {
                let Some(next) = value
                    .checked_mul(10)
                    .and_then(|value| value.checked_add(u64::from(digit - b'0')))
                else {
                    return Err(ParseError);
                };
                value = next;
                has_digit = true;
            }
            b'.' => return parse_fraction(src, idx + 1, value, has_digit, multiply),
            b'_' | b' ' => {}
            _ => return Err(ParseError),
        }

        idx += 1;
    }

    if !has_digit {
        return Err(ParseError);
    }

    value.checked_mul(multiply).ok_or(ParseError)
}

fn parse_fraction(
    src: &[u8],
    start: usize,
    integer: u64,
    has_integer_digit: bool,
    multiply: u64,
) -> Result<u64, ParseError> {
    if !has_integer_digit {
        return Err(ParseError);
    }

    let Some(base) = integer.checked_mul(multiply) else {
        return Err(ParseError);
    };

    let mut fraction = 0_u64;
    let mut scale = 1_u64;
    let mut digits = 0_u32;
    let mut idx = start;

    while idx < src.len() {
        match src[idx] {
            digit @ b'0'..=b'9' => {
                if digits == 19 {
                    return Err(ParseError);
                }

                fraction = fraction * 10 + u64::from(digit - b'0');
                scale *= 10;
                digits += 1;
            }
            b'_' | b' ' => {}
            _ => return Err(ParseError),
        }

        idx += 1;
    }

    let product = u128::from(fraction) * u128::from(multiply);
    let quotient = product / u128::from(scale);
    let remainder = product % u128::from(scale);
    let rounded = if digits != 0 && remainder >= u128::from(scale / 2) {
        quotient + 1
    } else {
        quotient
    };

    let value = u128::from(base) + rounded;
    if value > u128::from(u64::MAX) {
        Err(ParseError)
    } else {
        Ok(value as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::ParseError;
    use crate::types::BSize;

    #[test]
    fn parses_bytes() {
        assert_eq!(BSize::<u64>::parse(b"1_234B").unwrap(), BSize(1_234));
    }

    #[test]
    fn parses_units() {
        assert_eq!(
            BSize::<u64>::parse(b"1TB").unwrap(),
            BSize(1_000_000_000_000)
        );
        assert_eq!(
            BSize::<u64>::parse(b"1TiB").unwrap(),
            BSize(1_099_511_627_776)
        );
        assert_eq!(
            BSize::<u64>::parse(b"1PB").unwrap(),
            BSize(1_000_000_000_000_000)
        );
        assert_eq!(
            BSize::<u64>::parse(b"1PiB").unwrap(),
            BSize(1_125_899_906_842_624)
        );
    }

    #[test]
    fn parses_fractional_units() {
        assert_eq!(BSize::<u64>::parse(b"1.5KiB").unwrap(), BSize(1_536));
        assert_eq!(
            BSize::<u64>::parse(b"18.446744073709551615EB").unwrap(),
            BSize(u64::MAX)
        );
    }

    #[test]
    fn rejects_invalid_input() {
        assert_eq!(BSize::<u64>::parse(b""), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1"), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1K"), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1XB"), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1iB"), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1e+B"), Err(ParseError));
        assert_eq!(BSize::<u64>::parse(b"1.5e3KiB"), Err(ParseError));
    }

    #[test]
    fn rejects_overflow() {
        assert_eq!(
            BSize::<u64>::parse(b"0.00000000000000000001B"),
            Err(ParseError)
        );
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn parses_usize() {
        assert_eq!(BSize::<usize>::parse(b"1_234B").unwrap(), BSize(1_234));
    }
}
