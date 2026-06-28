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

use crate::types::BSize;

macro_rules! impl_ops {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ops::Add<BSize<$ty>> for BSize<$ty> {
                type Output = Self;

                #[inline(always)]
                fn add(self, rhs: BSize<$ty>) -> Self::Output {
                    BSize(self.0 + rhs.0)
                }
            }

            impl ops::AddAssign<BSize<$ty>> for BSize<$ty> {
                #[inline(always)]
                fn add_assign(&mut self, rhs: BSize<$ty>) {
                    self.0 += rhs.0;
                }
            }

            impl ops::Sub<BSize<$ty>> for BSize<$ty> {
                type Output = Self;

                #[inline(always)]
                fn sub(self, rhs: BSize<$ty>) -> Self::Output {
                    BSize(self.0 - rhs.0)
                }
            }

            impl ops::SubAssign<BSize<$ty>> for BSize<$ty> {
                #[inline(always)]
                fn sub_assign(&mut self, rhs: BSize<$ty>) {
                    self.0 -= rhs.0;
                }
            }
        )*
    };
}

impl_ops!(u8, u16, u32, u64, usize);
