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

impl_parse!(u8, parse_u8);
impl_parse_number!(u16, multiply_integer_u16, divide_integer_u16);

fn parse_u8(mut src: &[u8]) -> Result<u8, ParseError> {
    super::strip_b_suffix(&mut src)?;

    parse_number(src, 1_u16, u8::MAX as u16).map(|value| value as u8)
}

fn multiply_integer_u16(
    mantissa: u16,
    multiply: u16,
    exponent: u32,
    max: u16,
) -> Result<u16, ParseError> {
    let Some(power) = 10_u16.checked_pow(exponent) else {
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

fn divide_integer_u16(
    mantissa: u16,
    multiply: u16,
    exponent: u32,
    max: u16,
) -> Result<u16, ParseError> {
    if exponent >= 6 {
        return Ok(0);
    }

    let product = (mantissa as u32) * (multiply as u32);
    let power = 10_u32.pow(exponent);
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

    if value > max as u32 {
        Err(ParseError)
    } else {
        Ok(value as u16)
    }
}
