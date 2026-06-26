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

use crate::BSize;
use crate::Displayable;

/// Create a [`Display`] instance for displaying the byte size in various styles.
pub fn display(size: impl Displayable) -> Display {
    Display::new(size.canonicalize())
}

/// Display wrapper for [`BSize`].
///
/// Supports various styles:
///
/// # Examples
///
/// ```
/// # use bsize::BSize;
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
#[derive(Debug, Clone)]
pub struct Display {
    size: f64,
    options: DisplayOptions,
}

/// Display formatting options.
///
/// By default, values are formatted as bytes, with an automatically selected binary unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayOptions {
    base_unit: DisplayBaseUnit,
    scale: DisplayScale,
    unit_system: DisplayUnitSystem,
}

impl DisplayOptions {
    /// The default binary display options.
    pub const BINARY: Self = Self {
        base_unit: DisplayBaseUnit::Byte,
        scale: DisplayScale::Auto,
        unit_system: DisplayUnitSystem::Binary,
    };

    /// Decimal display options.
    pub const DECIMAL: Self = Self {
        base_unit: DisplayBaseUnit::Byte,
        scale: DisplayScale::Auto,
        unit_system: DisplayUnitSystem::Decimal,
    };

    /// Creates display options with the default settings.
    #[inline(always)]
    pub const fn new() -> Self {
        Self::BINARY
    }

    /// Sets the base unit used for display.
    #[inline(always)]
    pub const fn base_unit(mut self, base_unit: DisplayBaseUnit) -> Self {
        self.base_unit = base_unit;
        self
    }

    /// Sets the display scale.
    #[inline(always)]
    pub const fn scale(mut self, scale: DisplayScale) -> Self {
        self.scale = scale;
        self
    }

    /// Sets the unit system used for display.
    #[inline(always)]
    pub const fn unit_system(mut self, unit_system: DisplayUnitSystem) -> Self {
        self.unit_system = unit_system;
        self
    }
}

impl Default for DisplayOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Base unit used for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DisplayBaseUnit {
    /// Format values as bits.
    ///
    /// The byte count is converted to bits for display.
    Bit,
    /// Format values as bytes.
    Byte,
}

/// Display scale.
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

/// Unit system used for display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DisplayUnitSystem {
    /// Use the binary unit system, with a base of 1024.
    Binary,
    /// Use the decimal unit system, with a base of 1000.
    Decimal,
}

impl Display {
    /// Format using binary units (e.g., `11.8 MiB`)
    pub fn binary(mut self) -> Self {
        self.options = self.options.unit_system(DisplayUnitSystem::Binary);
        self
    }

    /// Format using decimal units (e.g., `11.8 MB`)
    pub fn decimal(mut self) -> Self {
        self.options = self.options.unit_system(DisplayUnitSystem::Decimal);
        self
    }

    /// Format with the provided display options.
    pub fn options(mut self, options: DisplayOptions) -> Self {
        self.options = options;
        self
    }

    fn new(size: f64) -> Self {
        Self {
            size,
            options: DisplayOptions::default(),
        }
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

        let exponent = match self.options.scale {
            DisplayScale::Auto => auto_exponent(value, divisor),
            scale => scale.exponent(),
        };

        let value = scaled_value(value, divisor, exponent);
        let unit_prefixes = match self.options.unit_system {
            DisplayUnitSystem::Binary => b"KMGTPE",
            DisplayUnitSystem::Decimal => b"kMGTPE",
        };
        let unit_separator = " ";
        let unit_suffix = self.options.suffix();

        if let Some(precision) = f.precision() {
            write!(f, "{value:.precision$}")?;
        } else if exponent == 0 {
            write!(f, "{value}")?;
        } else {
            write!(f, "{value:.1}")?;
        }

        f.write_str(unit_separator)?;

        if exponent == 0 {
            f.write_str(self.options.base_unit.suffix())
        } else {
            let unit_prefix = unit_prefixes[exponent - 1] as char;
            write!(f, "{unit_prefix}{unit_suffix}")
        }
    }
}

impl DisplayBaseUnit {
    fn suffix(self) -> &'static str {
        match self {
            Self::Bit => "bit",
            Self::Byte => "B",
        }
    }
}

