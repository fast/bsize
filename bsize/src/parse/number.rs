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

pub(in crate::parse) trait ParseNumber: Copy {
    const ZERO: Self;

    fn append_digit(self, digit: u8) -> Option<Self>;
    fn round_overflowed(self, digit: u8) -> Self;
    fn multiply_integer(self, multiply: Self, exponent: u32, max: Self)
    -> Result<Self, ParseError>;
    fn divide_integer(self, multiply: Self, exponent: u32, max: Self) -> Result<Self, ParseError>;
}

pub(in crate::parse) fn parse_number<N: ParseNumber>(
    src: &[u8],
    multiply: N,
    max: N,
) -> Result<N, ParseError> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum State {
        Empty,
        Integer,
        IntegerOverflow,
        Fraction,
        FractionOverflow,
        PosExponent,
        NegExponent,
    }

    let mut mantissa = N::ZERO;
    let mut fractional_exponent = 0_i32;
    let mut exponent = 0_i32;
    let mut has_exponent_digit = false;
    let mut state = State::Empty;

    for b in src {
        match (state, *b) {
            (State::Integer | State::Empty, b'0'..=b'9') => {
                if let Some(next) = mantissa.append_digit(*b) {
                    mantissa = next;
                    state = State::Integer;
                } else {
                    mantissa = mantissa.round_overflowed(*b);
                    fractional_exponent = fractional_exponent.saturating_add(1);
                    state = State::IntegerOverflow;
                }
            }
            (State::IntegerOverflow, b'0'..=b'9') => {
                fractional_exponent = fractional_exponent.saturating_add(1);
            }
            (State::Fraction, b'0'..=b'9') => {
                if let Some(next) = mantissa.append_digit(*b) {
                    mantissa = next;
                    fractional_exponent = fractional_exponent.saturating_sub(1);
                } else {
                    mantissa = mantissa.round_overflowed(*b);
                    state = State::FractionOverflow;
                }
            }
            (State::PosExponent, b'0'..=b'9') => {
                exponent = append_exponent_digit(exponent, *b, true)?;
                has_exponent_digit = true;
            }
            (State::NegExponent, b'0'..=b'9') => {
                exponent = append_exponent_digit(exponent, *b, false)?;
                has_exponent_digit = true;
            }
            (_, b'_' | b' ')
            | (State::PosExponent, b'+')
            | (State::FractionOverflow, b'0'..=b'9') => {}
            (
                State::Integer | State::Fraction | State::IntegerOverflow | State::FractionOverflow,
                b'e' | b'E',
            ) => {
                state = State::PosExponent;
                has_exponent_digit = false;
            }
            (State::PosExponent, b'-') => state = State::NegExponent,
            (State::Integer, b'.') => state = State::Fraction,
            (State::IntegerOverflow, b'.') => state = State::FractionOverflow,
            _ => return Err(ParseError),
        }
    }

    if state == State::Empty {
        return Err(ParseError);
    }
    if matches!(state, State::PosExponent | State::NegExponent) && !has_exponent_digit {
        return Err(ParseError);
    }

    let exponent = exponent.saturating_add(fractional_exponent);
    if exponent >= 0 {
        mantissa.multiply_integer(multiply, exponent.unsigned_abs(), max)
    } else {
        mantissa.divide_integer(multiply, exponent.unsigned_abs(), max)
    }
}

fn append_exponent_digit(exponent: i32, digit: u8, positive: bool) -> Result<i32, ParseError> {
    let digit = (digit - b'0') as i32;
    if positive {
        let Some(exponent) = exponent.checked_mul(10) else {
            return Err(ParseError);
        };
        let Some(exponent) = exponent.checked_add(digit) else {
            return Err(ParseError);
        };

        Ok(exponent)
    } else {
        let Some(exponent) = exponent.checked_mul(10) else {
            return Ok(i32::MIN);
        };
        let Some(exponent) = exponent.checked_sub(digit) else {
            return Ok(i32::MIN);
        };

        Ok(exponent)
    }
}
