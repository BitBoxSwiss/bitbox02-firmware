# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.8] - 2025-08-21
- add `portable-atomic` feature to replace `core::sync::atomic::AtomicUsize` with `portable_atomic::AtomicUsize`

## [0.1.7] - 2023-08-17
- fix breakage caused by using internal `log` APIs

## [0.1.6] - 2022-08-22
- `std-log` feature to use normal logging for testing etc. @robin-nitrokey

## [0.1.5]- 2022-06-25
- fix incorrect MIT license copyright info

## [0.1.4]- 2022-03-05

- add missing dyn trait, will yank 0.1.3

## [0.1.3]- 2022-03-05

- Doc-hide some of the macros so our dependees don't have documentation noise
- 2021 edition

## [0.1.2]- 2021-04-23

- Allow independent (e.g. smaller) render buffers @conorpp

## [0.1.1]- 2021-04-13

- Bug fixes by @conorpp

## [0.1.0]- 2021-02-27

- Promote existing state (0.1.0-alpha.3) to 0.1.0

## [0.1.0-alpha.1] - 2020-11-29

- Revamp of logs: only local/gated macros are generated now
- Remove optional semihosting dependency, relegate example implementation to QEMU test
- Relegate stdout/stderr flushers to example submodule
- Simplification of `hex` submodule, removing typenum and implementing all block sizes
- Addition of "truncated" hex formats like `hex_fmt`, remove it as dependency instead

## [0.1.0-alpha.1] - 2020-11-28

Test the release process, meditate on the `local_*!` issue.
