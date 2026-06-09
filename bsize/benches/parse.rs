// // Copyright 2026 FastLabs Developers
// //
// // Licensed under the Apache License, Version 2.0 (the "License");
// // you may not use this file except in compliance with the License.
// // You may obtain a copy of the License at
// //
// //     http://www.apache.org/licenses/LICENSE-2.0
// //
// // Unless required by applicable law or agreed to in writing, software
// // distributed under the License is distributed on an "AS IS" BASIS,
// // WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// // See the License for the specific language governing permissions and
// // limitations under the License.

// use bsize::BSize;
// use divan::black_box;
// use parse_size::parse_size;

fn main() {
    divan::main();
}
//
// #[divan::bench]
// fn bsize_bytes() -> u64 {
//     BSize::<u64>::parse(black_box(b"123456789B")).unwrap().0
// }
//
// #[divan::bench]
// fn parse_size_bytes() -> u64 {
//     parse_size(black_box(b"123456789B")).unwrap()
// }
//
// #[divan::bench]
// fn bsize_decimal() -> u64 {
//     BSize::<u64>::parse(black_box(b"123.456MB")).unwrap().0
// }
//
// #[divan::bench]
// fn parse_size_decimal() -> u64 {
//     parse_size(black_box(b"123.456MB")).unwrap()
// }
//
// #[divan::bench]
// fn bsize_binary_decimal() -> u64 {
//     BSize::<u64>::parse(black_box(b"1.5KiB")).unwrap().0
// }
//
// #[divan::bench]
// fn parse_size_binary_decimal() -> u64 {
//     parse_size(black_box(b"1.5KiB")).unwrap()
// }
//
// #[divan::bench]
// fn bsize_tiny_decimal() -> u64 {
//     BSize::<u64>::parse(black_box(b"0.001KB")).unwrap().0
// }
//
// #[divan::bench]
// fn parse_size_tiny_decimal() -> u64 {
//     parse_size(black_box(b"0.001KB")).unwrap()
// }
//
// #[divan::bench]
// fn bsize_u64_max() -> u64 {
//     BSize::<u64>::parse(black_box(b"18.446744073709551615EB"))
//         .unwrap()
//         .0
// }
//
// #[divan::bench]
// fn parse_size_u64_max() -> u64 {
//     parse_size(black_box(b"18.446744073709551615EB")).unwrap()
// }
