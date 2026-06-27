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

impl_unit_accessors!();

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
        assert_eq!(BSize::kib(16_u64), BSize::<u64>::b(16 * 1_024));
        assert_eq!(BSize::mib(16_u32), BSize::<u32>::b(16 * 1_048_576));
    }

    #[test]
    fn inferred_constructor_is_const() {
        const SIZE: BSize<u64> = BSize::kib(16_u64);

        assert_eq!(SIZE, BSize::<u64>::b(16 * 1_024));
    }

    #[cfg(target_pointer_width = "16")]
    #[test]
    fn returns_usize_units() {
        assert_eq!(BSize::<usize>::kb(2).0, 2_000);
        assert_eq!(BSize::<usize>::kib(2).0, 2_048);
        assert_close(BSize::<usize>::kb(2).as_kb(), 2.0);
        assert_close(BSize::<usize>::kib(2).as_kib(), 2.0);
    }

    #[cfg(target_pointer_width = "32")]
    #[test]
    fn returns_usize_units() {
        assert_eq!(BSize::<usize>::kb(2).0, 2_000);
        assert_eq!(BSize::<usize>::kib(2).0, 2_048);
        assert_close(BSize::<usize>::kb(2).as_kb(), 2.0);
        assert_close(BSize::<usize>::kib(2).as_kib(), 2.0);
        assert_eq!(BSize::<usize>::gb(2).0, 2_000_000_000);
        assert_eq!(BSize::<usize>::gib(2).0, 2_147_483_648);
        assert_close(BSize::<usize>::gb(2).as_gb(), 2.0);
        assert_close(BSize::<usize>::gib(2).as_gib(), 2.0);
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn returns_usize_units() {
        assert_eq!(BSize::<usize>::kb(2).0, 2_000);
        assert_eq!(BSize::<usize>::kib(2).0, 2_048);
        assert_close(BSize::<usize>::kb(2).as_kb(), 2.0);
        assert_close(BSize::<usize>::kib(2).as_kib(), 2.0);
        assert_eq!(BSize::<usize>::gb(2).0, 2_000_000_000);
        assert_eq!(BSize::<usize>::gib(2).0, 2_147_483_648);
        assert_close(BSize::<usize>::gb(2).as_gb(), 2.0);
        assert_close(BSize::<usize>::gib(2).as_gib(), 2.0);
        assert_eq!(BSize::<usize>::eb(2).0, 2_000_000_000_000_000_000);
        assert_eq!(BSize::<usize>::eib(2).0, 2_305_843_009_213_693_952);
        assert_close(BSize::<usize>::eb(2).as_eb(), 2.0);
        assert_close(BSize::<usize>::eib(2).as_eib(), 2.0);
    }
}
