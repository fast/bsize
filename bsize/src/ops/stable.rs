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

use crate::types::ByteSize;

macroweave::repeat!(Ty in [u8, u16, u32, u64, usize] {
    impl ops::Add<ByteSize<Ty>> for ByteSize<Ty> {
        type Output = Self;

        #[inline(always)]
        fn add(self, rhs: ByteSize<Ty>) -> Self::Output {
            ByteSize(self.0 + rhs.0)
        }
    }

    impl ops::AddAssign<ByteSize<Ty>> for ByteSize<Ty> {
        #[inline(always)]
        fn add_assign(&mut self, rhs: ByteSize<Ty>) {
            self.0 += rhs.0;
        }
    }

    impl ops::Sub<ByteSize<Ty>> for ByteSize<Ty> {
        type Output = Self;

        #[inline(always)]
        fn sub(self, rhs: ByteSize<Ty>) -> Self::Output {
            ByteSize(self.0 - rhs.0)
        }
    }

    impl ops::SubAssign<ByteSize<Ty>> for ByteSize<Ty> {
        #[inline(always)]
        fn sub_assign(&mut self, rhs: ByteSize<Ty>) {
            self.0 -= rhs.0;
        }
    }
});
