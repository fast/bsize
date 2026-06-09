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
use super::number::parse_number;

impl_parse!(u32, parse_u32);

#[cfg(target_pointer_width = "32")]
impl_parse!(usize, parse_usize);

fn parse_u32(mut src: &[u8]) -> Result<u32, ParseError> {
    let multiply = parse_unit(&mut src)?;

    parse_number(src, multiply, u32::MAX as u64).map(|value| value as u32)
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
        if super::is_ascii_unit(prefix) {
            return Err(ParseError);
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
