# Poly1305

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
![Maintenance Status: Experimental][maintenance-image]
[![Build Status][build-image]][build-link]

[Poly1305][1] is a [universal hash function][2] which, when combined with a cipher,
can be used as a [Message Authentication Code (MAC)][3].

In practice, Poly1305 is primarily combined with ciphers from the
[Salsa20 Family][4] such as in [ChaCha20Poly1305][5] and [XSalsa20Poly1305][6]
(a.k.a. NaCl `crypto_secretbox`).

[Documentation][docs-link]

## Security Warning

No security audits of this crate have ever been performed, and it has not been
thoroughly assessed to ensure its operation is constant-time on common CPU
architectures.

USE AT YOUR OWN RISK!

## License

Licensed under either of:

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/poly1305.svg
[crate-link]: https://crates.io/crates/poly1305
[docs-image]: https://docs.rs/poly1305/badge.svg
[docs-link]: https://docs.rs/poly1305/
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.34+-blue.svg
[maintenance-image]: https://img.shields.io/badge/maintenance-experimental-blue.svg
[build-image]: https://travis-ci.com/RustCrypto/universal-hashes.svg?branch=master
[build-link]: https://travis-ci.com/RustCrypto/universal-hashes

[//]: # (general links)

[1]: https://en.wikipedia.org/wiki/Poly1305
[2]: https://en.wikipedia.org/wiki/Universal_hashing
[3]: https://en.wikipedia.org/wiki/Message_authentication_code
[4]: https://cr.yp.to/snuffle/salsafamily-20071225.pdf
[5]: https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305
[6]: https://github.com/RustCrypto/AEADs/tree/master/xsalsa20poly1305
