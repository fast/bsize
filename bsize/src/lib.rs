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
#![cfg_attr(feature = "nightly", feature(const_ops, const_trait_impl))]
#![deny(missing_docs)]

//! This crate provides multiple semantic wrappers and utilities for byte size representations.
//!
//! # Features
//!
//! * `#![no_std]`-capable, no dependencies, and uses no heap allocation.
//! * [`BSize`] wrappers over `u8`, `u16`, `u32`, `u64`, and `usize` for representing byte sizes
//!   with different underlying types.
//! * `FromStr` impl for `BSize`, allowing for parsing string size representations like "1.5 KiB"
//!   and "521 TB".
//! * [`Display`] impl for `BSize`, allowing for formatting byte sizes as human-readable strings in
//!   both binary (e.g., "1.5 MiB") and decimal (e.g., "1.5 MB") styles.
//! * Optional `serde` support for binary and human-readable format.
//! * Optional `nightly` support for generic const unit constructors, allowing calls like
//!   `BSize::kib(16_u64)`.
//!
//! # Examples
//!
//! Construction using the binary or decimal constant helpers.
//!
//! ```
//! use bsize::BSize;
//!
//! assert!(BSize::<usize>::kib(4) > BSize::<usize>::kb(4));
//! ```
//!
//! Parse byte sizes from strings.
//!
//! ```
//! use bsize::BSize;
//!
//! let size: BSize<u64> = "1.5 MiB".parse().unwrap();
//!
//! assert_eq!(BSize::<u64>::mib(1).map(|bytes| bytes + 512 * 1024), size);
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
//! assert_eq!(
//!     "518.0 GiB",
//!     BSize::<usize>::gib(518).display().binary().to_string()
//! );
//!
//! assert_eq!(
//!     "556.2 GB",
//!     BSize::<usize>::gib(518).display().decimal().to_string()
//! );
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
//! let plus = BSize::<usize>::mb(1) + BSize::<usize>::kb(100);
//! println!("{plus}");
//!
//! let minus = BSize::<usize>::tb(1) - BSize::<usize>::gb(4);
//! assert_eq!(BSize::<usize>::gb(996), minus);
//! ```
//!
//! Arithmetic operations over the underlying types are supported.
//!
//!```
//! use bsize::BSize;
//!
//! let size = BSize::<usize>::mb(1);
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
pub use self::traits::ByteSize;
pub use self::traits::ExaByteSize;
pub use self::traits::GigaByteSize;
pub use self::traits::KiloByteSize;
pub use self::traits::MegaByteSize;
pub use self::traits::PetaByteSize;
pub use self::traits::TeraByteSize;
pub use self::types::BSize;

#[cfg(test)]
mod property_tests {
    use alloc::string::String;
    use alloc::string::ToString;

    use super::*;

    impl quickcheck::Arbitrary for BSize<u64> {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            Self(u64::arbitrary(g))
        }
    }

    quickcheck::quickcheck! {
        fn parsing_never_panics(size: String) -> bool {
            let _ = size.parse::<BSize<u64>>();
            true
        }

        fn to_string_never_blank(size: BSize<u64>) -> bool {
            !size.to_string().is_empty()
        }

        fn string_round_trip(size: BSize<u64>) -> bool {
            size.to_string().parse::<BSize<u64>>().unwrap() == size
        }
    }
}
