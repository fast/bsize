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

macroweave::repeat!((Ty, Name, Trait, Size) in [
    (u16, kb, KiloByteSize, KB),
    (u16, kib, KiloByteSize, KIB),
    (u32, kb, KiloByteSize, KB),
    (u32, kib, KiloByteSize, KIB),
    (u32, mb, MegaByteSize, MB),
    (u32, mib, MegaByteSize, MIB),
    (u32, gb, GigaByteSize, GB),
    (u32, gib, GigaByteSize, GIB),
    (u64, kb, KiloByteSize, KB),
    (u64, kib, KiloByteSize, KIB),
    (u64, mb, MegaByteSize, MB),
    (u64, mib, MegaByteSize, MIB),
    (u64, gb, GigaByteSize, GB),
    (u64, gib, GigaByteSize, GIB),
    (u64, tb, TeraByteSize, TB),
    (u64, tib, TeraByteSize, TIB),
    (u64, pb, PetaByteSize, PB),
    (u64, pib, PetaByteSize, PIB),
    (u64, eb, ExaByteSize, EB),
    (u64, eib, ExaByteSize, EIB),
] {
    impl BSize<Ty> {
        #[doc = concat!(
            "Constructs a byte size wrapper from a quantity of `",
            stringify!(Name),
            "` units."
        )]
        #[inline(always)]
        pub const fn Name(size: Ty) -> Self {
            BSize(size * <Ty as crate::traits::Trait>::Size)
        }
    }
});

macroweave::repeat!((PointerWidth, Name, Trait, Size) in [
    ("16", kb, KiloByteSize, KB),
    ("16", kib, KiloByteSize, KIB),
    ("32", kb, KiloByteSize, KB),
    ("32", kib, KiloByteSize, KIB),
    ("32", mb, MegaByteSize, MB),
    ("32", mib, MegaByteSize, MIB),
    ("32", gb, GigaByteSize, GB),
    ("32", gib, GigaByteSize, GIB),
    ("64", kb, KiloByteSize, KB),
    ("64", kib, KiloByteSize, KIB),
    ("64", mb, MegaByteSize, MB),
    ("64", mib, MegaByteSize, MIB),
    ("64", gb, GigaByteSize, GB),
    ("64", gib, GigaByteSize, GIB),
    ("64", tb, TeraByteSize, TB),
    ("64", tib, TeraByteSize, TIB),
    ("64", pb, PetaByteSize, PB),
    ("64", pib, PetaByteSize, PIB),
    ("64", eb, ExaByteSize, EB),
    ("64", eib, ExaByteSize, EIB),
] {
    #[cfg(target_pointer_width = PointerWidth)]
    impl BSize<usize> {
        #[doc = concat!(
            "Constructs a byte size wrapper from a quantity of `",
            stringify!(Name),
            "` units."
        )]
        #[inline(always)]
        pub const fn Name(size: usize) -> Self {
            BSize(size * <usize as crate::traits::Trait>::Size)
        }
    }
});

macroweave::repeat!(Ty in [u8, u16, u32, u64, usize] {
    impl BSize<Ty> {
        /// Returns byte count as bytes.
        ///
        /// The result is approximate when the byte count cannot be
        /// represented exactly as `f64`. Use `.0` for the exact underlying
        /// integer value.
        #[inline(always)]
        pub const fn as_b(&self) -> f64 {
            self.0 as f64
        }
    }
});

macroweave::repeat!((Ty, Name, Trait, Size, Unit) in [
    (u16, as_kb, KiloByteSize, KB, "kilobytes"),
    (u16, as_kib, KiloByteSize, KIB, "kibibytes"),
    (u32, as_kb, KiloByteSize, KB, "kilobytes"),
    (u32, as_kib, KiloByteSize, KIB, "kibibytes"),
    (u32, as_mb, MegaByteSize, MB, "megabytes"),
    (u32, as_mib, MegaByteSize, MIB, "mebibytes"),
    (u32, as_gb, GigaByteSize, GB, "gigabytes"),
    (u32, as_gib, GigaByteSize, GIB, "gibibytes"),
    (u64, as_kb, KiloByteSize, KB, "kilobytes"),
    (u64, as_kib, KiloByteSize, KIB, "kibibytes"),
    (u64, as_mb, MegaByteSize, MB, "megabytes"),
    (u64, as_mib, MegaByteSize, MIB, "mebibytes"),
    (u64, as_gb, GigaByteSize, GB, "gigabytes"),
    (u64, as_gib, GigaByteSize, GIB, "gibibytes"),
    (u64, as_tb, TeraByteSize, TB, "terabytes"),
    (u64, as_tib, TeraByteSize, TIB, "tebibytes"),
    (u64, as_pb, PetaByteSize, PB, "petabytes"),
    (u64, as_pib, PetaByteSize, PIB, "pebibytes"),
    (u64, as_eb, ExaByteSize, EB, "exabytes"),
    (u64, as_eib, ExaByteSize, EIB, "exbibytes"),
] {
    impl BSize<Ty> {
        #[doc = concat!("Returns byte count as ", Unit, ".")]
        ///
        /// The result is approximate when the byte count cannot be
        /// represented exactly as `f64`.
        #[inline(always)]
        pub const fn Name(&self) -> f64 {
            (self.0 as f64) / (<Ty as crate::traits::Trait>::Size as f64)
        }
    }
});

macroweave::repeat!((PointerWidth, Name, Trait, Size, Unit) in [
    ("16", as_kb, KiloByteSize, KB, "kilobytes"),
    ("16", as_kib, KiloByteSize, KIB, "kibibytes"),
    ("32", as_kb, KiloByteSize, KB, "kilobytes"),
    ("32", as_kib, KiloByteSize, KIB, "kibibytes"),
    ("32", as_mb, MegaByteSize, MB, "megabytes"),
    ("32", as_mib, MegaByteSize, MIB, "mebibytes"),
    ("32", as_gb, GigaByteSize, GB, "gigabytes"),
    ("32", as_gib, GigaByteSize, GIB, "gibibytes"),
    ("64", as_kb, KiloByteSize, KB, "kilobytes"),
    ("64", as_kib, KiloByteSize, KIB, "kibibytes"),
    ("64", as_mb, MegaByteSize, MB, "megabytes"),
    ("64", as_mib, MegaByteSize, MIB, "mebibytes"),
    ("64", as_gb, GigaByteSize, GB, "gigabytes"),
    ("64", as_gib, GigaByteSize, GIB, "gibibytes"),
    ("64", as_tb, TeraByteSize, TB, "terabytes"),
    ("64", as_tib, TeraByteSize, TIB, "tebibytes"),
    ("64", as_pb, PetaByteSize, PB, "petabytes"),
    ("64", as_pib, PetaByteSize, PIB, "pebibytes"),
    ("64", as_eb, ExaByteSize, EB, "exabytes"),
    ("64", as_eib, ExaByteSize, EIB, "exbibytes"),
] {
    #[cfg(target_pointer_width = PointerWidth)]
    impl BSize<usize> {
        #[doc = concat!("Returns byte count as ", Unit, ".")]
        ///
        /// The result is approximate when the byte count cannot be
        /// represented exactly as `f64`.
        #[inline(always)]
        pub const fn Name(&self) -> f64 {
            (self.0 as f64) / (<usize as crate::traits::Trait>::Size as f64)
        }
    }
});
