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

    #[test]
    fn adds_byte_sizes() {
        assert_eq!((BSize::<u8>(3) + BSize(5)).0, 8);
        assert_eq!((BSize::<u16>(3) + BSize(5)).0, 8);
        assert_eq!((BSize::<u32>(3) + BSize(5)).0, 8);
        assert_eq!((BSize::<u64>(3) + BSize(5)).0, 8);
        assert_eq!((BSize::<usize>(3) + BSize(5)).0, 8);
    }

    #[test]
    fn add_assigns_byte_sizes() {
        let mut size = BSize::<usize>(3);
        size += BSize(5);
        assert_eq!(size.0, 8);
    }

    #[test]
    fn subtracts_byte_sizes() {
        assert_eq!((BSize::<u8>(8) - BSize(5)).0, 3);
        assert_eq!((BSize::<u16>(8) - BSize(5)).0, 3);
        assert_eq!((BSize::<u32>(8) - BSize(5)).0, 3);
        assert_eq!((BSize::<u64>(8) - BSize(5)).0, 3);
        assert_eq!((BSize::<usize>(8) - BSize(5)).0, 3);
    }

    #[test]
    fn sub_assigns_byte_sizes() {
        let mut size = BSize::<usize>(8);
        size -= BSize(5);
        assert_eq!(size.0, 3);
    }
}
