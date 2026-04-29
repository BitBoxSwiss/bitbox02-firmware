# Commands

This file lists the repository commands agents should use and how to run them.

## Host Commands

Run regular Unix commands directly on the host:

- `git`
- `rg`
- `grep`
- `ls`
- `find`
- `sed`
- `cat`

## Development Container

- `make dockerpull`: fetch the maintained development container.
- `make dockerdev`: enter the maintained development container.

Do not stop the Rust docker container unless you restart it, for example after
`.containerversion` changes.

## Toolchain Commands

Use `./scripts/dev_exec.sh <command>` only for project-specific commands that
depend on the project toolchain or compiler environment.

In practice, the repository `make` targets in this file are project-specific
toolchain commands. When running from the host, invoke them through
`./scripts/dev_exec.sh make <target>`.

Do not wrap `./scripts/dev_exec.sh` itself in `bash -lc`. Prefer changing the
working directory with command arguments such as `tar -C <PATH>`. If a command
genuinely needs shell features such as pipes, pass an explicit shell as the
command:

```sh
./scripts/dev_exec.sh bash -lc 'cat versions.json | jq'
```

## Build Targets

Run these through `./scripts/dev_exec.sh` from the host:

- `./scripts/dev_exec.sh make firmware`: compile the firmware ELF into `build/`.
- `./scripts/dev_exec.sh make bootloader`: compile the bootloader ELF into
  `build/`.
- `./scripts/dev_exec.sh make simulator`: build the Linux simulator under
  `build-build-noasan/bin/`.
- `./scripts/dev_exec.sh make unit-test`: build the C cmocka/CTest suite with
  ASan/UBSan.
- `./scripts/dev_exec.sh make run-unit-tests`: run the C cmocka/CTest suite.
- `./scripts/dev_exec.sh make run-rust-clippy`: lint Rust code with the
  workspace configuration.
- `./scripts/dev_exec.sh make generate-protobufs`: regenerate protobuf outputs
  after changes to `messages/*.proto`.

You may use `make -j$(nproc)` to speed up compilation. Do not use `make -j`
without specifying the number of processing units.

Never run multiple `make` commands in parallel. For example, run this:

```sh
./scripts/dev_exec.sh make -j$(nproc) firmware
./scripts/dev_exec.sh make -j$(nproc) bootloader
./scripts/dev_exec.sh make -j$(nproc) factory-setup
```

Do not combine those targets into one parallel `make` invocation.

## Rust Workspace Commands

Cargo commands for the Rust workspace may be run directly on the host by passing
`--manifest-path src/rust/Cargo.toml`.

Use these forms from the repository root:

```sh
cargo test --manifest-path src/rust/Cargo.toml [ -p <crate> ] --all-features -- --test-threads 1
cargo check --manifest-path src/rust/Cargo.toml [ -p <crate> ] --all-features
```

If `messages/*.proto` changed, run
`./scripts/dev_exec.sh make generate-protobufs` before direct Rust `cargo`
commands. Plain `cargo test` and `cargo check` do not regenerate protobuf
outputs.

## Formatting Commands

- C code: `./scripts/dev_exec.sh ./scripts/format`
- Python code: `./scripts/dev_exec.sh ./scripts/format-python`
- Rust code:

```sh
./scripts/dev_exec.sh cargo fmt --manifest-path src/rust/Cargo.toml --all
```
