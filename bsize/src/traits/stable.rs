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

use core::ops::Mul;

use super::private;

/// A sealed trait for integer types that can back [`ByteSize`](crate::ByteSize).
///
/// This trait is implemented only for the unsigned integer types supported by this crate.
pub trait BaseByteSize: private::Sealed + Copy + Sized {
    /// Returns this byte count as an approximate `f64`.
    fn to_f64(&self) -> f64;
}

/// Provides kilobyte-scale unit constants for supported backing integer types.
pub trait KiloByteSize: BaseByteSize + Mul<Output = Self> {
    /// Number of bytes in one kilobyte.
    const KB: Self;

    /// Number of bytes in one kibibyte.
    const KIB: Self;
}

/// Provides megabyte-scale unit constants for supported backing integer types.
pub trait MegaByteSize: KiloByteSize {
    /// Number of bytes in one megabyte.
    const MB: Self;

    /// Number of bytes in one mebibyte.
    const MIB: Self;
}

/// Provides gigabyte-scale unit constants for supported backing integer types.
pub trait GigaByteSize: MegaByteSize {
    /// Number of bytes in one gigabyte.
    const GB: Self;

    /// Number of bytes in one gibibyte.
    const GIB: Self;
}

/// Provides terabyte-scale unit constants for supported backing integer types.
pub trait TeraByteSize: GigaByteSize {
    /// Number of bytes in one terabyte.
    const TB: Self;

    /// Number of bytes in one tebibyte.
    const TIB: Self;
}

/// Provides petabyte-scale unit constants for supported backing integer types.
pub trait PetaByteSize: TeraByteSize {
    /// Number of bytes in one petabyte.
    const PB: Self;

    /// Number of bytes in one pebibyte.
    const PIB: Self;
}

/// Provides exabyte-scale unit constants for supported backing integer types.
pub trait ExaByteSize: PetaByteSize {
    /// Number of bytes in one exabyte.
    const EB: Self;

    /// Number of bytes in one exbibyte.
    const EIB: Self;
}

macroweave::repeat!(Ty in [u8, u16, u32, u64, usize] {
    impl BaseByteSize for Ty {
        fn to_f64(&self) -> f64 {
            *self as f64
        }
    }
});

macroweave::repeat!((Cfg, Trait, Ty, DecimalName, BinaryName, Scale) in [
    (all(), KiloByteSize, u16, KB, KIB, 1),
    (all(), KiloByteSize, u32, KB, KIB, 1),
    (all(), MegaByteSize, u32, MB, MIB, 2),
    (all(), GigaByteSize, u32, GB, GIB, 3),
    (all(), KiloByteSize, u64, KB, KIB, 1),
    (all(), MegaByteSize, u64, MB, MIB, 2),
    (all(), GigaByteSize, u64, GB, GIB, 3),
    (all(), TeraByteSize, u64, TB, TIB, 4),
    (all(), PetaByteSize, u64, PB, PIB, 5),
    (all(), ExaByteSize, u64, EB, EIB, 6),
    (target_pointer_width = "16", KiloByteSize, usize, KB, KIB, 1),
    (target_pointer_width = "32", KiloByteSize, usize, KB, KIB, 1),
    (target_pointer_width = "32", MegaByteSize, usize, MB, MIB, 2),
    (target_pointer_width = "32", GigaByteSize, usize, GB, GIB, 3),
    (target_pointer_width = "64", KiloByteSize, usize, KB, KIB, 1),
    (target_pointer_width = "64", MegaByteSize, usize, MB, MIB, 2),
    (target_pointer_width = "64", GigaByteSize, usize, GB, GIB, 3),
    (target_pointer_width = "64", TeraByteSize, usize, TB, TIB, 4),
    (target_pointer_width = "64", PetaByteSize, usize, PB, PIB, 5),
    (target_pointer_width = "64", ExaByteSize, usize, EB, EIB, 6),
] {
    #[cfg(Cfg)]
    impl Trait for Ty {
        const DecimalName: Self = Ty::pow(1000, Scale);
        const BinaryName: Self = Ty::pow(1024, Scale);
    }
});
