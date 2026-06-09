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

/// The error returned when parsing a byte size fails.
#[derive(Debug, Clone)]
pub struct ParseError(());

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("malformed byte size")
    }
}

impl core::error::Error for ParseError {}

mod parse_u128;
mod parse_u16;
mod parse_u32;
mod parse_u64;
mod parse_u8;

fn strip_b_suffix(src: &mut &[u8]) -> Result<(), ParseError> {
    *src = src.trim_ascii();
    let (suffix, before_b) = src.split_last().ok_or(ParseError(()))?;
    if suffix.to_ascii_uppercase() != b'B' {
        return Err(ParseError(()));
    }
    *src = before_b.trim_ascii_end();
    Ok(())
}

fn is_ascii_unit(byte: u8) -> bool {
    matches!(
        byte.to_ascii_uppercase(),
        b'K' | b'M' | b'G' | b'T' | b'P' | b'E'
    )
}
