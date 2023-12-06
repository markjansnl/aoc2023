use aoc2023::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(criterion: &mut Criterion) {
    for day in DAYS {
        bench_day(criterion, day).unwrap();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
