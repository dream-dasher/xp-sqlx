/// abandoned attempt to bench async code
/// I strongly dislike the criterion crate (great deal of obstuse seeming structure)
/// and divan does not work with async at all
// use criterion::async_executor::AsyncExecutor
// use tokio::runtime::Runtime;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use xp_sqlx::stream_to_df::{direct_transpose, recopy_transpose, vstruct_transpose};

/// Note: name here is not structural, just descriptive
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("recopy transpose", |b| {
        b.iter(|| recopy_transpose(black_box(20)))
    });
}

/// For comparing multiple functions against one another
pub fn criterion_inter_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("My Group");

    // Now we can perform benchmarks with this group
    group.bench_function("direct_transpose", |b| {
        // b.to_async(Runtime)
        b.iter(|| direct_transpose(black_box(2_000)))
    });
    group.bench_function("recopy_transpose", |b| {
        // b.to_async(Runtime)
        b.iter(|| recopy_transpose(black_box(2_000)))
    });
    group.bench_function("vstruct_transpose", |b| {
        // b.to_async(Runtime)
        b.iter(|| vstruct_transpose(black_box(2_000)))
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark, criterion_inter_comparison);
criterion_main!(benches);
