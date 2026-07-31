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

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(
    feature = "nightly",
    feature(const_closures, const_destruct, const_ops, const_trait_impl)
)]
#![deny(missing_docs)]

//! This crate provides multiple semantic wrappers and utilities for byte size representations.
//!
//! # Features
//!
//! * `#![no_std]`-capable, no heap allocation, and no runtime dependencies by default.
//! * Generic [`ByteSize`] wrappers over supported unsigned integer base types, with [`BSize`] as
//!   the `usize` alias and [`BSize8`], [`BSize16`], [`BSize32`], and [`BSize64`] as shorter aliases
//!   for fixed-width base types.
//! * `FromStr` impl for `ByteSize`, allowing for parsing string size representations like "1.5 KiB"
//!   and "521 TB". Fractional values default to half-expand rounding, and all [`RoundMode`]
//!   variants can be selected explicitly with [`ByteSize::parse_with_rounding`].
//! * Exact [`core::fmt::Display`] impl for [`ByteSize`], rendering the underlying byte count in
//!   base bytes (e.g., "1572864 B").
//! * Configurable, approximate human-readable formatting in both binary (e.g., "1.5 MiB") and
//!   decimal (e.g., "1.6 MB") styles.
//! * Optional `serde` support for binary and human-readable format.
//! * Optional `nightly` support for a broader const-friendly API surface powered by nightly-only
//!   Rust features.
//!
//! # Nightly
//!
//! With the `nightly` feature enabled on a nightly compiler, this crate can use unstable Rust
//! capabilities such as const trait support. The visible effect is a broader const surface for
//! generic byte-size expressions, including unit helpers and simple transformations over the
//! underlying byte count. Because this follows Rust nightly, exact capabilities may evolve with
//! upstream language features.
//!
//! # Examples
//!
//! Construction using the binary or decimal constant helpers.
//!
//! ```
//! use bsize::BSize;
//!
//! assert!(BSize::kib(4) > BSize::kb(4));
//!
//! let size: BSize = BSize::b(4_096);
//! assert_eq!(size.bytes(), 4_096);
//! ```
//!
//! Parse byte sizes from strings.
//!
//! ```
//! use bsize::BSize64;
//!
//! let size: BSize64 = "1.5 MiB".parse().unwrap();
//!
//! assert_eq!(BSize64::mib(1).map(|bytes| bytes + 512 * 1024), size);
//! ```
//!
//! Format the exact byte count or an approximate human-readable string.
//!
//! ```
//! use bsize::BSize;
//! use bsize::DisplayBaseUnit;
//! use bsize::DisplayOptions;
//! use bsize::DisplayScale;
//!
//! let size = BSize::mib(1);
//! assert_eq!("1048576 B", size.to_string());
//! assert_eq!("1.0 MiB", size.display().to_string());
//!
//! assert_eq!("518.0 GiB", BSize::gib(518).display().binary().to_string());
//!
//! assert_eq!("556.2 GB", BSize::gib(518).display().decimal().to_string());
//!
//! let network_units = DisplayOptions::DECIMAL
//!     .base_unit(DisplayBaseUnit::Bit)
//!     .scale(DisplayScale::Mega);
//! let display = bsize::display(125_000u64).options(|_opts| network_units);
//! assert_eq!("1.0 Mbit", display.to_string());
//! ```
//!
//! Arithmetic operations are supported.
//!
//! ```
//! use bsize::BSize;
//!
//! let plus = BSize::mb(1) + BSize::kb(100);
//! println!("{plus}");
//!
//! let minus = BSize::tb(1) - BSize::gb(4);
//! assert_eq!(BSize::gb(996), minus);
//! ```
//!
//! Arithmetic operations over the underlying types are supported.
//!
//!```
//! use bsize::BSize;
//!
//! let size = BSize::mb(1);
//! let size = size.map(|b| b * 4); // 4x scale
//! println!("{size}");
//! ```

#![no_std]

#[cfg(test)] // no-alloc; only used for tests
extern crate alloc;

mod display;
mod impls;
mod ops;
mod parse;
#[cfg(feature = "serde")]
mod serde;
mod traits;

