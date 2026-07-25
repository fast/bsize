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

use super::ByteSize;
use crate::traits::BaseByteSize;

impl<T: BaseByteSize> ByteSize<T> {
    /// Calculate a new byte size with the provided function, returning a new struct.
    pub fn map(self, f: impl FnOnce(T) -> T) -> Self {
        ByteSize(f(self.0))
    }
}

macroweave::repeat!((Cfg, Ty, Name, Trait, Size) in [
    (all(), u16, kb, KiloByteSize, KB),
    (all(), u16, kib, KiloByteSize, KIB),
    (all(), u32, kb, KiloByteSize, KB),
    (all(), u32, kib, KiloByteSize, KIB),
    (all(), u32, mb, MegaByteSize, MB),
    (all(), u32, mib, MegaByteSize, MIB),
    (all(), u32, gb, GigaByteSize, GB),
    (all(), u32, gib, GigaByteSize, GIB),
    (all(), u64, kb, KiloByteSize, KB),
    (all(), u64, kib, KiloByteSize, KIB),
    (all(), u64, mb, MegaByteSize, MB),
    (all(), u64, mib, MegaByteSize, MIB),
    (all(), u64, gb, GigaByteSize, GB),
    (all(), u64, gib, GigaByteSize, GIB),
    (all(), u64, tb, TeraByteSize, TB),
    (all(), u64, tib, TeraByteSize, TIB),
    (all(), u64, pb, PetaByteSize, PB),
    (all(), u64, pib, PetaByteSize, PIB),
    (all(), u64, eb, ExaByteSize, EB),
    (all(), u64, eib, ExaByteSize, EIB),
    (target_pointer_width = "16", usize, kb, KiloByteSize, KB),
    (target_pointer_width = "16", usize, kib, KiloByteSize, KIB),
    (target_pointer_width = "32", usize, kb, KiloByteSize, KB),
    (target_pointer_width = "32", usize, kib, KiloByteSize, KIB),
    (target_pointer_width = "32", usize, mb, MegaByteSize, MB),
    (target_pointer_width = "32", usize, mib, MegaByteSize, MIB),
    (target_pointer_width = "32", usize, gb, GigaByteSize, GB),
    (target_pointer_width = "32", usize, gib, GigaByteSize, GIB),
    (target_pointer_width = "64", usize, kb, KiloByteSize, KB),
    (target_pointer_width = "64", usize, kib, KiloByteSize, KIB),
    (target_pointer_width = "64", usize, mb, MegaByteSize, MB),
    (target_pointer_width = "64", usize, mib, MegaByteSize, MIB),
    (target_pointer_width = "64", usize, gb, GigaByteSize, GB),
    (target_pointer_width = "64", usize, gib, GigaByteSize, GIB),
    (target_pointer_width = "64", usize, tb, TeraByteSize, TB),
    (target_pointer_width = "64", usize, tib, TeraByteSize, TIB),
    (target_pointer_width = "64", usize, pb, PetaByteSize, PB),
    (target_pointer_width = "64", usize, pib, PetaByteSize, PIB),
    (target_pointer_width = "64", usize, eb, ExaByteSize, EB),
    (target_pointer_width = "64", usize, eib, ExaByteSize, EIB),
] {
    #[cfg(Cfg)]
    impl ByteSize<Ty> {
        #[doc = concat!(
            "Constructs a byte size wrapper from a quantity of `",
            stringify!(Name),
            "` units."
        )]
        #[inline(always)]
        pub const fn Name(size: Ty) -> Self {
            ByteSize(size * <Ty as crate::traits::Trait>::Size)
        }
    }
});

macroweave::repeat!(Ty in [u8, u16, u32, u64, usize] {
    impl ByteSize<Ty> {
        /// Returns byte count as bytes.
        ///
        /// The result is approximate when the byte count cannot be
        /// represented exactly as `f64`. Use [`ByteSize::bytes`] for the
        /// exact underlying integer value.
        #[inline(always)]
        pub const fn as_b(&self) -> f64 {
            self.0 as f64
        }
    }
});

