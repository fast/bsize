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

/// The mode used to round a fractional byte count to a whole number of bytes.
///
/// Parsing only accepts non-negative byte sizes, so these variants cover the distinct behaviors
/// relevant here without separate zero-oriented aliases such as truncation or expansion.
///
/// [`RoundMode::HalfCeil`] is the default used by [`ParseOptions`] and [`core::str::FromStr`]. Use
/// [`ByteSize::parse_with`] to select another mode.
///
/// # Examples
///
/// ```
/// use bsize::BSize64;
/// use bsize::ParseOptions;
/// use bsize::RoundMode;
///
/// let mut options = ParseOptions::default();
/// options.round_mode = RoundMode::HalfEven;
/// assert_eq!(
///     BSize64::b(2),
///     BSize64::parse_with("2.5 B", options).unwrap(),
/// );
/// options.round_mode = RoundMode::HalfCeil;
/// assert_eq!(
///     BSize64::b(3),
///     BSize64::parse_with("2.5 B", options).unwrap(),
/// );
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum RoundMode {
    /// Rounds toward the larger whole byte count.
    Ceil,
    /// Rounds toward the smaller whole byte count, discarding any fractional byte.
    Floor,
    /// Rounds to the nearest whole byte, with ties toward the larger byte count.
    ///
    /// This is the default used by [`core::str::FromStr`].
    HalfCeil,
    /// Rounds to the nearest whole byte, with ties toward the smaller byte count.
    HalfFloor,
    /// Rounds to the nearest whole byte, with ties toward the even byte count.
    HalfEven,
}

impl RoundMode {
    const fn needs_trailing_nonzero(self) -> bool {
        matches!(self, Self::Ceil | Self::HalfFloor | Self::HalfEven)
    }
}

/// Options that control byte size parsing.
///
/// Use [`ParseOptions::default`] for the standard parsing behavior, then update fields to select
/// different behavior. Pass the options to [`ByteSize::parse_with`].
///
/// # Examples
///
/// ```
/// use bsize::BSize64;
/// use bsize::ParseOptions;
/// use bsize::RoundMode;
///
/// let mut options = ParseOptions::default();
/// options.round_mode = RoundMode::Floor;
///
/// assert_eq!(
///     BSize64::b(1),
///     BSize64::parse_with("1.9 B", options).unwrap()
/// );
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct ParseOptions {
    /// The mode used to round a fractional byte count to a whole number of bytes.
    ///
    /// Defaults to [`RoundMode::HalfCeil`].
    pub round_mode: RoundMode,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            round_mode: RoundMode::HalfCeil,
        }
    }
}

/// The error returned when parsing a byte size fails.
#[derive(Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ParseError {
    /// The input contains no number.
    Empty,
    /// The input contains malformed bytes.
    Malformed,
    /// The resulting byte count is too large for the target integer type.
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

impl<T> ByteSize<T>
where
    T: BaseByteSize + TryFrom<u64>,
{
    /// Parses a byte size using the given options.
    ///
    /// The unit multiplier is applied before the resulting value is rounded once to a whole number
    /// of bytes. Decimal fractions are evaluated exactly without first converting them to floating
    /// point. Overflow is checked after rounding, both against `u64` and against the integer type
    /// backing this [`ByteSize`].
    ///
    /// Use the standard [`core::str::FromStr`] implementation when [`ParseOptions::default`] is
    /// sufficient.
    ///
    /// # Examples
    ///
    /// ```
    /// use bsize::BSize8;
    /// use bsize::ParseOptions;
    /// use bsize::RoundMode;
    ///
    /// let mut options = ParseOptions::default();
    /// options.round_mode = RoundMode::Floor;
    /// assert_eq!(BSize8::b(1), BSize8::parse_with("1.9 B", options).unwrap());
    /// options.round_mode = RoundMode::Ceil;
    /// assert_eq!(BSize8::b(2), BSize8::parse_with("1.1 B", options).unwrap());
    /// ```
    pub fn parse_with(s: &str, options: ParseOptions) -> Result<Self, ParseError> {
        let mode = options.round_mode;
        let size = if mode.needs_trailing_nonzero() {
            parse_size::<true>(s.as_bytes(), mode)?
        } else {
            parse_size::<false>(s.as_bytes(), mode)?
        };
        bsize_from_u64(size)
    }
}

