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

impl BSize<u32> {
    /// Parses a byte size from a byte slice.
    ///
    /// The input must end with a `B` or `b` byte suffix. Supported units are
    /// `B`, `KB`, `KiB`, `MB`, `MiB`, `GB`, and `GiB`, case-insensitively.
    ///
    /// The numeric part may be an integer or an ordinary decimal number.
    /// Scientific notation is not supported.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError`] if the input cannot be parsed as a `u32` byte
    /// size.
    pub fn parse(src: impl AsRef<[u8]>) -> Result<Self, ParseError> {
        parse_u32(src.as_ref()).map(BSize)
    }
}

impl FromStr for BSize<u32> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::parse(src.as_bytes())
    }
}

#[cfg(target_pointer_width = "32")]
impl BSize<usize> {
    /// Parses a byte size from a byte slice.
    ///
    /// The input must end with a `B` or `b` byte suffix. On 32-bit targets,
    /// supported units are `B`, `KB`, `KiB`, `MB`, `MiB`, `GB`, and `GiB`,
    /// case-insensitively.
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

#[cfg(target_pointer_width = "32")]
impl FromStr for BSize<usize> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::parse(src.as_bytes())
    }
}

fn parse_u32(mut src: &[u8]) -> Result<u32, ParseError> {
    let multiply = parse_unit(&mut src)?;

    parse_number(src, multiply).map(|value| value as u32)
}

#[cfg(target_pointer_width = "32")]
fn parse_usize(src: &[u8]) -> Result<usize, ParseError> {
    parse_u32(src).map(|value| value as usize)
}

fn parse_unit(src: &mut &[u8]) -> Result<u64, ParseError> {
    super::strip_b_suffix(src)?;

    if let Some((&infix, before_i)) = src.split_last() {
        if infix.eq_ignore_ascii_case(&b'i') {
            let Some((&prefix, before_prefix)) = before_i.split_last() else {
                return Err(ParseError(()));
            };
            let Some(factor) = binary_factor(prefix) else {
                return Err(ParseError(()));
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
        if super::is_ascii_unit(prefix) {
            return Err(ParseError(()));
        }
    }

    Ok(1)
}

fn decimal_factor(prefix: u8) -> Option<u64> {
    Some(match prefix.to_ascii_uppercase() {
        b'K' => 1_000,
        b'M' => 1_000_000,
        b'G' => 1_000_000_000,
        _ => return None,
    })
}

fn binary_factor(prefix: u8) -> Option<u64> {
    Some(match prefix.to_ascii_uppercase() {
        b'K' => 1_u64 << 10,
        b'M' => 1_u64 << 20,
        b'G' => 1_u64 << 30,
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
                    return Err(ParseError(()));
                };
                value = next;
                has_digit = true;
            }
            b'.' => return parse_fraction(src, idx + 1, value, has_digit, multiply),
            b'_' | b' ' => {}
            _ => return Err(ParseError(())),
        }

        idx += 1;
    }

    if !has_digit {
        return Err(ParseError(()));
    }

    let Some(value) = value.checked_mul(multiply) else {
        return Err(ParseError(()));
    };
    if value > u64::from(u32::MAX) {
        Err(ParseError(()))
    } else {
        Ok(value)
    }
}

fn parse_fraction(
    src: &[u8],
    start: usize,
    integer: u64,
    has_integer_digit: bool,
    multiply: u64,
) -> Result<u64, ParseError> {
    if !has_integer_digit {
        return Err(ParseError(()));
    }

    let Some(base) = integer.checked_mul(multiply) else {
        return Err(ParseError(()));
    };
    if base > u64::from(u32::MAX) {
        return Err(ParseError(()));
    }

    let mut fraction = 0_u64;
    let mut scale = 1_u64;
    let mut digits = 0_u32;
    let mut idx = start;

    while idx < src.len() {
        match src[idx] {
            digit @ b'0'..=b'9' => {
                if digits == 19 {
                    return Err(ParseError(()));
                }

                fraction = fraction * 10 + u64::from(digit - b'0');
                scale *= 10;
                digits += 1;
            }
            b'_' | b' ' => {}
            _ => return Err(ParseError(())),
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
    if value > u128::from(u32::MAX) {
        Err(ParseError(()))
    } else {
        Ok(value as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_units() {
        assert_eq!(
            BSize::<u32>::parse(b"2MB").unwrap(),
            BSize::<u32>(2_000_000)
        );
        assert_eq!(
            BSize::<u32>::parse(b"2MiB").unwrap(),
            BSize::<u32>(2_097_152)
        );
        assert_eq!(
            BSize::<u32>::parse(b"1GB").unwrap(),
            BSize::<u32>(1_000_000_000)
        );
        assert_eq!(
            BSize::<u32>::parse(b"1GiB").unwrap(),
            BSize::<u32>(1_073_741_824)
        );
    }

    #[test]
    fn parses_fractional_units() {
        assert_eq!(BSize::<u32>::parse(b"1.5KB").unwrap(), BSize::<u32>(1_500));
        assert_eq!(
            BSize::<u32>::parse(b"4294967295.4B").unwrap(),
            BSize::<u32>(u32::MAX)
        );
    }

    #[test]
    fn rejects_unsupported_units() {
        BSize::<u32>::parse(b"1TB").unwrap_err();
        BSize::<u32>::parse(b"1TiB").unwrap_err();
    }

    #[test]
    fn rejects_overflow() {
        BSize::<u32>::parse(b"4294967295.5B").unwrap_err();
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn parses_usize() {
        assert_eq!(
            BSize::<usize>::parse(b"1_234B").unwrap(),
            BSize::<usize>(1_234)
        );
    }
}
