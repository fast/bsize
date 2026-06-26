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
/// Supports various styles, see methods. By default, the [`decimal`] style is used.
///
/// [`decimal`]: Display::decimal
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
    unit: DisplayUnit,
}

/// Display unit options.
///
/// By default, values are formatted as bytes, with an automatically selected decimal unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayUnit {
    base: DisplayBaseUnit,
    fixed: DisplayFixedUnit,
    system: DisplayUnitSystem,
}

impl DisplayUnit {
    /// Creates display unit options with the default settings.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            base: DisplayBaseUnit::Byte,
            fixed: DisplayFixedUnit::None,
            system: DisplayUnitSystem::Decimal,
        }
    }

    /// Sets the base unit used for display.
    #[inline(always)]
    pub const fn base(mut self, base: DisplayBaseUnit) -> Self {
        self.base = base;
        self
    }

    /// Sets the fixed display unit.
    #[inline(always)]
    pub const fn fixed(mut self, fixed: DisplayFixedUnit) -> Self {
        self.fixed = fixed;
        self
    }

    /// Sets the unit system used for display.
    #[inline(always)]
    pub const fn system(mut self, system: DisplayUnitSystem) -> Self {
        self.system = system;
        self
    }

    /// Returns the configured base unit.
    #[inline(always)]
    pub const fn base_unit(&self) -> DisplayBaseUnit {
        self.base
    }

    /// Returns the configured fixed display unit.
    #[inline(always)]
    pub const fn fixed_unit(&self) -> DisplayFixedUnit {
        self.fixed
    }

    /// Returns the configured unit system.
    #[inline(always)]
    pub const fn unit_system(&self) -> DisplayUnitSystem {
        self.system
    }
}

impl Default for DisplayUnit {
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

/// Fixed display unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DisplayFixedUnit {
    /// Select the display unit automatically.
    None,
    /// Format values in the base unit without a prefix.
    One,
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
        self.unit = self.unit.system(DisplayUnitSystem::Binary);
        self
    }

    /// Format using decimal units (e.g., `11.8 MB`)
    pub fn decimal(mut self) -> Self {
        self.unit = self.unit.system(DisplayUnitSystem::Decimal);
        self
    }

    /// Format with the provided display unit options.
    pub fn options(mut self, unit: DisplayUnit) -> Self {
        self.unit = unit;
        self
    }

    /// Format using the provided base unit.
    pub fn base_unit(mut self, base: DisplayBaseUnit) -> Self {
        self.unit = self.unit.base(base);
        self
    }

    /// Format using the provided fixed display unit.
    pub fn fixed_unit(mut self, fixed: DisplayFixedUnit) -> Self {
        self.unit = self.unit.fixed(fixed);
        self
    }

    /// Format using the provided unit system.
    pub fn unit_system(mut self, system: DisplayUnitSystem) -> Self {
        self.unit = self.unit.system(system);
        self
    }

    fn new(size: f64) -> Self {
        Self {
            size,
            unit: DisplayUnit::default(),
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.unit.base {
            DisplayBaseUnit::Bit => self.size * 8.0,
            DisplayBaseUnit::Byte => self.size,
        };

        let divisor = match self.unit.system {
            DisplayUnitSystem::Binary => 1024.0,
            DisplayUnitSystem::Decimal => 1000.0,
        };

        let exponent = match self.unit.fixed {
            DisplayFixedUnit::None => auto_exponent(value, divisor),
            fixed => fixed.exponent(),
        };

        let value = value / divisor.powi(exponent as i32);
        let unit_prefixes = match self.unit.system {
            DisplayUnitSystem::Binary => b"KMGTPE",
            DisplayUnitSystem::Decimal => b"kMGTPE",
        };
        let unit_separator = " ";
        let unit_suffix = self.unit.suffix();

        if let Some(precision) = f.precision() {
            write!(f, "{value:.precision$}")?;
        } else if exponent == 0 {
            write!(f, "{value}")?;
        } else {
            write!(f, "{value:.1}")?;
        }

        f.write_str(unit_separator)?;

        if exponent == 0 {
            f.write_str(self.unit.base.suffix())
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

impl DisplayFixedUnit {
    fn exponent(self) -> usize {
        match self {
            Self::None | Self::One => 0,
            Self::Kilo => 1,
            Self::Mega => 2,
            Self::Giga => 3,
            Self::Tera => 4,
            Self::Peta => 5,
            Self::Exa => 6,
        }
    }
}

impl DisplayUnit {
    fn suffix(self) -> &'static str {
        match (self.system, self.base) {
            (DisplayUnitSystem::Binary, DisplayBaseUnit::Bit) => "ibit",
            (DisplayUnitSystem::Binary, DisplayBaseUnit::Byte) => "iB",
            (DisplayUnitSystem::Decimal, DisplayBaseUnit::Bit) => "bit",
            (DisplayUnitSystem::Decimal, DisplayBaseUnit::Byte) => "B",
        }
    }
}

fn auto_exponent(mut value: f64, divisor: f64) -> usize {
    let mut exponent = 0;

    while value >= divisor && exponent < DisplayFixedUnit::Exa.exponent() {
        value /= divisor;
        exponent += 1;
    }

    exponent
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
    fn test_display_unit_default() {
        let unit = DisplayUnit::default();
        assert_eq!(unit.base_unit(), DisplayBaseUnit::Byte);
        assert_eq!(unit.fixed_unit(), DisplayFixedUnit::None);
        assert_eq!(unit.unit_system(), DisplayUnitSystem::Decimal);
    }

    #[test]
    fn test_formatting_snapshots() {
        use DisplayUnitSystem::*;

        fn display(size: u64, system: DisplayUnitSystem) -> Display {
            super::display(size).unit_system(system)
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
    fn test_formats_default_decimal() {
        assert_snapshot!(super::display(999u64), @"999 B");
        assert_snapshot!(super::display(1000u64), @"1.0 kB");
    }

    #[test]
    fn test_formats_fixed_units() {
        assert_snapshot!(
            super::display(1536u64).fixed_unit(DisplayFixedUnit::One),
            @"1536 B"
        );
        assert_snapshot!(
            super::display(1536u64)
                .binary()
                .fixed_unit(DisplayFixedUnit::Kilo),
            @"1.5 KiB"
        );
        assert_snapshot!(
            format!(
                "{:.3}",
                super::display(1536u64)
                    .decimal()
                    .fixed_unit(DisplayFixedUnit::Mega)
            ),
            @"0.002 MB"
        );
    }

    #[test]
    fn test_formats_bits() {
        assert_snapshot!(
            super::display(1u64).base_unit(DisplayBaseUnit::Bit),
            @"8 bit"
        );
        assert_snapshot!(
            super::display(125u64).base_unit(DisplayBaseUnit::Bit),
            @"1.0 kbit"
        );
        assert_snapshot!(
            super::display(128u64)
                .binary()
                .base_unit(DisplayBaseUnit::Bit),
            @"1.0 Kibit"
        );
    }

    #[test]
    fn test_formats_with_display_unit_options() {
        let unit = DisplayUnit::default()
            .base(DisplayBaseUnit::Bit)
            .fixed(DisplayFixedUnit::Kilo)
            .system(DisplayUnitSystem::Binary);

        assert_snapshot!(super::display(1536u64).options(unit), @"12.0 Kibit");
    }
}
