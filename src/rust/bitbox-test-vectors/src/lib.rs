// SPDX-License-Identifier: Apache-2.0

//! Readable sources and deterministic generators for portable BitBox test vectors.

// The firmware workspace links rust-bitcoin against its unprefixed secp256k1-zkp build.
use bitbox_secp256k1 as _;

pub mod btc_transaction;
