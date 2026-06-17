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

mod private {
    pub trait Sealed {}

    impl Sealed for usize {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
}

/// A marker trait for all supported byte size underneath type.
pub trait ByteSize: private::Sealed {}

impl ByteSize for usize {}
impl ByteSize for u8 {}
impl ByteSize for u16 {}
impl ByteSize for u32 {}
impl ByteSize for u64 {}

/// A trait for all displayable byte size underneath type.
pub trait Displayable: ByteSize {
    /// Convert the byte size payload to a canonicalized floating point representation,
    /// which will then be used for display purposes.
    fn canonicalize(&self) -> f64;
}

macro_rules! impl_displayable {
  ($($ty:ty),* $(,)?) => ($(
      impl Displayable for $ty {
          fn canonicalize(&self) -> f64 {
              *self as f64
          }
      }
  )*)
}

impl_displayable!(u8, u16, u32, u64, usize);
