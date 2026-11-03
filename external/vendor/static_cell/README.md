# static-cell
[![crates.io](https://img.shields.io/crates/d/static_cell.svg)](https://crates.io/crates/static_cell)
[![crates.io](https://img.shields.io/crates/v/static_cell.svg)](https://crates.io/crates/static_cell)
[![Documentation](https://docs.rs/static_cell/badge.svg)](https://docs.rs/static_cell)

Statically allocated, initialized at runtime cell.

`StaticCell` provides a no-std-compatible, no-alloc way to reserve memory at compile time for
a value, but initialize it at runtime, and get a `'static` reference to it.

This is useful in the following scenarios:

- You need `&'static T`, but `T` can't be constructed in const context so you can't simply use a `static`.
- You need `&'static mut T`, not just `&'static T`.

## Example

```rust
use static_cell::StaticCell;

// Statically allocate memory for a `u32`.
static SOME_INT: StaticCell<u32> = StaticCell::new();

// Initialize it at runtime. This returns a `&'static mut`.
let x: &'static mut u32 = SOME_INT.init(42);
assert_eq!(*x, 42);

// Trying to call `.init()` again would panic, because the StaticCell is already initialized.
// SOME_INT.init(42);
```

## Alternatives

- If you can use `alloc`, you can use `Box::leak()`.
- If you're OK with `unsafe`, you can use `static mut THING: MaybeUninit<T>`.
- If you need just `&'static T` (instead of `&'static mut T`), there's [`OnceCell`](https://doc.rust-lang.org/stable/core/cell/struct.OnceCell.html) (not thread-safe though) or [`OnceLock`](https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html) (thread-safe, but requires `std`).

## Interoperability

This crate uses [`portable-atomic`](https://crates.io/crates/portable-atomic), so on targets without native
atomics you must import a crate that provides a [`critical-section`](https://github.com/rust-embedded/critical-section) 
implementation. See the `critical-section` README for details.

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.56 and up. It might compile with
older versions but that may change in any new patch release.

## License

This work is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
