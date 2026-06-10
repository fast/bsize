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
    use alloc::format;
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_formatting_equivalence() {
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

        for &bytes in &test_values {
            for mode in [DisplayMode::Binary, DisplayMode::Decimal] {
                let display = Display {
                    size: bytes,
                    mode: mode.clone(),
                };
                let formatted_new = display.to_string();

                let formatted_old = format_old(bytes, &mode);

                assert_eq!(
                    formatted_new, formatted_old,
                    "formatting mismatch for bytes={bytes} in mode={mode:?}",
                );
            }
        }
    }

    fn format_old(bytes: u64, mode: &DisplayMode) -> alloc::string::String {
        let unit = match mode {
            DisplayMode::Binary => 1024,
            DisplayMode::Decimal => 1000,
        };

        let unit_prefixes = match mode {
            DisplayMode::Binary => b"KMGTPE",
            DisplayMode::Decimal => b"kMGTPE",
        };
        let unit_suffix = match mode {
            DisplayMode::Binary => "iB",
            DisplayMode::Decimal => "B",
        };
        let unit_separator = " ";
        let precision = 1;

        if bytes < unit {
            format!("{bytes}{unit_separator}B")
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

            format!(
                "{:.precision$}{unit_separator}{unit_prefix}{unit_suffix}",
                size / unit.pow(exp as u32) as f64,
            )
        }
    }
}
