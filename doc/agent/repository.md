# Repository Guide

This file captures repository layout and architecture constraints for agents.

## Layout

- Core firmware and bootloader code is in `src/`.
- Important C subsystems under `src/` include `bootloader`, `usb`, `ui`, and
  `securechip`.
- Rust code lives in the workspace under `src/rust/`.
- Tests live in `test/`:
  - `test/unit-test/` contains cmocka suites.
  - `test/hardware-fakes/` contains device shims.
  - `test/simulator/` contains C simulator assets.
  - `test/simulator-graphical/` contains rust simulator assets.
  - `test/simulator-graphical-bb03/` contains rust simulator assets.
- The BitBox02 Python client library is in `py/bitbox02`.
- Supporting tooling is in `scripts/`, including CI and J-Link macros.
- Manuals and human-facing project documentation are in `doc/`.
- Agent-maintained instruction detail lives in `doc/agent/`.
- Vendored dependencies are tracked in `external/`.

## Rust Crate Boundaries

The firmware has C and Rust code. The most important Rust crates are:

- `bitbox02-rust`: main app logic. It may expose functions to C with
  `extern "C"`. If it needs access to C functions, it must go through
  `bitbox-hal`. Never add `bitbox02` or `bitbox02-sys` as dependencies of
  `bitbox02-rust`.
- `bitbox02-sys`: generated bindings to BitBox02-specific C code. `build.rs`
  contains the exposed functions. `wrapper.h` must include any C headers or
  declarations added to `build.rs`.
- `bitbox-hal`: device-specific interface used by device-agnostic app logic.
- `bitbox02`: safe idiomatic wrapper around `bitbox02-sys` that implements the
  `bitbox-hal` interface.

`bitbox02-rust` is pure Rust and device agnostic. It must access
device-specific functionality through `bitbox-hal`. The migration is in
progress; migrate only what is necessary for the current scope.
