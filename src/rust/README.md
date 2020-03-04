# Enums

arm-none-eabi-gcc compiles with -fshort-enums by default. This means that if all variants of an
enum fits in a small type (such as `uint8_t`), then the enum will be backed by such a small type.


With `-fno-short-enums` (the default on other platforms) all enums will be `int32_t` sized as long
as they fit, otherwise `int64_t`.  `repr(C)` in rust also follows this and can therefore not be
used. Instead all enums that are exported must have an explicit size using `repr(u8)` for example.


# Crate/Libraries organisation

We create three archives/crates/libraries from the `bitbox02-rust-c` crate. Different versions of
the crate is created using cargo "features".

* `libbootloader_rust_c.a`
* `libbitbox02_rust_c.a`
* `libbitboxbase_rust_c.a`

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
