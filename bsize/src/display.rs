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
use core::fmt::Write as _;

use crate::BSize;
use crate::ByteSize;

/// Create a [`Display`] instance for displaying a byte size.
///
/// See [`Display`] for examples. Use [`Display::new`] when the byte count is already represented
/// as an `f64`.
pub fn display(size: impl ByteSize) -> Display {
    Display::new(size.as_f64())
}

impl<T: ByteSize> BSize<T> {
    /// Returns a [`Display`] wrapper.
    ///
    /// See [`Display`] for examples.
    pub fn display(&self) -> Display {
        Display::new(self.0.as_f64())
    }
}

/// Display wrapper for formatting byte sizes as human-readable strings.
///
/// You may create this wrapper with [`Display::new`], [`display`], or [`BSize::display`], then
/// pass custom [`DisplayOptions`] with [`Display::options`].
///
/// # Examples
///
/// Display with the [`DisplayOptions::BINARY`] and [`DisplayOptions::DECIMAL`] presets.
///
/// ```
/// use bsize::BSize;
///
/// assert_eq!(
///     "41.0 KiB",
///     BSize::<u64>::kb(42).display().to_string(), // default to binary
/// );
///
/// assert_eq!(
///     "1.0 MiB",
///     BSize::<u64>::mib(1).display().binary().to_string(),
/// );
///
/// assert_eq!(
///     "42.0 kB",
///     BSize::<u64>::kb(42).display().decimal().to_string(),
/// );
/// ```
///
/// The free [`display`] function accepts any supported integer byte size.
///
/// ```
/// assert_eq!("1.5 KiB", bsize::display(1536u64).to_string());
/// ```
///
/// Use [`Display::new`] when the byte count is already represented as an `f64`.
///
/// ```
/// use bsize::Display;
///
/// assert_eq!("1.5 KiB", Display::new(1536.5).to_string());
/// assert_eq!("1.54 kB", format!("{:.2}", Display::new(1536.5).decimal()));
/// ```
///
/// Use standard formatter precision to control the number of fractional digits.
///
/// ```
/// use bsize::BSize;
///
/// assert_eq!(
///     "1.54 KiB",
///     format!("{:.2}", BSize::<u64>::b(1575).display())
/// );
/// assert_eq!("1.575 KiB", format!("{:.3}", bsize::display(1613u64)));
/// ```
///
/// Standard formatter width, fill, and alignment options are supported.
///
/// ```
/// let size = bsize::display(1536u64);
///
/// assert_eq!("1.5 KiB   ", format!("{size:10}"));
/// assert_eq!("   1.5 KiB", format!("{size:>10}"));
/// assert_eq!(" 1.5 KiB  ", format!("{size:^10}"));
/// assert_eq!("*1.5 KiB**", format!("{size:*^10}"));
/// assert_eq!("**1.50 KiB", format!("{size:*>10.2}"));
/// ```
///
/// Use [`DisplayOptions`] to choose a fixed scale or show values as bits.
///
/// ```
/// use bsize::DisplayBaseUnit;
/// use bsize::DisplayScale;
///
/// let as_kibits = bsize::display(1536u64).options(|opts| {
///     opts.base_unit(DisplayBaseUnit::Bit)
///         .scale(DisplayScale::Kilo)
/// });
///
/// assert_eq!("12.0 Kibit", as_kibits.to_string());
/// ```
///
/// Decimal units use a base of 1000 and SI prefixes.
///
/// ```
/// use bsize::DisplayScale;
/// use bsize::DisplayUnitSystem;
///
/// let display = bsize::display(1_500_000u64).options(|opts| {
///     opts.unit_system(DisplayUnitSystem::Decimal)
///         .scale(DisplayScale::Mega)
/// });
///
/// assert_eq!("1.500 MB", format!("{display:.3}"));
/// ```
#[derive(Debug, Clone)]
pub struct Display {
    size: f64,
    options: DisplayOptions,
}

