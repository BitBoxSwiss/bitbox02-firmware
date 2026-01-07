# 1.0.1 - 2025-12-02

- Remove `doc_auto_cfg`

# 1.0.0 - 2025-07-11

- Removed niche reexports from root
- Various documentation improvements
- A few test improvements

# 1.0.0 release candidate 2 -- 2025-06-27

- Removed iterator API that got published by accident

# 1.0.0 release candidate 1 -- 2025-05-16

- Exports `decode_to_array`, `decode_to_vec`, and all error types.

The goal of this release is to enable library authors to use hex-conservative,
exposing error types from this library in their own APIs, and not need to worry
that they will face API breakage because of this.

Future minor releases will add encoding support and a more expansive set of traits.

# 0.3.0 - 2024-09-18

- Re-implement `HexWriter` [#113](https://github.com/rust-bitcoin/hex-conservative/pull/113)
- Fix `Display` width and precision of `DisplayByteSlice` [#114](https://github.com/rust-bitcoin/hex-conservative/pull/114)
- Encoding performance improvements [#117](https://github.com/rust-bitcoin/hex-conservative/pull/117)
- Relax bounds of `BytesToHexIter` [#118](https://github.com/rust-bitcoin/hex-conservative/pull/118)
- Add case to `BytesToHexIter` [#120](https://github.com/rust-bitcoin/hex-conservative/pull/120)
- Encapsulate unsafe code inside the table module [#121](https://github.com/rust-bitcoin/hex-conservative/pull/121)
- Fix `HexToBytesIter::size_hint` [#122](https://github.com/rust-bitcoin/hex-conservative/pull/122)
- Restrict `BufEncoder` to uniform case encoding [#119](https://github.com/rust-bitcoin/hex-conservative/pull/119)
- Remove the `core2` dependency [#105](https://github.com/rust-bitcoin/hex-conservative/pull/105)
- Introduce more serde utilities [#92](https://github.com/rust-bitcoin/hex-conservative/pull/92)
- Add `impl_fmt_traits` macro [#90](https://github.com/rust-bitcoin/hex-conservative/pull/90)
- Enable serialization of byte slices [#96](https://github.com/rust-bitcoin/hex-conservative/pull/96)
- Bump MSRV to Rust `1.63.0` [#102](https://github.com/rust-bitcoin/hex-conservative/pull/102)
- Store position of invalid char in `InvalidCharError` [#107](https://github.com/rust-bitcoin/hex-conservative/pull/107)
- Only encode the bytes formatted in the hex string [#108](https://github.com/rust-bitcoin/hex-conservative/pull/108)

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

# 0.1.2 - 2024-05-14

- Fix bug in output of `InvalidError` [#88](https://github.com/rust-bitcoin/hex-conservative/pull/88).

# 0.1.1 - 2023-07-19

- [Add `test_hex_unwrap`](https://github.com/rust-bitcoin/hex-conservative/pull/24) hex parsing macro for test usage.
- [Improve formatting](https://github.com/rust-bitcoin/hex-conservative/pull/25) hex for bytes slices e.g., support padding.

# 0.1.0 - 2023-06-20 Initial Release

- [Import](https://github.com/rust-bitcoin/hex-conservative/pull/1) code from the `bitcoin_hashes` and `bitcoin-internals` crates.
- [Add `Iterator` implementations](https://github.com/rust-bitcoin/hex-conservative/pull/9)
