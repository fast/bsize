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

macro_rules! impl_marker {
  ($t:ident for $($ty:ty),* $(,)?) => ($(
      impl $t for $ty {}
  )*)
}

mod private {
    pub trait Sealed {}
    impl_marker!(Sealed for u8, u16, u32, u64, usize);
}

macro_rules! define_byte_size_trait {
    (trait_prefix = [$($trait_prefix:tt)*];) => {
        /// A marker trait for all supported byte size underlying types.
        ///
        /// The conversion to `f64` is approximate when the byte count cannot be
        /// represented exactly as `f64`.
        pub $($trait_prefix)* trait ByteSize: Copy + private::Sealed {
            /// Returns the value as an approximate `f64`.
            fn as_f64(&self) -> f64;
        }
    };
}

#[cfg(not(feature = "nightly"))]
define_byte_size_trait! {
    trait_prefix = [];
}

#[cfg(feature = "nightly")]
define_byte_size_trait! {
    trait_prefix = [const];
}

macro_rules! impl_byte_size {
    (impl_prefix = []; $($ty:ty),* $(,)?) => {
        $(
            impl ByteSize for $ty {
                fn as_f64(&self) -> f64 {
                    *self as f64
                }
            }
        )*
    };
    (impl_prefix = [const]; $($ty:ty),* $(,)?) => {
        $(
            const impl ByteSize for $ty {
                fn as_f64(&self) -> f64 {
                    *self as f64
                }
            }
        )*
    };
}

#[cfg(not(feature = "nightly"))]
impl_byte_size!(impl_prefix = []; u8, u16, u32, u64, usize);

#[cfg(feature = "nightly")]
impl_byte_size!(impl_prefix = [const]; u8, u16, u32, u64, usize);

macro_rules! define_unit_traits {
    (
        trait_prefix = [$($trait_prefix:tt)*];
        const_bound = [$($const_bound:tt)*];
    ) => {
        /// A trait for byte size payload types that can represent kilobyte-scale units.
        pub $($trait_prefix)* trait KiloByteSize: ByteSize + $($const_bound)* Mul<Output = Self> + Sized {
            /// Number of bytes in one kilobyte.
            const KB: Self;

            /// Number of bytes in one kibibyte.
            const KIB: Self;
        }

        /// A trait for byte size payload types that can represent megabyte-scale units.
        pub $($trait_prefix)* trait MegaByteSize: $($const_bound)* KiloByteSize {
            /// Number of bytes in one megabyte.
            const MB: Self;

            /// Number of bytes in one mebibyte.
            const MIB: Self;
        }

        /// A trait for byte size payload types that can represent gigabyte-scale units.
        pub $($trait_prefix)* trait GigaByteSize: $($const_bound)* MegaByteSize {
            /// Number of bytes in one gigabyte.
            const GB: Self;

            /// Number of bytes in one gibibyte.
            const GIB: Self;
        }

        /// A trait for byte size payload types that can represent terabyte-scale units.
        pub $($trait_prefix)* trait TeraByteSize: $($const_bound)* GigaByteSize {
            /// Number of bytes in one terabyte.
            const TB: Self;

            /// Number of bytes in one tebibyte.
            const TIB: Self;
        }

        /// A trait for byte size payload types that can represent petabyte-scale units.
        pub $($trait_prefix)* trait PetaByteSize: $($const_bound)* TeraByteSize {
            /// Number of bytes in one petabyte.
            const PB: Self;

            /// Number of bytes in one pebibyte.
            const PIB: Self;
        }

        /// A trait for byte size payload types that can represent exabyte-scale units.
        pub $($trait_prefix)* trait ExaByteSize: $($const_bound)* PetaByteSize {
            /// Number of bytes in one exabyte.
            const EB: Self;

            /// Number of bytes in one exbibyte.
            const EIB: Self;
        }
    };
}

macro_rules! impl_size_trait {
    ([$($impl_prefix:tt)*] $trait:ident for $ty:ty { $($name:ident = $value:literal),* $(,)? }) => {
        $($impl_prefix)* impl $trait for $ty {
            $(const $name: Self = $value;)*
        }
    };
}

