# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

## Unreleased

## 0.1.1

- Deprecate `unsafe_pinned!` and `unsafe_unpinned!`. Use `pin-project` or `pin-project-lite` crate instead. (#33)
- Soft-deprecate `pin_mut!` in favor of `std::pin::pin!` that stabilized in Rust 1.68. (#39)

## 0.1.0

- First non-alpha release.
- Fix some `core` symbol hygiene.
