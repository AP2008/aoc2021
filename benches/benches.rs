use aoc2021::{day1, day10, day2, day5, day6, day7, day8, day9};
use criterion::{criterion_group, criterion_main, Criterion};

fn day1_bench(c: &mut Criterion) {
    c.bench_function("Day 1", |b| b.iter(day1::runner));
}

fn day2_bench(c: &mut Criterion) {
    c.bench_function("Day 2", |b| b.iter(day2::runner));
}

fn day5_bench(c: &mut Criterion) {
    c.bench_function("Day 5", |b| b.iter(day5::runner));
}

fn day6_bench(c: &mut Criterion) {
    c.bench_function("Day 6", |b| b.iter(day6::runner));
}

fn day7_bench(c: &mut Criterion) {
    c.bench_function("Day 7", |b| b.iter(day7::runner));
}

fn day8_bench(c: &mut Criterion) {
    c.bench_function("Day 8", |b| b.iter(day8::runner));
}

fn day9_bench(c: &mut Criterion) {
    c.bench_function("Day 9", |b| b.iter(day9::runner));
}

fn day10_bench(c: &mut Criterion) {
    c.bench_function("Day 10", |b| b.iter(day10::runner));
}

fn days_all(c: &mut Criterion) {
    c.bench_function("All Days", |b| {
        b.iter(|| {
            day1::runner();
            day2::runner();
            day5::runner();
            day6::runner();
            day7::runner();
            day8::runner();
            day9::runner();
            day10::runner();
        })
    });
}

criterion_group!(
    benches,
    day1_bench,
    day2_bench,
    day5_bench,
    day6_bench,
    day7_bench,
    day8_bench,
    day9_bench,
    day10_bench,
    days_all
);
criterion_main!(benches);