/// Formatting options for [`Display`].
///
/// See [`Display`] for examples.
#[derive(Debug, Clone, Copy)]
pub struct DisplayOptions {
    base_unit: DisplayBaseUnit,
    scale: DisplayScale,
    unit_system: DisplayUnitSystem,
}

impl DisplayOptions {
    /// The default binary display options.
    ///
    /// See [`Display`] for examples.
    pub const BINARY: Self = Self {
        base_unit: DisplayBaseUnit::Byte,
        scale: DisplayScale::Auto,
        unit_system: DisplayUnitSystem::Binary,
    };

    /// Decimal display options.
    ///
    /// See [`Display`] for examples.
    pub const DECIMAL: Self = Self {
        base_unit: DisplayBaseUnit::Byte,
        scale: DisplayScale::Auto,
        unit_system: DisplayUnitSystem::Decimal,
    };

    /// Construct a new instance with the `BINARY` preset.
    ///
    /// See [`Display`] for examples.
    #[inline(always)]
    pub const fn new() -> Self {
        DisplayOptions::BINARY
    }

    /// Set the base unit used for display.
    ///
    /// See [`Display`] for examples.
    #[inline(always)]
    pub const fn base_unit(mut self, base_unit: DisplayBaseUnit) -> Self {
        self.base_unit = base_unit;
        self
    }

    /// Set the display scale.
    ///
    /// See [`Display`] for examples.
    #[inline(always)]
    pub const fn scale(mut self, scale: DisplayScale) -> Self {
        self.scale = scale;
        self
    }

    /// Set the unit system used for display.
    ///
    /// See [`Display`] for examples.
    #[inline(always)]
    pub const fn unit_system(mut self, unit_system: DisplayUnitSystem) -> Self {
        self.unit_system = unit_system;
        self
    }
}

impl Default for DisplayOptions {
    /// Same as [`DisplayOptions::new`].
    fn default() -> Self {
        Self::new()
    }
}

/// Base unit used by [`DisplayOptions`].
///
/// See [`Display`] for examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DisplayBaseUnit {
    /// Format values as bits.
    ///
    /// The byte count is converted to bits for display.
    ///
    /// # Examples
    ///
    /// ```
    /// use bsize::DisplayBaseUnit;
    ///
    /// let display = bsize::display(1usize).options(|opts| opts.base_unit(DisplayBaseUnit::Bit));
    ///
    /// assert_eq!("8 bit", display.to_string());
    /// ```
    Bit,
    /// Format values as bytes.
    Byte,
}

/// Scale used by [`DisplayOptions`].
///
/// See [`Display`] for examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DisplayScale {
    /// Select the display scale automatically.
    Auto,
    /// Format values in the base unit without a prefix.
    Base,
    /// Format values in kilo units.
    Kilo,
    /// Format values in mega units.
    Mega,
    /// Format values in giga units.
    Giga,
    /// Format values in tera units.
    Tera,
    /// Format values in peta units.
    Peta,
    /// Format values in exa units.
    Exa,
}

/// Unit system used by [`DisplayOptions`].
///
/// See [`Display`] for examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DisplayUnitSystem {
    /// Use the binary unit system, with a base of 1024.
    Binary,
    /// Use the decimal unit system, with a base of 1000.
    Decimal,
}

impl Display {
    /// Set the display option to the [`DisplayOptions::BINARY`] preset.
    ///
    /// See [`Display`] for examples.
    pub fn binary(mut self) -> Self {
        self.options = DisplayOptions::BINARY;
        self
    }

    /// Set the display option to the [`DisplayOptions::DECIMAL`] preset.
    ///
    /// See [`Display`] for examples.
    pub fn decimal(mut self) -> Self {
        self.options = DisplayOptions::DECIMAL;
        self
    }

