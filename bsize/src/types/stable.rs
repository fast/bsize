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

macro_rules! impl_byte_accessors {
    ($($ty:ty),* $(,)?) => {
        $(
            impl BSize<$ty> {
                /// Returns byte count as bytes.
                ///
                /// The result is approximate when the byte count cannot be
                /// represented exactly as `f64`. Use `.0` or [`BSize::with`]
                /// for the exact underlying integer value.
                #[inline(always)]
                pub const fn as_b(&self) -> f64 {
                    self.0 as f64
                }
            }
        )*
    };
}

macro_rules! impl_accessors {
    ($ty:ty => { $($name:ident = $trait:ident::$size:ident => $unit:literal),* $(,)? }) => {
        impl BSize<$ty> {
            $(
                #[doc = concat!("Returns byte count as ", $unit, ".")]
                ///
                /// The result is approximate when the byte count cannot be
                /// represented exactly as `f64`.
                #[inline(always)]
                pub const fn $name(&self) -> f64 {
                    (self.0 as f64) / (<$ty as $crate::traits::$trait>::$size as f64)
                }
            )*
        }
    };
}

macro_rules! impl_usize_accessors {
    (through_kilo) => {
        impl_accessors!(usize => {
            as_kb = KiloByteSize::KB => "kilobytes",
            as_kib = KiloByteSize::KIB => "kibibytes",
        });
    };
    (through_giga) => {
        impl_usize_accessors!(through_kilo);
        impl_accessors!(usize => {
            as_mb = MegaByteSize::MB => "megabytes",
            as_mib = MegaByteSize::MIB => "mebibytes",
            as_gb = GigaByteSize::GB => "gigabytes",
            as_gib = GigaByteSize::GIB => "gibibytes",
        });
    };
    (through_exa) => {
        impl_usize_accessors!(through_giga);
        impl_accessors!(usize => {
            as_tb = TeraByteSize::TB => "terabytes",
            as_tib = TeraByteSize::TIB => "tebibytes",
            as_pb = PetaByteSize::PB => "petabytes",
            as_pib = PetaByteSize::PIB => "pebibytes",
            as_eb = ExaByteSize::EB => "exabytes",
            as_eib = ExaByteSize::EIB => "exbibytes",
        });
    };
}

macro_rules! impl_unit_accessors {
    () => {
        impl_accessors!(u16 => {
            as_kb = KiloByteSize::KB => "kilobytes",
            as_kib = KiloByteSize::KIB => "kibibytes",
        });

        impl_accessors!(u32 => {
            as_kb = KiloByteSize::KB => "kilobytes",
            as_kib = KiloByteSize::KIB => "kibibytes",
            as_mb = MegaByteSize::MB => "megabytes",
            as_mib = MegaByteSize::MIB => "mebibytes",
            as_gb = GigaByteSize::GB => "gigabytes",
            as_gib = GigaByteSize::GIB => "gibibytes",
        });

        impl_accessors!(u64 => {
            as_kb = KiloByteSize::KB => "kilobytes",
            as_kib = KiloByteSize::KIB => "kibibytes",
            as_mb = MegaByteSize::MB => "megabytes",
            as_mib = MegaByteSize::MIB => "mebibytes",
            as_gb = GigaByteSize::GB => "gigabytes",
            as_gib = GigaByteSize::GIB => "gibibytes",
            as_tb = TeraByteSize::TB => "terabytes",
            as_tib = TeraByteSize::TIB => "tebibytes",
            as_pb = PetaByteSize::PB => "petabytes",
            as_pib = PetaByteSize::PIB => "pebibytes",
            as_eb = ExaByteSize::EB => "exabytes",
            as_eib = ExaByteSize::EIB => "exbibytes",
        });

        #[cfg(target_pointer_width = "16")]
        impl_usize_accessors!(through_kilo);
        #[cfg(target_pointer_width = "32")]
        impl_usize_accessors!(through_giga);
        #[cfg(target_pointer_width = "64")]
        impl_usize_accessors!(through_exa);
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

impl_byte_accessors!(u8, u16, u32, u64, usize);
impl_unit_accessors!();
