use criterion::{Criterion, criterion_group, criterion_main};
use fast_powi::*;
use num_bigint::BigInt;
use num_complex::Complex;
use num_rational::Ratio;
use num_traits::{Pow, Zero};
use std::hint::black_box;

fn criterion_benchmark(c: &mut Criterion) {
    let bases: [f64; 3] = [0.3, -0.7, 1.4];
    let big_bases = [BigInt::from(3), BigInt::from(-7), BigInt::from(14)];
    let little_bases: [u64; 2] = [2, 3];
    let ratio_bases: [Ratio<BigInt>; 3] =
        big_bases.clone().map(|n| Ratio::new(n, BigInt::from(10)));
    let complex_bases: [Complex<Ratio<BigInt>>; 3] = [
        Complex::new(ratio_bases[0].clone(), ratio_bases[1].clone()),
        Complex::new(ratio_bases[1].clone(), ratio_bases[2].clone()),
        Complex::new(ratio_bases[2].clone(), ratio_bases[0].clone()),
    ];
    let exps = -64..64;
    let big_exps = 2..64;
    let little_exps = 2..32;
    c.bench_function("f64_fast", |b| {
        b.iter(|| {
            let mut sum = 0.0;
            for base in bases {
                for exp in exps.clone() {
                    sum += black_box(base).pow_i8(black_box(exp));
                }
            }
            sum
        })
    });
    c.bench_function("f64_const", |b| {
        b.iter(|| {
            let mut sum = 0.0;
            for base in bases {
                for exp in exps.clone() {
                    sum += f64_const::powi(black_box(base), black_box(exp as i32));
                }
            }
            sum
        })
    });
    c.bench_function("f64_std", |b| {
        b.iter(|| {
            let mut sum = 0.0;
            for base in bases {
                for exp in exps.clone() {
                    sum += black_box(base).powi(black_box(exp as i32));
                }
            }
            sum
        })
    });
    c.bench_function("bigint_fast", |b| {
        b.iter(|| {
            let mut sum = BigInt::from(0);
            for base in &big_bases {
                for exp in big_exps.clone() {
                    sum += black_box(base).pow_u8(black_box(exp));
                }
            }
            sum
        })
    });
    c.bench_function("bigint_num", |b| {
        b.iter(|| {
            let mut sum = BigInt::from(0);
            for base in &big_bases {
                for exp in big_exps.clone() {
                    sum += black_box(base).pow(black_box(exp as u32));
                }
            }
            sum
        })
    });
    c.bench_function("ratio_fast", |b| {
        b.iter(|| {
            let mut sum = BigInt::zero();
            for base in &ratio_bases {
                for exp in exps.clone() {
                    sum += black_box(base).pow_i8(black_box(exp)).numer();
                }
            }
            sum
        })
    });
    c.bench_function("ratio_num", |b| {
        b.iter(|| {
            let mut sum = BigInt::zero();
            for base in &ratio_bases {
                for exp in exps.clone() {
                    sum += black_box(base).pow(black_box(exp as i32)).numer();
                }
            }
            sum
        })
    });
    c.bench_function("u64_fast", |b| {
        b.iter(|| {
            let mut sum = 0;
            for base in &little_bases {
                for exp in little_exps.clone() {
                    sum += black_box(base).pow_u8(black_box(exp));
                }
            }
            sum
        })
    });
    c.bench_function("u64_num", |b| {
        b.iter(|| {
            let mut sum = 0;
            for base in &little_bases {
                for exp in little_exps.clone() {
                    sum += black_box(base).pow(black_box(exp as u32));
                }
            }
            sum
        })
    });
    c.bench_function("complex_fast", |b| {
        b.iter(|| {
            let mut sum = BigInt::zero();
            for base in &complex_bases {
                for exp in exps.clone() {
                    sum += black_box(base).pow_i8(black_box(exp)).re.numer();
                }
            }
            sum
        })
    });
    c.bench_function("complex_num", |b| {
        b.iter(|| {
            let mut sum = BigInt::zero();
            for base in &complex_bases {
                for exp in exps.clone() {
                    sum += black_box(base).pow(black_box(exp as i32)).re.numer();
                }
            }
            sum
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
