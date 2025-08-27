# binascii
[![Build Status](https://travis-ci.org/naim94a/binascii-rs.svg?branch=master)](https://travis-ci.org/naim94a/binascii-rs)

Rust implementation of useful binascii functions.

* Encode & Decode support for:
    + Base16 (Hex)
    + Base32
    + Base64
* no_std support
* never panics

## Getting Started
* Add `binascii` to your package's `Cargo.toml`:
    ```toml
    [dependencies]
    binascii = "0.1"
    ```
* Encoders and decoders are enabled by default. To enable only decoders, use the
  `"decode"` feature. To enable only encoders, use the `"encode"` feature:
    ```toml
    # Enable encoders only.
    [dependencies]
    binascii = { version = "0.1", default-features = false, features = ["encode"] }
    
    # Enable decoders only.
    [dependencies]
    binascii = { version = "0.1", default-features = false, features = ["decode"] }
    ```
* The API is very simple, head over to https://docs.rs/binascii/.

## Why `binascii`?
- This library was written with security in mind, and includes unit tests to prevent vulnerabilities found in many other implementations (many can be found [here](https://www.google.com/search?q=site%3Acvedetails.com+"base64"+inurl%3Acve&oq=site%3Acvedetails.com+"base64"+inurl%3Acve)).
- There are no "unsafe" blocks, such blocks are forbidden.
- `no-std` is supported for your bare-metal & embedded projects.
