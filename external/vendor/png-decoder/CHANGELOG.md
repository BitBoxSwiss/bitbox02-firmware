# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## Unreleased - ReleaseDate

## 0.2.0 - 2022-10-09

- Update `miniz_oxide` dependency from 0.4 to 0.8
- Update `num_enum` dependency from 0.5 to 0.7
- Delete unused `pub struct Color`.
- `decode()` now produces `Vec<[u8; 4]>` instead of `Vec<u8>`. Use `Vec::into_flattened()` to get the flattened `Vec` of bytes.

## 0.1.1 - 2022-10-09

- Fix an issue with a trailing zero causing a premature EOF in the stream
- Relicense to MIT OR Apache-2.0 OR Zlib

## 0.1.0 - 2021-01-04

- First release
