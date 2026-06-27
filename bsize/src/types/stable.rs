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

macro_rules! impl_constructors {
    ($ty:ty => { $($name:ident = $trait:ident::$size:ident),* $(,)? }) => {
        impl BSize<$ty> {
            $(
                #[doc = concat!(
                    "Constructs a byte size wrapper from a quantity of `",
                    stringify!($name),
                    "` units."
                )]
                #[inline(always)]
                pub const fn $name(size: $ty) -> Self {
                    BSize(size * <$ty as $crate::traits::$trait>::$size)
                }
            )*
        }
    };
}

macro_rules! impl_usize_constructors {
    (through_kilo) => {
        impl_constructors!(usize => {
            kb = KiloByteSize::KB,
            kib = KiloByteSize::KIB,
        });
    };
    (through_giga) => {
        impl_usize_constructors!(through_kilo);
        impl_constructors!(usize => {
            mb = MegaByteSize::MB,
            mib = MegaByteSize::MIB,
            gb = GigaByteSize::GB,
            gib = GigaByteSize::GIB,
        });
    };
    (through_exa) => {
        impl_usize_constructors!(through_giga);
        impl_constructors!(usize => {
            tb = TeraByteSize::TB,
            tib = TeraByteSize::TIB,
            pb = PetaByteSize::PB,
            pib = PetaByteSize::PIB,
            eb = ExaByteSize::EB,
            eib = ExaByteSize::EIB,
        });
    };
}

impl_constructors!(u16 => {
    kb = KiloByteSize::KB,
    kib = KiloByteSize::KIB,
});

impl_constructors!(u32 => {
    kb = KiloByteSize::KB,
    kib = KiloByteSize::KIB,
    mb = MegaByteSize::MB,
    mib = MegaByteSize::MIB,
    gb = GigaByteSize::GB,
    gib = GigaByteSize::GIB,
});

impl_constructors!(u64 => {
    kb = KiloByteSize::KB,
    kib = KiloByteSize::KIB,
    mb = MegaByteSize::MB,
    mib = MegaByteSize::MIB,
    gb = GigaByteSize::GB,
    gib = GigaByteSize::GIB,
    tb = TeraByteSize::TB,
    tib = TeraByteSize::TIB,
    pb = PetaByteSize::PB,
    pib = PetaByteSize::PIB,
    eb = ExaByteSize::EB,
    eib = ExaByteSize::EIB,
});

#[cfg(target_pointer_width = "16")]
impl_usize_constructors!(through_kilo);
#[cfg(target_pointer_width = "32")]
impl_usize_constructors!(through_giga);
#[cfg(target_pointer_width = "64")]
impl_usize_constructors!(through_exa);

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
