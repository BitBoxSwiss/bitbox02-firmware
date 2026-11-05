# pin-utils

[![crates-badge]][crates-url]
[![docs-badge]][docs-url]
[![license-badge]][license]
[![rustc-badge]][rustc-url]

[crates-badge]: https://img.shields.io/crates/v/pin-utils.svg
[crates-url]: https://crates.io/crates/pin-utils
[docs-badge]: https://docs.rs/pin-utils/badge.svg
[docs-url]: https://docs.rs/pin-utils
[license-badge]: https://img.shields.io/crates/l/pin-utils.svg
[license]: #license
[rustc-badge]: https://img.shields.io/badge/rustc-1.33+-lightgray.svg
[rustc-url]: https://blog.rust-lang.org/2019/02/28/Rust-1.33.0.html

Utilities for pinning

[Documentation][docs-url]

**Note:** Since Rust 1.68, all APIs in this crate are now soft-deprecated or deprecated:

- `pin_utils::pin_mut!` is soft-deprecated in favor of [`pin!` macro in the standard library](https://doc.rust-lang.org/std/pin/macro.pin.html) that stabilized in Rust 1.68.
- `pin_utils::{unsafe_pinned,unsafe_unpinned}` are **deprecated** in favor of safe alternatives: [pin-project](https://crates.io/crates/pin-project), [pin-project-lite](https://crates.io/crates/pin-project-lite)

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
pin-utils = "0.1"
```

Now, you can use it:

```rust
use pin_utils::pin_mut; // And more...
```

The current pin-utils requires Rust 1.33 or later.

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in pin-utils by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