macroweave::repeat!(Ty in [u8, u16, u32, u64, usize] {
    impl FromStr for ByteSize<Ty> {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::parse_with(s, ParseOptions::default())
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

fn parse_size<const TRACK_TRAILING_NONZERO: bool>(
    mut src: &[u8],
    mode: RoundMode,
) -> Result<u64, ParseError> {
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
                if fraction_start.is_none() {
                    integer = integer
                        .checked_mul(10)
                        .and_then(|v| v.checked_add(u64::from(b - b'0')))
                        .ok_or(ParseError::Overflow)?;
                }
            }
            b'_' => {}
            b'.' if saw_digit && fraction_start.is_none() => {
                fraction_start = Some(index + 1);
            }
            _ => return Err(ParseError::Malformed),
        }
    }

    if !saw_digit {
        return Err(ParseError::Empty);
    }

    let mut bytes = integer
        .checked_mul(multiplier)
        .ok_or(ParseError::Overflow)?;

    if let Some(start) = fraction_start {
        // Multiply the fraction by the unit multiplier from right to left in base 10.
        // Once all fractional digits are consumed, carry is the integral byte count and
        // the last remainder digit determines rounding to the nearest byte.
        debug_assert!(multiplier <= u64::MAX / 10);
        let mut carry = 0u64;
        let mut rounding_digit = 0u64;
        let mut trailing_nonzero = false;
        for b in src[start..].iter().copied().rev() {
            if b == b'_' {
                continue;
            }

            let product = u64::from(b - b'0') * multiplier + carry;
            if TRACK_TRAILING_NONZERO {
                trailing_nonzero |= rounding_digit != 0;
            }
            rounding_digit = product % 10;
            carry = product / 10;
        }

        bytes = bytes.checked_add(carry).ok_or(ParseError::Overflow)?;
        let round_up = should_round_up(mode, bytes, rounding_digit, trailing_nonzero);
        bytes = bytes
            .checked_add(u64::from(round_up))
            .ok_or(ParseError::Overflow)?;
    }

    Ok(bytes)
}

