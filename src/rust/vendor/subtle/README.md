# subtle [![](https://img.shields.io/crates/v/subtle.svg)](https://crates.io/crates/subtle) [![](https://img.shields.io/badge/dynamic/json.svg?label=docs&uri=https%3A%2F%2Fcrates.io%2Fapi%2Fv1%2Fcrates%2Fsubtle%2Fversions&query=%24.versions%5B0%5D.num&colorB=4F74A6)](https://doc.dalek.rs/subtle) [![](https://travis-ci.org/dalek-cryptography/subtle.svg?branch=master)](https://travis-ci.org/dalek-cryptography/subtle)

**Pure-Rust traits and utilities for constant-time cryptographic implementations.**

It consists of a `Choice` type, and a collection of traits using `Choice`
instead of `bool` which are intended to execute in constant-time.  The `Choice`
type is a wrapper around a `u8` that holds a `0` or `1`.

This crate represents a “best-effort” attempt, since side-channels
are ultimately a property of a deployed cryptographic system
including the hardware it runs on, not just of software.

The traits are implemented using bitwise operations, and should execute in
constant time provided that a) the bitwise operations are constant-time and b)
the operations are not optimized into a branch.

To prevent the latter possibility, the crate attempts to hide the value of a
`Choice`'s inner `u8` from the optimizer, by passing it through either an
inline assembly block or a volatile read.  For more information, see the
_About_ section below.

```toml
[dependencies.subtle]
version = "2.2"
```

Versions prior to `2.2` recommended use of the `nightly` feature to enable an
optimization barrier; this is not required in versions `2.2` and above.

## Documentation

Documentation is available [here][docs].

## About

This library aims to be the Rust equivalent of Go’s `crypto/subtle` module.

The optimization barrier in `impl From<u8> for Choice` was based on Tim
Maclean's [work on `rust-timing-shield`][rust-timing-shield], which attempts to
provide a more comprehensive approach for preventing software side-channels in
Rust code.

`subtle` is authored by isis agora lovecruft and Henry de Valence.

## Warning

This code is a low-level library, intended for specific use-cases implementing
cryptographic protocols.  It represents a best-effort attempt to protect
against some software side-channels.  Because side-channel resistance is not a
property of software alone, but of software together with hardware, any such
effort is fundamentally limited.

**USE AT YOUR OWN RISK**

[docs]: https://docs.rs/subtle
[rust-timing-shield]: https://www.chosenplaintext.ca/open-source/rust-timing-shield/security