    /// Set the options for display.
    ///
    /// The provided closure receives the current options, so customizations can build on the
    /// default binary preset or preconfigured options.
    ///
    /// # Examples
    ///
    /// ```
    /// use bsize::DisplayScale;
    ///
    /// let display = bsize::display(1536u64)
    ///     .decimal()
    ///     .options(|opts| opts.scale(DisplayScale::Kilo));
    ///
    /// assert_eq!("1.5 kB", display.to_string());
    /// ```
    ///
    /// Use `|_| options` when the current options should be replaced as a whole.
    ///
    /// ```
    /// use bsize::DisplayBaseUnit;
    /// use bsize::DisplayOptions;
    /// use bsize::DisplayScale;
    ///
    /// let network_units = DisplayOptions::DECIMAL
    ///     .base_unit(DisplayBaseUnit::Bit)
    ///     .scale(DisplayScale::Mega);
    ///
    /// let display = bsize::display(125_000u64).options(|_| network_units);
    ///
    /// assert_eq!("1.0 Mbit", display.to_string());
    /// ```
    pub fn options(mut self, f: impl FnOnce(DisplayOptions) -> DisplayOptions) -> Self {
        self.options = f(self.options);
        self
    }

    /// Create a [`Display`] instance from a byte count.
    ///
    /// This constructor is useful when the byte count is already represented as an `f64`. For
    /// supported integer byte counts, use [`display`] or [`BSize::display`].
    ///
    /// # Examples
    ///
    /// ```
    /// use bsize::Display;
    ///
    /// assert_eq!("2.5 KiB", Display::new(2560.0).to_string());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the `size` is not finite or is negative.
    pub fn new(size: f64) -> Self {
        assert!(size.is_finite() && size >= 0.0);
        let options = DisplayOptions::BINARY;
        Self { size, options }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.options.base_unit {
            DisplayBaseUnit::Bit => self.size * 8.0,
            DisplayBaseUnit::Byte => self.size,
        };
        let divisor = match self.options.unit_system {
            DisplayUnitSystem::Binary => 1024.0,
            DisplayUnitSystem::Decimal => 1000.0,
        };
        let (value, exponent) = scaled_value(value, divisor, self.options.scale);
        let precision = f.precision();

        let Some(width) = f.width() else {
            // fast path for no padding
            return write_display(f, value, exponent, self.options, precision);
        };

        let mut counter = WidthCounter { width: 0 };
        write_display(&mut counter, value, exponent, self.options, precision)?;

        let padding = width.saturating_sub(counter.width);
        let (left_padding, right_padding) = match f.align().unwrap_or(fmt::Alignment::Left) {
            fmt::Alignment::Left => (0, padding),
            fmt::Alignment::Right => (padding, 0),
            fmt::Alignment::Center => (padding / 2, padding - padding / 2),
        };

        let fill = f.fill();
        for _ in 0..left_padding {
            f.write_char(fill)?;
        }
        write_display(f, value, exponent, self.options, precision)?;
        for _ in 0..right_padding {
            f.write_char(fill)?;
        }
        Ok(())
    }
}

fn write_display(
    f: &mut impl fmt::Write,
    value: f64,
    exponent: usize,
    options: DisplayOptions,
    precision: Option<usize>,
) -> fmt::Result {
    if let Some(precision) = precision {
        write!(f, "{value:.precision$}")?;
    } else if exponent == 0 {
        write!(f, "{value}")?;
    } else {
        write!(f, "{value:.1}")?;
    }

    let unit_separator = " ";
    f.write_str(unit_separator)?;

    if exponent == 0 {
        f.write_str(match options.base_unit {
            DisplayBaseUnit::Bit => "bit",
            DisplayBaseUnit::Byte => "B",
        })
    } else {
        // Unit system references
        // * https://en.wikipedia.org/wiki/Kilobyte
        // * https://en.wikipedia.org/wiki/Bit#Multiple_bits
        let unit_prefixes = match options.unit_system {
            DisplayUnitSystem::Binary => b"KMGTPE",
            DisplayUnitSystem::Decimal => b"kMGTPE",
        };
        let unit_suffix = match (options.unit_system, options.base_unit) {
            (DisplayUnitSystem::Binary, DisplayBaseUnit::Bit) => "ibit",
            (DisplayUnitSystem::Binary, DisplayBaseUnit::Byte) => "iB",
            (DisplayUnitSystem::Decimal, DisplayBaseUnit::Bit) => "bit",
            (DisplayUnitSystem::Decimal, DisplayBaseUnit::Byte) => "B",
        };
        let unit_prefix = unit_prefixes[exponent - 1] as char;
        write!(f, "{unit_prefix}{unit_suffix}")
    }
}

