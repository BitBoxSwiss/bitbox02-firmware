#![cfg(feature = "cipher")]
#![feature(test)]

cipher::stream_cipher_sync_bench!(chacha20::ChaCha8);
