// Copyright 2024 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_std]

extern crate alloc;

mod hash;

pub use bitcoin;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1::{self, PublicKey, Scalar, Secp256k1, SecretKey, XOnlyPublicKey};

use alloc::vec::Vec;

const DLEQ_PROOF_SIZE: usize = 33 + 64;
const PUBKEY_LEN: usize = 33;

/// Generated silent payment Taproot output.
pub struct TransactionOutput {
    /// Generated silent payment output public key.
    pub pubkey: XOnlyPublicKey,
    /// DLEQ proof that the host can use to verify that the pubkey has been created correctly.
    pub dleq_proof: [u8; DLEQ_PROOF_SIZE],
}

pub enum Network {
    Btc,
    Tbtc,
}

impl Network {
    /// Human-readable part of a silent payment bech32 address.
    fn sp_hrp(&self) -> &str {
        match self {
            Network::Btc => "sp",
            Network::Tbtc => "tsp",
        }
    }
}

pub struct SilentPayment {
    secp: Secp256k1<secp256k1::All>,
    network: Network,
    smallest_outpoint: Option<bitcoin::OutPoint>,
    a_sum: Option<SecretKey>,
    // Done streaming inputs?
    inputs_done: bool,
    // We only allow one silent payment output for now. This tracks whether we've seen it.
    output_created: bool,
}

fn calculate_t_k(ecdh_shared_secret: &PublicKey, k: u32) -> Result<SecretKey, ()> {
    let hash = hash::SharedSecretHash::from_ecdh_and_k(ecdh_shared_secret, k).to_byte_array();
    SecretKey::from_slice(&hash).map_err(|_| ())
}

struct SilentPaymentAddress {
    scan_pubkey: PublicKey,
    spend_pubkey: PublicKey,
}

fn decode_address(address: &str, expected_hrp: &str) -> Result<SilentPaymentAddress, ()> {
    let mut decoded_addr =
        bech32::primitives::decode::CheckedHrpstring::new::<bech32::Bech32m>(address)
            .map_err(|_| ())?;

    let hrp = decoded_addr.hrp();
    if hrp.as_str() != expected_hrp {
        return Err(());
    }
    let witness_version = decoded_addr.remove_witness_version().unwrap();
    if witness_version != bech32::Fe32::Q {
        return Err(());
    }

    let data: Vec<u8> = decoded_addr.byte_iter().collect();
    if data.len() != 2 * PUBKEY_LEN {
        return Err(());
    }
    Ok(SilentPaymentAddress {
        scan_pubkey: PublicKey::from_slice(&data[..PUBKEY_LEN]).map_err(|_| ())?,
        spend_pubkey: PublicKey::from_slice(&data[PUBKEY_LEN..]).map_err(|_| ())?,
    })
}

pub enum InputType {
    P2pkh,
    P2wpkhP2sh,
    P2wpkh,
    P2trKeypathSpend,
}

impl InputType {
    fn is_taproot(&self) -> bool {
        matches!(self, InputType::P2trKeypathSpend)
    }
}

// Create a proof that the silent payment output was created correctly, which the host wallet can
// verify.
//
// Based on https://gist.github.com/andrewtoth/df97c3260cc8d12f09d3855ee61322ea.
//
// a_sum is the sum of the input private keys.
// a_sum_pubkey is the corresponding pubkey.
// scan_pubkey is the output's silent payment scan pubkey (extracted from the silent payment address).
//
// We create c_pubkey = a_sum*scan_pubkey and a DLEQ (discrete log equivalence) proof that
// a_sum_pubkey has the same discrete log with respect to the secp256k1 base G as c_pubkey to the
// base scan_pubkey. The result returned to the host is
// `<33 byte compressed c_pubkey><64 byte dleq proof>`.
//
// The host can then check that the generated output is correct by:
// - independently computing a_sum_pubkey by summing the pubkeys of the inputs (see
//   https://github.com/bitcoin/bips/blob/ad1d3bc2a7b0d84247c29f847e85c35283094e2f/bip-0352.mediawiki#user-content-Inputs_For_Shared_Secret_Derivation)
//
// - verifying using the dleq proof that the discrete log of a_sum_pubkey to G is the same as the
//   discrete log of c_pubkey to the base scan_pubkey.
// - re-computing the output (https://github.com/bitcoin/bips/blob/ad1d3bc2a7b0d84247c29f847e85c35283094e2f/bip-0352.mediawiki#creating-outputs)
//   using ecdsa_shared_secret = input_hash·a_sum·scan_pubkey = input_hash·c_pubkey
// - verifying that the created output matches the re-computed output.
fn create_dleq_proof(
    secp: &Secp256k1<secp256k1::All>,
    a_sum: &SecretKey,
    a_sum_pubkey: &PublicKey,
    scan_pubkey: &PublicKey,
) -> Result<[u8; DLEQ_PROOF_SIZE], ()> {
    let c_pubkey = scan_pubkey
        .mul_tweak(secp, &Scalar::from(*a_sum))
        .map_err(|_| ())?;

    let proof =
        bitbox02::secp256k1::dleq_prove(a_sum.as_ref(), scan_pubkey, a_sum_pubkey, &c_pubkey)?;
    // Sanity check.
    bitbox02::secp256k1::dleq_verify(
        proof.as_slice().try_into().unwrap(),
        scan_pubkey,
        a_sum_pubkey,
        &c_pubkey,
    )?;

    let mut result = c_pubkey.serialize().to_vec();
    result.extend(&proof);
    Ok(result.try_into().unwrap())
}

