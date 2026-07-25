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

use std::hint::black_box;

use bsize::BSize64;
use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;

fn benchmark_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");

    for (name, input) in [
        ("plain", "42"),
        ("decimal-unit", "42 MB"),
        ("binary-unit", "1 KiB"),
        ("fraction", "1.5 MiB"),
        ("small-fraction", "0.0025 KB"),
        ("grouped", "1_234_567_890"),
        ("u64-max", "18_446_744_073_709_551_615"),
        ("high-precision-decimal", "1.84467440737095516145 EB"),
        (
            "high-precision-binary",
            "0.0000000000000000004336808689942017736029811203479766845703125 EiB",
        ),
        ("malformed", "not-a-size"),
    ] {
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, input| {
            b.iter(|| black_box(black_box(input).parse::<BSize64>()))
        });
    }

    group.finish();
}

criterion_group!(benches, benchmark_parse);
criterion_main!(benches);
