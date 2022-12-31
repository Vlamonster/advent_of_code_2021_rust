use advent_of_code_2021_rust::d01;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn d01a(c: &mut Criterion) {
    c.bench_function("d01a", |b| {
        b.iter(|| d01::p1(black_box(include_str!("../inputs/d01.txt"))))
    });
}

fn d01b(c: &mut Criterion) {
    c.bench_function("d01b", |b| {
        b.iter(|| d01::p2(black_box(include_str!("../inputs/d01.txt"))))
    });
}

criterion_group!(benches, d01a, d01b);
criterion_main!(benches);
