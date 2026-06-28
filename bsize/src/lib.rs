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
//!   and "521 TB".
//! * [`Display`] impl for `ByteSize`, allowing for formatting byte sizes as human-readable strings
//!   in both binary (e.g., "1.5 MiB") and decimal (e.g., "1.5 MB") styles.
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
//! assert_eq!(size.0, 4_096);
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
//! Display as human-readable string.
//!
//! ```
//! use bsize::BSize;
//! use bsize::DisplayBaseUnit;
//! use bsize::DisplayOptions;
//! use bsize::DisplayScale;
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
mod ops;
mod parse;
#[cfg(feature = "serde")]
mod serde;
mod traits;
mod types;

pub use self::display::Display;
pub use self::display::DisplayBaseUnit;
pub use self::display::DisplayOptions;
pub use self::display::DisplayScale;
pub use self::display::DisplayUnitSystem;
pub use self::display::display;
pub use self::parse::ParseError;
pub use self::traits::BaseByteSize;
pub use self::traits::ExaByteSize;
pub use self::traits::GigaByteSize;
pub use self::traits::KiloByteSize;
pub use self::traits::MegaByteSize;
pub use self::traits::PetaByteSize;
pub use self::traits::TeraByteSize;
pub use self::types::BSize;
pub use self::types::BSize8;
pub use self::types::BSize16;
pub use self::types::BSize32;
pub use self::types::BSize64;
pub use self::types::ByteSize;

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
            Self(u64::arbitrary(g))
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