fn should_round_up(
    mode: RoundMode,
    lower: u64,
    rounding_digit: u64,
    trailing_nonzero: bool,
) -> bool {
    let has_remainder = rounding_digit != 0 || trailing_nonzero;
    let greater_than_half = rounding_digit > 5 || (rounding_digit == 5 && trailing_nonzero);
    let tie = rounding_digit == 5 && !trailing_nonzero;

    match mode {
        RoundMode::Ceil => has_remainder,
        RoundMode::Floor => false,
        RoundMode::HalfCeil => greater_than_half || tie,
        RoundMode::HalfFloor => greater_than_half,
        RoundMode::HalfEven => greater_than_half || (tie && lower % 2 == 1),
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::string::ToString;

    use super::*;

    const ROUND_MODES: [RoundMode; 5] = [
        RoundMode::Ceil,
        RoundMode::Floor,
        RoundMode::HalfCeil,
        RoundMode::HalfFloor,
        RoundMode::HalfEven,
    ];

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

    fn assert_rounds(input: &str, mode: RoundMode, expected: u64) {
        let options = ParseOptions { round_mode: mode };
        let actual = ByteSize::<u64>::parse_with(input, options).unwrap();
        assert_eq!(
            actual,
            ByteSize::b(expected),
            "input: {input:?}, mode: {mode:?}"
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
    fn fractional_values_round_half_ceil() {
        for (input, expected) in [
            ("0.499 B", 0),
            ("0.5 B", 1),
            ("0.501 B", 1),
            ("1.499 B", 1),
            ("1.5 B", 2),
            ("2.5 B", 3),
            ("0.0004 kB", 0),
            ("0.0005 kB", 1),
            ("0.0006 kB", 1),
            ("0.00048828125 KiB", 1),
        ] {
            assert_parse_ok(input, expected);
        }
    }

    #[test]
    fn supports_all_rounding_modes() {
        assert_rounds("1 B", RoundMode::Ceil, 1);
        assert_rounds("1.0000000000000000001 B", RoundMode::Ceil, 2);
        assert_rounds("1.9 B", RoundMode::Ceil, 2);

        assert_rounds("1 B", RoundMode::Floor, 1);
        assert_rounds("1.1 B", RoundMode::Floor, 1);
        assert_rounds("1.9999999999999999999 B", RoundMode::Floor, 1);

        assert_rounds("1.4999999999999999999 B", RoundMode::HalfCeil, 1);
        assert_rounds("1.5 B", RoundMode::HalfCeil, 2);
        assert_rounds("1.5000000000000000001 B", RoundMode::HalfCeil, 2);

        assert_rounds("1.4999999999999999999 B", RoundMode::HalfFloor, 1);
        assert_rounds("1.5 B", RoundMode::HalfFloor, 1);
        assert_rounds("1.5000000000000000001 B", RoundMode::HalfFloor, 2);

        assert_rounds("0.5 B", RoundMode::HalfEven, 0);
        assert_rounds("1.5 B", RoundMode::HalfEven, 2);
        assert_rounds("2.5 B", RoundMode::HalfEven, 2);
        assert_rounds("3.5 B", RoundMode::HalfEven, 4);
        assert_rounds("2.5000000000000000001 B", RoundMode::HalfEven, 3);
    }

    #[test]
    fn applies_units_before_rounding() {
        for input in ["0.0005 kB", "0.00048828125 KiB"] {
            assert_rounds(input, RoundMode::HalfCeil, 1);
            assert_rounds(input, RoundMode::HalfFloor, 0);
            assert_rounds(input, RoundMode::HalfEven, 0);
        }

        assert_rounds("0.0015 kB", RoundMode::HalfEven, 2);
        assert_rounds("0.0025 kB", RoundMode::HalfEven, 2);
    }

    #[test]
    fn rounding_precedes_target_range_check() {
        assert_eq!("255.4 B".parse::<ByteSize<u8>>(), Ok(ByteSize::b(255)));
        assert_eq!("255.5 B".parse::<ByteSize<u8>>(), Err(ParseError::Overflow),);

        let mut options = ParseOptions {
            round_mode: RoundMode::Floor,
        };
        assert_eq!(
            ByteSize::<u8>::parse_with("255.9 B", options),
            Ok(ByteSize::b(255)),
        );
        options.round_mode = RoundMode::Ceil;
        assert_eq!(
            ByteSize::<u8>::parse_with("255.1 B", options),
            Err(ParseError::Overflow),
        );
        options.round_mode = RoundMode::HalfFloor;
        assert_eq!(
            ByteSize::<u8>::parse_with("255.5 B", options),
            Ok(ByteSize::b(255)),
        );
        options.round_mode = RoundMode::HalfEven;
        assert_eq!(
            ByteSize::<u8>::parse_with("255.5 B", options),
            Err(ParseError::Overflow),
        );
    }

    quickcheck::quickcheck! {
        fn eib_fractions_follow_each_rounding_mode(whole: u8, fraction: u64) -> bool {
            const MULTIPLIER: u128 = 1 << 60;
            const SCALE: u128 = 1_000_000_000_000_000_000;

            let whole = whole % 16;
            let fraction = fraction % SCALE as u64;
            let input = format!("{whole}.{fraction:018} EiB");
            let exact = (u128::from(whole) * SCALE + u128::from(fraction)) * MULTIPLIER;
            let lower = exact / SCALE;
            let remainder = exact % SCALE;
            let twice_remainder = remainder * 2;

            for mode in ROUND_MODES {
                let round_up = match mode {
                    RoundMode::Ceil => remainder != 0,
                    RoundMode::Floor => false,
                    RoundMode::HalfCeil => twice_remainder >= SCALE,
                    RoundMode::HalfFloor => twice_remainder > SCALE,
                    RoundMode::HalfEven => {
                        twice_remainder > SCALE || (twice_remainder == SCALE && lower % 2 == 1)
                    }
                };
                let expected = lower + u128::from(round_up);
                let options = ParseOptions { round_mode: mode };
                let actual = ByteSize::<u64>::parse_with(&input, options);

                if expected > u128::from(u64::MAX) {
                    if actual != Err(ParseError::Overflow) {
                        return false;
                    }
                } else if actual != Ok(ByteSize::b(u64::try_from(expected).unwrap())) {
                    return false;
                }
            }

            true
        }

        fn fractional_trailing_zero_preserves_value_for_each_mode(
            whole: u8,
            fraction: u64
        ) -> bool {
            const SCALE: u64 = 1_000_000_000_000_000_000;

            let whole = whole % 16;
            let fraction = fraction % SCALE;
            let input = format!("{whole}.{fraction:018} EiB");
            let input_with_zero = format!("{whole}.{fraction:018}0 EiB");

            ROUND_MODES.into_iter().all(|mode| {
                let options = ParseOptions { round_mode: mode };
                ByteSize::<u64>::parse_with(&input, options)
                    == ByteSize::<u64>::parse_with(&input_with_zero, options)
            })
        }
    }
}
