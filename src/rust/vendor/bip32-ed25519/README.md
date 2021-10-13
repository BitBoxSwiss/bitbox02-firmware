# BIP32-Ed25519

This crate implements BIP32 for Ed25519 according to the paper *BIP32-Ed25519 Hierarchical Deterministic Keys over a Non-linear
Keyspace*:
- [Mirror 1](https://github.com/input-output-hk/adrestia/blob/74d3a0ae793a73ebed93aa1df49f0b4d2ccf1a6d/user-guide/static/Ed25519_BIP.pdf)
- [Mirror 2](https://github.com/LedgerHQ/orakolo/blob/0b2d5e669ec61df9a824df9fa1a363060116b490/papers/Ed25519_BIP%20Final.pdf)

## Fork information

This crate contains some code copied from https://github.com/typed-io/rust-ed25519-bip32, especially:

- The `add_28_mul8()` and `add_256bits()` functions.
- Some parts of the public and private key derivation functions.

This crate deviates from the above project in the following ways:

- Using RustCrypto libraries for SHA512, HMAC, and Curve25519 operations instead of
  [cryptoxide](https://docs.rs/cryptoxide/). Rationale: the primary user of this library, the
  BitBox02 firmware, already includes RustCrypto dependencies, so reusing these does not grow the
  firmware binary size unnecessarily.
- `no_std` - no dependency on `std`. There is a dependency on `alloc`, so an allocator is required.
- Use of [zeroize](https://docs.rs/zeroize/).
- Better unit test coverage.

## Table tests

In [tests/table_test.rs](./tests/table_test.rs), this implementation is tested to yield the same
results as:

- https://github.com/typed-io/rust-ed25519-bip32
- Ledger's [reference
  implementation](https://github.com/LedgerHQ/orakolo/blob/0b2d5e669ec61df9a824df9fa1a363060116b490/src/python/orakolo/HDEd25519.py),
  which showcases what their SDK's `os_perso_derive_node_bip32_seed_key()` function computes. Ledger
  uses this to derive e.g. Cardano keys.
