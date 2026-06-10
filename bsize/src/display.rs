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
    size: u64,
    mode: DisplayMode,
}

#[derive(Debug, Clone)]
enum DisplayMode {
    Binary,
    Decimal,
}

impl Display {
    /// Format using binary units (e.g., `11.8MiB`)
    pub fn binary(mut self) -> Self {
        self.mode = DisplayMode::Binary;
        self
    }

    /// Format using decimal units (e.g., `11.8MB`)
    pub fn decimal(mut self) -> Self {
        self.mode = DisplayMode::Decimal;
        self
    }

    fn new(size: u64) -> Self {
        Self {
            size,
            mode: DisplayMode::Binary,
        }
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.size;

        let unit = match self.mode {
            DisplayMode::Binary => 1024,
            DisplayMode::Decimal => 1000,
        };

        let unit_prefixes = match self.mode {
            DisplayMode::Binary => b"KMGTPE",
            DisplayMode::Decimal => b"kMGTPE",
        };
        let unit_suffix = match self.mode {
            DisplayMode::Binary => "iB",
            DisplayMode::Decimal => "B",
        };
        let unit_separator = " ";
        let precision = f.precision().unwrap_or(1);

        if bytes < unit {
            write!(f, "{bytes}{unit_separator}B")?;
        } else {
            let size = bytes as f64;

            let mut ideal_prefix = 0usize;
            let mut ideal_size = size;
            loop {
                ideal_prefix += 1;
                ideal_size /= unit as f64;

                if ideal_size < unit as f64 {
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
                    Display::new(self.0 as u64)
                }
            }
        )*
    };
}

impl_display!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::format;
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_formatting_snapshots() {
        let test_values = [
            0u64,
            1,
            500,
            999,
            1000,
            1023,
            1024,
            1025,
            1500,
            2048,
            1000000,
            1048576,
            987654321,
            1099511627776,
            1125899906842624,
            1152921504606846976,
            u64::MAX - 1,
            u64::MAX,
        ];

        let mut results = alloc::string::String::new();
        for &bytes in &test_values {
            for mode in [DisplayMode::Binary, DisplayMode::Decimal] {
                let disp = Display {
                    size: bytes,
                    mode: mode.clone(),
                };
                let formatted = disp.to_string();
                let mode_str = match mode {
                    DisplayMode::Binary => "Binary",
                    DisplayMode::Decimal => "Decimal",
                };
                let line = format!("{bytes:>20} ({mode_str:<7}) => {formatted}\n");
                results.push_str(&line);
            }
        }

        insta::assert_snapshot!(results, @"
                           0 (Binary ) => 0 B
                           0 (Decimal) => 0 B
                           1 (Binary ) => 1 B
                           1 (Decimal) => 1 B
                         500 (Binary ) => 500 B
                         500 (Decimal) => 500 B
                         999 (Binary ) => 999 B
                         999 (Decimal) => 999 B
                        1000 (Binary ) => 1000 B
                        1000 (Decimal) => 1.0 kB
                        1023 (Binary ) => 1023 B
                        1023 (Decimal) => 1.0 kB
                        1024 (Binary ) => 1.0 KiB
                        1024 (Decimal) => 1.0 kB
                        1025 (Binary ) => 1.0 KiB
                        1025 (Decimal) => 1.0 kB
                        1500 (Binary ) => 1.5 KiB
                        1500 (Decimal) => 1.5 kB
                        2048 (Binary ) => 2.0 KiB
                        2048 (Decimal) => 2.0 kB
                     1000000 (Binary ) => 976.6 KiB
                     1000000 (Decimal) => 1.0 MB
                     1048576 (Binary ) => 1.0 MiB
                     1048576 (Decimal) => 1.0 MB
                   987654321 (Binary ) => 941.9 MiB
                   987654321 (Decimal) => 987.7 MB
               1099511627776 (Binary ) => 1.0 TiB
               1099511627776 (Decimal) => 1.1 TB
            1125899906842624 (Binary ) => 1.0 PiB
            1125899906842624 (Decimal) => 1.1 PB
         1152921504606846976 (Binary ) => 1.0 EiB
         1152921504606846976 (Decimal) => 1.2 EB
        18446744073709551614 (Binary ) => 16.0 EiB
        18446744073709551614 (Decimal) => 18.4 EB
        18446744073709551615 (Binary ) => 16.0 EiB
        18446744073709551615 (Decimal) => 18.4 EB
        ");
    }
}
