
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc2019::day4;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day4 pt1", |b| b.iter(|| day4::part1()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);