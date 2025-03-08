use rust::medium::alphametics::solve;

use criterion::{Criterion, black_box, criterion_group, criterion_main};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("six letters", |b| {
        b.iter(|| solve(black_box("NO + NO + TOO == LATE")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
