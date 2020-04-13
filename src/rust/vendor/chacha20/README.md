# ChaCha20

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Build Status][build-image]][build-link]

[ChaCha20][1] is a [stream cipher][2] which is designed to support
high-performance software implementations.

It improves upon the previous [Salsa20][3] stream cipher with increased
per-round diffusion at no cost to performance.

This crate also contains an implementation of [XChaCha20][4]: a variant
of ChaCha20 with an extended 192-bit (24-byte) nonce, gated under the
`xchacha20` Cargo feature (on-by-default).

[Documentation][docs-link]

## Implementations

This crate contains the following implementations of ChaCha20, all of which
work on stable Rust with the following `RUSTFLAGS`:

- `x86` / `x86_64`
  - `avx2`: (~1.4cpb) `-Ctarget-cpu=haswell -Ctarget-feature=+avx2`
  - `sse2`: (~2.5cpb) `-Ctarget-feature=+sse2` (on by default on x86 CPUs)
- Portable
  - `soft`: (~5 cpb on x86/x86_64)

NOTE: cpb = cycles per byte (smaller is better)

## Security Warning

This crate does not ensure ciphertexts are authentic! Thus ciphertext integrity
is not verified, which can lead to serious vulnerabilities! To avoid this, use
the [ChaCha20Poly1305][5] Authenticated Encryption with Associated Data (AEAD)
algorithm instead.

No security audits of this crate have ever been performed, and it has not been
thoroughly assessed to ensure its operation is constant-time on common CPU
architectures.

USE AT YOUR OWN RISK!

## License

Licensed under either of:

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/chacha20.svg
[crate-link]: https://crates.io/crates/chacha20
[docs-image]: https://docs.rs/chacha20/badge.svg
[docs-link]: https://docs.rs/chacha20/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.34+-blue.svg
[build-image]: https://travis-ci.org/RustCrypto/stream-ciphers.svg?branch=master
[build-link]: https://travis-ci.org/RustCrypto/stream-ciphers

[//]: # (general links)

[1]: https://en.wikipedia.org/wiki/Salsa20#ChaCha_variant
[2]: https://en.wikipedia.org/wiki/Stream_cipher
[3]: https://en.wikipedia.org/wiki/Salsa20
[4]: https://tools.ietf.org/html/draft-arciszewski-xchacha-02
[5]: https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305
