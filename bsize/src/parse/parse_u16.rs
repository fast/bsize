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

impl_parse!(u16, parse_u16);
impl_parse_number!(u32, multiply_integer_u32, divide_integer_u32);

#[cfg(target_pointer_width = "16")]
impl_parse!(usize, parse_usize);

fn parse_u16(mut src: &[u8]) -> Result<u16, ParseError> {
    let multiply = parse_unit(&mut src)?;

    parse_number(src, multiply, u16::MAX as u32).map(|value| value as u16)
}

#[cfg(target_pointer_width = "16")]
fn parse_usize(src: &[u8]) -> Result<usize, ParseError> {
    parse_u16(src).map(|value| value as usize)
}

fn parse_unit(src: &mut &[u8]) -> Result<u32, ParseError> {
    super::strip_b_suffix(src)?;

    if let Some((&infix, before_i)) = src.split_last() {
        if infix.eq_ignore_ascii_case(&b'i') {
            let Some((&prefix, before_prefix)) = before_i.split_last() else {
                return Err(ParseError);
            };
            if prefix.eq_ignore_ascii_case(&b'K') {
                *src = before_prefix;
                return Ok(1_024);
            }

            return Err(ParseError);
        }
    }

    if let Some((&prefix, before_prefix)) = src.split_last() {
        if prefix.eq_ignore_ascii_case(&b'K') {
            *src = before_prefix;
            return Ok(1_000);
        }
        if super::is_ascii_unit(prefix) {
            return Err(ParseError);
        }
    }

    Ok(1)
}

fn multiply_integer_u32(
    mantissa: u32,
    multiply: u32,
    exponent: u32,
    max: u32,
) -> Result<u32, ParseError> {
    let Some(power) = 10_u32.checked_pow(exponent) else {
        return Err(ParseError);
    };
    let Some(multiply) = multiply.checked_mul(power) else {
        return Err(ParseError);
    };
    let Some(value) = mantissa.checked_mul(multiply) else {
        return Err(ParseError);
    };

    if value > max {
        Err(ParseError)
    } else {
        Ok(value)
    }
}

fn divide_integer_u32(
    mantissa: u32,
    multiply: u32,
    exponent: u32,
    max: u32,
) -> Result<u32, ParseError> {
    if exponent >= 13 {
        return Ok(0);
    }

    let product = (mantissa as u64) * (multiply as u64);
    let power = 10_u64.pow(exponent);
    let quotient = product / power;
    let remainder = product % power;
    let value = if exponent != 0 && remainder >= power / 2 {
        let Some(value) = quotient.checked_add(1) else {
            return Err(ParseError);
        };
        value
    } else {
        quotient
    };

    if value > max as u64 {
        Err(ParseError)
    } else {
        Ok(value as u32)
    }
}
