# Enums

arm-none-eabi-gcc compiles with -fshort-enums by default. This means that if all variants of an
enum fits in a small type (such as `uint8_t`), then the enum will be backed by such a small type.


With `-fno-short-enums` (the default on other platforms) all enums will be `int32_t` sized as long
as they fit, otherwise `int64_t`.  `repr(C)` in rust also follows this and can therefore not be
used. Instead all enums that are exported must have an explicit size using `repr(u8)` for example.


# Crate/Libraries organisation

We create one archive/crate/library per binary target from the `bitbox02-rust-c` crate, by
activating appropriate target feature.

* `libbootloader_rust_c.a`
* `libfirmware-btc_rust_c.a`
* etc. for each firmware and bootloader target made by CMake.

In the next layer of crates we have "business logic":

* bitbox02-rust
* util

In the next layer of crates we have safe wrappers for FFI methods:

* bitbox02

The bottom-most layer are bindings generated from C header files:

* bitbox02-sys

# Header file

We generate one header file `rust.h` and ever product specific function is `#ifdeffed` with
`RUST_PRODUCT_*` macro.

# Vendoring

Run the vendoring script `vendor-rust.sh` to vendor dependencies from
crates.io. The script will ensure that also rust std libs dependencies are
vendored.

If dependencies are updated, remember to also update in `test/simulator-ng`. You can use

```
cargo update <package> --precise <version>
```

to specify specific versions to up/downgrade to.

When the toolchain (and standard libraries) are updated they usually depend on
newer versions of crates. Update our lockfile to the same versions to avoid
vendoringing duplicate versions.
