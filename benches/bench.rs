use aoc2023::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(criterion: &mut Criterion) {
    for day in DAYS {
        criterion_bench_day(criterion, day).unwrap();
    }
}

pub fn criterion_bench_day(criterion: &mut Criterion, day: &def::Day) -> Result<()> {
    let mut group = criterion.benchmark_group(format!("Day {:02}", day.day));

    group.warm_up_time(std::time::Duration::from_secs(1));

    bench_day(day.day, get_input(day.day, 0)?, &mut group)?;

    group.finish();

    Ok(())
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
