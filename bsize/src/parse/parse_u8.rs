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
    let multiply = super::strip_unit_suffix(&mut src)?;
    if src.is_empty() {
        return Err(ParseError::Empty);
    }

    let mut value = 0u8;
    for idx in 0..src.len() {
        match src[idx] {
            b'_' => {}
            b'.' => {
                let mut frac = 1u64;
                for idx in idx + 1..src.len() {
                    match src[idx] {
                        b'_' => {}
                        n @ b'0'..=b'9' => {
                            frac *= 10;
                            let n = (n - b'0') as u64;
                            let n = n.checked_mul(multiply).ok_or(ParseError::PosOverflow)?;
                            // let n = n.div_euclid(division);
                        }
                        _ => return Err(ParseError::InvalidDigit),
                    }
                }
            }
            n @ b'0'..=b'9' => {
                value = value.checked_mul(10).ok_or(ParseError::PosOverflow)?;
                let n = (n - b'0') as u64;
                let n = n.checked_mul(multiply).ok_or(ParseError::PosOverflow)?;
                let n = u8::try_from(n).map_err(|_| ParseError::PosOverflow)?;
                value = value.checked_add(n).ok_or(ParseError::PosOverflow)?;
            }
            _ => return Err(ParseError::InvalidDigit),
        }
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bytes() {
        assert_eq!(BSize::<u8>::parse(b"255B").unwrap(), BSize::<u8>(255));
        assert_eq!(BSize::<u8>::parse(b"255 B").unwrap(), BSize::<u8>(255));
    }

    #[test]
    fn parses_fractional_bytes() {
        assert_eq!(BSize::<u8>::parse(b"25.55B").unwrap(), BSize::<u8>(26));
        assert_eq!(BSize::<u8>::parse(b"255.4B").unwrap(), BSize::<u8>(u8::MAX));
    }

    #[test]
    fn rejects_overflow() {
        assert_eq!(BSize::<u8>::parse(b"1KB"), Err(ParseError::PosOverflow));
        assert_eq!(BSize::<u8>::parse(b"1eB"), Err(ParseError::PosOverflow));
        assert_eq!(BSize::<u8>::parse(b"1EB"), Err(ParseError::PosOverflow));
        assert_eq!(BSize::<u8>::parse(b"256B"), Err(ParseError::PosOverflow));
        assert_eq!(BSize::<u8>::parse(b"0.001KB"), Err(ParseError::PosOverflow));
        assert_eq!(BSize::<u8>::parse(b"255.5B"), Err(ParseError::PosOverflow));
    }
}