impl DisplayScale {
    fn exponent(self) -> usize {
        match self {
            Self::Auto | Self::Base => 0,
            Self::Kilo => 1,
            Self::Mega => 2,
            Self::Giga => 3,
            Self::Tera => 4,
            Self::Peta => 5,
            Self::Exa => 6,
        }
    }
}

impl DisplayOptions {
    fn suffix(self) -> &'static str {
        match (self.unit_system, self.base_unit) {
            (DisplayUnitSystem::Binary, DisplayBaseUnit::Bit) => "ibit",
            (DisplayUnitSystem::Binary, DisplayBaseUnit::Byte) => "iB",
            (DisplayUnitSystem::Decimal, DisplayBaseUnit::Bit) => "bit",
            (DisplayUnitSystem::Decimal, DisplayBaseUnit::Byte) => "B",
        }
    }
}

fn auto_exponent(mut value: f64, divisor: f64) -> usize {
    let mut exponent = 0;

    while value >= divisor && exponent < DisplayScale::Exa.exponent() {
        value /= divisor;
        exponent += 1;
    }

    exponent
}

fn scaled_value(mut value: f64, divisor: f64, exponent: usize) -> f64 {
    for _ in 0..exponent {
        value /= divisor;
    }

    value
}

macro_rules! impl_display {
    ($($ty:ty),* $(,)?) => {
        $(
            impl BSize<$ty> {
                /// Returns a display wrapper.
                pub fn display(self) -> Display {
                    Display::new(self.0.canonicalize())
                }
            }
        )*
    };
}

impl_display!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use alloc::format;

    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn test_display_options_default() {
        let options = DisplayOptions::default();
        assert_eq!(options, DisplayOptions::BINARY);
        assert_eq!(options.base_unit, DisplayBaseUnit::Byte);
        assert_eq!(options.scale, DisplayScale::Auto);
        assert_eq!(options.unit_system, DisplayUnitSystem::Binary);
        assert_eq!(
            DisplayOptions::DECIMAL.unit_system,
            DisplayUnitSystem::Decimal
        );
    }

    #[test]
    fn test_formatting_snapshots() {
        use DisplayUnitSystem::*;

        fn display(size: u64, system: DisplayUnitSystem) -> Display {
            super::display(size).options(DisplayOptions::BINARY.unit_system(system))
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
    fn test_formats_default_binary() {
        assert_snapshot!(display(999u64), @"999 B");
        assert_snapshot!(display(1000u64), @"1000 B");
    }

    #[test]
    fn test_formats_scales() {
        assert_snapshot!(
            display(1536u64).options(DisplayOptions::BINARY.scale(DisplayScale::Base)),
            @"1536 B"
        );
        assert_snapshot!(
            display(1536u64).options(DisplayOptions::BINARY.scale(DisplayScale::Kilo)),
            @"1.5 KiB"
        );
        assert_snapshot!(
            format!(
                "{:.3}",
                display(1536u64).options(DisplayOptions::DECIMAL.scale(DisplayScale::Mega))
            ),
            @"0.002 MB"
        );
    }

    #[test]
    fn test_formats_bits() {
        assert_snapshot!(
            display(1u64).options(DisplayOptions::BINARY.base_unit(DisplayBaseUnit::Bit)),
            @"8 bit"
        );
        assert_snapshot!(
            display(125u64).options(DisplayOptions::BINARY.base_unit(DisplayBaseUnit::Bit)),
            @"1000 bit"
        );
        assert_snapshot!(
            display(125u64).options(DisplayOptions::DECIMAL.base_unit(DisplayBaseUnit::Bit)),
            @"1.0 kbit"
        );
        assert_snapshot!(
            display(128u64).options(DisplayOptions::BINARY.base_unit(DisplayBaseUnit::Bit)),
            @"1.0 Kibit"
        );
    }

    #[test]
    fn test_formats_with_display_options() {
        let options = DisplayOptions::BINARY
            .base_unit(DisplayBaseUnit::Bit)
            .scale(DisplayScale::Kilo);

        assert_snapshot!(display(1536u64).options(options), @"12.0 Kibit");
    }
}
