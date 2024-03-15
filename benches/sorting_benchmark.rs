use criterion::{criterion_group, criterion_main, Criterion};
use learning_rust::{sort_algo_1, sort_algo_2};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers = vec![
        1, 2, 3, 5, 7, 8, 52, 10, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ];

    c.bench_function("Sorting algo 1", |b| b.iter(|| sort_algo_2(&mut numbers)));
}

criterion_group!(benches, sort_benchmark);

criterion_main!(benches);
