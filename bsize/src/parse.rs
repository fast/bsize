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

use core::convert::TryFrom;
use core::fmt;
use core::str::FromStr;

use crate::BaseByteSize;
use crate::ByteSize;

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

macroweave::repeat!(Ty in [u8, u16, u32, u64, usize] {
    impl FromStr for ByteSize<Ty> {
        type Err = ParseError;

        #[inline]
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let src = s.as_bytes();
            let size = if src.len() <= MAX_RELEVANT_FRACTION_DIGITS {
                parse_size::<false>(src)?
            } else {
                parse_long_size(src)?
            };
            bsize_from_u64(size)
        }
    }
});

fn bsize_from_u64<T>(size: u64) -> Result<ByteSize<T>, ParseError>
where
    T: BaseByteSize + TryFrom<u64>,
{
    T::try_from(size)
        .map(ByteSize::b)
        .map_err(|_| ParseError::Overflow)
}

// Half-ceil transitions occur at (2n + 1) / (2 * multiplier). Supported multipliers factor only
// into 2s and 5s, so every transition terminates in decimal; 2^60 is the worst case at 61 digits.
// Keeping shorter inputs on the general path avoids adding dispatch overhead to normal parsing.
const MAX_RELEVANT_FRACTION_DIGITS: usize = 61;

// Keep the uncommon long-input code out of the monomorphization used by normal size strings. This
// preserves the original parser's code generation while allowing `FromStr` to inline the dispatch.
#[cold]
#[inline(never)]
fn parse_long_size(src: &[u8]) -> Result<u64, ParseError> {
    parse_size::<true>(src)
}

