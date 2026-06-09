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

mod parse_u128;
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

fn strip_unit_suffix(src: &mut &[u8]) -> Result<u64, ParseError> {
    if let [init @ .., b'b' | b'B'] = src {
        *src = init;
    };

    let mut multiply = 1;

    if let [init @ .., b'i' | b'I'] = src {
        *src = init;
        if let [init @ .., prefix] = src {
            match prefix {
                b'k' | b'K' => multiply = 1 << 10,
                b'm' | b'M' => multiply = 1 << 20,
                b'g' | b'G' => multiply = 1 << 30,
                b't' | b'T' => multiply = 1 << 40,
                b'p' | b'P' => multiply = 1 << 50,
                b'e' | b'E' => multiply = 1 << 60,
                _ => return Err(ParseError::InvalidDigit),
            }

            *src = init;
        } else {
            // [iI][bB] is not a valid suffix.
            return Err(ParseError::InvalidDigit);
        }
    } else {
        if let [init @ .., prefix] = src {
            'skip: {
                match prefix {
                    b'k' | b'K' => multiply = 1_000,
                    b'm' | b'M' => multiply = 1_000_000,
                    b'g' | b'G' => multiply = 1_000_000_000,
                    b't' | b'T' => multiply = 1_000_000_000_000,
                    b'p' | b'P' => multiply = 1_000_000_000_000_000,
                    b'e' | b'E' => multiply = 1_000_000_000_000_000_000,
                    _ => break 'skip,
                }
                *src = init;
            }
        }
    }

    while let [init @ .., b' '] = src {
        *src = init;
    }

    Ok(multiply)
}
