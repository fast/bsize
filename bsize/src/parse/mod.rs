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

mod parse_u16;
mod parse_u32;
mod parse_u64;
mod parse_u8;

use core::fmt;

/// The error returned when parsing a byte size fails.
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ParseError {
    /// The input contains no number.
    Empty,
    /// The input contains malformed bytes.
    Malformed,
    /// The parsed byte count is too large for the target integer type.
    Overflow,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Empty => "cannot parse integer from empty string",
            Self::Malformed => "malformed bytes found in string",
            Self::Overflow => "number too large to fit in target type",
        })
    }
}

impl core::error::Error for ParseError {}

fn strip_unit_suffix(src: &mut &str) -> Result<u64, ParseError> {
    let mut strip = src.len();
    let mut multiply = 1;

    let mut unit = src.as_bytes();
    if let [init @ .., b'b' | b'B'] = unit {
        unit = init;
        strip -= 1;
    };
    if let [init @ .., b'i' | b'I'] = unit {
        unit = init;
        strip -= 1;
        if let [.., prefix] = unit {
            match prefix {
                b'k' | b'K' => multiply = 1 << 10,
                b'm' | b'M' => multiply = 1 << 20,
                b'g' | b'G' => multiply = 1 << 30,
                b't' | b'T' => multiply = 1 << 40,
                b'p' | b'P' => multiply = 1 << 50,
                b'e' | b'E' => multiply = 1 << 60,
                _ => return Err(ParseError::Malformed),
            }
        } else {
            // [iI][bB] is not a valid suffix.
            return Err(ParseError::Malformed);
        }
    } else {
        if let [.., prefix] = unit {
            'outer: {
                match prefix {
                    b'k' | b'K' => multiply = 1_000,
                    b'm' | b'M' => multiply = 1_000_000,
                    b'g' | b'G' => multiply = 1_000_000_000,
                    b't' | b'T' => multiply = 1_000_000_000_000,
                    b'p' | b'P' => multiply = 1_000_000_000_000_000,
                    b'e' | b'E' => multiply = 1_000_000_000_000_000_000,
                    _ => break 'outer,
                }
                strip -= 1;
            }
        }
    }

    *src = &src[..strip];
    Ok(multiply)
}
