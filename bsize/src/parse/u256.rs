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

#[derive(Clone, Copy)]
pub(in crate::parse) struct U256 {
    pub(in crate::parse) hi: u128,
    pub(in crate::parse) lo: u128,
}

impl U256 {
    pub(in crate::parse) fn multiply(lhs: u128, rhs: u128) -> Self {
        let mask = u64::MAX as u128;
        let lhs_lo = lhs & mask;
        let lhs_hi = lhs >> 64;
        let rhs_lo = rhs & mask;
        let rhs_hi = rhs >> 64;

        let low = lhs_lo * rhs_lo;
        let mid_left = lhs_lo * rhs_hi;
        let mid_right = lhs_hi * rhs_lo;
        let high = lhs_hi * rhs_hi;

        let carry = (low >> 64) + (mid_left & mask) + (mid_right & mask);
        let lo = (low & mask) | ((carry & mask) << 64);
        let hi = high + (mid_left >> 64) + (mid_right >> 64) + (carry >> 64);

        Self { hi, lo }
    }

    pub(in crate::parse) fn div_rem_10(self) -> (Self, u8) {
        let mut remainder = 0_u128;

        let (hi_hi, next) = div_limb(self.hi >> 64, remainder);
        remainder = next;
        let (hi_lo, next) = div_limb(self.hi as u64 as u128, remainder);
        remainder = next;
        let (lo_hi, next) = div_limb(self.lo >> 64, remainder);
        remainder = next;
        let (lo_lo, remainder) = div_limb(self.lo as u64 as u128, remainder);

        (
            Self {
                hi: (hi_hi << 64) | hi_lo,
                lo: (lo_hi << 64) | lo_lo,
            },
            remainder as u8,
        )
    }

    pub(in crate::parse) fn checked_add_one(self) -> Option<Self> {
        let (lo, carry) = self.lo.overflowing_add(1);
        let hi = if carry {
            self.hi.checked_add(1)?
        } else {
            self.hi
        };

        Some(Self { hi, lo })
    }

    pub(in crate::parse) fn try_into_u128(self) -> Option<u128> {
        if self.hi == 0 { Some(self.lo) } else { None }
    }
}

fn div_limb(limb: u128, remainder: u128) -> (u128, u128) {
    let value = (remainder << 64) | limb;
    (value / 10, value % 10)
}