struct WidthCounter {
    width: usize,
}

impl fmt::Write for WidthCounter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // HACK - all display character is ASCII
        self.width += s.len();
        Ok(())
    }

    fn write_char(&mut self, _: char) -> fmt::Result {
        self.width += 1;
        Ok(())
    }
}

fn scaled_value(mut value: f64, divisor: f64, scale: DisplayScale) -> (f64, usize) {
    const MAX_EXPONENT: usize = 6;

    let exponent = match scale {
        DisplayScale::Auto => {
            let mut exponent = 0;
            while value >= divisor && exponent < MAX_EXPONENT {
                value /= divisor;
                exponent += 1;
            }
            return (value, exponent);
        }
        DisplayScale::Base => 0,
        DisplayScale::Kilo => 1,
        DisplayScale::Mega => 2,
        DisplayScale::Giga => 3,
        DisplayScale::Tera => 4,
        DisplayScale::Peta => 5,
        DisplayScale::Exa => 6,
    };

    for _ in 0..exponent {
        value /= divisor;
    }

    (value, exponent)
}

#[cfg(test)]
mod tests {
    use alloc::format;

    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn test_formatting_snapshots() {
        use DisplayUnitSystem::*;

        fn display(size: u64, system: DisplayUnitSystem) -> Display {
            super::display(size).options(|opts| opts.unit_system(system))
        }

        assert_snapshot!(display(0, Binary), @"0 B");
        assert_snapshot!(display(0, Decimal), @"0 B");
        assert_snapshot!(display(1, Binary), @"1 B");
        assert_snapshot!(display(1, Decimal), @"1 B");
        assert_snapshot!(display(500, Binary), @"500 B");
        assert_snapshot!(display(500, Decimal), @"500 B");
        assert_snapshot!(display(999, Binary), @"999 B");
        assert_snapshot!(display(999, Decimal), @"999 B");
        assert_snapshot!(display(1000, Binary), @"1000 B");
        assert_snapshot!(display(1000, Decimal), @"1.0 kB");
        assert_snapshot!(display(1023, Binary), @"1023 B");
        assert_snapshot!(display(1023, Decimal), @"1.0 kB");
        assert_snapshot!(display(1024, Binary), @"1.0 KiB");
        assert_snapshot!(display(1024, Decimal), @"1.0 kB");
        assert_snapshot!(display(1025, Binary), @"1.0 KiB");
        assert_snapshot!(display(1025, Decimal), @"1.0 kB");
        assert_snapshot!(display(1500, Binary), @"1.5 KiB");
        assert_snapshot!(display(1500, Decimal), @"1.5 kB");
        assert_snapshot!(display(2048, Binary), @"2.0 KiB");
        assert_snapshot!(display(2048, Decimal), @"2.0 kB");
        assert_snapshot!(display(1_000_000, Binary), @"976.6 KiB");
        assert_snapshot!(display(1_000_000, Decimal), @"1.0 MB");
        assert_snapshot!(display(1_048_576, Binary), @"1.0 MiB");
        assert_snapshot!(display(1_048_576, Decimal), @"1.0 MB");
        assert_snapshot!(display(987_654_321, Binary), @"941.9 MiB");
        assert_snapshot!(display(987_654_321, Decimal), @"987.7 MB");
        assert_snapshot!(display(1_099_511_627_776, Binary), @"1.0 TiB");
        assert_snapshot!(display(1_099_511_627_776, Decimal), @"1.1 TB");
        assert_snapshot!(display(1_125_899_906_842_624, Binary), @"1.0 PiB");
        assert_snapshot!(display(1_125_899_906_842_624, Decimal), @"1.1 PB");
        assert_snapshot!(display(1_152_921_504_606_846_976, Binary), @"1.0 EiB");
        assert_snapshot!(display(1_152_921_504_606_846_976, Decimal), @"1.2 EB");
        assert_snapshot!(display(u64::MAX - 1, Binary), @"16.0 EiB");
        assert_snapshot!(display(u64::MAX - 1, Decimal), @"18.4 EB");
        assert_snapshot!(display(u64::MAX, Binary), @"16.0 EiB");
        assert_snapshot!(display(u64::MAX, Decimal), @"18.4 EB");
    }

