# BitBox test vectors

## Bitcoin transactions

The Bitcoin transaction vector schema and generator are namespaced under `src/btc_transaction/`,
leaving room for sibling vector families. Its readable Rust constructors in
`src/btc_transaction/cases/` are the source of truth. Each scenario is authored once as a PSBT.
Client libraries consume that PSBT through their public signing API, which converts it to the
firmware protocol request. The firmware tests derive the same request directly from the PSBT and
its options. A consumer should explicitly document and skip only the options its public API cannot
express.

Version expectations and the previous-transaction requirement live on the vector.

Explicit derivation paths use canonical `m/...` strings rather than protocol-level integer arrays.
Confirm and transaction-fee screens include their `longtouch` requirement. Client simulators whose
stdout protocol omits that flag compare the remaining observable screen fields.
`expected_signatures` describes the signature slots newly inserted by the signer, not the complete
post-signing signature set. It records the input, key, optional Taproot leaf and sighash semantics
without pinning nondeterministic signature bytes. Pre-existing cosigner signatures may remain in a
PSBT, but an expected insertion slot must not already be populated.
Consumers may separately pin a small set of deterministic signatures for implementation-specific
regression coverage.

Generate the canonical JSON explicitly from the firmware repository root:

```sh
cargo run --manifest-path src/rust/Cargo.toml \
  -p bitbox-test-vectors --bin generate-btc-test-vectors
```

This is intentionally not a `build.rs`: ordinary builds must not rewrite committed fixtures. To
verify that the checked-in artifact matches the constructors without writing anything, run:

```sh
cargo run --manifest-path src/rust/Cargo.toml \
  -p bitbox-test-vectors --bin generate-btc-test-vectors -- --check
```

Passing one output path instead writes the generated JSON there. This is useful for inspecting a
candidate artifact without changing the canonical file:

```sh
cargo run --manifest-path src/rust/Cargo.toml \
  -p bitbox-test-vectors --bin generate-btc-test-vectors -- /tmp/btc-vectors.json
```

The canonical artifact is `testdata/btc-transaction-test-vectors.json`. Copy it byte-for-byte to:

- `bitbox02-api-go/api/firmware/testdata/btc-transaction-test-vectors.json`
- `bitbox-api-rs/tests/data/btc-transaction-test-vectors.json`

Do not regenerate or edit either client copy independently.