impl SilentPayment {
    pub fn new(network: Network) -> Self {
        SilentPayment {
            secp: Secp256k1::new(),
            network,
            smallest_outpoint: None,
            a_sum: None,
            inputs_done: false,
            output_created: false,
        }
    }

    pub fn get_secp(&self) -> &Secp256k1<secp256k1::All> {
        &self.secp
    }

    /// This must be called for *every* input of the transaction.
    ///
    /// Important: if the input type cannot be represented by `InputType`, the transaction must be
    /// aborted, as other input types may influence the silent payment outputs (e.g. P2TR script
    /// path spends, which we currently do not support).
    pub fn add_input(
        &mut self,
        input_type: InputType,
        input_key: &SecretKey,
        prevout: bitcoin::OutPoint,
    ) -> Result<(), ()> {
        if self.inputs_done {
            return Err(());
        }
        match self.smallest_outpoint {
            None => self.smallest_outpoint = Some(prevout),
            Some(ref mut p) => {
                if bitcoin::consensus::serialize(&prevout) < bitcoin::consensus::serialize(p) {
                    *p = prevout
                }
            }
        }

        let (_, parity) = input_key.x_only_public_key(&self.secp);
        let negated_key: SecretKey = if input_type.is_taproot() && parity == secp256k1::Parity::Odd
        {
            input_key.negate()
        } else {
            *input_key
        };

        match self.a_sum {
            None => self.a_sum = Some(negated_key),
            Some(ref mut p) => {
                *p = p.add_tweak(&Scalar::from(negated_key)).map_err(|_| ())?;
            }
        }

        Ok(())
    }

