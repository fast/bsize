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

use std::fmt;

use bsize::BSize64;
use bsize::ParseError;

#[derive(Clone, Copy)]
struct ParseCase {
    name: &'static str,
    input: &'static str,
}

impl fmt::Display for ParseCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

const fn case(name: &'static str, input: &'static str) -> ParseCase {
    ParseCase { name, input }
}

const CASES: [ParseCase; 10] = [
    case("plain", "42"),
    case("decimal-unit", "42 MB"),
    case("binary-unit", "1 KiB"),
    case("fraction", "1.5 MiB"),
    case("small-fraction", "0.0025 KB"),
    case("grouped", "1_234_567_890"),
    case("u64-max", "18_446_744_073_709_551_615"),
    case("high-precision-decimal", "1.84467440737095516145 EB"),
    case(
        "high-precision-binary",
        "0.0000000000000000004336808689942017736029811203479766845703125 EiB",
    ),
    case("malformed", "not-a-size"),
];

fn main() {
    divan::main();
}

#[divan::bench(args = CASES, sample_size = 1024)]
fn parse(case: ParseCase) -> Result<BSize64, ParseError> {
    divan::black_box(case.input).parse()
}
