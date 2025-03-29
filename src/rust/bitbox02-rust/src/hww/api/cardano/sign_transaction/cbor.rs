// Copyright 2021 Shift Crypto AG
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

use super::super::params;
use super::super::pb;
use super::super::Error;

use alloc::vec::Vec;

use digest::Update;
use minicbor::encode::{Encoder, Write};

use pb::cardano_sign_transaction_request::{certificate, Certificate, Withdrawal};

use super::super::address::{decode_payment_address, pubkey_hash_at_keypath, ADDRESS_HASH_SIZE};

/// A newtype for hashers to implement the Write trait, enabling serializing cbor directly into the
/// hasher.
pub struct HashedWriter<'a, U: Update>(&'a mut U);

impl<'a, U: Update> HashedWriter<'a, U> {
    pub fn new(hasher: &'a mut U) -> Self {
        HashedWriter(hasher)
    }
}

impl<U: Update> Write for HashedWriter<'_, U> {
    type Error = ();
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.0.update(buf);
        Ok(())
    }
}

/// See https://github.com/input-output-hk/cardano-ledger-specs/blob/d0aa86ded0b973b09b629e5aa62aa1e71364d088/eras/alonzo/test-suite/cddl-files/alonzo.cddl#L176
fn encode_stake_credential<W: Write>(
    encoder: &mut Encoder<W>,
    keypath: &[u32],
) -> Result<(), Error> {
    let pubkey_hash = pubkey_hash_at_keypath(keypath)?;
    encoder.array(2)?.u8(0)?.bytes(&pubkey_hash)?;
    Ok(())
}

/// Encode a withdrawal/reward address.
///
/// See https://github.com/input-output-hk/cardano-ledger-specs/blob/d0aa86ded0b973b09b629e5aa62aa1e71364d088/eras/alonzo/test-suite/cddl-files/alonzo.cddl#L130
pub fn encode_withdrawal_address(
    params: &params::Params,
    keypath: &[u32],
) -> Result<Vec<u8>, Error> {
    let pubkey_hash = pubkey_hash_at_keypath(keypath)?;
    let mut encoded: Vec<u8> = Vec::with_capacity(1 + ADDRESS_HASH_SIZE);
    let address_tag = 0b1110; // reward address using a stake keyhash.
    let header = (address_tag << 4) | params.network_id;
    encoded.push(header);
    encoded.extend_from_slice(&pubkey_hash);
    Ok(encoded)
}

/// Start encoding a set:
/// https://github.com/IntersectMBO/cardano-ledger/blob/6e2d37cc0f47bd02e89b4ce9f78b59c35c958e96/eras/conway/impl/cddl-files/extra.cddl#L5
/// tag258 indicates whether to use the tagged version or the untagged version.
fn encode_set_header<W: Write>(
    encoder: &mut Encoder<W>,
    len: u64,
    tag258: bool,
) -> Result<(), Error> {
    if tag258 {
        encoder.tag(minicbor::data::Tag::new(258))?;
    }
    encoder.array(len)?;
    Ok(())
}

