use criterion::{Criterion, criterion_group, criterion_main};
use openlab::compute::Compute;
use parser::options::ParserOptions;
use std::hint::black_box;
fn compute_matrix(crit: &mut Criterion) {
    let size = 16;
    let s = format!(
        "[[{}]]",
        (0..size)
            .map(|i| (0..size)
                .map(|j| (i + j).to_string())
                .collect::<Vec<_>>()
                .join(","))
            .collect::<Vec<_>>()
            .join("],[")
    );
    let mut group = crit.benchmark_group("sample");
    group.sample_size(128);
    {
        let mut val = black_box(Compute::parse(ParserOptions::new(None), &s).unwrap());
        group.bench_function("make_matrix", |bencher| {
            bencher.iter(|| val.compute())
        });
    }
    {
        let str = format!("{s}*{s}");
        let mut val = black_box(Compute::parse(ParserOptions::new(None), &str).unwrap());
        group.bench_function("mul_matrix", |bencher| {
            bencher.iter(|| val.compute())
        });
    }
    group.finish()
}
criterion_group!(compute, compute_matrix);
criterion_main!(compute);
