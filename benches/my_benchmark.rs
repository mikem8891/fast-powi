use std::{hint::black_box};
use criterion::{Criterion, criterion_group, criterion_main};
use fast_powi::*;

fn criterion_benchmark(c: &mut Criterion) {
    let bases: [f64; 3] = [0.3, -0.7, 1.4];
    let exps = -52..52;
    c.bench_function("static", |b| b.iter(|| {
        let mut sum = 0.0;
        for base in bases {
            for exp in exps.clone() {
                sum += black_box(base).pow_i8(black_box(exp));
            }
        }
        sum
    }));
    c.bench_function("builtin", |b| b.iter(|| {
        let mut sum = 0.0;
        for base in bases {
            for exp in exps.clone() {
                sum += f64::powi(black_box(base),black_box(exp as i32));
            }
        }
        sum
    }));
    c.bench_function("std", |b| b.iter(|| {
        let mut sum = 0.0;
        for base in bases {
            for exp in exps.clone() {
                sum += black_box(base).powi(black_box(exp as i32));
            }
        }
        sum
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);