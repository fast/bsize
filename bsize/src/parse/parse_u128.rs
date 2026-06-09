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
use super::u256::U256;

impl_parse!(u128, parse_u128);
impl_parse_number!(u128, multiply_integer_u128, divide_integer_u128);

#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
impl_parse!(usize, parse_usize);

fn parse_u128(mut src: &[u8]) -> Result<u128, ParseError> {
    let multiply = parse_unit(&mut src)?;

    parse_number(src, multiply, u128::MAX)
}

#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
fn parse_usize(src: &[u8]) -> Result<usize, ParseError> {
    let value = parse_u128(src)?;
    if value > usize::MAX as u128 {
        Err(ParseError)
    } else {
        Ok(value as usize)
    }
}

fn parse_unit(src: &mut &[u8]) -> Result<u128, ParseError> {
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

fn decimal_factor(prefix: u8) -> Option<u128> {
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

fn binary_factor(prefix: u8) -> Option<u128> {
    Some(match prefix.to_ascii_uppercase() {
        b'K' => 1_u128 << 10,
        b'M' => 1_u128 << 20,
        b'G' => 1_u128 << 30,
        b'T' => 1_u128 << 40,
        b'P' => 1_u128 << 50,
        b'E' => 1_u128 << 60,
        _ => return None,
    })
}

fn multiply_integer_u128(
    mantissa: u128,
    multiply: u128,
    exponent: u32,
    max: u128,
) -> Result<u128, ParseError> {
    let Some(power) = 10_u128.checked_pow(exponent) else {
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

fn divide_integer_u128(
    mantissa: u128,
    multiply: u128,
    exponent: u32,
    max: u128,
) -> Result<u128, ParseError> {
    if let Some(result) = divide_integer_u128_fast(mantissa, multiply, exponent, max) {
        return result;
    }

    if exponent >= 58 {
        return Ok(0);
    }

    let mut value = U256::multiply(mantissa, multiply);
    let mut round = false;
    let mut idx = 0;

    while idx < exponent {
        let (quotient, remainder) = value.div_rem_10();
        value = quotient;
        round = remainder >= 5;
        idx += 1;
    }

    if round {
        let Some(rounded) = value.checked_add_one() else {
            return Err(ParseError);
        };
        value = rounded;
    }

    let Some(value) = value.try_into_u128() else {
        return Err(ParseError);
    };
    if value > max {
        Err(ParseError)
    } else {
        Ok(value)
    }
}

fn divide_integer_u128_fast(
    mantissa: u128,
    multiply: u128,
    exponent: u32,
    max: u128,
) -> Option<Result<u128, ParseError>> {
    let product = mantissa.checked_mul(multiply)?;

    if exponent >= 39 {
        return Some(Ok(0));
    }

    let Some(power) = 10_u128.checked_pow(exponent) else {
        return Some(Err(ParseError));
    };
    let quotient = product / power;
    let remainder = product % power;
    let value = if remainder >= power / 2 {
        let Some(value) = quotient.checked_add(1) else {
            return Some(Err(ParseError));
        };
        value
    } else {
        quotient
    };

    if value > max {
        Some(Err(ParseError))
    } else {
        Some(Ok(value))
    }
}
