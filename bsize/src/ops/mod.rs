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

#[cfg(feature = "nightly")]
mod nightly;
#[cfg(not(feature = "nightly"))]
mod stable;

#[cfg(test)]
mod tests {
    use crate::BSize;
    use crate::BSize8;
    use crate::BSize16;
    use crate::BSize32;
    use crate::BSize64;

    #[test]
    fn adds_byte_sizes() {
        assert_eq!((BSize8::b(3) + BSize8::b(5)).0, 8);
        assert_eq!((BSize16::b(3) + BSize16::b(5)).0, 8);
        assert_eq!((BSize32::b(3) + BSize32::b(5)).0, 8);
        assert_eq!((BSize64::b(3) + BSize64::b(5)).0, 8);
        assert_eq!((BSize::b(3) + BSize::b(5)).0, 8);
    }

    #[test]
    fn add_assigns_byte_sizes() {
        let mut size = BSize::b(3);
        size += BSize::b(5);
        assert_eq!(size.0, 8);
    }

    #[test]
    fn subtracts_byte_sizes() {
        assert_eq!((BSize8::b(8) - BSize8::b(5)).0, 3);
        assert_eq!((BSize16::b(8) - BSize16::b(5)).0, 3);
        assert_eq!((BSize32::b(8) - BSize32::b(5)).0, 3);
        assert_eq!((BSize64::b(8) - BSize64::b(5)).0, 3);
        assert_eq!((BSize::b(8) - BSize::b(5)).0, 3);
    }

    #[test]
    fn sub_assigns_byte_sizes() {
        let mut size = BSize::b(8);
        size -= BSize::b(5);
        assert_eq!(size.0, 3);
    }
}
