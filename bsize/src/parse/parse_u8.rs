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
use core::num::IntErrorKind;
use core::str::FromStr;

impl FromStr for BSize<u8> {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let mut src = src;
        let multiply = super::strip_unit_suffix(&mut src)?;

        let mut dot = false;
        let mut offset = 0;
        for ch in src.chars() {
            match ch {
                '.' => {
                    dot = true;
                    offset += 1;
                }
                '_' | '0'..='9' => offset += 1,
                _ => return Err(ParseError::Malformed),
            }
        }

        if dot {
            let n = match f64::from_str(&src[..offset]) {
                Ok(n) => n,
                Err(_) => return Err(ParseError::Malformed),
            };

            let n = n * (multiply as f64);
            let n = core::f64::math::round();
        } else {
            let n = match u64::from_str(&src[..offset]) {
                Ok(n) => n,
                Err(err) => {
                    return Err(match err.kind() {
                        IntErrorKind::PosOverflow => ParseError::Overflow,
                        IntErrorKind::Empty => ParseError::Empty,
                        _ => ParseError::Malformed,
                    });
                }
            };
        }

        Ok(BSize(0))
    }
}

fn parse_u8(mut src: &[u8]) -> Result<u8, ParseError> {
    let multiply = super::strip_unit_suffix(&mut src)?;
    while let [init @ .., b' '] = src {
        src = init;
    }
    if src.is_empty() {
        return Err(ParseError::Empty);
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
        assert_eq!(BSize::<u8>::parse(b"1KB"), Err(ParseError::Overflow));
        assert_eq!(BSize::<u8>::parse(b"1eB"), Err(ParseError::Overflow));
        assert_eq!(BSize::<u8>::parse(b"1EB"), Err(ParseError::Overflow));
        assert_eq!(BSize::<u8>::parse(b"256B"), Err(ParseError::Overflow));
        assert_eq!(BSize::<u8>::parse(b"0.001KB"), Err(ParseError::Overflow));
        assert_eq!(BSize::<u8>::parse(b"255.5B"), Err(ParseError::Overflow));
    }
}
