# png-decoder

A pure-Rust, no_std compatible PNG decoder.

See [examples/basic.rs](examples/basic.rs) for basic usage. The `decode()` function returns a PNG header and associated byte data, represented as RGBA (8 bits per channel).

## Dependencies
- cargo
- rustc

## Build

```
$ cargo build --release
```

## Testing

```
$ cargo test
```

## Code Format

The formatting options currently use nightly-only options.

```
$ cargo +nightly fmt
```

## Code Linting

```
$ cargo clippy
```

## Code Fuzzing

Fuzzing requires a nightly toolchain. Fuzzing for this project is currently confirmed to work with:

```
+nightly-2020-10-07
```

## Running

```
cargo install cargo-fuzz
cargo +nightly-2020-10-07 fuzz run png_decoder_fuzzer
```
