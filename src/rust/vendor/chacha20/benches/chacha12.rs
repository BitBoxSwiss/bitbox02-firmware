#![cfg(feature = "stream-cipher")]
#![feature(test)]

stream_cipher::bench_sync!(chacha20::ChaCha12);
