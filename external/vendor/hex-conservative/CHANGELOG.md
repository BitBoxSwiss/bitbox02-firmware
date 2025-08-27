# 0.2.1 - 2024-05-17

- Add a new `impl_fmt_traits` macro that can be used to implement `fmt::{LowerHex, UpperHex,
  Display, Debug}` [#90](https://github.com/rust-bitcoin/hex-conservative/pull/90)

# 0.2.0 - 2024-02-27

### Breaking changes

There are a bunch of breaking changes in this release, including:

- Re-write the `FromHex` trait [#80](https://github.com/rust-bitcoin/hex-conservative/pull/80)
- Revamp `BufEncoder` [#52](https://github.com/rust-bitcoin/hex-conservative/pull/52)
- A bunch of public errors have changed.

### Other improvements

- Bump MSRV to 1.56.1
- For improved ergonomics, add a prelude module [#36](https://github.com/rust-bitcoin/hex-conservative/pull/36)
- Add a `serde` module [#37](https://github.com/rust-bitcoin/hex-conservative/pull/37)
- Remove arbitrary padding limit [#41](https://github.com/rust-bitcoin/hex-conservative/pull/41)
- Make `fmt_hex_exact` honour `Formatter::precision`[#81](https://github.com/rust-bitcoin/hex-conservative/pull/81)

### Improve error handling

- Update the derives on error types [#31](https://github.com/rust-bitcoin/hex-conservative/pull/31)
- Hide error internal [#44](https://github.com/rust-bitcoin/hex-conservative/pull/44)
- Return specific error from `HexToByesIter::new` [#62](https://github.com/rust-bitcoin/hex-conservative/pull/62)

# 0.1.1 - 2023-07-19

- [Add `test_hex_unwrap`](https://github.com/rust-bitcoin/hex-conservative/pull/24) hex parsing macro for test usage.
- [Improve formatting](https://github.com/rust-bitcoin/hex-conservative/pull/25) hex for bytes slices e.g., support padding.

# 0.1.0 - 2023-06-20 Initial Release

- [Import](https://github.com/rust-bitcoin/hex-conservative/pull/1) code from the `bitcoin_hashes` and `bitcoin-internals` crates.
- [Add `Iterator` implementations](https://github.com/rust-bitcoin/hex-conservative/pull/9)