    /// Call this for silent payment outputs.
    /// `silent_payment_address` is the output address.
    /// This returns the SegWit v1 Taproot output key of the created output.
    /// See: https://github.com/bitcoin/bips/blob/ad1d3bc2a7b0d84247c29f847e85c35283094e2f/bip-0352.mediawiki#user-content-Creating_outputs
    pub fn create_output(&mut self, silent_payment_address: &str) -> Result<TransactionOutput, ()> {
        self.inputs_done = true;
        if self.output_created {
            return Err(());
        }
        self.output_created = true;

        let SilentPaymentAddress {
            scan_pubkey,
            spend_pubkey,
        } = decode_address(silent_payment_address, self.network.sp_hrp())?;

        let a_sum = self.a_sum.as_ref().unwrap();
        let a_sum_pubkey = a_sum.public_key(&self.secp);

        let inputs_hash =
            hash::calculate_input_hash(self.smallest_outpoint.as_ref().ok_or(())?, a_sum_pubkey);

        let partial_secret = a_sum.mul_tweak(&inputs_hash).map_err(|_| ())?;

        let ecdh_shared_secret: PublicKey = scan_pubkey
            .mul_tweak(&self.secp, &partial_secret.into())
            .map_err(|_| ())?;

        // If we want to support more than one silent payment output, we need to get this value from
        // the host per output, and check before signing the tx that for each SP output with the
        // same scan pubkey has a different `k` and they are consecutive starting at 0, so the
        // recipient is sure to be able to find the output.  With only one silent payment output
        // supported, `k` must be 0.
        let silent_payment_k = 0;

        let t_k = calculate_t_k(&ecdh_shared_secret, silent_payment_k).map_err(|_| ())?;

        let res = t_k.public_key(&self.secp);
        let reskey = res.combine(&spend_pubkey).map_err(|_| ())?;
        let (reskey_xonly, _) = reskey.x_only_public_key();

        Ok(TransactionOutput {
            pubkey: reskey_xonly,
            dleq_proof: create_dleq_proof(&self.secp, a_sum, &a_sum_pubkey, &scan_pubkey)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use core::str::FromStr;

    #[test]
    fn test_basic() {
        let mut v = SilentPayment::new(Network::Btc);
        v.add_input(
            InputType::P2wpkh,
            &SecretKey::from_str(
                "eadc78165ff1f8ea94ad7cfdc54990738a4c53f6e0507b42154201b8e5dff3b1",
            )
            .unwrap(),
            bitcoin::OutPoint::new(
                bitcoin::Txid::from_str(
                    "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16",
                )
                .unwrap(),
                0,
            ),
        )
        .unwrap();

        v.add_input(
            InputType::P2wpkh,
            &SecretKey::from_str(
                "93f5ed907ad5b2bdbbdcb5d9116ebc0a4e1f92f910d5260237fa45a9408aad16",
            )
            .unwrap(),
            bitcoin::OutPoint::new(
                bitcoin::Txid::from_str(
                    "a1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d",
                )
                .unwrap(),
                0,
            ),
        )
        .unwrap();

        assert_eq!(
            v.smallest_outpoint.unwrap().to_string().as_str(),
            "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16:0"
        );

        let expected_pubkey = XOnlyPublicKey::from_str(
            "3e9fce73d4e77a4809908e3c3a2e54ee147b9312dc5044a193d1fc85de46e3c1",
        )
        .unwrap();
        let expected_dleq_proof = "02bd6cf6542e272a81a6aba9d35c0140d73758ad74c992d8808c0f0d76a642fe9977ecc511315c7fa44e54af3676ee212ca21031ef4a763dc841a49b59431ef8e4e2ac48d74324d5115602e2720c365c836da738f8c43c513f0022a40d6e71a048";
        let output = v.create_output("sp1qqgste7k9hx0qftg6qmwlkqtwuy6cycyavzmzj85c6qdfhjdpdjtdgqjuexzk6murw56suy3e0rd2cgqvycxttddwsvgxe2usfpxumr70xc9pkqwv").unwrap();
        assert_eq!(output.pubkey, expected_pubkey);
        assert_eq!(hex::encode(output.dleq_proof), expected_dleq_proof);
    }

    #[test]
    fn test_only_one_output() {
        let mut v = SilentPayment::new(Network::Btc);
        v.add_input(
            InputType::P2wpkh,
            &SecretKey::from_str(
                "eadc78165ff1f8ea94ad7cfdc54990738a4c53f6e0507b42154201b8e5dff3b1",
            )
            .unwrap(),
            bitcoin::OutPoint::new(
                bitcoin::Txid::from_str(
                    "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16",
                )
                .unwrap(),
                0,
            ),
        )
        .unwrap();

        v.add_input(
            InputType::P2wpkh,
            &SecretKey::from_str(
                "93f5ed907ad5b2bdbbdcb5d9116ebc0a4e1f92f910d5260237fa45a9408aad16",
            )
            .unwrap(),
            bitcoin::OutPoint::new(
                bitcoin::Txid::from_str(
                    "a1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d",
                )
                .unwrap(),
                0,
            ),
        )
        .unwrap();
        let _ = v.create_output("sp1qqgste7k9hx0qftg6qmwlkqtwuy6cycyavzmzj85c6qdfhjdpdjtdgqjuexzk6murw56suy3e0rd2cgqvycxttddwsvgxe2usfpxumr70xc9pkqwv").unwrap();
        assert!(v.create_output("sp1qqgste7k9hx0qftg6qmwlkqtwuy6cycyavzmzj85c6qdfhjdpdjtdgqjuexzk6murw56suy3e0rd2cgqvycxttddwsvgxe2usfpxumr70xc9pkqwv").is_err());
    }

    #[test]
    fn test_no_input_after_output() {
        let mut v = SilentPayment::new(Network::Btc);
        v.add_input(
            InputType::P2wpkh,
            &SecretKey::from_str(
                "eadc78165ff1f8ea94ad7cfdc54990738a4c53f6e0507b42154201b8e5dff3b1",
            )
            .unwrap(),
            bitcoin::OutPoint::new(
                bitcoin::Txid::from_str(
                    "f4184fc596403b9d638783cf57adfe4c75c605f6356fbc91338530e9831e9e16",
                )
                .unwrap(),
                0,
            ),
        )
        .unwrap();

        v.add_input(
            InputType::P2wpkh,
            &SecretKey::from_str(
                "93f5ed907ad5b2bdbbdcb5d9116ebc0a4e1f92f910d5260237fa45a9408aad16",
            )
            .unwrap(),
            bitcoin::OutPoint::new(
                bitcoin::Txid::from_str(
                    "a1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d",
                )
                .unwrap(),
                0,
            ),
        )
        .unwrap();
        let _ = v.create_output("sp1qqgste7k9hx0qftg6qmwlkqtwuy6cycyavzmzj85c6qdfhjdpdjtdgqjuexzk6murw56suy3e0rd2cgqvycxttddwsvgxe2usfpxumr70xc9pkqwv").unwrap();

        assert!(v
            .add_input(
                InputType::P2wpkh,
                &SecretKey::from_str(
                    "93f5ed907ad5b2bdbbdcb5d9116ebc0a4e1f92f910d5260237fa45a9408aad16",
                )
                .unwrap(),
                bitcoin::OutPoint::new(
                    bitcoin::Txid::from_str(
                        "a1075db55d416d3ca199f55b6084e2115b9345e16c5cf302fc80e9d5fbf5d48d",
                    )
                    .unwrap(),
                    0,
                ),
            )
            .is_err());
    }
}
