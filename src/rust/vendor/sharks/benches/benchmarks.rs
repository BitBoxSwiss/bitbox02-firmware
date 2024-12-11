use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::convert::TryFrom;

use sharks::{Share, Sharks};

fn dealer(c: &mut Criterion) {
    let sharks = Sharks(255);
    let mut dealer = sharks.dealer(&[1]);

    c.bench_function("obtain_shares_dealer", |b| {
        b.iter(|| sharks.dealer(black_box(&[1])))
    });
    c.bench_function("step_shares_dealer", |b| b.iter(|| dealer.next()));
}

fn recover(c: &mut Criterion) {
    let sharks = Sharks(255);
    let shares: Vec<Share> = sharks.dealer(&[1]).take(255).collect();

    c.bench_function("recover_secret", |b| {
        b.iter(|| sharks.recover(black_box(shares.as_slice())))
    });
}

fn share(c: &mut Criterion) {
    let bytes_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let bytes = bytes_vec.as_slice();
    let share = Share::try_from(bytes).unwrap();

    c.bench_function("share_from_bytes", |b| {
        b.iter(|| Share::try_from(black_box(bytes)))
    });

    c.bench_function("share_to_bytes", |b| {
        b.iter(|| Vec::from(black_box(&share)))
    });
}

criterion_group!(benches, dealer, recover, share);
criterion_main!(benches);
