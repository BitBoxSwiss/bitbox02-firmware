//! ChaCha20 `stream-cipher` benchmark

#[cfg(not(feature = "stream-cipher"))]
compile_error!("run benchmarks with `cargo bench --all-features`");

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use criterion_cycles_per_byte::CyclesPerByte;

use chacha20::{
    stream_cipher::{NewStreamCipher, SyncStreamCipher},
    ChaCha20,
};

const KB: usize = 1024;

fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("stream-cipher");

    for size in &[KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB] {
        let mut buf = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_function(BenchmarkId::new("apply_keystream", size), |b| {
            let key = Default::default();
            let nonce = Default::default();
            let mut cipher = ChaCha20::new(&key, &nonce);

            b.iter(|| cipher.apply_keystream(&mut buf));
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
