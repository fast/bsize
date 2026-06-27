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
    ($ty:ty => { $($name:ident = $size:literal),* $(,)? }) => {
        impl BSize<$ty> {
            $(
                #[doc = concat!(
                    "Constructs a byte size wrapper from a quantity of `",
                    stringify!($name),
                    "` units."
                )]
                #[inline(always)]
                pub const fn $name(size: $ty) -> Self {
                    BSize(size * $size)
                }
            )*
        }
    };
}

macro_rules! impl_accessors {
    ($ty:ty => { $($name:ident = $size:literal => $unit:literal),* $(,)? }) => {
        impl BSize<$ty> {
            $(
                #[doc = concat!("Returns byte count as ", $unit, ".")]
                ///
                /// The result is approximate when the byte count cannot be
                /// represented exactly as `f64`.
                #[inline(always)]
                pub const fn $name(&self) -> f64 {
                    (self.0 as f64) / ($size as f64)
                }
            )*
        }
    };
}

macro_rules! impl_usize_constructors {
    (through_kilo) => {
        impl_constructors!(usize => {
            kb = 1_000,
            kib = 1_024,
        });
    };
    (through_giga) => {
        impl_usize_constructors!(through_kilo);
        impl_constructors!(usize => {
            mb = 1_000_000,
            mib = 1_048_576,
            gb = 1_000_000_000,
            gib = 1_073_741_824,
        });
    };
    (through_exa) => {
        impl_usize_constructors!(through_giga);
        impl_constructors!(usize => {
            tb = 1_000_000_000_000,
            tib = 1_099_511_627_776,
            pb = 1_000_000_000_000_000,
            pib = 1_125_899_906_842_624,
            eb = 1_000_000_000_000_000_000,
            eib = 1_152_921_504_606_846_976,
        });
    };
}

macro_rules! impl_usize_accessors {
    (through_kilo) => {
        impl_accessors!(usize => {
            as_kb = 1_000usize => "kilobytes",
            as_kib = 1_024usize => "kibibytes",
        });
    };
    (through_giga) => {
        impl_usize_accessors!(through_kilo);
        impl_accessors!(usize => {
            as_mb = 1_000_000usize => "megabytes",
            as_mib = 1_048_576usize => "mebibytes",
            as_gb = 1_000_000_000usize => "gigabytes",
            as_gib = 1_073_741_824usize => "gibibytes",
        });
    };
    (through_exa) => {
        impl_usize_accessors!(through_giga);
        impl_accessors!(usize => {
            as_tb = 1_000_000_000_000usize => "terabytes",
            as_tib = 1_099_511_627_776usize => "tebibytes",
            as_pb = 1_000_000_000_000_000usize => "petabytes",
            as_pib = 1_125_899_906_842_624usize => "pebibytes",
            as_eb = 1_000_000_000_000_000_000usize => "exabytes",
            as_eib = 1_152_921_504_606_846_976usize => "exbibytes",
        });
    };
}

impl_constructors!(u16 => {
    kb = 1_000,
    kib = 1_024,
});

impl_constructors!(u32 => {
    kb = 1_000,
    kib = 1_024,
    mb = 1_000_000,
    mib = 1_048_576,
    gb = 1_000_000_000,
    gib = 1_073_741_824,
});

impl_constructors!(u64 => {
    kb = 1_000,
    kib = 1_024,
    mb = 1_000_000,
    mib = 1_048_576,
    gb = 1_000_000_000,
    gib = 1_073_741_824,
    tb = 1_000_000_000_000,
    tib = 1_099_511_627_776,
    pb = 1_000_000_000_000_000,
    pib = 1_125_899_906_842_624,
    eb = 1_000_000_000_000_000_000,
    eib = 1_152_921_504_606_846_976,
});

impl_accessors!(u16 => {
    as_kb = 1_000u16 => "kilobytes",
    as_kib = 1_024u16 => "kibibytes",
});

impl_accessors!(u32 => {
    as_kb = 1_000u32 => "kilobytes",
    as_kib = 1_024u32 => "kibibytes",
    as_mb = 1_000_000u32 => "megabytes",
    as_mib = 1_048_576u32 => "mebibytes",
    as_gb = 1_000_000_000u32 => "gigabytes",
    as_gib = 1_073_741_824u32 => "gibibytes",
});

impl_accessors!(u64 => {
    as_kb = 1_000u64 => "kilobytes",
    as_kib = 1_024u64 => "kibibytes",
    as_mb = 1_000_000u64 => "megabytes",
    as_mib = 1_048_576u64 => "mebibytes",
    as_gb = 1_000_000_000u64 => "gigabytes",
    as_gib = 1_073_741_824u64 => "gibibytes",
    as_tb = 1_000_000_000_000u64 => "terabytes",
    as_tib = 1_099_511_627_776u64 => "tebibytes",
    as_pb = 1_000_000_000_000_000u64 => "petabytes",
    as_pib = 1_125_899_906_842_624u64 => "pebibytes",
    as_eb = 1_000_000_000_000_000_000u64 => "exabytes",
    as_eib = 1_152_921_504_606_846_976u64 => "exbibytes",
});

#[cfg(target_pointer_width = "16")]
impl_usize_constructors!(through_kilo);
#[cfg(target_pointer_width = "32")]
impl_usize_constructors!(through_giga);
#[cfg(target_pointer_width = "64")]
impl_usize_constructors!(through_exa);

#[cfg(target_pointer_width = "16")]
impl_usize_accessors!(through_kilo);
#[cfg(target_pointer_width = "32")]
impl_usize_accessors!(through_giga);
#[cfg(target_pointer_width = "64")]
impl_usize_accessors!(through_exa);

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
