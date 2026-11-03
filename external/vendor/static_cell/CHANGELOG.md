# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 2.1.1 - 2025-06-22

- Soundness fix: ConstStaticCell should only be Send/Sync if T: Send. (#19, #20)

## 2.1.0 - 2024-04-26 - YANKED

- Added `ConstStaticCell` (#14)

## 2.0.0 - 2023-11-02

- Do not enable feature `critical-section` in `portable-atomic`. In targets that need
  atomic polyfilling, you now need to explicitly enable either of `critical-section` or
  `unsafe-assume-single-core` in `portable-atomic`.

## 1.3.0 - 2023-11-02

- Switch from `atomic-polyfill` to `portable-atomic`. Note: to be backwards compatible
  this crate is enabling the `critical-section` feature in `portable-atomic`. If your
  chip is single-core, you might want to upgrade to `static-cell` v2.0 so you can enable
  the feature `unsafe-assume-single-core` instead, which is slightly more efficient.

## 1.2.0 - 2023-07-03

- Add panic-free `try_init()`, `try_init_with()`, `try_uninit()`.

## 1.1.0 - 2023-06-01

- Add `uninit()`.
- Add `make_static!` macro.

## 1.0.0 - 2022-08-22

- First release
