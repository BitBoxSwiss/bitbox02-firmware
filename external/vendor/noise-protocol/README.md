# Noise-Rust

[![Crates.io](https://img.shields.io/crates/v/noise-protocol.svg)](https://crates.io/crates/noise-protocol)
[![Docs.rs](https://docs.rs/noise-protocol/badge.svg)](https://docs.rs/noise-protocol)

Implementation of the [Noise Protocol
Framework](http://noiseprotocol.org) in Rust.

## Status

Revision 34 is implemented.

Test vectors from [cacophony](https://github.com/centromere/cacophony) and [snow](https://github.com/mcginty/snow) are successfully verified.

## Philosophy

* Simple: straightforward implementation, small amount of code, almost no
  dependencies, supports `no_std`. Feature `use_alloc` can optionallly be used
  as an alternative to std.
* Fast: static dispatch, no heap allocation necessary.
* Unopinionated: flexible, primitive API, does not dictate how it should be
  used.

## Documentation

* [noise-protocol](https://docs.rs/noise-protocol)
* [noise-rust-crypto](https://docs.rs/noise-rust-crypto)

## Crates

This repository contains several crates. The `noise-protocol` crate contains the
abstract implementation of the protocol framework. `noise-rust-crypto` provides concrete implementations of
the needed crypto primitives. It is a wrapper for `x25519-dalek` and
[RustCrypto](`https://github.com/RustCrypto`) crates.

The following table shows what primitives each of these crates
supports:

|             | X25519 | AES-256-GCM | Chacha20-Poly1305 | SHA-256 | SHA-512 | BLAKE2s | BLAKE2b |
|-------------|:------:|:-----------:|:-----------------:|:-------:|:-------:|:-------:|:-------:|
| rust-ring   |        | ✔           | ✔                 | ✔       | ✔       |         |         |
| rust-crypto | ✔      | ✔           | ✔                 | ✔       | ✔       | ✔       | ✔       |

You can also plug in other primitive implementations by implementing the `DH`,
`Cipher` and `Hash` traits.

## `no_std` usage

The `noise-protocol` crate supports `no_std`, if default features are
disabled.

## License

Unlicense.
