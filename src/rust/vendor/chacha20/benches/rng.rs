//! `ChaCha20Rng` benchmark

#[cfg(not(feature = "rng"))]
compile_error!("run benchmarks with `cargo bench --all-features`");

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;

use chacha20::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};

const KB: usize = 1024;

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("rng");

    for size in &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB] {
        let mut buf = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(BenchmarkId::new("fill_bytes", size), |b| {
            let mut rng = ChaCha20Rng::from_seed(Default::default());
            b.iter(|| rng.fill_bytes(&mut buf));
        });
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = bench
);
criterion_main!(benches);
