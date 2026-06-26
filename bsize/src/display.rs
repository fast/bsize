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
/// Supports various styles, see methods. By default, the [`binary`] style is used.
///
/// [`binary`]: Display::binary
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
    mode: DisplayUnit,
}

#[derive(Debug, Clone)]
enum DisplayUnit {
    Binary,
    Decimal,
}

impl Display {
    /// Format using binary units (e.g., `11.8 MiB`)
    pub fn binary(mut self) -> Self {
        self.mode = DisplayUnit::Binary;
        self
    }

    /// Format using decimal units (e.g., `11.8 MB`)
    pub fn decimal(mut self) -> Self {
        self.mode = DisplayUnit::Decimal;
        self
    }

    fn new(size: f64) -> Self {
        Self {
            size,
            mode: DisplayUnit::Binary,
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.size;

        let unit = match self.mode {
            DisplayUnit::Binary => 1024.0,
            DisplayUnit::Decimal => 1000.0,
        };

        let unit_prefixes = match self.mode {
            DisplayUnit::Binary => b"KMGTPE",
            DisplayUnit::Decimal => b"kMGTPE",
        };
        let unit_suffix = match self.mode {
            DisplayUnit::Binary => "iB",
            DisplayUnit::Decimal => "B",
        };
        let unit_separator = " ";
        let precision = f.precision().unwrap_or(1);

        if bytes < unit {
            write!(f, "{bytes}{unit_separator}B")?;
        } else {
            let mut ideal_prefix = 0usize;
            let mut ideal_size = bytes;
            loop {
                ideal_prefix += 1;
                ideal_size /= unit;

                if ideal_size < unit {
                    break;
                }
            }
            let exp = ideal_prefix;

            let unit_prefix = unit_prefixes[exp - 1] as char;

            write!(
                f,
                "{:.precision$}{unit_separator}{unit_prefix}{unit_suffix}",
                ideal_size,
            )?;
        }

        Ok(())
    }
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
    fn test_formatting_snapshots() {
        use DisplayUnit::*;

        fn display(size: u64, mode: DisplayUnit) -> Display {
            let mut display = super::display(size);
            display.mode = mode;
            display
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
}
