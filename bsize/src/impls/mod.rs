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

use crate::ByteSize;
use crate::traits::BaseByteSize;

impl<T: BaseByteSize + fmt::Debug> fmt::Debug for ByteSize<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ByteSize<{}>({:?})", type_name::<T>(), self.0)
    }
}

impl<T: BaseByteSize + fmt::Display> fmt::Display for ByteSize<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Stick to a base scale, so that users would not be surprised by:
        //
        //   println!("{}", BSize::kb(42))
        //
        // returns "41.0 KiB" rather than "42.0 KB".
        write!(f, "{} B", self.0)
    }
}

impl<T: BaseByteSize> ByteSize<T> {
    /// Constructs a byte size wrapper from a quantity of bytes.
    #[inline(always)]
    pub const fn b(size: T) -> Self {
        ByteSize(size)
    }

    /// Returns the exact byte count as the underlying integer type.
    #[inline(always)]
    pub const fn bytes(self) -> T {
        self.0
    }
}

#[cfg(feature = "nightly")]
mod nightly;
#[cfg(not(feature = "nightly"))]
mod stable;

#[cfg(test)]
mod tests {
    use crate::BSize;
    use crate::BSize8;
    use crate::BSize16;
    use crate::BSize32;
    use crate::BSize64;
    use crate::ByteSize;
    use crate::assert_close;

    #[test]
    fn defaults() {
        assert_eq!(BSize8::default(), BSize8::b(0));
        assert_eq!(BSize16::default(), BSize16::b(0));
        assert_eq!(BSize32::default(), BSize32::b(0));
        assert_eq!(BSize64::default(), BSize64::b(0));
        assert_eq!(BSize::default(), BSize::b(0));
    }

    #[test]
    fn bsize_alias_is_usize() {
        let default: BSize = BSize::b(2);
        let explicit: ByteSize<usize> = ByteSize::b(2);

        assert_eq!(default, explicit);
    }

    #[test]
    fn aliases_use_expected_underlying_types() {
        assert_eq!(BSize8::b(2), ByteSize::<u8>::b(2));
        assert_eq!(BSize16::kib(2), ByteSize::<u16>::kib(2));
        assert_eq!(BSize32::gib(2), ByteSize::<u32>::gib(2));
        assert_eq!(BSize64::eib(2), ByteSize::<u64>::eib(2));
    }

    #[test]
    fn returns_exact_bytes() {
        const KIB: u64 = BSize64::kib(1).bytes();
        assert_eq!(KIB, 1_024);
    }

    #[test]
    fn constructs_u8_units() {
        assert_eq!(BSize8::b(2).bytes(), 2);
    }

    #[test]
    fn returns_byte_units() {
        assert_close(BSize8::b(2).as_b(), 2.0);
        assert_close(BSize16::b(2).as_b(), 2.0);
        assert_close(BSize32::b(2).as_b(), 2.0);
        assert_close(BSize64::b(2).as_b(), 2.0);
        assert_close(BSize::b(2).as_b(), 2.0);
    }

    #[test]
    fn maps_underlying_byte_count() {
        assert_eq!(BSize64::kib(4).map(|bytes| bytes + 64), BSize64::b(4_160),);
    }

    #[test]
    fn constructs_u16_units() {
        assert_eq!(BSize16::kb(2).bytes(), 2_000);
        assert_eq!(BSize16::kib(2).bytes(), 2_048);
    }

    #[test]
    fn returns_u16_units() {
        assert_close(BSize16::kb(2).as_kb(), 2.0);
        assert_close(BSize16::kib(2).as_kib(), 2.0);
    }

    #[test]
    fn returns_fractional_u16_units() {
        let bytes = u16::MAX;
        let kb = 1_000u16;
        let kib = 1_024u16;

        assert_close(BSize16::b(bytes).as_kb(), (bytes as f64) / (kb as f64));
        assert_close(BSize16::b(bytes).as_kib(), (bytes as f64) / (kib as f64));
    }

    #[test]
    fn constructs_u32_units() {
        assert_eq!(BSize32::gb(2).bytes(), 2_000_000_000);
        assert_eq!(BSize32::gib(2).bytes(), 2_147_483_648);
    }

    #[test]
    fn returns_u32_units() {
        assert_close(BSize32::gb(2).as_gb(), 2.0);
        assert_close(BSize32::gib(2).as_gib(), 2.0);
    }

    #[test]
    fn returns_fractional_u32_units() {
        let bytes = u32::MAX;
        let gb = 1_000_000_000u32;
        let gib = 1_073_741_824u32;

        assert_close(BSize32::b(bytes).as_gb(), (bytes as f64) / (gb as f64));
        assert_close(BSize32::b(bytes).as_gib(), (bytes as f64) / (gib as f64));
    }

    #[test]
    fn constructs_u64_units() {
        assert_eq!(BSize64::eb(2).bytes(), 2_000_000_000_000_000_000);
        assert_eq!(BSize64::eib(2).bytes(), 2_305_843_009_213_693_952);
    }

    #[test]
    fn returns_u64_units() {
        assert_close(BSize64::eib(2).as_eib(), 2.0);
    }

    #[cfg(target_pointer_width = "16")]
    #[test]
    fn returns_usize_units() {
        assert_eq!(BSize::kb(2).bytes(), 2_000);
        assert_eq!(BSize::kib(2).bytes(), 2_048);
        assert_close(BSize::kb(2).as_kb(), 2.0);
        assert_close(BSize::kib(2).as_kib(), 2.0);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn returns_usize_units() {
        assert_eq!(BSize::kb(2).bytes(), 2_000);
        assert_eq!(BSize::kib(2).bytes(), 2_048);
        assert_close(BSize::kb(2).as_kb(), 2.0);
        assert_close(BSize::kib(2).as_kib(), 2.0);
        assert_eq!(BSize::gb(2).bytes(), 2_000_000_000);
        assert_eq!(BSize::gib(2).bytes(), 2_147_483_648);
        assert_close(BSize::gb(2).as_gb(), 2.0);
        assert_close(BSize::gib(2).as_gib(), 2.0);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn returns_usize_units() {
        assert_eq!(BSize::kb(2).bytes(), 2_000);
        assert_eq!(BSize::kib(2).bytes(), 2_048);
        assert_close(BSize::kb(2).as_kb(), 2.0);
        assert_close(BSize::kib(2).as_kib(), 2.0);
        assert_eq!(BSize::gb(2).bytes(), 2_000_000_000);
        assert_eq!(BSize::gib(2).bytes(), 2_147_483_648);
        assert_close(BSize::gb(2).as_gb(), 2.0);
        assert_close(BSize::gib(2).as_gib(), 2.0);
        assert_eq!(BSize::eb(2).bytes(), 2_000_000_000_000_000_000);
        assert_eq!(BSize::eib(2).bytes(), 2_305_843_009_213_693_952);
        assert_close(BSize::eb(2).as_eb(), 2.0);
        assert_close(BSize::eib(2).as_eib(), 2.0);
    }

    #[test]
    fn returns_large_fractional_u64_units() {
        let bytes = 9_876_543_210_987_654_321_u64;
        let eb = 1_000_000_000_000_000_000u64;
        let eib = 1_152_921_504_606_846_976u64;

        assert_close(BSize64::b(bytes).as_eb(), (bytes as f64) / (eb as f64));
        assert_close(BSize64::b(bytes).as_eib(), (bytes as f64) / (eib as f64));
    }
}
