use advent_of_code_2022_rust::{
    d01, d02, d03, d04, d05, d06, d07, d08, d09, d10, d11, d12, d13, d14, d15, d18, d20,
};
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
        b.iter(|| d02::p1(black_box(include_str!("../inputs/d02.txt"))))
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

fn d05a(c: &mut Criterion) {
    c.bench_function("d05a", |b| {
        b.iter(|| d05::p1(black_box(include_str!("../inputs/d05.txt"))))
    });
}

fn d05b(c: &mut Criterion) {
    c.bench_function("d05b", |b| {
        b.iter(|| d05::p2(black_box(include_str!("../inputs/d05.txt"))))
    });
}

fn d06a(c: &mut Criterion) {
    c.bench_function("d06a", |b| {
        b.iter(|| d06::p1(black_box(include_str!("../inputs/d06.txt"))))
    });
}

fn d06b(c: &mut Criterion) {
    c.bench_function("d06b", |b| {
        b.iter(|| d06::p2(black_box(include_str!("../inputs/d06.txt"))))
    });
}

fn d07a(c: &mut Criterion) {
    c.bench_function("d07a", |b| {
        b.iter(|| d07::p1(black_box(include_str!("../inputs/d07.txt"))))
    });
}

fn d07b(c: &mut Criterion) {
    c.bench_function("d07b", |b| {
        b.iter(|| d07::p2(black_box(include_str!("../inputs/d07.txt"))))
    });
}

fn d08a(c: &mut Criterion) {
    c.bench_function("d08a", |b| {
        b.iter(|| d08::p1(black_box(include_str!("../inputs/d08.txt"))))
    });
}

fn d08b(c: &mut Criterion) {
    c.bench_function("d08b", |b| {
        b.iter(|| d08::p2(black_box(include_str!("../inputs/d08.txt"))))
    });
}

fn d09a(c: &mut Criterion) {
    c.bench_function("d09a", |b| {
        b.iter(|| d09::p1(black_box(include_str!("../inputs/d09.txt"))))
    });
}

fn d09b(c: &mut Criterion) {
    c.bench_function("d09b", |b| {
        b.iter(|| d09::p2(black_box(include_str!("../inputs/d09.txt"))))
    });
}

fn d10a(c: &mut Criterion) {
    c.bench_function("d10a", |b| {
        b.iter(|| d10::p1(black_box(include_str!("../inputs/d10.txt"))))
    });
}

fn d10b(c: &mut Criterion) {
    c.bench_function("d10b", |b| {
        b.iter(|| d10::p2(black_box(include_str!("../inputs/d10.txt"))))
    });
}

fn d11a(c: &mut Criterion) {
    c.bench_function("d11a", |b| {
        b.iter(|| d11::p1(black_box(include_str!("../inputs/d11.txt"))))
    });
}

fn d11b(c: &mut Criterion) {
    c.bench_function("d11b", |b| {
        b.iter(|| d11::p2(black_box(include_str!("../inputs/d11.txt"))))
    });
}

fn d12a(c: &mut Criterion) {
    c.bench_function("d12a", |b| {
        b.iter(|| d12::p1(black_box(include_str!("../inputs/d12.txt"))))
    });
}

fn d12b(c: &mut Criterion) {
    c.bench_function("d12b", |b| {
        b.iter(|| d12::p2(black_box(include_str!("../inputs/d12.txt"))))
    });
}

fn d13a(c: &mut Criterion) {
    c.bench_function("d13a", |b| {
        b.iter(|| d13::p1(black_box(include_str!("../inputs/d13.txt"))))
    });
}

fn d13b(c: &mut Criterion) {
    c.bench_function("d13b", |b| {
        b.iter(|| d13::p2(black_box(include_str!("../inputs/d13.txt"))))
    });
}

fn d14a(c: &mut Criterion) {
    c.bench_function("d14a", |b| {
        b.iter(|| d14::p1(black_box(include_str!("../inputs/d14.txt"))))
    });
}

fn d14b(c: &mut Criterion) {
    c.bench_function("d14b", |b| {
        b.iter(|| d14::p2(black_box(include_str!("../inputs/d14.txt"))))
    });
}

fn d15a(c: &mut Criterion) {
    c.bench_function("d15a", |b| {
        b.iter(|| d15::p1(black_box(include_str!("../inputs/d15.txt"))))
    });
}

fn d15b(c: &mut Criterion) {
    c.bench_function("d15b", |b| {
        b.iter(|| d15::p2(black_box(include_str!("../inputs/d15.txt"))))
    });
}

fn d18a(c: &mut Criterion) {
    c.bench_function("d18a", |b| {
        b.iter(|| d18::p1(black_box(include_str!("../inputs/d18.txt"))))
    });
}

fn d18b(c: &mut Criterion) {
    c.bench_function("d18b", |b| {
        b.iter(|| d18::p2(black_box(include_str!("../inputs/d18.txt"))))
    });
}

fn d20a(c: &mut Criterion) {
    c.bench_function("d20a", |b| {
        b.iter(|| d20::p1(black_box(include_str!("../inputs/d20.txt"))))
    });
}

fn d20b(c: &mut Criterion) {
    c.bench_function("d20b", |b| {
        b.iter(|| d20::p2(black_box(include_str!("../inputs/d20.txt"))))
    });
}

criterion_group!(
    benches, d01a, d01b, d02a, d02b, d03a, d03b, d04a, d04b, d05a, d05b, d06a, d06b, d07a, d07b,
    d08a, d08b, d09a, d09b, d10a, d10b, d11a, d11b, d12a, d12b, d13a, d13b, d14a, d14b, d15a, d15b,
    d18a, d18b, d20a, d20b
);
criterion_main!(benches);
