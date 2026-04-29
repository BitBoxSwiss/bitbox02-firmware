# Testing Guide

This file captures test layout, test style, and expected verification.

## C Tests

- Place new C specs in `test/unit-test`.
- Add doubles to `test/hardware-fakes` when hardware behavior is mocked.
- Follow the `test_<feature>.c` naming pattern.
- Update CMake lists when adding C tests.

## Rust Tests

- Rust crates use standard `tests/` modules or `#[cfg(test)]` blocks.
- In Rust unit tests, prefer `.unwrap()` over `.expect()`.
- If testing a function `foo`, name the test `test_foo`, or `test_foo_xyz` when
  qualifiers are needed.
- If a Rust unit test involves futures, use `#[async_test::test] async fn
  test_...`.
- Prefer `.as_slice()` instead of `&*` for wrapped or zeroized `Vec<u8>`.

## Verification Before PR

Before opening a PR, run:

```sh
./scripts/dev_exec.sh make run-unit-tests
cargo test --manifest-path src/rust/Cargo.toml --all-features -- --test-threads 1
```

For cryptography or security-sensitive areas, refresh coverage with:

```sh
./scripts/dev_exec.sh make coverage
```

If a full verification command is too expensive or cannot be run in the current
environment, run the most relevant narrower checks and state the limitation in
the final response.
