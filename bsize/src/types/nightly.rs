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

use super::BSize;
use crate::traits::ByteSize;
use crate::traits::ExaByteSize;
use crate::traits::GigaByteSize;
use crate::traits::KiloByteSize;
use crate::traits::MegaByteSize;
use crate::traits::PetaByteSize;
use crate::traits::TeraByteSize;

macro_rules! impl_constructors {
    ($trait:ident => { $($name:ident = $size:ident),* $(,)? }) => {
        impl<T: $trait> BSize<T> {
            $(
                #[doc = concat!(
                    "Constructs a byte size wrapper from a quantity of `",
                    stringify!($name),
                    "` units."
                )]
                #[inline(always)]
                pub const fn $name(size: T) -> Self
                where
                    T: [const] $trait,
                {
                    BSize(size * T::$size)
                }
            )*
        }
    };
}

impl_constructors!(KiloByteSize => {
    kb = KB,
    kib = KIB,
});

impl_constructors!(MegaByteSize => {
    mb = MB,
    mib = MIB,
});

impl_constructors!(GigaByteSize => {
    gb = GB,
    gib = GIB,
});

impl_constructors!(TeraByteSize => {
    tb = TB,
    tib = TIB,
});

impl_constructors!(PetaByteSize => {
    pb = PB,
    pib = PIB,
});

impl_constructors!(ExaByteSize => {
    eb = EB,
    eib = EIB,
});

impl<T: ByteSize> BSize<T> {
    /// Returns byte count as bytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`. Use `.0` or [`BSize::with`] for the exact underlying
    /// integer value.
    #[inline(always)]
    pub const fn as_b(&self) -> f64
    where
        T: [const] ByteSize,
    {
        self.0.as_f64()
    }
}

macro_rules! impl_accessors {
    ($trait:ident => { $($name:ident = $size:ident => $unit:literal),* $(,)? }) => {
        impl<T: $trait> BSize<T> {
            $(
                #[doc = concat!("Returns byte count as ", $unit, ".")]
                ///
                /// The result is approximate when the byte count cannot be
                /// represented exactly as `f64`.
                #[inline(always)]
                pub const fn $name(&self) -> f64
                where
                    T: [const] $trait + [const] ByteSize,
                {
                    self.0.as_f64() / T::$size.as_f64()
                }
            )*
        }
    };
}

impl_accessors!(KiloByteSize => {
    as_kb = KB => "kilobytes",
    as_kib = KIB => "kibibytes",
});

impl_accessors!(MegaByteSize => {
    as_mb = MB => "megabytes",
    as_mib = MIB => "mebibytes",
});

impl_accessors!(GigaByteSize => {
    as_gb = GB => "gigabytes",
    as_gib = GIB => "gibibytes",
});

impl_accessors!(TeraByteSize => {
    as_tb = TB => "terabytes",
    as_tib = TIB => "tebibytes",
});

impl_accessors!(PetaByteSize => {
    as_pb = PB => "petabytes",
    as_pib = PIB => "pebibytes",
});

impl_accessors!(ExaByteSize => {
    as_eb = EB => "exabytes",
    as_eib = EIB => "exbibytes",
});

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
    fn infers_constructor_type_from_argument() {
        assert_eq!(BSize::kib(16_u64), BSize::b(16 * 1_024));
        assert_eq!(BSize::mib(16_u32), BSize::b(16 * 1_048_576));
    }

    #[test]
    fn inferred_constructor_is_const() {
        const SIZE: BSize<u64> = BSize::kib(16_u64);

        assert_eq!(SIZE, BSize::b(16 * 1_024));
    }

    #[test]
    fn inferred_accessors_are_const() {
        const BYTES: f64 = BSize::b(16_u64).as_b();
        const KIB: f64 = BSize::kib(16_u64).as_kib();

        assert_close(BYTES, 16.0);
        assert_close(KIB, 16.0);
    }

    #[cfg(target_pointer_width = "16")]
    #[test]
    fn returns_usize_units() {
        assert_eq!(BSize::kb(2_usize).0, 2_000);
        assert_eq!(BSize::kib(2_usize).0, 2_048);
        assert_close(BSize::kb(2_usize).as_kb(), 2.0);
        assert_close(BSize::kib(2_usize).as_kib(), 2.0);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn returns_usize_units() {
        assert_eq!(BSize::kb(2_usize).0, 2_000);
        assert_eq!(BSize::kib(2_usize).0, 2_048);
        assert_close(BSize::kb(2_usize).as_kb(), 2.0);
        assert_close(BSize::kib(2_usize).as_kib(), 2.0);
        assert_eq!(BSize::gb(2_usize).0, 2_000_000_000);
        assert_eq!(BSize::gib(2_usize).0, 2_147_483_648);
        assert_close(BSize::gb(2_usize).as_gb(), 2.0);
        assert_close(BSize::gib(2_usize).as_gib(), 2.0);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn returns_usize_units() {
        assert_eq!(BSize::kb(2_usize).0, 2_000);
        assert_eq!(BSize::kib(2_usize).0, 2_048);
        assert_close(BSize::kb(2_usize).as_kb(), 2.0);
        assert_close(BSize::kib(2_usize).as_kib(), 2.0);
        assert_eq!(BSize::gb(2_usize).0, 2_000_000_000);
        assert_eq!(BSize::gib(2_usize).0, 2_147_483_648);
        assert_close(BSize::gb(2_usize).as_gb(), 2.0);
        assert_close(BSize::gib(2_usize).as_gib(), 2.0);
        assert_eq!(BSize::eb(2_usize).0, 2_000_000_000_000_000_000);
        assert_eq!(BSize::eib(2_usize).0, 2_305_843_009_213_693_952);
        assert_close(BSize::eb(2_usize).as_eb(), 2.0);
        assert_close(BSize::eib(2_usize).as_eib(), 2.0);
    }
}
