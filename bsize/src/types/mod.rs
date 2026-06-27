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

use core::any::type_name;
use core::fmt;

use crate::traits::ByteSize;

/// Byte size representation.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BSize<T: ByteSize>(pub T);

impl<T: ByteSize + fmt::Debug> fmt::Debug for BSize<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BSize<{}>({:?})", type_name::<T>(), self.0)
    }
}

impl<T: ByteSize + fmt::Display> fmt::Display for BSize<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Stick to a base scale, so that users would not be surprised by:
        //
        //   println!("{}", BSize::<usize>::kb(42))
        //
        // returns "41.0 KiB" rather than "42.0 KB".
        write!(f, "{} B", self.0)
    }
}

impl<T: ByteSize> BSize<T> {
    /// Calculate a new byte size with the provided function, returning a new struct.
    pub fn map(self, f: impl FnOnce(T) -> T) -> Self {
        BSize(f(self.0))
    }

    /// Calculate a new value with the provided function.
    pub fn with<R>(self, f: impl FnOnce(T) -> R) -> R {
        f(self.0)
    }

    /// Constructs a byte size wrapper from a quantity of bytes.
    #[inline(always)]
    pub const fn b(size: T) -> Self {
        BSize(size)
    }
}

#[cfg(feature = "nightly")]
mod nightly;
#[cfg(not(feature = "nightly"))]
mod stable;

#[cfg(test)]
mod tests {
    use super::BSize;

    fn assert_close(actual: f64, expected: f64) {
        let delta = (actual - expected).abs();
        let tolerance = f64::EPSILON;

        assert!(
            delta <= tolerance,
            "actual: {actual}, expected: {expected}, delta: {delta}, tolerance: {tolerance}",
        );
    }

    #[test]
    fn defaults() {
        assert_eq!(BSize::<u8>::default(), BSize::b(0));
        assert_eq!(BSize::<u16>::default(), BSize::b(0));
        assert_eq!(BSize::<u32>::default(), BSize::b(0));
        assert_eq!(BSize::<u64>::default(), BSize::b(0));
        assert_eq!(BSize::<usize>::default(), BSize::b(0));
    }

    #[test]
    fn constructs_u8_units() {
        assert_eq!(BSize::<u8>::b(2).0, 2);
    }

    #[test]
    fn maps_underlying_byte_count() {
        assert_eq!(
            BSize::<u64>::kib(4).map(|bytes| bytes + 64),
            BSize::b(4_160),
        );
    }

    #[test]
    fn with_returns_closure_result() {
        assert!(BSize::<u64>::kib(4).with(|bytes| bytes == 4_096));
    }

    #[test]
    fn constructs_u16_units() {
        assert_eq!(BSize::<u16>::kb(2).0, 2_000);
        assert_eq!(BSize::<u16>::kib(2).0, 2_048);
    }

    #[test]
    fn returns_u16_units() {
        assert_close(BSize::<u16>::kb(2).as_kb(), 2.0);
        assert_close(BSize::<u16>::kib(2).as_kib(), 2.0);
    }

    #[test]
    fn returns_fractional_u16_units() {
        let bytes = u16::MAX;
        let kb = 1_000u16;
        let kib = 1_024u16;

        assert_close(BSize::<u16>::b(bytes).as_kb(), (bytes as f64) / (kb as f64));
        assert_close(
            BSize::<u16>::b(bytes).as_kib(),
            (bytes as f64) / (kib as f64),
        );
    }

    #[test]
    fn constructs_u32_units() {
        assert_eq!(BSize::<u32>::gb(2).0, 2_000_000_000);
        assert_eq!(BSize::<u32>::gib(2).0, 2_147_483_648);
    }

    #[test]
    fn returns_u32_units() {
        assert_close(BSize::<u32>::gb(2).as_gb(), 2.0);
        assert_close(BSize::<u32>::gib(2).as_gib(), 2.0);
    }

    #[test]
    fn returns_fractional_u32_units() {
        let bytes = u32::MAX;
        let gb = 1_000_000_000u32;
        let gib = 1_073_741_824u32;

        assert_close(BSize::<u32>::b(bytes).as_gb(), (bytes as f64) / (gb as f64));
        assert_close(
            BSize::<u32>::b(bytes).as_gib(),
            (bytes as f64) / (gib as f64),
        );
    }

    #[test]
    fn constructs_u64_units() {
        assert_eq!(BSize::<u64>::eb(2).0, 2_000_000_000_000_000_000);
        assert_eq!(BSize::<u64>::eib(2).0, 2_305_843_009_213_693_952);
    }

    #[test]
    fn returns_u64_units() {
        assert_close(BSize::<u64>::eib(2).as_eib(), 2.0);
    }

    #[test]
    fn returns_large_fractional_u64_units() {
        let bytes = 9_876_543_210_987_654_321_u64;
        let eb = 1_000_000_000_000_000_000u64;
        let eib = 1_152_921_504_606_846_976u64;

        assert_close(BSize::<u64>::b(bytes).as_eb(), (bytes as f64) / (eb as f64));
        assert_close(
            BSize::<u64>::b(bytes).as_eib(),
            (bytes as f64) / (eib as f64),
        );
    }
}
