use aoc2023::prelude::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(criterion: &mut Criterion) {
    all_days(criterion).unwrap();

    for day in DAYS {
        bench_day(criterion, day).unwrap();
    }
}

pub fn all_days(criterion: &mut Criterion) -> Result<()> {
    let mut group = criterion.benchmark_group("All days");
    // group.sample_size(10);

    for day in DAYS {
        group.bench_function(format!("Day {}", day.day), |b| {
            b.iter(|| run_day(day.day, get_input(day.day, 0)?, true, true))
        });
    }

    group.finish();

    Ok(())
}

pub fn bench_day(criterion: &mut Criterion, day: &def::Day) -> Result<()> {
    let mut group = criterion.benchmark_group(format!("Day {}", day.day));
    // group.sample_size(10);

    if reuse_parsed(day.day)? {
        group.bench_function("Parse", |b| {
            b.iter(|| bench_parse_day(day.day, get_input(day.day, 0)?, Part1))
        });
    } else {
        group.bench_function("Parse Part 1", |b| {
            b.iter(|| bench_parse_day(day.day, get_input(day.day, 0)?, Part1))
        });

        group.bench_function("Parse Part 2", |b| {
            b.iter(|| bench_parse_day(day.day, get_input(day.day, 0)?, Part2))
        });
    }

    group.bench_function("Part 1", |b| {
        b.iter(|| bench_day_part(day.day, get_input(day.day, 0)?, Part1))
    });

    group.bench_function("Part 2", |b| {
        b.iter(|| bench_day_part(day.day, get_input(day.day, 0)?, Part2))
    });

    group.finish();

    Ok(())
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