macro_rules! impl_usize_size_traits {
    ([$($impl_prefix:tt)*] through_kilo) => {
        impl_size_trait!([$($impl_prefix)*] KiloByteSize for usize {
            KB = 1_000,
            KIB = 1_024,
        });
    };
    ([$($impl_prefix:tt)*] through_giga) => {
        impl_usize_size_traits!([$($impl_prefix)*] through_kilo);
        impl_size_trait!([$($impl_prefix)*] MegaByteSize for usize {
            MB = 1_000_000,
            MIB = 1_048_576,
        });
        impl_size_trait!([$($impl_prefix)*] GigaByteSize for usize {
            GB = 1_000_000_000,
            GIB = 1_073_741_824,
        });
    };
    ([$($impl_prefix:tt)*] through_exa) => {
        impl_usize_size_traits!([$($impl_prefix)*] through_giga);
        impl_size_trait!([$($impl_prefix)*] TeraByteSize for usize {
            TB = 1_000_000_000_000,
            TIB = 1_099_511_627_776,
        });
        impl_size_trait!([$($impl_prefix)*] PetaByteSize for usize {
            PB = 1_000_000_000_000_000,
            PIB = 1_125_899_906_842_624,
        });
        impl_size_trait!([$($impl_prefix)*] ExaByteSize for usize {
            EB = 1_000_000_000_000_000_000,
            EIB = 1_152_921_504_606_846_976,
        });
    };
}

macro_rules! impl_unit_traits {
    (impl_prefix = [$($impl_prefix:tt)*];) => {
        impl_size_trait!([$($impl_prefix)*] KiloByteSize for u16 {
            KB = 1_000,
            KIB = 1_024,
        });

        impl_size_trait!([$($impl_prefix)*] KiloByteSize for u32 {
            KB = 1_000,
            KIB = 1_024,
        });

        impl_size_trait!([$($impl_prefix)*] MegaByteSize for u32 {
            MB = 1_000_000,
            MIB = 1_048_576,
        });

        impl_size_trait!([$($impl_prefix)*] GigaByteSize for u32 {
            GB = 1_000_000_000,
            GIB = 1_073_741_824,
        });

        impl_size_trait!([$($impl_prefix)*] KiloByteSize for u64 {
            KB = 1_000,
            KIB = 1_024,
        });

        impl_size_trait!([$($impl_prefix)*] MegaByteSize for u64 {
            MB = 1_000_000,
            MIB = 1_048_576,
        });

        impl_size_trait!([$($impl_prefix)*] GigaByteSize for u64 {
            GB = 1_000_000_000,
            GIB = 1_073_741_824,
        });

        impl_size_trait!([$($impl_prefix)*] TeraByteSize for u64 {
            TB = 1_000_000_000_000,
            TIB = 1_099_511_627_776,
        });

        impl_size_trait!([$($impl_prefix)*] PetaByteSize for u64 {
            PB = 1_000_000_000_000_000,
            PIB = 1_125_899_906_842_624,
        });

        impl_size_trait!([$($impl_prefix)*] ExaByteSize for u64 {
            EB = 1_000_000_000_000_000_000,
            EIB = 1_152_921_504_606_846_976,
        });

        #[cfg(target_pointer_width = "16")]
        impl_usize_size_traits!([$($impl_prefix)*] through_kilo);

        #[cfg(target_pointer_width = "32")]
        impl_usize_size_traits!([$($impl_prefix)*] through_giga);

        #[cfg(target_pointer_width = "64")]
        impl_usize_size_traits!([$($impl_prefix)*] through_exa);
    };
}

#[cfg(feature = "nightly")]
mod nightly;
#[cfg(not(feature = "nightly"))]
mod stable;

#[cfg(feature = "nightly")]
pub use self::nightly::*;
#[cfg(not(feature = "nightly"))]
pub use self::stable::*;
