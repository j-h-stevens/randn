use criterion::{black_box, criterion_group, criterion_main, Criterion};
use randn::{randn_vector, randn_matrix, randn_matrices};

fn bench_randn_vector(c: &mut Criterion) {
    println!("Running bench_randn_vector");
    let n = 50_000;
    c.bench_function("randn_vector", |b| {
        b.iter(|| black_box(randn_vector(n)))
    });
}

fn bench_randn_matrix(c: &mut Criterion) {
    let r = 50;
    let cols = 1000; 
    c.bench_function("randn_matrix", |b| {
        b.iter(|| black_box(randn_matrix(r, cols)))
    });
}

fn bench_randn_matrices(c: &mut Criterion) {
    let rows = 50;
    let cols = 17; 
    let sims = 1000;
    c.bench_function("randn_matrices", |b| {
        b.iter(|| black_box(randn_matrices(rows, cols, sims)))
    });
}

criterion_group!(benches, bench_randn_vector, bench_randn_matrix, bench_randn_matrices);
criterion_main!(benches);
