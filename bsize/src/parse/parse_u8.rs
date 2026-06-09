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
        let bytes = src.as_ref();
        parse_u8(bytes).map(BSize)
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

    let mut value = 0u8;
    let mut valid = false;

    let len = src.len();
    for idx in 0..len {
        match src[idx] {
            b'_' => {}
            b'.' => {
                if idx + 1 < len {
                    match src[idx + 1] {
                        b'0'..=b'4' => {}
                        b'5'..=b'9' => {
                            value = value.checked_add(1).ok_or(ParseError(()))?;
                        }
                        _ => return Err(ParseError(())),
                    }
                    valid = true;
                }

                for idx in idx + 2..len {
                    if !matches!(src[idx], b'_' | b'0'..=b'9') {
                        return Err(ParseError(()));
                    }
                }
                break;
            }
            digit @ b'0'..=b'9' => {
                value = value.checked_mul(10).ok_or(ParseError(()))?;
                value = value.checked_add(digit - b'0').ok_or(ParseError(()))?;
                valid = true;
            }
            _ => return Err(ParseError(())),
        }
    }

    if valid {
        Ok(value)
    } else {
        Err(ParseError(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bytes() {
        assert_eq!(BSize::<u8>::parse(b"255B").unwrap(), BSize::<u8>(255));
    }

    #[test]
    fn parses_fractional_bytes() {
        assert_eq!(BSize::<u8>::parse(b"25.55B").unwrap(), BSize::<u8>(26));
        assert_eq!(BSize::<u8>::parse(b"255.4B").unwrap(), BSize::<u8>(u8::MAX));
    }

    #[test]
    fn rejects_units() {
        BSize::<u8>::parse(b"1KB").unwrap_err();
        BSize::<u8>::parse(b"1eB").unwrap_err();
        BSize::<u8>::parse(b"1EB").unwrap_err();
    }

    #[test]
    fn rejects_overflow() {
        BSize::<u8>::parse(b"256B").unwrap_err();
        BSize::<u8>::parse(b"0.001KB").unwrap_err();
        BSize::<u8>::parse(b"255.5B").unwrap_err();
    }
}
