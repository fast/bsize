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
use core::str::FromStr;

use crate::BSize;

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

macro_rules! impl_from_str {
    ($($ty:ty),* $(,)?) => {
        $(
            impl FromStr for BSize<$ty> {
                type Err = ParseError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let size = parse_size(s.as_bytes())?;
                    if size <= <$ty>::MAX as u64 {
                        Ok(BSize(size as $ty))
                    } else {
                        Err(ParseError::Overflow)
                    }
                }
            }
        )*
    };
}

impl_from_str!(u8, u16, u32, u64, usize);

// This is derived from `parse-size` [1].
//
// [1]: https://github.com/kennytm/parse-size/blob/8f2bc5a8/src/lib.rs#L364-L495
fn parse_size(mut src: &[u8]) -> Result<u64, ParseError> {
    // trim starting and trailing spaces
    while let [b' ', init @ ..] = src {
        src = init;
    }
    while let [init @ .., b' '] = src {
        src = init;
    }

    // trim trailing 'b' or 'B'
    if let [init @ .., b'b' | b'B'] = src {
        src = init;
    };

    let mut multiply = 1u64;
    if let [init @ .., b'i' | b'I'] = src {
        src = init;
        if let [init @ .., prefix] = src {
            match prefix {
                b'k' | b'K' => multiply = 1 << 10,
                b'm' | b'M' => multiply = 1 << 20,
                b'g' | b'G' => multiply = 1 << 30,
                b't' | b'T' => multiply = 1 << 40,
                b'p' | b'P' => multiply = 1 << 50,
                b'e' | b'E' => multiply = 1 << 60,
                _ => return Err(ParseError::Malformed),
            }

            src = init;
        } else {
            // [iI][bB] is malformed suffix.
            return Err(ParseError::Malformed);
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
                src = init;
            }
        }
    }

    // trim spaces between numeric part and unit part
    while let [init @ .., b' '] = src {
        src = init;
    }

    macro_rules! append_digit {
        ($before:expr, $method:ident, $digit_char:expr) => {
            $before
                .checked_mul(10)
                .and_then(|v| v.$method(($digit_char - b'0').into()))
        };
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum ParseState {
        Empty,
        Integer,
        IntegerOverflow,
        Fraction,
        FractionOverflow,
    }

    let mut mantissa = 0u64;
    let mut exponent = 0i32;
    let mut state = ParseState::Empty;

    for b in src {
        match (state, *b) {
            (ParseState::Integer | ParseState::Empty, b'0'..=b'9') => {
                if let Some(m) = append_digit!(mantissa, checked_add, *b) {
                    mantissa = m;
                    state = ParseState::Integer;
                } else {
                    if *b >= b'5' {
                        mantissa += 1;
                    }
                    state = ParseState::IntegerOverflow;
                    exponent += 1;
                }
            }
            (ParseState::IntegerOverflow, b'0'..=b'9') => {
                exponent += 1;
            }
            (ParseState::Fraction, b'0'..=b'9') => {
                if let Some(m) = append_digit!(mantissa, checked_add, *b) {
                    mantissa = m;
                    exponent -= 1;
                } else {
                    if *b >= b'5' {
                        mantissa += 1;
                    }
                    state = ParseState::FractionOverflow;
                }
            }
            (_, b'_') => {}
            (ParseState::Integer, b'.') => state = ParseState::Fraction,
            (ParseState::IntegerOverflow, b'.') => state = ParseState::FractionOverflow,
            _ => return Err(ParseError::Malformed),
        }
    }

    if matches!(state, ParseState::Empty) {
        return Err(ParseError::Empty);
    }

    let abs_exponent = exponent.unsigned_abs();
    if exponent >= 0 {
        let power = 10_u64
            .checked_pow(abs_exponent)
            .ok_or(ParseError::Overflow)?;
        let multiply = multiply.checked_mul(power).ok_or(ParseError::Overflow)?;
        mantissa.checked_mul(multiply).ok_or(ParseError::Overflow)
    } else if exponent >= -38 {
        let power = 10_u128.pow(abs_exponent);
        let result = (u128::from(mantissa) * u128::from(multiply) + power / 2) / power;
        u64::try_from(result).map_err(|_| ParseError::Overflow)
    } else {
        // (2^128) * 1e-39 < 1, always, and thus saturate to 0.
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;
    
    use alloc::string::ToString;

    use super::*;

    fn assert_parse_ok(input: &str, expected: u64) {
        let actual = BSize::<u64>::from_str(input).unwrap();
        let expected = BSize::<u64>(expected);
        assert_eq!(actual, expected, "input: {input:?}");

        let round_trip = actual.to_string().parse::<BSize<u64>>().unwrap();
        assert_eq!(round_trip, expected, "input: {input:?}");
    }

    fn assert_parse_err(input: &str, expected: ParseError) {
        assert_eq!(
            input.parse::<BSize<u64>>(),
            Err(expected),
            "input: {input:?}",
        );
    }

    #[test]
    fn test_parse_ok() {
        for (input, expected) in [
            ("0", 0),
            ("3", 3),
            ("30", 30),
            ("32", 32),
            ("500", 500),
            ("_5_", 5),
            ("1_234_567", 1_234_567),
            (" 42 ", 42),
            ("1B", 1),
            ("1 b", 1),
            ("1kB", 1_000),
            ("1K", 1_000),
            ("1KB", 1_000),
            ("2MB", 2_000_000),
            ("3GB", 3_000_000_000),
            ("4TB", 4_000_000_000_000),
            ("5PB", 5_000_000_000_000_000),
            ("6EB", 6_000_000_000_000_000_000),
            ("8P", 8_000_000_000_000_000),
            ("1Ki", 1 << 10),
            ("1KiB", 1 << 10),
            ("1.5Ki", 1_536),
            ("1.5KiB", 1_536),
            ("7 KiB", 7 << 10),
            ("8 MiB", 8 << 20),
            ("9 GiB", 9 << 30),
            ("10 TiB", 10 << 40),
            ("11 PiB", 11 << 50),
            ("12 EiB", 12 << 60),
            ("  7 KiB  ", 7 << 10),
            ("1mib", 1_048_576),
            ("1.1 K", 1_100),
            ("1.2345 K", 1_235),
            ("1.2345m", 1_234_500),
            ("5.k", 5_000),
            ("0.0024KB", 2),
            ("0.0025KB", 3),
            ("0.4B", 0),
            ("0.5B", 1),
            ("18_446_744_073_709_551_581", 18_446_744_073_709_551_581),
            ("18_446_744_073_709_551_615", u64::MAX),
            ("18.446_744_073_709_551_615 EB", u64::MAX),
            ("1.000_000_000_000_000_001 EB", 1_000_000_000_000_000_001),
        ] {
            assert_parse_ok(input, expected);
        }
    }

    #[test]
    fn test_parse_err() {
        for input in ["", " ", "  ", "__", "k", "kb", "KiB"] {
            assert_parse_err(input, ParseError::Empty);
        }

        for input in [
            ".",
            ".5k",
            "a",
            "a124GB",
            "-1",
            "1,5",
            "1 234 567",
            "1 000 B",
            "1.3 42.0 B",
            "1.3 ... B",
            "IB",
            "iB",
            "1iB",
            "1 ZiB",
            "1 YiB",
            "1e2 KIB",
            "1E+6",
            "0.1234567890123456789012",
            "\t1",
            "1\tKB",
        ] {
            assert_parse_err(input, ParseError::Malformed);
        }

        for input in [
            "18_446_744_073_709_551_616",
            "18_446_744_073_709_551_620",
            "18.446_744_073_709_551_616 EB",
            "19EB",
            "16EiB",
            "100000000000000000000",
        ] {
            assert_parse_err(input, ParseError::Overflow);
        }

        assert_eq!("256".parse::<BSize<u8>>(), Err(ParseError::Overflow));
        assert_eq!("64 KiB".parse::<BSize<u16>>(), Err(ParseError::Overflow));
        assert_eq!("4GiB".parse::<BSize<u32>>(), Err(ParseError::Overflow));
    }
}
