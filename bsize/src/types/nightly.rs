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

impl<T: KiloByteSize> BSize<T> {
    /// Constructs a byte size wrapper from a quantity of `kb` units.
    #[inline(always)]
    pub const fn kb(size: T) -> Self
    where
        T: [const] KiloByteSize,
    {
        BSize(size * T::KB)
    }

    /// Constructs a byte size wrapper from a quantity of `kib` units.
    #[inline(always)]
    pub const fn kib(size: T) -> Self
    where
        T: [const] KiloByteSize,
    {
        BSize(size * T::KIB)
    }

    /// Returns byte count as kilobytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_kb(&self) -> f64
    where
        T: [const] KiloByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::KB.as_f64()
    }

    /// Returns byte count as kibibytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_kib(&self) -> f64
    where
        T: [const] KiloByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::KIB.as_f64()
    }
}

impl<T: MegaByteSize> BSize<T> {
    /// Constructs a byte size wrapper from a quantity of `mb` units.
    #[inline(always)]
    pub const fn mb(size: T) -> Self
    where
        T: [const] MegaByteSize,
    {
        BSize(size * T::MB)
    }

    /// Constructs a byte size wrapper from a quantity of `mib` units.
    #[inline(always)]
    pub const fn mib(size: T) -> Self
    where
        T: [const] MegaByteSize,
    {
        BSize(size * T::MIB)
    }

    /// Returns byte count as megabytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_mb(&self) -> f64
    where
        T: [const] MegaByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::MB.as_f64()
    }

    /// Returns byte count as mebibytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_mib(&self) -> f64
    where
        T: [const] MegaByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::MIB.as_f64()
    }
}

impl<T: GigaByteSize> BSize<T> {
    /// Constructs a byte size wrapper from a quantity of `gb` units.
    #[inline(always)]
    pub const fn gb(size: T) -> Self
    where
        T: [const] GigaByteSize,
    {
        BSize(size * T::GB)
    }

    /// Constructs a byte size wrapper from a quantity of `gib` units.
    #[inline(always)]
    pub const fn gib(size: T) -> Self
    where
        T: [const] GigaByteSize,
    {
        BSize(size * T::GIB)
    }

    /// Returns byte count as gigabytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_gb(&self) -> f64
    where
        T: [const] GigaByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::GB.as_f64()
    }

    /// Returns byte count as gibibytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_gib(&self) -> f64
    where
        T: [const] GigaByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::GIB.as_f64()
    }
}

impl<T: TeraByteSize> BSize<T> {
    /// Constructs a byte size wrapper from a quantity of `tb` units.
    #[inline(always)]
    pub const fn tb(size: T) -> Self
    where
        T: [const] TeraByteSize,
    {
        BSize(size * T::TB)
    }

    /// Constructs a byte size wrapper from a quantity of `tib` units.
    #[inline(always)]
    pub const fn tib(size: T) -> Self
    where
        T: [const] TeraByteSize,
    {
        BSize(size * T::TIB)
    }

    /// Returns byte count as terabytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_tb(&self) -> f64
    where
        T: [const] TeraByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::TB.as_f64()
    }

    /// Returns byte count as tebibytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_tib(&self) -> f64
    where
        T: [const] TeraByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::TIB.as_f64()
    }
}

impl<T: PetaByteSize> BSize<T> {
    /// Constructs a byte size wrapper from a quantity of `pb` units.
    #[inline(always)]
    pub const fn pb(size: T) -> Self
    where
        T: [const] PetaByteSize,
    {
        BSize(size * T::PB)
    }

    /// Constructs a byte size wrapper from a quantity of `pib` units.
    #[inline(always)]
    pub const fn pib(size: T) -> Self
    where
        T: [const] PetaByteSize,
    {
        BSize(size * T::PIB)
    }

    /// Returns byte count as petabytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_pb(&self) -> f64
    where
        T: [const] PetaByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::PB.as_f64()
    }

    /// Returns byte count as pebibytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_pib(&self) -> f64
    where
        T: [const] PetaByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::PIB.as_f64()
    }
}

impl<T: ExaByteSize> BSize<T> {
    /// Constructs a byte size wrapper from a quantity of `eb` units.
    #[inline(always)]
    pub const fn eb(size: T) -> Self
    where
        T: [const] ExaByteSize,
    {
        BSize(size * T::EB)
    }

    /// Constructs a byte size wrapper from a quantity of `eib` units.
    #[inline(always)]
    pub const fn eib(size: T) -> Self
    where
        T: [const] ExaByteSize,
    {
        BSize(size * T::EIB)
    }

    /// Returns byte count as exabytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_eb(&self) -> f64
    where
        T: [const] ExaByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::EB.as_f64()
    }

    /// Returns byte count as exbibytes.
    ///
    /// The result is approximate when the byte count cannot be represented
    /// exactly as `f64`.
    #[inline(always)]
    pub const fn as_eib(&self) -> f64
    where
        T: [const] ExaByteSize + [const] ByteSize,
    {
        self.0.as_f64() / T::EIB.as_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::BSize;
    use crate::assert_close;

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
}
