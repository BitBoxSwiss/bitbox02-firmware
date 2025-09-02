// File copied and adapted from:
// https://github.com/cygnet3/rust-silentpayments/blob/395b153b6d98ea33a59306c1a8a189d4ca152571/src/utils/hash.rs

#![allow(non_snake_case)]

use bitcoin::hashes::{Hash, HashEngine, sha256t_hash_newtype};
use bitcoin::secp256k1::{PublicKey, Scalar};

sha256t_hash_newtype! {
    struct InputsTag = hash_str("BIP0352/Inputs");

    /// BIP0352-tagged hash with tag \"Inputs\".
    ///
    /// This is used for computing the inputs hash.
    #[hash_newtype(forward)]
    struct InputsHash(_);

    pub(crate) struct SharedSecretTag = hash_str("BIP0352/SharedSecret");

    /// BIP0352-tagged hash with tag \"SharedSecret\".
    ///
    /// This hash type is for computing the shared secret.
    #[hash_newtype(forward)]
    pub(crate) struct SharedSecretHash(_);
}

impl InputsHash {
    pub(crate) fn from_outpoint_and_A_sum(
        smallest_outpoint: &bitcoin::OutPoint,
        A_sum: PublicKey,
    ) -> InputsHash {
        let mut eng = InputsHash::engine();
        eng.input(&bitcoin::consensus::serialize(smallest_outpoint));
        eng.input(&A_sum.serialize());
        InputsHash::from_engine(eng)
    }
    pub(crate) fn to_scalar(self) -> Scalar {
        // This is statistically extremely unlikely to panic.
        Scalar::from_be_bytes(self.to_byte_array()).unwrap()
    }
}

impl SharedSecretHash {
    pub(crate) fn from_ecdh_and_k(ecdh: &PublicKey, k: u32) -> SharedSecretHash {
        let mut eng = SharedSecretHash::engine();
        eng.input(&ecdh.serialize());
        eng.input(&k.to_be_bytes());
        SharedSecretHash::from_engine(eng)
    }
}

pub(crate) fn calculate_input_hash(outpoint: &bitcoin::OutPoint, A_sum: PublicKey) -> Scalar {
    InputsHash::from_outpoint_and_A_sum(outpoint, A_sum).to_scalar()
}
