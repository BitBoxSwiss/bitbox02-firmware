# Changelog

## [Unreleased]
- Add [rlp] as an optional dependency of the bitbox02 package using `ethereum`
  extras key, as well as `all` for all optional deps.

[rlp]: https://pypi.org/project/rlp/

## 6.0.0
- Offset the recoverable ID by 27 in the signature returned by `eth_sign_msg()`.
- Rename `BTCOutputExternal.hash` to `BTCOutputExternal.payload`.
