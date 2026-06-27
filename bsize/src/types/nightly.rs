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

use crate::traits::ExaByteSize;
use crate::traits::GigaByteSize;
use crate::traits::KiloByteSize;
use crate::traits::MegaByteSize;
use crate::traits::PetaByteSize;
use crate::traits::TeraByteSize;
use crate::types::BSize;

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
}
