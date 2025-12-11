# Repository Guidelines for bitbox02-firmware

## Project Structure & Module Organization
Core firmware and bootloader code sits in `src/`, grouped by subsystem (`bootloader`, `usb`, `ui`,
`securechip`, plus a `rust/` workspace). Tests live in `test/`: `unit-test/` for cmocka suites,
`hardware-fakes/` for device shims, and `simulator/` assets. The BitB02 Python client library is in
`py/bitbox02`. Supporting tooling is in `scripts/` (CI, J-Link macros), and `doc/` for
manuals. Vendored dependencies are tracked in `external/`.

The firmware has C and Rust code. Rust code lines in src/rust. The rust crates are:
- bitbox02-rust: the main app logic. It can expose functions to C using extern "C". If it needs
  access to C functions, it has to go through the bitbox02 crate. Never add bitbox02-sys dep to
  bitbox02-rust or use if the dep is present.
- bitbox02-sys: generated bindings to C code. build.rs contains the functions etc that are
  exposed. See also `wrapper.h`, it needs to include any C headers/declarations that are added to
  build.rs.
- bitbox02: wraps bitbox02-sys functions as idiomatic safe Rust

bitbox02-rust is pure Rust. If it needs to use a C function, it should instead use the safe C
wrapper in the bitbox02 crate.

## Build, Test, and Development Commands
- `make dockerpull` / `make dockerdev`: fetch and enter the maintained development container.

All make commands are to be run inside docker like this: `./scripts/docker_exec.sh make -j <command>`, e.g.  `./scripts/docker_exec.sh make -j firmware`.

- `make firmware` / `make bootloader`: compile firmware or bootloader ELFs into `build/`.
- `make simulator`: build the Linux simulator under `build-build-noasan/bin/`.
- `make unit-test && make run-unit-tests`:  build and run the C cmocka/CTest suite with ASan/UBSan.
- `make run-rust-unit-tests`: build and run the Rust unit tests
- `make run-rust-clippy`: lint Rust code with the workspace configuration.

## Coding Style & Naming Conventions
`.clang-format` (Chromium base, 4-space indent, Linux braces) and `.clang-tidy` govern C/C++. Use
`snake_case` for symbols, `PascalCase` for types, and `ALL_CAPS` for macros. Python utilities follow
`.pylintrc` rules (100-column limit, explicit imports). Rust crates rely on `rustfmt.toml` and the
pinned toolchain in `rust-toolchain.toml`; keep module paths aligned with `src/rust` and regenerate
bindings (`cbindgen`, protobuf) when interfaces change.

For C code changes, run ./scripts/format to format the code. For Python changes, run `black` to format the code.

## Testing Guidelines
Place new C specs in `test/unit-test` and add doubles to `test/hardware-fakes` when hardware
behavior is mocked; follow the `test_<feature>.c` naming pattern and update CMake lists. Rust crates
use standard `tests/` modules or `#[cfg(test)]` blocks. Before opening a PR, run both `make
run-unit-tests` and `make run-rust-unit-tests`, and refresh `make coverage` for cryptography or
security-sensitive areas.

- in Rust unit tests, prefer .unwrap() over .expect().
- In Rust unit tests, if testing a function foo, name the test `test_foo` (or `test_foo_xyz` if it needs qualifiers).
- in Rust unit tests, prefer .as_slice() instead of `&*` for wrapped/zeroized Vec<u8>.

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
- when wrapping C functions in the bitbox02 crate, make it safe idiomatic Rust, with no C types in
  the input/output. Results should be returned, not passed to an out param.
- don't stop the Rust docker container unless you have restart it, e.g. if the .containerversion
  changed after checking out a different commit.
- never commit a change if not explicitly being instructed to
- when porting or refactoring or rewriting code, retain original comments and docstrings if they
  still apply. when reviewing commits that refactor/move/rewrite code, point out if comments were
  dropped.
- when editing code, unless otherwise instructed, try to keep the diff small and not do any
  unprompted refactorings or core reorganizations.
- rust fmt all Rust files you modify, using 2024 edition.
