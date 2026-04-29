# Coding Conventions

This file captures language and architecture conventions for code changes.

## C and C++

- `.clang-format` uses Chromium base style, 4-space indent, and Linux braces.
- `.clang-tidy` governs C and C++ checks.
- Use `snake_case` for symbols.
- Use `PascalCase` for types.
- Use `ALL_CAPS` for macros.
- For C code changes, run `./scripts/dev_exec.sh ./scripts/format`.

## Python

- Python utilities follow `.pylintrc`.
- Keep lines within the configured 100-column limit.
- Use explicit imports.
- For Python changes, run `./scripts/dev_exec.sh ./scripts/format-python`.

## Rust

- Rust crates rely on `rustfmt.toml` and the pinned toolchain in
  `rust-toolchain.toml`.
- Keep module paths aligned with `src/rust`.
- Rust fmt all Rust files you modify, using the 2024 edition configuration.
- For Rust code changes, run:

  ```sh
  ./scripts/dev_exec.sh cargo fmt --manifest-path src/rust/Cargo.toml --all
  ```
- Prefer `hex!` literals for byte arrays and constants.

## Protobuf and Bindings

- When changing protobuf interfaces, run
  `./scripts/dev_exec.sh make generate-protobufs`.
- Regenerate bindings with `cbindgen` or protobuf tooling when interfaces
  change and the repository requires generated outputs.
- If you modify `messages/*.proto`, run
  `./scripts/dev_exec.sh make generate-protobufs` before direct Rust `cargo`
  commands.

## C to Rust Migration

- When converting C code to Rust code, make the Rust code idiomatic instead of a
  1:1 rewrite.
- Preserve original comments and docstrings during ports, refactors, and
  rewrites when they still apply.
- Migrate only what is necessary for the current scope.

## Rust and C FFI

- When exposing Rust functions to C with `extern "C"`, use `util::bytes::Bytes`
  and `util::bytes::BytesMut` to pass buffers and write to output buffers.
- When using `Zeroizing<...>` for buffers, use `Zeroizing<Vec<u8>>`.
- For other sensitive data, use `Zeroizing<Box<...>>`.
- When wrapping C functions, always use a `-sys` crate for the bindings.
- Safe Rust wrappers around C functions must be idiomatic and must not expose C
  types in their input or output, especially pointers.
- Return results from wrappers instead of passing output parameters.
- Check all invariants in C-facing wrapper code and panic if they are violated.

## BitBox03 UI

- When working on BitBox03 UI code, do not reach into `bitbox_lvgl` FFI directly
  from feature code.
- If needed, add safe idiomatic wrappers to `bitbox-lvgl` first and use those
  wrappers instead.
