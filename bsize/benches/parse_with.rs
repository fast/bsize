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
use bsize::ParseOptions;
use bsize::RoundMode;

#[derive(Clone, Copy)]
struct ParseWithCase {
    name: &'static str,
    mode: RoundMode,
}

impl fmt::Display for ParseWithCase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name)
    }
}

const CASES: [ParseWithCase; 5] = [
    ParseWithCase {
        name: "ceil",
        mode: RoundMode::Ceil,
    },
    ParseWithCase {
        name: "floor",
        mode: RoundMode::Floor,
    },
    ParseWithCase {
        name: "half-ceil",
        mode: RoundMode::HalfCeil,
    },
    ParseWithCase {
        name: "half-floor",
        mode: RoundMode::HalfFloor,
    },
    ParseWithCase {
        name: "half-even",
        mode: RoundMode::HalfEven,
    },
];

const HIGH_PRECISION_BINARY: &str =
    "0.0000000000000000004336808689942017736029811203479766845703125 EiB";

fn main() {
    divan::main();
}

#[divan::bench(args = CASES, sample_size = 1024)]
fn parse_with(case: ParseWithCase) -> Result<BSize64, ParseError> {
    let mut options = ParseOptions::default();
    options.round_mode = divan::black_box(case.mode);
    BSize64::parse_with(divan::black_box(HIGH_PRECISION_BINARY), options)
}
