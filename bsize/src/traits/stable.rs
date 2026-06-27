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

use super::ByteSize;

/// A trait for byte size payload types that can represent kilobyte-scale units.
pub trait KiloByteSize: ByteSize + Mul<Output = Self> + Sized {
    /// Number of bytes in one kilobyte.
    const KB: Self;

    /// Number of bytes in one kibibyte.
    const KIB: Self;
}

/// A trait for byte size payload types that can represent megabyte-scale units.
pub trait MegaByteSize: KiloByteSize {
    /// Number of bytes in one megabyte.
    const MB: Self;

    /// Number of bytes in one mebibyte.
    const MIB: Self;
}

/// A trait for byte size payload types that can represent gigabyte-scale units.
pub trait GigaByteSize: MegaByteSize {
    /// Number of bytes in one gigabyte.
    const GB: Self;

    /// Number of bytes in one gibibyte.
    const GIB: Self;
}

/// A trait for byte size payload types that can represent terabyte-scale units.
pub trait TeraByteSize: GigaByteSize {
    /// Number of bytes in one terabyte.
    const TB: Self;

    /// Number of bytes in one tebibyte.
    const TIB: Self;
}

/// A trait for byte size payload types that can represent petabyte-scale units.
pub trait PetaByteSize: TeraByteSize {
    /// Number of bytes in one petabyte.
    const PB: Self;

    /// Number of bytes in one pebibyte.
    const PIB: Self;
}

/// A trait for byte size payload types that can represent exabyte-scale units.
pub trait ExaByteSize: PetaByteSize {
    /// Number of bytes in one exabyte.
    const EB: Self;

    /// Number of bytes in one exbibyte.
    const EIB: Self;
}

macro_rules! impl_size_trait {
    ($trait:ident for $ty:ty { $($name:ident = $value:literal),* $(,)? }) => {
        impl $trait for $ty {
            $(const $name: Self = $value;)*
        }
    };
}

macro_rules! impl_usize_size_traits {
    (through_kilo) => {
        impl_size_trait!(KiloByteSize for usize {
            KB = 1_000,
            KIB = 1_024,
        });
    };
    (through_giga) => {
        impl_usize_size_traits!(through_kilo);
        impl_size_trait!(MegaByteSize for usize {
            MB = 1_000_000,
            MIB = 1_048_576,
        });
        impl_size_trait!(GigaByteSize for usize {
            GB = 1_000_000_000,
            GIB = 1_073_741_824,
        });
    };
    (through_exa) => {
        impl_usize_size_traits!(through_giga);
        impl_size_trait!(TeraByteSize for usize {
            TB = 1_000_000_000_000,
            TIB = 1_099_511_627_776,
        });
        impl_size_trait!(PetaByteSize for usize {
            PB = 1_000_000_000_000_000,
            PIB = 1_125_899_906_842_624,
        });
        impl_size_trait!(ExaByteSize for usize {
            EB = 1_000_000_000_000_000_000,
            EIB = 1_152_921_504_606_846_976,
        });
    };
}

impl_size_trait!(KiloByteSize for u16 {
    KB = 1_000,
    KIB = 1_024,
});

impl_size_trait!(KiloByteSize for u32 {
    KB = 1_000,
    KIB = 1_024,
});

impl_size_trait!(MegaByteSize for u32 {
    MB = 1_000_000,
    MIB = 1_048_576,
});

impl_size_trait!(GigaByteSize for u32 {
    GB = 1_000_000_000,
    GIB = 1_073_741_824,
});

impl_size_trait!(KiloByteSize for u64 {
    KB = 1_000,
    KIB = 1_024,
});

impl_size_trait!(MegaByteSize for u64 {
    MB = 1_000_000,
    MIB = 1_048_576,
});

impl_size_trait!(GigaByteSize for u64 {
    GB = 1_000_000_000,
    GIB = 1_073_741_824,
});

impl_size_trait!(TeraByteSize for u64 {
    TB = 1_000_000_000_000,
    TIB = 1_099_511_627_776,
});

impl_size_trait!(PetaByteSize for u64 {
    PB = 1_000_000_000_000_000,
    PIB = 1_125_899_906_842_624,
});

impl_size_trait!(ExaByteSize for u64 {
    EB = 1_000_000_000_000_000_000,
    EIB = 1_152_921_504_606_846_976,
});

#[cfg(target_pointer_width = "16")]
impl_usize_size_traits!(through_kilo);
#[cfg(target_pointer_width = "32")]
impl_usize_size_traits!(through_giga);
#[cfg(target_pointer_width = "64")]
impl_usize_size_traits!(through_exa);
