# Hex literals without proc macros.

This crate implements minimalistic hex literal macros without use of proc macros.
The advantages are much faster compile times, ability to work with non-literal const values and
easier auditing.
However, because of the use of `const fn` the crate has some limitations depending on the Rust
version.

Either way, the resulting type is a byte array (`[u8; N]`) that doesn't force you to write down
its length. This is already very useful since the compiler can prove the length and you avoid
runtime allocations.

The crate is `no_std` and does **not** require an allocator.

## Usage

Just pass a `&str` *constant* (usually a literal) into the hex macro.

Example

```rust
use hex_lit::hex;

let array = hex!("2a15ff");
assert_eq!(&array, &[42, 21, 255]);

```

The input MUST NOT contain any spaces or other separators and it MUST have even length.
Note that you can still separate long strings into chunks using the concat macro:

```rust
use hex_lit::hex;

let array = hex!(concat!(
    "0000002a000000",
    "ffffffffffffff",
));
assert_eq!(&array, &[0, 0, 0, 42, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255]);

```

## Features depending on Rust version

* 1.41.1+ - the MSRV, use in const contexts is impossible, only the hex! macro is available.
* 1.46.0+ - usage in const contexts is available and (regardless of cargo features) correctness
            of input is checked at compile time. 
* 1.57+ - nicer error messages for bad inputs (regardless of cargo features)

## Cargo features

* `rust_v_1_46` - acknowledges bumping MSRV to 1.46+ and enables usage in const context.

Bumping MSRV is intentionally explicit.

Because of improved input checking it is recommended to use Rust 1.46+, prefereably 1.57+ in CI
even if your targeted MSRV is lower.

## License

MITNFA
