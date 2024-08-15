r-efi-alloc
===========

UEFI Memory Allocator Integration

The r-efi-alloc project integrates the UEFI memory allocator routines with the
rust standard library allocation hooks. This allows using the `alloc` standard
library of rust on UEFI systems.

### Project

 * **Website**: <https://github.com/r-efi/r-efi/wiki>
 * **Bug Tracker**: <https://github.com/r-efi/r-efi-alloc/issues>

### Requirements

The requirements for this project are:

 * `rustc >= 1.62.0`
 * `r-efi >= 4.0.0`

### Build

To build this project, run:

```sh
cargo build
```

Available configuration options are:

 * **allocator_api**: Provide integration with the experimental upstream rust
                      allocators (tracked with the `allocator_api` feature).

 * **examples**: This feature-selector enables compilation of examples. This
                 is disabled by default, since they will only compile
                 successfully on UEFI targets.

No special requirements exist to compile for UEFI targets. Native compilations
work out of the box without any adjustments. For cross-compilations, either use
the toolchains distributed through `rustup`, build your own toolchains, or use
`-Zbuild-std` as shown below.

If you do not use the official toolchains, you will likely need a nightly rust
compiler with the rust-src component enabled:

```sh
rustup toolchain install nightly
# OR
rustup update

rustup component add --toolchain nightly rust-src
```

Be sure to update all components to the most recent version.

##### Build via: cargo/rustc nightly with -Zbuild-std

```sh
cargo +nightly build \
    -Zbuild-std=core,compiler_builtins,alloc \
    -Zbuild-std-features=compiler-builtins-mem \
    --target x86_64-unknown-uefi \
    --features examples \
    --examples
```

### Repository:

 - **web**:   <https://github.com/r-efi/r-efi-alloc>
 - **https**: `https://github.com/r-efi/r-efi-alloc.git`
 - **ssh**:   `git@github.com:r-efi/r-efi-alloc.git`

### License:

 - **MIT** OR **Apache-2.0** OR **LGPL-2.1-or-later**
 - See AUTHORS file for details.