pub use self::display::Display;
pub use self::display::DisplayBaseUnit;
pub use self::display::DisplayOptions;
pub use self::display::DisplayScale;
pub use self::display::DisplayUnitSystem;
pub use self::display::display;
pub use self::parse::ParseError;
pub use self::parse::RoundMode;
pub use self::traits::BaseByteSize;
pub use self::traits::ExaByteSize;
pub use self::traits::GigaByteSize;
pub use self::traits::KiloByteSize;
pub use self::traits::MegaByteSize;
pub use self::traits::PetaByteSize;
pub use self::traits::TeraByteSize;

/// Byte size representation.
///
/// Use [`ByteSize::b`] to construct a value from bytes and [`ByteSize::bytes`] to get
/// the exact underlying byte count. Its standard [`core::fmt::Display`] implementation renders
/// that exact count in base bytes. Use [`ByteSize::display`] for configurable, approximate
/// human-readable formatting.
///
/// # Parsing and rounding
///
/// Parsing applies the unit multiplier before rounding the resulting value once to a whole number
/// of bytes. The standard [`core::str::FromStr`] implementation uses [`RoundMode::HalfExpand`]: the
/// nearest whole byte is selected, and a value exactly halfway between two byte counts is rounded
/// away from zero. Since byte sizes are non-negative, this means that a tie is rounded toward the
/// larger byte count. Use [`ByteSize::parse_with_rounding`] to select another mode.
///
/// Decimal fractions are evaluated exactly without first converting them to floating point.
/// Overflow is checked after rounding, both against `u64` and against the integer type backing the
/// parsed [`ByteSize`].
///
/// ```
/// use bsize::BSize8;
/// use bsize::BSize64;
/// use bsize::ParseError;
/// use bsize::RoundMode;
///
/// assert_eq!(BSize64::b(0), "0.499 B".parse().unwrap());
/// assert_eq!(BSize64::b(1), "0.5 B".parse().unwrap());
/// assert_eq!(BSize64::b(1_235), "1.2345 kB".parse().unwrap());
/// assert_eq!(
///     BSize64::b(2),
///     BSize64::parse_with_rounding("2.5 B", RoundMode::HalfEven).unwrap(),
/// );
///
/// // The rounded result fits in u8.
/// assert_eq!(BSize8::b(255), "255.4 B".parse().unwrap());
/// // The rounded result does not.
/// assert_eq!(
///     ParseError::Overflow,
///     "255.5 B".parse::<BSize8>().unwrap_err(),
/// );
/// ```
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteSize<T: BaseByteSize>(T);

/// Byte size representation backed by `usize`.
pub type BSize = ByteSize<usize>;

/// Byte size representation backed by `u8`.
pub type BSize8 = ByteSize<u8>;

/// Byte size representation backed by `u16`.
pub type BSize16 = ByteSize<u16>;

/// Byte size representation backed by `u32`.
pub type BSize32 = ByteSize<u32>;

/// Byte size representation backed by `u64`.
pub type BSize64 = ByteSize<u64>;

#[cfg(test)]
fn assert_close(actual: f64, expected: f64) {
    let delta = (actual - expected).abs();
    let tolerance = f64::EPSILON;

    assert!(
        delta <= tolerance,
        "actual: {actual}, expected: {expected}, delta: {delta}, tolerance: {tolerance}",
    );
}

#[cfg(test)]
mod property_tests {
    use alloc::string::String;
    use alloc::string::ToString;

    use super::*;

    impl quickcheck::Arbitrary for ByteSize<u64> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            ByteSize::b(u64::arbitrary(g))
        }
    }

    quickcheck::quickcheck! {
        fn parsing_never_panics(size: String) -> bool {
            let _ = size.parse::<ByteSize<u64>>();
            true
        }

        fn to_string_never_blank(size: ByteSize<u64>) -> bool {
            !size.to_string().is_empty()
        }

        fn string_round_trip(size: ByteSize<u64>) -> bool {
            size.to_string().parse::<ByteSize<u64>>().unwrap() == size
        }
    }
}
