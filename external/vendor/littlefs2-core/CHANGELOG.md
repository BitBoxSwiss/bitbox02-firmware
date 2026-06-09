# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

-

## [v0.1.2](https://github.com/trussed-dev/littlefs2/releases/tag/core-0.1.2) - 2025-10-16

- Make `io::Result` alias accept other error types
- Add `debug-error` feature to have a proper debug implementation of errors
- Add support for `heapless` v0.9 and `heapless-bytes` v0.5

## [v0.1.1](https://github.com/trussed-dev/littlefs2/releases/tag/core-0.1.1) - 2025-01-16

- Make `Path` and `PathBuf` more const-friendly:
  - Make `Path::as_ptr` and `PathBuf::from_buffer_unchecked` const.
  - Add const `Path::const_eq`, `PathBuf::from_path`, `PathBuf::as_path` and `PathBuf::as_str` methods.
- Add `path_buf` macro to construct a `PathBuf` from a string literal.

## [v0.1.0](https://github.com/trussed-dev/littlefs2/releases/tag/core-0.1.0) - 2024-10-17

Initial release with the core types from `littlefs2`.
