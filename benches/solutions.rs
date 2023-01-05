use advent_of_code_2021_rust::{d01, d02, d03, d04};
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

fn d02a(c: &mut Criterion) {
    c.bench_function("d02a", |b| {
        b.iter(|| d02::p2(black_box(include_str!("../inputs/d02.txt"))))
    });
}

fn d02b(c: &mut Criterion) {
    c.bench_function("d02b", |b| {
        b.iter(|| d02::p2(black_box(include_str!("../inputs/d02.txt"))))
    });
}

fn d03a(c: &mut Criterion) {
    c.bench_function("d03a", |b| {
        b.iter(|| d03::p1(black_box(include_str!("../inputs/d03.txt"))))
    });
}

fn d03b(c: &mut Criterion) {
    c.bench_function("d03b", |b| {
        b.iter(|| d03::p2(black_box(include_str!("../inputs/d03.txt"))))
    });
}

fn d04a(c: &mut Criterion) {
    c.bench_function("d04a", |b| {
        b.iter(|| d04::p1(black_box(include_str!("../inputs/d04.txt"))))
    });
}

fn d04b(c: &mut Criterion) {
    c.bench_function("d04b", |b| {
        b.iter(|| d04::p2(black_box(include_str!("../inputs/d04.txt"))))
    });
}

criterion_group!(benches, d01a, d01b, d02a, d02b, d03a, d03b, d04a, d04b);
criterion_main!(benches);
