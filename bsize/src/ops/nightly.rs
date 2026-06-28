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

use crate::ByteSize;
use crate::traits::BaseByteSize;

const impl<T> ops::Add<ByteSize<T>> for ByteSize<T>
where
    T: [const] BaseByteSize + [const] ops::Add<Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: ByteSize<T>) -> Self::Output {
        ByteSize(self.0 + rhs.0)
    }
}

const impl<T> ops::AddAssign<ByteSize<T>> for ByteSize<T>
where
    T: [const] BaseByteSize + [const] ops::AddAssign,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: ByteSize<T>) {
        self.0 += rhs.0;
    }
}

const impl<T> ops::Sub<ByteSize<T>> for ByteSize<T>
where
    T: [const] BaseByteSize + [const] ops::Sub<Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: ByteSize<T>) -> Self::Output {
        ByteSize(self.0 - rhs.0)
    }
}

const impl<T> ops::SubAssign<ByteSize<T>> for ByteSize<T>
where
    T: [const] BaseByteSize + [const] ops::SubAssign,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: ByteSize<T>) {
        self.0 -= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use crate::BSize64;

    #[test]
    fn arithmetic_is_const() {
        const SUM: BSize64 = BSize64::b(3) + BSize64::b(5);
        const DIFFERENCE: BSize64 = BSize64::b(8) - BSize64::b(5);
        const ASSIGNED: BSize64 = {
            let mut size = BSize64::b(3);
            size += BSize64::b(5);
            size -= BSize64::b(2);
            size
        };

        assert_eq!(SUM, BSize64::b(8));
        assert_eq!(DIFFERENCE, BSize64::b(3));
        assert_eq!(ASSIGNED, BSize64::b(6));
    }
}
