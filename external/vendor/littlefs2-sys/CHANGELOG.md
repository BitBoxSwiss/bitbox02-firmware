# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

-

## [0.3.2] - 2026-02-25

### Added

- Derive `Default` if possible
- Add `unstable-littlefs-patched` feature.  Enabling this feature may break semantic versioning guarantees. If this feature is enabled, a patched version of littlefs ([v2.9-trussed.1](https://github.com/trussed-dev/littlefs/releases/tag/v2.9-trussed.1)) is used with the following changes from v2.9.3:
  - [Add config flag to disable block count check on mount](https://github.com/trussed-dev/littlefs/commit/5328ae4b2ad95088a8079c0dfbc623df45598a88)
  - [Add support for shrinking a filesystem](https://github.com/trussed-dev/littlefs/commit/9a6ef46eb43e7edfdfdba04e50c602a8173b456c)

## [0.3.1] - 2025-02-27

### Fixed

- Fix code generation with `multiversion` feature

## [0.3.0] - 2025-02-26

### Changed

- Update littlefs to [v2.9.3](https://github.com/littlefs-project/littlefs/releases/tag/v2.9.3)
  - Add `multiversion` feature

### Fixed

- Fix build script for platforms where the Rust target is not equal to the clang target ([#16](https://github.com/trussed-dev/littlefs2-sys/pull/16))

## [0.2.0] - 2024-05-28

### Added

- Add `malloc` feature flag. It allows `littlefs` to link to `malloc` and `free` instead of relying on the caller to allocate memory ([#9][])
- Add a `software-intrinsics` feature flag, to disable the use of compiler intrinsics when compiling littlefs ([#12][])

### Changed

- Upgrade `bindgen` to 0.69.4 and limit symbols to those prefixed with `lfs_` and `LFS_` ([#10][])
- Use `core::ffi::*` instead of `cty::*` ([#14][])

[#9]: https://github.com/trussed-dev/littlefs2-sys/pull/10
[#10]: https://github.com/trussed-dev/littlefs2-sys/pull/10
[#12]: https://github.com/trussed-dev/littlefs2-sys/pull/12
[#14]: https://github.com/trussed-dev/littlefs2-sys/pull/14

## [0.1.7] - 2022-01-26

### Fixed

- Fixed compilation issue caused by other crates also using `bindgen` by selecting the `runtime` feature ([#5])

[#5]: https://github.com/trussed-dev/littlefs2-sys/pull/5
[#9]: https://github.com/trussed-dev/littlefs2-sys/pull/9

[Unreleased]: https://github.com/trussed-dev/littlefs2-sys/compare/0.3.2...HEAD
[0.1.7]: https://github.com/trussed-dev/littlefs2-sys/compare/0.1.6...0.1.7
[0.2.0]: https://github.com/trussed-dev/littlefs2-sys/compare/0.1.7...0.2.0
[0.3.0]: https://github.com/trussed-dev/littlefs2-sys/compare/0.2.0...0.3.0
[0.3.1]: https://github.com/trussed-dev/littlefs2-sys/compare/0.3.0...0.3.1
[0.3.2]: https://github.com/trussed-dev/littlefs2-sys/compare/0.3.1...0.3.2
