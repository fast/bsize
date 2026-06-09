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

impl BSize<u128> {
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
    /// Returns [`ParseError`] if the input cannot be parsed as a `u128` byte
    /// size.
    pub fn parse(src: impl AsRef<[u8]>) -> Result<Self, ParseError> {
        parse_u128(src.as_ref()).map(BSize)
    }
}

impl FromStr for BSize<u128> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::parse(src.as_bytes())
    }
}

fn parse_u128(mut src: &[u8]) -> Result<u128, ParseError> {
    let multiply = parse_unit(&mut src)?;

    parse_number(src, multiply)
}

fn parse_unit(src: &mut &[u8]) -> Result<u128, ParseError> {
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

fn parse_number(src: &[u8], multiply: u128) -> Result<u128, ParseError> {
    let mut value = 0_u128;
    let mut has_digit = false;
    let mut idx = 0;

    while idx < src.len() {
        match src[idx] {
            digit @ b'0'..=b'9' => {
                let Some(next) = value
                    .checked_mul(10)
                    .and_then(|value| value.checked_add(u128::from(digit - b'0')))
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

    value.checked_mul(multiply).ok_or(ParseError(()))
}

fn parse_fraction(
    src: &[u8],
    start: usize,
    integer: u128,
    has_integer_digit: bool,
    multiply: u128,
) -> Result<u128, ParseError> {
    if !has_integer_digit {
        return Err(ParseError(()));
    }

    let Some(base) = integer.checked_mul(multiply) else {
        return Err(ParseError(()));
    };

    let mut fraction = 0_u128;
    let mut scale = 1_u128;
    let mut digits = 0_u32;
    let mut idx = start;

    while idx < src.len() {
        match src[idx] {
            digit @ b'0'..=b'9' => {
                if digits == 19 {
                    return Err(ParseError(()));
                }

                fraction = fraction * 10 + u128::from(digit - b'0');
                scale *= 10;
                digits += 1;
            }
            b'_' | b' ' => {}
            _ => return Err(ParseError(())),
        }

        idx += 1;
    }

    let product = fraction * multiply;
    let quotient = product / scale;
    let remainder = product % scale;
    let rounded = if digits != 0 && remainder >= scale / 2 {
        quotient + 1
    } else {
        quotient
    };

    base.checked_add(rounded).ok_or(ParseError(()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_u128_max() {
        assert_eq!(
            BSize::<u128>::parse(b"340282366920938463463374607431768211455B").unwrap(),
            BSize::<u128>(u128::MAX)
        );
    }

    #[test]
    fn rejects_overflow() {
        BSize::<u128>::parse(b"340282366920938463463374607431768211456B").unwrap_err();
    }

    #[test]
    fn parses_u128_max_with_decimal_unit() {
        assert_eq!(
            BSize::<u128>::parse(b"340282366920938463463.374607431768211455EB").unwrap(),
            BSize::<u128>(u128::MAX),
        );
    }

    #[test]
    fn rejects_scientific_notation() {
        BSize::<u128>::parse(b"1.5e3KiB").unwrap_err();
    }
}
