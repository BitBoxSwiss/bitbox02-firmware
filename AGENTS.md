# Repository Guidelines for bitbox02-firmware

## Project Structure & Module Organization
Core firmware and bootloader code sits in `src/`, grouped by subsystem (`bootloader`, `usb`, `ui`,
`securechip`, plus a `rust/` workspace). Tests live in `test/`: `unit-test/` for cmocka suites,
`hardware-fakes/` for device shims, and `simulator/` assets. The BitB02 Python client library is in
`py/bitbox02`. Supporting tooling is in `scripts/` (CI, J-Link macros), and `doc/` for
manuals. Vendored dependencies are tracked in `external/`.

The firmware has C and Rust code. Rust code lives in src/rust. The most important rust crates are:
- bitbox02-rust: the main app logic. It can expose functions to C using extern "C". If it needs
  access to C functions, it has to go through the bitbox-hal crate. Never add bitbox02 or
  bitbox02-sys dep to bitbox02-rust.
- bitbox02-sys: generated bindings to bitbox02 specific C code. build.rs contains the functions etc
  that are exposed. See also `wrapper.h`, it needs to include any C headers/declarations that are
  added to build.rs.
- bitbox-hal: provides an interface to device specific functionality
- bitbox02: wraps bitbox02-sys as idiomatic safe Rust and implements the bitbox-hal interface.

bitbox02-rust is pure Rust and device agnostic. To access device specific functionality it must
always go through bitbox-hal. The migration is a work in progress, only migrate what is necessary
for the current scope.

## Build, Test, and Development Commands
- `make dockerpull` / `make dockerdev`: fetch and enter the maintained development container.

Run regular Unix commands such as `git`, `rg`, `grep`, `ls`, `find`, `sed`, and `cat` directly on
the host.

Cargo commands for the Rust workspace, such as `cargo test`, `cargo check`, and `cargo clippy`, may
also be run directly on the host by passing `--manifest-path src/rust/Cargo.toml`.

Use `./scripts/dev_exec.sh <command>` only for project-specific commands that depend on the project
toolchain or compiler environment.

In practice, the repository `make` targets in this file are project-specific toolchain commands.
When running from the host, invoke them via `./scripts/dev_exec.sh make <target>`.

Do not wrap `./scripts/dev_exec.sh` itself in `bash -lc`. Prefer changing CWD
with CLI args like `tar -C <PATH>`. If a command genuinely needs shell features such as pipes, pass
an explicit shell as the command, e.g. `./scripts/dev_exec.sh bash -lc 'cat versions.json | jq'`.

- `make firmware` / `make bootloader`: compile firmware or bootloader ELFs into `build/`.
- `make simulator`: build the Linux simulator under `build-build-noasan/bin/`.
- `make unit-test && make run-unit-tests`:  build and run the C cmocka/CTest suite with ASan/UBSan.
- `make run-rust-clippy`: lint Rust code with the workspace configuration.
- When invoking the above `make` targets from the host, prefer
  `./scripts/dev_exec.sh make <target>`.
- Rust workspace commands may also be run directly with `cargo`, without `./scripts/dev_exec.sh`:
  -  From the repository root on the host, use
     `cargo test --manifest-path src/rust/Cargo.toml [ -p <crate> ] --all-features -- --test-threads 1`.
  -  For checks, use
     `cargo check --manifest-path src/rust/Cargo.toml [ -p <crate> ] --all-features`.
  -  If you modify `messages/*.proto`, run `make generate-protobufs` before direct Rust `cargo`
     commands. Plain `cargo test`/`cargo check` does not regenerate the protobuf outputs.

## Coding Style & Naming Conventions
`.clang-format` (Chromium base, 4-space indent, Linux braces) and `.clang-tidy` govern C/C++. Use
`snake_case` for symbols, `PascalCase` for types, and `ALL_CAPS` for macros. Python utilities follow
`.pylintrc` rules (100-column limit, explicit imports). Rust crates rely on `rustfmt.toml` and the
pinned toolchain in `rust-toolchain.toml`; keep module paths aligned with `src/rust` and regenerate
bindings (`cbindgen`, protobuf) when interfaces change. When changing protobuf interfaces, run
`make generate-protobufs`.

* For C code changes, run `./scripts/dev_exec.sh ./scripts/format` to format the code.
* For Python changes, run `./scripts/dev_exec.sh ./scripts/format-python` to format the code.
* For Rust code changes, run
  `./scripts/dev_exec.sh cargo fmt --manifest-path src/rust/Cargo.toml --all` to format the code.

## Testing Guidelines
Place new C specs in `test/unit-test` and add doubles to `test/hardware-fakes` when hardware
behavior is mocked; follow the `test_<feature>.c` naming pattern and update CMake lists. Rust crates
use standard `tests/` modules or `#[cfg(test)]` blocks. Before opening a PR, run both `make
run-unit-tests` and `cargo test --manifest-path src/rust/Cargo.toml --all-features -- --test-threads 1`,
and refresh `make coverage` for cryptography or security-sensitive areas.

- in Rust unit tests, prefer .unwrap() over .expect().
- In Rust unit tests, if testing a function foo, name the test `test_foo` (or `test_foo_xyz` if it
  needs qualifiers).
- if a Rust unit test involves futures, use `#[async_test::test] async fn test_...`.
- in Rust unit tests, prefer .as_slice() instead of `&*` for wrapped/zeroized Vec<u8>.
- in Rust unit tests, prefer `hex!` literals for byte arrays/constants.

## Review Guidelines

- when reviewing a removed function call, check that the removed behavior was not required and was
  not dropped by accident during a refactor.
- when reviewing a removed function call, check if the callee became unused and should also be removed.
- Focus on memory issues

## Commit & Pull Request Guidelines
Write commits with a ≤50 character subject, blank line, and explanatory body per `CONTRIBUTING.md`;
reference issues via `refs #1234` or `fixes #1234`. Keep patches atomic—avoid mixing formatting and
logic. Pull requests should outline the change, list verification commands or screenshots, and flag
hardware requirements; mark drafts with `[WIP]` until they are ready. Wait to squash until reviews
conclude.


## Various

- when converting C code to Rust code, make the Rust code idiomatic, not a 1:1 rewrite.
- when exposing Rust functions to C using extern "C", use util::bytes::Bytes and
  util::Bytes::BytesMut ot pass in buffers and write to out buffers.
- when using Zeroizing<...> for buffers, use Zeroizing<Vec<u8>>. For other sensitive data, use
  Zeroizing<Box<...>>.
- when wrapping C functions, always use a '-sys' crate for the bindings, make it safe idiomatic
  Rust, with no C types in the input/output, especially no pointers. Results should be returned,
  not passed to an out param. Check all invariants in the C code and panic in case they are not met.
- don't stop the Rust docker container unless you have restart it, e.g. if the .containerversion
  changed after checking out a different commit.
- never commit a change if not explicitly being instructed to
- when porting or refactoring or rewriting code, retain original comments and docstrings if they
  still apply. when reviewing commits that refactor/move/rewrite code, point out if comments were
  dropped.
- when editing code, unless otherwise instructed, try to keep the diff small and not do any
  unprompted refactorings or core reorganizations.
- rust fmt all Rust files you modify, using 2024 edition.
- when working on BitBox03 UI code, do not reach into `bitbox_lvgl` FFI directly from feature code.
  If needed, add safe idiomatic wrappers to `bitbox-lvgl` first and use those wrappers instead.
