use criterion::{Criterion, criterion_group, criterion_main};
use openlab::compute::Compute;
use parser::options::ParserOptions;
use std::hint::black_box;
fn criterion_benchmark(crit: &mut Criterion) {
    let size = 4;
    let mut group = crit.benchmark_group("sample");
    group.sample_size(128);
    group.bench_function("make_matrix", |bencher| {
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
        println!("{s}");
        let mut val = black_box(Compute::parse(ParserOptions::new(None), &s).unwrap());
        bencher.iter(|| black_box(val.compute()))
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
