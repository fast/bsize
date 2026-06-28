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

use core::ops;

use crate::traits::ByteSize;
use crate::types::BSize;

const impl<T> ops::Add<BSize<T>> for BSize<T>
where
    T: [const] ByteSize + [const] ops::Add<Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: BSize<T>) -> Self::Output {
        BSize(self.0 + rhs.0)
    }
}

const impl<T> ops::AddAssign<BSize<T>> for BSize<T>
where
    T: [const] ByteSize + [const] ops::AddAssign,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: BSize<T>) {
        self.0 += rhs.0;
    }
}

const impl<T> ops::Sub<BSize<T>> for BSize<T>
where
    T: [const] ByteSize + [const] ops::Sub<Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: BSize<T>) -> Self::Output {
        BSize(self.0 - rhs.0)
    }
}

const impl<T> ops::SubAssign<BSize<T>> for BSize<T>
where
    T: [const] ByteSize + [const] ops::SubAssign,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: BSize<T>) {
        self.0 -= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::BSize;

    #[test]
    fn arithmetic_is_const() {
        const SUM: BSize<u64> = BSize::b(3_u64) + BSize::b(5_u64);
        const DIFFERENCE: BSize<u64> = BSize::b(8_u64) - BSize::b(5_u64);
        const ASSIGNED: BSize<u64> = {
            let mut size = BSize::b(3_u64);
            size += BSize::b(5_u64);
            size -= BSize::b(2_u64);
            size
        };

        assert_eq!(SUM, BSize::b(8));
        assert_eq!(DIFFERENCE, BSize::b(3));
        assert_eq!(ASSIGNED, BSize::b(6));
    }
}