#[inline]
fn parse_size<const LIMIT_LONG_FRACTION: bool>(mut src: &[u8]) -> Result<u64, ParseError> {
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

    let mut multiplier = 1u64;
    if let [init @ .., b'i' | b'I'] = src {
        src = init;
        if let [init @ .., prefix] = src {
            match prefix {
                b'k' | b'K' => multiplier = 1 << 10,
                b'm' | b'M' => multiplier = 1 << 20,
                b'g' | b'G' => multiplier = 1 << 30,
                b't' | b'T' => multiplier = 1 << 40,
                b'p' | b'P' => multiplier = 1 << 50,
                b'e' | b'E' => multiplier = 1 << 60,
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
                    b'k' | b'K' => multiplier = 1_000,
                    b'm' | b'M' => multiplier = 1_000_000,
                    b'g' | b'G' => multiplier = 1_000_000_000,
                    b't' | b'T' => multiplier = 1_000_000_000_000,
                    b'p' | b'P' => multiplier = 1_000_000_000_000_000,
                    b'e' | b'E' => multiplier = 1_000_000_000_000_000_000,
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

    let mut integer = 0u64;
    let mut saw_digit = false;
    let mut fraction_start = None;

    for (index, b) in src.iter().copied().enumerate() {
        match b {
            b'0'..=b'9' => {
                saw_digit = true;
                integer = integer
                    .checked_mul(10)
                    .and_then(|v| v.checked_add(u64::from(b - b'0')))
                    .ok_or(ParseError::Overflow)?;
            }
            b'_' => {}
            b'.' if saw_digit => {
                fraction_start = Some(index + 1);
                break;
            }
            _ => return Err(ParseError::Malformed),
        }
    }

    if !saw_digit {
        return Err(ParseError::Empty);
    }

    let integer_bytes = integer.checked_mul(multiplier);

    if let Some(start) = fraction_start {
        let fraction = &src[start..];
        if LIMIT_LONG_FRACTION && fraction.len() > MAX_RELEVANT_FRACTION_DIGITS {
            return parse_long_fraction(fraction, multiplier, integer_bytes);
        }

        // Multiply the fraction by the unit multiplier from right to left in base 10.
        // Once all digits are consumed, carry is the integral byte count and the last remainder
        // digit determines rounding to the nearest byte.
        debug_assert!(multiplier <= u64::MAX / 10);
        let mut carry = 0u64;
        let mut rounding_digit = 0u64;
        for b in fraction.iter().copied().rev() {
            match b {
                b'0'..=b'9' => {
                    let product = u64::from(b - b'0') * multiplier + carry;
                    rounding_digit = product % 10;
                    carry = product / 10;
                }
                b'_' => {}
                _ => return Err(ParseError::Malformed),
            }
        }

        let mut bytes = integer_bytes.ok_or(ParseError::Overflow)?;
        let fraction = carry + u64::from(rounding_digit >= 5);
        bytes = bytes.checked_add(fraction).ok_or(ParseError::Overflow)?;

        return Ok(bytes);
    }

    integer_bytes.ok_or(ParseError::Overflow)
}

#[cold]
#[inline(never)]
fn parse_long_fraction(
    src: &[u8],
    multiplier: u64,
    integer_bytes: Option<u64>,
) -> Result<u64, ParseError> {
    // A half-byte boundary has a terminating decimal representation for every supported unit.
    // Digits after that representation cannot change half-ceil rounding, so validate them without
    // including them in the multiplication loop.
    let relevant_digits = if multiplier == 1 || !multiplier.is_power_of_two() {
        multiplier.ilog10() as usize + 1
    } else {
        multiplier.ilog2() as usize + 1
    };
    let mut digits = 0usize;
    let mut end = src.len();
    for (index, b) in src.iter().copied().enumerate() {
        match b {
            b'0'..=b'9' => {
                digits += 1;
                if digits == relevant_digits {
                    end = index + 1;
                }
            }
            b'_' => {}
            _ => return Err(ParseError::Malformed),
        }
    }

    debug_assert!(multiplier <= u64::MAX / 10);
    let mut carry = 0u64;
    let mut rounding_digit = 0u64;
    for b in src[..end].iter().copied().rev() {
        match b {
            b'0'..=b'9' => {
                let product = u64::from(b - b'0') * multiplier + carry;
                rounding_digit = product % 10;
                carry = product / 10;
            }
            b'_' => {}
            _ => unreachable!(),
        }
    }

    let mut bytes = integer_bytes.ok_or(ParseError::Overflow)?;
    let fraction = carry + u64::from(rounding_digit >= 5);
    bytes = bytes.checked_add(fraction).ok_or(ParseError::Overflow)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::string::String;
    use alloc::string::ToString;

    use super::*;

    fn assert_parse_ok(input: &str, expected: u64) {
        let actual = ByteSize::<u64>::from_str(input).unwrap();
        let expected = ByteSize::<u64>::b(expected);
        assert_eq!(actual, expected, "input: {input:?}");

        let round_trip = actual.to_string().parse::<ByteSize<u64>>().unwrap();
        assert_eq!(round_trip, expected, "input: {input:?}");
    }

    fn assert_parse_err(input: &str, expected: ParseError) {
        assert_eq!(
            input.parse::<ByteSize<u64>>(),
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
            ("0.1234567890123456789012", 0),
            ("1.84467440737095516155", 2),
            ("1.84467440737095516145 EB", 1_844_674_407_370_955_161),
            ("1.844674407370955161450 EB", 1_844_674_407_370_955_161),
            (
                "0.0000000000000000004336808689942017736029811203479766845703124 EiB",
                0,
            ),
            (
                "0.0000000000000000004336808689942017736029811203479766845703125 EiB",
                1,
            ),
            ("18_446_744_073_709_551_581", 18_446_744_073_709_551_581),
            ("18_446_744_073_709_551_615", u64::MAX),
            ("18.446_744_073_709_551_615 EB", u64::MAX),
            ("18.4467440737095516154 EB", u64::MAX),
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
            "19.aE",
            concat!(
                "1.0000000000000000000000000000000000000000000000000000000000000000",
                "x EB",
            ),
            concat!(
                "0.0004882812500000000000000000000000000000000000000000000000000000",
                "x KiB",
            ),
            "IB",
            "iB",
            "1iB",
            "1 ZiB",
            "1 YiB",
            "1e2 KIB",
            "1E+6",
            "\t1",
            "1\tKB",
        ] {
            assert_parse_err(input, ParseError::Malformed);
        }

        for input in [
            "18_446_744_073_709_551_616",
            "18_446_744_073_709_551_620",
            "18446744073709551615.5",
            "184467440737095516155",
            "18.446_744_073_709_551_616 EB",
            "18.4467440737095516155 EB",
            "19EB",
            "16EiB",
            "100000000000000000000",
        ] {
            assert_parse_err(input, ParseError::Overflow);
        }

        assert_eq!("256".parse::<ByteSize<u8>>(), Err(ParseError::Overflow));
        assert_eq!("64 KiB".parse::<ByteSize<u16>>(), Err(ParseError::Overflow));
        assert_eq!("4GiB".parse::<ByteSize<u32>>(), Err(ParseError::Overflow));
    }

    #[test]
    fn ignores_fractional_digits_that_cannot_affect_half_ceil() {
        assert_parse_ok("0.499999999999999999999999999999 B", 0);
        assert_parse_ok("0.500000000000000000000000000001 B", 1);
        assert_parse_ok("0.000488281249999999999999999999 KiB", 0);
        assert_parse_ok("0.000488281250000000000000000001 KiB", 1);
        assert_parse_ok(
            "0.000000000000000000433680868994201773602981120347976684570312499999999 EiB",
            0,
        );
        assert_parse_ok(
            "0.000000000000000000433680868994201773602981120347976684570312500000001 EiB",
            1,
        );
    }

    fn scale_fraction_reference(src: &[u8], multiplier: u64) -> u64 {
        let mut carry = 0u64;
        let mut rounding_digit = 0u64;
        for b in src.iter().copied().rev() {
            match b {
                b'0'..=b'9' => {
                    let product = u64::from(b - b'0') * multiplier + carry;
                    rounding_digit = product % 10;
                    carry = product / 10;
                }
                b'_' => {}
                _ => unreachable!(),
            }
        }
        carry + u64::from(rounding_digit >= 5)
    }

    quickcheck::quickcheck! {
        fn long_fractions_match_full_precision_reference(
            unit_index: u8,
            digits: alloc::vec::Vec<u8>
        ) -> bool {
            const UNITS: [(&str, u64); 13] = [
                ("B", 1),
                ("kB", 1_000),
                ("MB", 1_000_000),
                ("GB", 1_000_000_000),
                ("TB", 1_000_000_000_000),
                ("PB", 1_000_000_000_000_000),
                ("EB", 1_000_000_000_000_000_000),
                ("KiB", 1 << 10),
                ("MiB", 1 << 20),
                ("GiB", 1 << 30),
                ("TiB", 1 << 40),
                ("PiB", 1 << 50),
                ("EiB", 1 << 60),
            ];

            let (unit, multiplier) = UNITS[usize::from(unit_index) % UNITS.len()];
            let mut fraction = String::with_capacity(146);
            for index in 0..128 {
                if index > 0 && index % 7 == 0 {
                    fraction.push('_');
                }
                let digit = digits
                    .get(index % digits.len().max(1))
                    .copied()
                    .unwrap_or(index as u8)
                    % 10;
                fraction.push(char::from(b'0' + digit));
            }

            let expected = scale_fraction_reference(fraction.as_bytes(), multiplier);
            let input = format!("0.{fraction} {unit}");
            input.parse::<ByteSize<u64>>() == Ok(ByteSize::b(expected))
        }

        fn parses_eib_fractions_exactly(whole: u8, fraction: u64) -> bool {
            const MULTIPLIER: u128 = 1 << 60;
            const SCALE: u128 = 1_000_000_000_000_000_000;

            let whole = whole % 16;
            let fraction = fraction % SCALE as u64;
            let input = format!("{whole}.{fraction:018} EiB");
            let actual = input.parse::<ByteSize<u64>>();
            let expected = u128::from(whole) * MULTIPLIER
                + (u128::from(fraction) * MULTIPLIER + SCALE / 2) / SCALE;

            if expected > u128::from(u64::MAX) {
                actual == Err(ParseError::Overflow)
            } else {
                actual == Ok(ByteSize::b(u64::try_from(expected).unwrap()))
            }
        }

        fn fractional_trailing_zero_preserves_value(whole: u8, fraction: u64) -> bool {
            const SCALE: u64 = 1_000_000_000_000_000_000;

            let whole = whole % 16;
            let fraction = fraction % SCALE;
            let input = format!("{whole}.{fraction:018} EiB");
            let input_with_zero = format!("{whole}.{fraction:018}0 EiB");

            input.parse::<ByteSize<u64>>() == input_with_zero.parse::<ByteSize<u64>>()
        }
    }
}
