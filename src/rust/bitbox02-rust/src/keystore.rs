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

#[cfg(feature = "ed25519")]
pub mod ed25519;

use alloc::vec::Vec;

use crate::bip32;
use bitbox02::keystore;

/// Derives an xpub from the keystore seed at the given keypath.
pub fn get_xpub(keypath: &[u32]) -> Result<bip32::Xpub, ()> {
    // Convert from C keystore to Rust by encoding the xpub in C and decoding it in Rust.
    bip32::Xpub::from_bytes(&keystore::encode_xpub_at_keypath(keypath)?)
}

/// Returns fingerprint of the root public key at m/, which are the first four bytes of its hash160
/// according to:
/// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
pub fn root_fingerprint() -> Result<Vec<u8>, ()> {
    Ok(get_xpub(&[])?.pubkey_hash160().get(..4).ok_or(())?.to_vec())
}

/// Return the tweaked taproot pubkey at the given keypath.
///
/// Instead of returning the original pubkey at the keypath directly, it is tweaked with the hash of
/// the pubkey.
///
/// See
/// https://github.com/bitcoin/bips/blob/edffe529056f6dfd33d8f716fb871467c3c09263/bip-0086.mediawiki#address-derivation
pub fn secp256k1_schnorr_bip86_pubkey(keypath: &[u32]) -> Result<[u8; 32], ()> {
    let xpub = get_xpub(keypath)?;
    keystore::secp256k1_schnorr_bip86_pubkey(xpub.public_key())
}

#[cfg(test)]
mod tests {
    use super::*;

    use bitbox02::testing::{mock_unlocked, mock_unlocked_using_mnemonic};
    use util::bip32::HARDENED;

    #[test]
    fn test_get_xpub() {
        let keypath = &[44 + HARDENED, 0 + HARDENED, 0 + HARDENED];

        keystore::lock();
        assert!(get_xpub(keypath).is_err());

        // 24 words
        mock_unlocked_using_mnemonic("sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man");
        assert_eq!(
            get_xpub(&[]).unwrap().serialize_str(bip32::XPubType::Xpub).unwrap(),
            "xpub661MyMwAqRbcEhX8d9WJh78SZrxusAzWFoykz4n5CF75uYRzixw5FZPUSoWyhaaJ1bpiPFdzdHSQqJN38PcTkyrLmxT4J2JDYfoGJQ4ioE2",
        );
        assert_eq!(
            get_xpub(keypath).unwrap().serialize_str(bip32::XPubType::Xpub).unwrap(),
            "xpub6Cj6NNCGj2CRPHvkuEG1rbW3nrNCAnLjaoTg1P67FCGoahSsbg9WQ7YaMEEP83QDxt2kZ3hTPAPpGdyEZcfAC1C75HfR66UbjpAb39f4PnG",
        );

        // 18 words
        mock_unlocked_using_mnemonic("sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before subject");
        assert_eq!(
            get_xpub(keypath).unwrap().serialize_str(bip32::XPubType::Xpub).unwrap(),
            "xpub6C7fKxGtTzEVxCC22U2VHx4GpaVy77DzU6KdZ1CLuHgoUGviBMWDc62uoQVxqcRa5RQbMPnffjpwxve18BG81VJhJDXnSpRe5NGKwVpXiAb",
        );

        // 12 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass sniff",
        );
        assert_eq!(
            get_xpub(keypath).unwrap().serialize_str(bip32::XPubType::Xpub).unwrap(),
            "xpub6DLvpzjKpJ8k4xYrWYPmZQkUe9dkG1eRig2v6Jz4iYgo8hcpHWx87gGoCGDaB2cHFZ3ExUfe1jDiMu7Ch6gA4ULCBhvwZj29mHCPYSux3YV",
        )
    }

    #[test]
    fn test_root_fingerprint() {
        keystore::lock();
        assert_eq!(root_fingerprint(), Err(()));

        mock_unlocked_using_mnemonic(
            "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay"
        );
        assert_eq!(root_fingerprint(), Ok(vec![0x02, 0x40, 0xe9, 0x2a]));

        mock_unlocked_using_mnemonic(
            "small agent wife animal marine cloth exit thank stool idea steel frame",
        );
        assert_eq!(root_fingerprint(), Ok(vec![0xf4, 0x0b, 0x46, 0x9a]));
    }

    #[test]
    fn test_secp2e56k1_schnorr_bip86_pubkey() {
        // Test vectors from:
        // https://github.com/bitcoin/bips/blob/edffe529056f6dfd33d8f716fb871467c3c09263/bip-0086.mediawiki#test-vectors
        // Here we only test the creation of the tweaked pubkkey. See `Payload::from_simple` for address generation.

        keystore::lock();
        assert_eq!(
            secp256k1_schnorr_bip86_pubkey(&[86 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0]),
            Err(())
        );

        mock_unlocked_using_mnemonic("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about");
        assert_eq!(secp256k1_schnorr_bip86_pubkey(&[
            86 + HARDENED,
            0 + HARDENED,
            0 + HARDENED,
            0,
            0
        ])
        .unwrap(),
        *b"\xa6\x08\x69\xf0\xdb\xcf\x1d\xc6\x59\xc9\xce\xcb\xaf\x80\x50\x13\x5e\xa9\xe8\xcd\xc4\x87\x05\x3f\x1d\xc6\x88\x09\x49\xdc\x68\x4c",
        );

        assert_eq!(secp256k1_schnorr_bip86_pubkey(&[
            86 + HARDENED,
            0 + HARDENED,
            0 + HARDENED,
            0,
            1
        ])
        .unwrap(),
        *b"\xa8\x2f\x29\x94\x4d\x65\xb8\x6a\xe6\xb5\xe5\xcc\x75\xe2\x94\xea\xd6\xc5\x93\x91\xa1\xed\xc5\xe0\x16\xe3\x49\x8c\x67\xfc\x7b\xbb",
        );

        assert_eq!(secp256k1_schnorr_bip86_pubkey(&[
            86 + HARDENED,
            0 + HARDENED,
            0 + HARDENED,
            1,
            0,
        ])
        .unwrap(),
        *b"\x88\x2d\x74\xe5\xd0\x57\x2d\x5a\x81\x6c\xef\x00\x41\xa9\x6b\x6c\x1d\xe8\x32\xf6\xf9\x67\x6d\x96\x05\xc4\x4d\x5e\x9a\x97\xd3\xdc",
        );
    }
}
