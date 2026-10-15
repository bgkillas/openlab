use core::ops::Mul;
use criterion::{Criterion, criterion_group, criterion_main};
use number::float::complex::Complex;
use number::float::real::Real;
use number::matrix::dense::Matrix;
use number::matrix::size::MatrixDimension;
use number::traits::complex::ComplexImpl;
use number::units::{NumberUnits, Units};
use std::hint::black_box;
fn criterion_benchmark(crit: &mut Criterion) {
    let size = 4;
    let mut group = crit.benchmark_group("sample");
    group.sample_size(128);
    {
        let a = black_box(Matrix::<f64>::new_with(
            MatrixDimension::new(size, size),
            |u, _| u as f64,
        ));
        let b = black_box(Matrix::<f64>::new_with(
            MatrixDimension::new(size, size),
            |u, _| (size * size - u as u32) as f64,
        ));
        group.bench_function("matrix multiplication", |bencher| {
            bencher.iter(|| black_box(a.mul(&b)))
        });
    }
    {
        let a = black_box(Matrix::<Complex<f64>>::new_with(
            MatrixDimension::new(size, size),
            |u, _| Complex::new_real(Real(u as f64)),
        ));
        let b = black_box(Matrix::<Complex<f64>>::new_with(
            MatrixDimension::new(size, size),
            |u, _| Complex::new_real(Real((size * size - u as u32) as f64)),
        ));
        group.bench_function("matrix multiplication complex", |bencher| {
            bencher.iter(|| black_box(a.mul(&b)))
        });
    }
    {
        let a = black_box(Matrix::<NumberUnits<f64>>::new_with(
            MatrixDimension::new(size, size),
            |u, _| NumberUnits::new(u as f64, Units::default()),
        ));
        let b = black_box(Matrix::<NumberUnits<f64>>::new_with(
            MatrixDimension::new(size, size),
            |u, _| NumberUnits::new((size * size - u as u32) as f64, Units::default()),
        ));
        group.bench_function("matrix multiplication units", |bencher| {
            bencher.iter(|| black_box(a.mul(&b)))
        });
    }
    {
        let a = black_box(Matrix::<NumberUnits<Complex<f64>>>::new_with(
            MatrixDimension::new(size, size),
            |u, _| NumberUnits::new(Complex::new_real(Real(u as f64)), Units::default()),
        ));
        let b = black_box(Matrix::<NumberUnits<Complex<f64>>>::new_with(
            MatrixDimension::new(size, size),
            |u, _| {
                NumberUnits::new(
                    Complex::new_real(Real((size * size - u as u32) as f64)),
                    Units::default(),
                )
            },
        ));
        group.bench_function("matrix multiplication complex units", |bencher| {
            bencher.iter(|| black_box(a.mul(&b)))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