macroweave::repeat!((Cfg, Ty, Name, Trait, Size, Unit) in [
    (all(), u16, as_kb, KiloByteSize, KB, "kilobytes"),
    (all(), u16, as_kib, KiloByteSize, KIB, "kibibytes"),
    (all(), u32, as_kb, KiloByteSize, KB, "kilobytes"),
    (all(), u32, as_kib, KiloByteSize, KIB, "kibibytes"),
    (all(), u32, as_mb, MegaByteSize, MB, "megabytes"),
    (all(), u32, as_mib, MegaByteSize, MIB, "mebibytes"),
    (all(), u32, as_gb, GigaByteSize, GB, "gigabytes"),
    (all(), u32, as_gib, GigaByteSize, GIB, "gibibytes"),
    (all(), u64, as_kb, KiloByteSize, KB, "kilobytes"),
    (all(), u64, as_kib, KiloByteSize, KIB, "kibibytes"),
    (all(), u64, as_mb, MegaByteSize, MB, "megabytes"),
    (all(), u64, as_mib, MegaByteSize, MIB, "mebibytes"),
    (all(), u64, as_gb, GigaByteSize, GB, "gigabytes"),
    (all(), u64, as_gib, GigaByteSize, GIB, "gibibytes"),
    (all(), u64, as_tb, TeraByteSize, TB, "terabytes"),
    (all(), u64, as_tib, TeraByteSize, TIB, "tebibytes"),
    (all(), u64, as_pb, PetaByteSize, PB, "petabytes"),
    (all(), u64, as_pib, PetaByteSize, PIB, "pebibytes"),
    (all(), u64, as_eb, ExaByteSize, EB, "exabytes"),
    (all(), u64, as_eib, ExaByteSize, EIB, "exbibytes"),
    (target_pointer_width = "16", usize, as_kb, KiloByteSize, KB, "kilobytes"),
    (target_pointer_width = "16", usize, as_kib, KiloByteSize, KIB, "kibibytes"),
    (target_pointer_width = "32", usize, as_kb, KiloByteSize, KB, "kilobytes"),
    (target_pointer_width = "32", usize, as_kib, KiloByteSize, KIB, "kibibytes"),
    (target_pointer_width = "32", usize, as_mb, MegaByteSize, MB, "megabytes"),
    (target_pointer_width = "32", usize, as_mib, MegaByteSize, MIB, "mebibytes"),
    (target_pointer_width = "32", usize, as_gb, GigaByteSize, GB, "gigabytes"),
    (target_pointer_width = "32", usize, as_gib, GigaByteSize, GIB, "gibibytes"),
    (target_pointer_width = "64", usize, as_kb, KiloByteSize, KB, "kilobytes"),
    (target_pointer_width = "64", usize, as_kib, KiloByteSize, KIB, "kibibytes"),
    (target_pointer_width = "64", usize, as_mb, MegaByteSize, MB, "megabytes"),
    (target_pointer_width = "64", usize, as_mib, MegaByteSize, MIB, "mebibytes"),
    (target_pointer_width = "64", usize, as_gb, GigaByteSize, GB, "gigabytes"),
    (target_pointer_width = "64", usize, as_gib, GigaByteSize, GIB, "gibibytes"),
    (target_pointer_width = "64", usize, as_tb, TeraByteSize, TB, "terabytes"),
    (target_pointer_width = "64", usize, as_tib, TeraByteSize, TIB, "tebibytes"),
    (target_pointer_width = "64", usize, as_pb, PetaByteSize, PB, "petabytes"),
    (target_pointer_width = "64", usize, as_pib, PetaByteSize, PIB, "pebibytes"),
    (target_pointer_width = "64", usize, as_eb, ExaByteSize, EB, "exabytes"),
    (target_pointer_width = "64", usize, as_eib, ExaByteSize, EIB, "exbibytes"),
] {
    #[cfg(Cfg)]
    impl ByteSize<Ty> {
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
