use criterion::{Criterion, criterion_group, criterion_main};
use openlab::compute::Compute;
use parser::options::ParserOptions;
use parser::parser::Block;
use std::hint::black_box;
use std::io::Empty;
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
        let val = black_box(Block::parse(ParserOptions::new(None), &s).unwrap());
        let mut compute = Compute::<Empty>::default();
        group.bench_function("make_matrix", |bencher| {
            bencher.iter(|| compute.compute(&val))
        });
    }
    {
        let str = format!("{s}*{s}");
        let val = black_box(Block::parse(ParserOptions::new(None), &str).unwrap());
        let mut compute = Compute::<Empty>::default();
        group.bench_function("mul_matrix", |bencher| {
            bencher.iter(|| compute.compute(&val))
        });
    }
    group.finish()
}
criterion_group!(compute, compute_matrix);
criterion_main!(compute);
