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

impl BSize<u8> {
    /// Parses a byte size from a byte slice.
    ///
    /// The input must end with a `B` or `b` byte suffix. No other unit is
    /// supported for `u8`.
    ///
    /// The numeric part may be an integer or an ordinary decimal number.
    /// Scientific notation is not supported.
    ///
    /// # Errors
    ///
    /// Returns [`ParseError`] if the input cannot be parsed as a `u8` byte
    /// size.
    pub fn parse(src: impl AsRef<[u8]>) -> Result<Self, ParseError> {
        parse_u8(src.as_ref()).map(BSize)
    }
}

impl FromStr for BSize<u8> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Self::parse(src.as_bytes())
    }
}

fn parse_u8(mut src: &[u8]) -> Result<u8, ParseError> {
    super::strip_b_suffix(&mut src)?;

    parse_number(src)
}

fn parse_number(src: &[u8]) -> Result<u8, ParseError> {
    let mut value = 0_u8;
    let mut has_digit = false;
    let mut idx = 0;

    while idx < src.len() {
        match src[idx] {
            digit @ b'0'..=b'9' => {
                let Some(next) = value
                    .checked_mul(10)
                    .and_then(|value| value.checked_add(digit - b'0'))
                else {
                    return Err(ParseError);
                };
                value = next;
                has_digit = true;
            }
            b'.' => return parse_fraction(src, idx + 1, value, has_digit),
            b'_' | b' ' => {}
            _ => return Err(ParseError),
        }

        idx += 1;
    }

    if has_digit {
        Ok(value)
    } else {
        Err(ParseError)
    }
}

fn parse_fraction(
    src: &[u8],
    start: usize,
    integer: u8,
    has_integer_digit: bool,
) -> Result<u8, ParseError> {
    if !has_integer_digit {
        return Err(ParseError);
    }

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

    let rounded = if digits != 0 && fraction >= scale / 2 {
        1
    } else {
        0
    };

    integer.checked_add(rounded).ok_or(ParseError)
}

#[cfg(test)]
mod tests {
    use super::ParseError;
    use crate::types::BSize;

    #[test]
    fn parses_bytes() {
        assert_eq!(BSize::<u8>::parse(b"255B").unwrap(), BSize(255));
    }

    #[test]
    fn parses_fractional_bytes() {
        assert_eq!(BSize::<u8>::parse(b"25.55B").unwrap(), BSize(26));
        assert_eq!(BSize::<u8>::parse(b"255.4B").unwrap(), BSize(u8::MAX));
    }

    #[test]
    fn rejects_units() {
        assert_eq!(BSize::<u8>::parse(b"1KB"), Err(ParseError));
        assert_eq!(BSize::<u8>::parse(b"1eB"), Err(ParseError));
        assert_eq!(BSize::<u8>::parse(b"1EB"), Err(ParseError));
    }

    #[test]
    fn rejects_overflow() {
        assert_eq!(BSize::<u8>::parse(b"256B"), Err(ParseError));
        assert_eq!(BSize::<u8>::parse(b"0.001KB"), Err(ParseError));
        assert_eq!(BSize::<u8>::parse(b"255.5B"), Err(ParseError));
    }
}
