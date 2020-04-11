# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.4 (2020-03-02)
### Fixed
- Avoid accidental `alloc` and `std` linking ([#105])

[#105]: https://github.com/RustCrypto/stream-ciphers/pull/105

## 0.3.3 (2020-01-18)
### Changed
- Replace macros with `Rounds` trait + generics ([#100])

### Fixed
- Fix warnings when building with `rng` feature alone ([#99])

[#99]: https://github.com/RustCrypto/stream-ciphers/pull/99
[#100]: https://github.com/RustCrypto/stream-ciphers/pull/100

## 0.3.2 (2020-01-17)
### Added
- `CryptoRng` marker on all `ChaCha*Rng` types ([#91])

[#91]: https://github.com/RustCrypto/stream-ciphers/pull/91

## 0.3.1 (2020-01-16)
### Added
- Parallelize AVX2 backend ([#87])
- Benchmark for `ChaCha20Rng` ([#87])

### Fixed
- Fix broken buffering logic ([#86])

[#86]: https://github.com/RustCrypto/stream-ciphers/pull/86
[#87]: https://github.com/RustCrypto/stream-ciphers/pull/87

## 0.3.0 (2020-01-15) [YANKED]

NOTE: This release was yanked due to a showstopper bug in the newly added
buffering logic which when seeking in the keystream could result in plaintexts
being clobbered with the keystream instead of XOR'd correctly.

The bug was addressed in v0.3.1 ([#86]).

### Added
- AVX2 accelerated implementation ([#83])
- ChaCha8 and ChaCha20 reduced round variants ([#84])

### Changed
- Simplify portable implementation ([#76])
- Make 2018 edition crate; MSRV 1.34+ ([#77])
- Replace `salsa20-core` dependency with `ctr`-derived buffering ([#81])

### Removed
- `byteorder` dependency ([#80])

[#76]: https://github.com/RustCrypto/stream-ciphers/pull/76
[#77]: https://github.com/RustCrypto/stream-ciphers/pull/77
[#80]: https://github.com/RustCrypto/stream-ciphers/pull/80
[#81]: https://github.com/RustCrypto/stream-ciphers/pull/81
[#83]: https://github.com/RustCrypto/stream-ciphers/pull/83
[#84]: https://github.com/RustCrypto/stream-ciphers/pull/84

## 0.2.3 (2019-10-23)
### Security
- Ensure block counter < MAX_BLOCKS ([#68])

[#68]: https://github.com/RustCrypto/stream-ciphers/pull/68

## 0.2.2 (2019-10-22)
### Added
- SSE2 accelerated implementation ([#61])

[#61]: https://github.com/RustCrypto/stream-ciphers/pull/61

## 0.2.1 (2019-08-19)
### Added
- Add `MAX_BLOCKS` and `BLOCK_SIZE` constants ([#47])

[#47]: https://github.com/RustCrypto/stream-ciphers/pull/47

## 0.2.0 (2019-08-18)
### Added
- `impl SyncStreamCipher` ([#39])
- `XChaCha20` ([#36])
- Support for 12-byte nonces ala RFC 8439 ([#19])

### Changed
- Refactor around a `ctr`-like type ([#44])
- Extract and encapsulate `Cipher` type ([#43])
- Switch tests to use `new_sync_test!` ([#42])
- Refactor into `ChaCha20` and `ChaCha20Legacy` ([#25])

### Fixed
- Fix `zeroize` cargo feature ([#21])
- Fix broken Cargo feature attributes ([#21])

[#44]: https://github.com/RustCrypto/stream-ciphers/pull/44
[#43]: https://github.com/RustCrypto/stream-ciphers/pull/43
[#42]: https://github.com/RustCrypto/stream-ciphers/pull/42
[#39]: https://github.com/RustCrypto/stream-ciphers/pull/39
[#36]: https://github.com/RustCrypto/stream-ciphers/pull/36
[#25]: https://github.com/RustCrypto/stream-ciphers/pull/25
[#21]: https://github.com/RustCrypto/stream-ciphers/pull/21
[#19]: https://github.com/RustCrypto/stream-ciphers/pull/19

## 0.1.0 (2019-06-24)

- Initial release
