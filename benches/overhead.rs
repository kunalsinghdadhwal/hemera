use criterion::{Criterion, black_box, criterion_group, criterion_main};
use hemera::measure_time;

// Function without macro
fn baseline_sync(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    sum
}

// Function with macro
#[measure_time(threshold = "999s")] // High threshold so it never logs
fn instrumented_sync(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    sum
}

fn benchmark_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("macro_overhead");

    group.bench_function("baseline", |b| b.iter(|| baseline_sync(black_box(1000))));

    group.bench_function("instrumented", |b| {
        b.iter(|| instrumented_sync(black_box(1000)))
    });

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_output_color(true);
    targets = benchmark_overhead
}
criterion_main!(benches);