    #[test]
    fn test_formats_fractional_sizes() {
        assert_snapshot!(Display::new(42.5).binary(), @"42.5 B");
        assert_snapshot!(Display::new(1000.5).decimal(), @"1.0 kB");
        assert_snapshot!(format!("{:.2}", Display::new(2500.5).decimal()), @"2.50 kB");
    }

    #[test]
    #[should_panic]
    fn test_new_rejects_nan_size() {
        Display::new(f64::NAN);
    }

    #[test]
    #[should_panic]
    fn test_new_rejects_infinite_size() {
        Display::new(f64::INFINITY);
    }

    #[test]
    #[should_panic]
    fn test_new_rejects_negative_size() {
        Display::new(-1.0);
    }

    #[test]
    fn test_formats_default_binary() {
        assert_snapshot!(display(999u64), @"999 B");
        assert_snapshot!(display(1000u64), @"1000 B");
    }

    #[test]
    fn test_formats_scales() {
        assert_snapshot!(
            display(1536u64).options(|opts| opts.scale(DisplayScale::Base)),
            @"1536 B"
        );
        assert_snapshot!(
            display(1536u64).options(|opts| opts.scale(DisplayScale::Kilo)),
            @"1.5 KiB"
        );
        assert_snapshot!(
            format!(
                "{:.3}",
                display(1536u64).options(|opts| opts
                    .unit_system(DisplayUnitSystem::Decimal)
                    .scale(DisplayScale::Mega))
            ),
            @"0.002 MB"
        );
    }

    #[test]
    fn test_formats_bits() {
        assert_snapshot!(
            display(1u64).options(|opts| opts.base_unit(DisplayBaseUnit::Bit)),
            @"8 bit"
        );
        assert_snapshot!(
            display(125u64).options(|opts| opts.base_unit(DisplayBaseUnit::Bit)),
            @"1000 bit"
        );
        assert_snapshot!(
            display(125u64).options(|opts| opts
                .base_unit(DisplayBaseUnit::Bit)
                .unit_system(DisplayUnitSystem::Decimal)),
            @"1.0 kbit"
        );
        assert_snapshot!(
            display(128u64).options(|opts| opts.base_unit(DisplayBaseUnit::Bit)),
            @"1.0 Kibit"
        );
    }

    #[test]
    fn test_formats_with_display_options() {
        assert_snapshot!(
            display(1536u64).options(|opts| opts
                .base_unit(DisplayBaseUnit::Bit)
                .scale(DisplayScale::Kilo)),
            @"12.0 Kibit"
        );
    }

    #[test]
    fn test_formats_with_width_fill_and_alignment() {
        assert_snapshot!(format!("{:10}", display(1536u64)), @"1.5 KiB   ");
        assert_snapshot!(format!("{:<10}", display(1536u64)), @"1.5 KiB   ");
        assert_snapshot!(format!("{:>10}", display(1536u64)), @"   1.5 KiB");
        assert_snapshot!(format!("{:^10}", display(1536u64)), @" 1.5 KiB  ");
        assert_snapshot!(format!("{:*^10}", display(1536u64)), @"*1.5 KiB**");
        assert_snapshot!(format!("{:*>10.2}", display(1536u64)), @"**1.50 KiB");
    }
}