/// CBOR encoding for Cardano transactions.
///
/// The transaction must be verified/validated before calling this function.
///
/// References:
/// - Transaction body encoding spec: https://github.com/input-output-hk/cardano-ledger-specs/blob/d0aa86ded0b973b09b629e5aa62aa1e71364d088/eras/alonzo/test-suite/cddl-files/alonzo.cddl#L50
/// - Serialization implementation: https://github.com/input-output-hk/cardano-ledger-specs/blob/c6c4be1562e23a3dd48282387c4e48ff918fbab0/eras/shelley-ma/impl/src/Cardano/Ledger/ShelleyMA/TxBody.hs#L208
pub fn encode_transaction_body<W: Write>(
    tx: &pb::CardanoSignTransactionRequest,
    writer: W,
) -> Result<(), Error> {
    let params = params::get(pb::CardanoNetwork::try_from(tx.network)?);
    let mut encoder = Encoder::new(writer);

    let mut num_map_entries = 3; // inputs, outputs, fee
    if tx.ttl != 0 || tx.allow_zero_ttl {
        num_map_entries += 1;
    }
    if !tx.certificates.is_empty() {
        num_map_entries += 1;
    }
    if !tx.withdrawals.is_empty() {
        num_map_entries += 1;
    }
    if tx.validity_interval_start != 0 {
        num_map_entries += 1;
    }

    encoder.map(num_map_entries)?;
    // Map entry 0 is an array of inputs.
    encoder.u8(0)?;
    encode_set_header(&mut encoder, tx.inputs.len() as _, tx.tag_cbor_sets)?;
    for input in tx.inputs.iter() {
        if input.prev_out_hash.len() != 32 {
            return Err(Error::InvalidInput);
        }
        encoder
            .array(2)?
            .bytes(&input.prev_out_hash)?
            .u32(input.prev_out_index)?;
    }
    // Map entry 1 is an array of outputs.
    encoder.u8(1)?.array(tx.outputs.len() as _)?;
    for output in tx.outputs.iter() {
        let decoded_address = decode_payment_address(params, &output.encoded_address)?;
        encoder.array(2)?.bytes(&decoded_address)?;
        // Second array entry is either the ADA amount, or [ADA amount, assets].
        //
        // See
        // https://github.com/input-output-hk/cardano-ledger/blob/bd9bdb17e493ec1b3c8f329b25a5907d8b3d1cd1/eras/alonzo/test-suite/cddl-files/alonzo.cddl#L362
        if output.asset_groups.is_empty() {
            encoder.u64(output.value)?;
        } else {
            encoder
                .array(2)?
                .u64(output.value)?
                .map(output.asset_groups.len() as _)?;
            for asset_group in output.asset_groups.iter() {
                encoder
                    .bytes(&asset_group.policy_id)?
                    .map(asset_group.tokens.len() as _)?;
                for token in asset_group.tokens.iter() {
                    encoder.bytes(&token.asset_name)?.u64(token.value)?;
                }
            }
        }
    }
    // Map entry 2 is the fee.
    encoder.u8(2)?.u64(tx.fee)?;
    // Optional map entry 3 is ttl.
    if tx.ttl != 0 || tx.allow_zero_ttl {
        encoder.u8(3)?.u64(tx.ttl)?;
    }
    // Optional map entry 4 are the certificates:
    if !tx.certificates.is_empty() {
        encoder.u8(4)?;
        encode_set_header(&mut encoder, tx.certificates.len() as _, tx.tag_cbor_sets)?;
        for Certificate { cert } in tx.certificates.iter() {
            match cert.as_ref().ok_or(Error::InvalidInput)? {
                certificate::Cert::StakeRegistration(pb::Keypath { keypath }) => {
                    encoder.array(2)?.u8(0)?;
                    encode_stake_credential(&mut encoder, keypath)?;
                }
                certificate::Cert::StakeDeregistration(pb::Keypath { keypath }) => {
                    encoder.array(2)?.u8(1)?;
                    encode_stake_credential(&mut encoder, keypath)?;
                }
                certificate::Cert::StakeDelegation(certificate::StakeDelegation {
                    keypath,
                    pool_keyhash,
                }) => {
                    if pool_keyhash.len() != 28 {
                        return Err(Error::InvalidInput);
                    }
                    encoder.array(3)?.u8(2)?;
                    encode_stake_credential(&mut encoder, keypath)?;
                    encoder.bytes(pool_keyhash)?;
                }
                certificate::Cert::VoteDelegation(certificate::VoteDelegation {
                    keypath,
                    r#type,
                    drep_credhash,
                }) => {
                    encoder.array(3)?.u8(9)?;
                    encode_stake_credential(&mut encoder, keypath)?;
                    let drep_type =
                        certificate::vote_delegation::CardanoDRepType::try_from(*r#type)?;
                    match drep_type {
                        certificate::vote_delegation::CardanoDRepType::KeyHash => {
                            encoder.array(2)?.u8(0)?;
                            match drep_credhash {
                                Some(hash) if hash.len() == 28 => {
                                    encoder.bytes(hash)?;
                                }
                                Some(_hash) => return Err(Error::InvalidInput),
                                None => return Err(Error::InvalidInput),
                            }
                        }
                        certificate::vote_delegation::CardanoDRepType::ScriptHash => {
                            encoder.array(2)?.u8(1)?;
                            match drep_credhash {
                                Some(hash) if hash.len() == 28 => {
                                    encoder.bytes(hash)?;
                                }
                                Some(_hash) => return Err(Error::InvalidInput),
                                None => return Err(Error::InvalidInput),
                            }
                        }
                        certificate::vote_delegation::CardanoDRepType::AlwaysAbstain => {
                            if !drep_credhash.is_none() {
                                return Err(Error::InvalidInput);
                            }
                            encoder.array(1)?.u8(2)?;
                        }
                        certificate::vote_delegation::CardanoDRepType::AlwaysNoConfidence => {
                            if !drep_credhash.is_none() {
                                return Err(Error::InvalidInput);
                            }
                            encoder.array(1)?.u8(3)?;
                        }
                    }
                }
            }
        }
    }

    // Optional map entry 5 are the withdrawals:
    if !tx.withdrawals.is_empty() {
        encoder.u8(5)?.map(tx.withdrawals.len() as _)?;
        for Withdrawal { keypath, value } in tx.withdrawals.iter() {
            let withdrawal_address = encode_withdrawal_address(params, keypath)?;
            encoder.bytes(&withdrawal_address)?.u64(*value)?;
        }
    }
    // Optional map entry 8 is validity_interval_start.
    if tx.validity_interval_start != 0 {
        encoder.u8(8)?.u64(tx.validity_interval_start)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use blake2::{digest::VariableOutput, Blake2bVar};

    fn encode_something<W: Write>(
        encoder: &mut Encoder<W>,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        // From https://twittner.gitlab.io/minicbor/minicbor/#example-ad-hoc-encoding
        encoder
            .begin_map()? // using an indefinite map here
            .str("hello")?
            .str("world")?
            .str("submap")?
            .map(2)?
            .u8(1)?
            .bool(true)?
            .u8(2)?
            .bool(false)?
            .u16(34234)?
            .array(3)?
            .u8(1)?
            .u8(2)?
            .u8(3)?
            .bool(true)?
            .null()?
            .end()?;
        Ok(())
    }

    #[test]
    fn test_hashed_encoding() {
        // Compute expected hash by encoding to CBOR and then hashing.
        let expected_hash: [u8; 32] = {
            let mut cbor_encoded: Vec<u8> = Vec::new();
            let mut encoder = Encoder::new(&mut cbor_encoded);
            encode_something(&mut encoder).unwrap();

            let mut hasher = Blake2bVar::new(32).unwrap();
            hasher.update(&cbor_encoded);
            let mut out = [0u8; 32];
            hasher.finalize_variable(&mut out).unwrap();
            out
        };

        // Now encode CBOR into the hasher directly and compare results.
        let hash = {
            let mut hasher = Blake2bVar::new(32).unwrap();
            let mut encoder = Encoder::new(HashedWriter::new(&mut hasher));
            encode_something(&mut encoder).unwrap();
            let mut out = [0u8; 32];
            hasher.finalize_variable(&mut out).unwrap();
            out
        };
        assert_eq!(hash, expected_hash);
    }
}
