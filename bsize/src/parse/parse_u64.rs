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

impl_parse!(u64, parse_u64);
impl_parse_number!(u64, multiply_integer_u64, divide_integer_u64);

#[cfg(target_pointer_width = "64")]
impl_parse!(usize, parse_usize);

fn parse_u64(mut src: &[u8]) -> Result<u64, ParseError> {
    let multiply = parse_unit(&mut src)?;

    parse_number(src, multiply, u64::MAX)
}

#[cfg(target_pointer_width = "64")]
fn parse_usize(src: &[u8]) -> Result<usize, ParseError> {
    parse_u64(src).map(|value| value as usize)
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
    }

    Ok(1)
}

fn decimal_factor(prefix: u8) -> Option<u64> {
    Some(match prefix.to_ascii_uppercase() {
        b'K' => 1_000,
        b'M' => 1_000_000,
        b'G' => 1_000_000_000,
        b'T' => 1_000_000_000_000,
        b'P' => 1_000_000_000_000_000,
        b'E' => 1_000_000_000_000_000_000,
        _ => return None,
    })
}

fn binary_factor(prefix: u8) -> Option<u64> {
    Some(match prefix.to_ascii_uppercase() {
        b'K' => 1_u64 << 10,
        b'M' => 1_u64 << 20,
        b'G' => 1_u64 << 30,
        b'T' => 1_u64 << 40,
        b'P' => 1_u64 << 50,
        b'E' => 1_u64 << 60,
        _ => return None,
    })
}

fn multiply_integer_u64(
    mantissa: u64,
    multiply: u64,
    exponent: u32,
    max: u64,
) -> Result<u64, ParseError> {
    let Some(power) = 10_u64.checked_pow(exponent) else {
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

fn divide_integer_u64(
    mantissa: u64,
    multiply: u64,
    exponent: u32,
    max: u64,
) -> Result<u64, ParseError> {
    let product = (mantissa as u128) * (multiply as u128);
    divide_integer_u128_product(product, exponent, max as u128).map(|value| value as u64)
}

fn divide_integer_u128_product(
    product: u128,
    exponent: u32,
    max: u128,
) -> Result<u128, ParseError> {
    if exponent > 38 {
        return Ok(0);
    }

    let power = 10_u128.pow(exponent);
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

    if value > max {
        Err(ParseError)
    } else {
        Ok(value)
    }
}
