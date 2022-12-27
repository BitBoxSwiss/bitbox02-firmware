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

use super::pb;

use alloc::vec::Vec;

use crate::bip32;
use bitbox02::keystore;

/// Derives an xpub from the keystore seed at the given keypath.
pub fn get_xpub(keypath: &[u32]) -> Result<pb::XPub, ()> {
    // Convert from C keystore to Rust by encoding the xpub in C and decoding it in Rust.
    bip32::parse_xpub_bytes(&keystore::encode_xpub_at_keypath(keypath)?)
}

/// Return the hash160 of the secp256k1 public key at the keypath.
pub fn secp256k1_pubkey_hash160(keypath: &[u32]) -> Result<Vec<u8>, ()> {
    let xpub = get_xpub(keypath)?;
    Ok(bitbox02::hash160(&xpub.public_key).to_vec())
}

pub fn secp256k1_pubkey_uncompressed(
    keypath: &[u32],
) -> Result<[u8; keystore::EC_PUBLIC_KEY_UNCOMPRESSED_LEN], ()> {
    let xpub = get_xpub(keypath)?;
    keystore::secp256k1_pubkey_compressed_to_uncompressed(&xpub.public_key)
}

/// Returns fingerprint of the root public key at m/, which are the first four bytes of its hash160
/// according to:
/// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
pub fn root_fingerprint() -> Result<Vec<u8>, ()> {
    Ok(secp256k1_pubkey_hash160(&[])?.get(..4).ok_or(())?.to_vec())
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
            bip32::serialize_xpub_str(
                &get_xpub(&[]).unwrap(),
                bip32::XPubType::Xpub
            )
            .unwrap(),
            "xpub661MyMwAqRbcEhX8d9WJh78SZrxusAzWFoykz4n5CF75uYRzixw5FZPUSoWyhaaJ1bpiPFdzdHSQqJN38PcTkyrLmxT4J2JDYfoGJQ4ioE2",
        );
        assert_eq!(
            bip32::serialize_xpub_str(
                &get_xpub(keypath).unwrap(),
                bip32::XPubType::Xpub
            )
            .unwrap(),
            "xpub6Cj6NNCGj2CRPHvkuEG1rbW3nrNCAnLjaoTg1P67FCGoahSsbg9WQ7YaMEEP83QDxt2kZ3hTPAPpGdyEZcfAC1C75HfR66UbjpAb39f4PnG",
        );

        // 18 words
        mock_unlocked_using_mnemonic("sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before subject");
        assert_eq!(
            bip32::serialize_xpub_str(
                &get_xpub(keypath).unwrap(),
                bip32::XPubType::Xpub
            )
            .unwrap(),
            "xpub6C7fKxGtTzEVxCC22U2VHx4GpaVy77DzU6KdZ1CLuHgoUGviBMWDc62uoQVxqcRa5RQbMPnffjpwxve18BG81VJhJDXnSpRe5NGKwVpXiAb",
        );

        // 12 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass sniff",
        );
        assert_eq!(
            bip32::serialize_xpub_str(
                &get_xpub(keypath).unwrap(),
                bip32::XPubType::Xpub
            )
            .unwrap(),
            "xpub6DLvpzjKpJ8k4xYrWYPmZQkUe9dkG1eRig2v6Jz4iYgo8hcpHWx87gGoCGDaB2cHFZ3ExUfe1jDiMu7Ch6gA4ULCBhvwZj29mHCPYSux3YV",
        )
    }

    #[test]
    fn test_secp256k1_pubkey_hash160() {
        let keypath = &[84 + HARDENED, HARDENED, HARDENED, 0, 0];

        keystore::lock();
        assert!(secp256k1_pubkey_hash160(keypath).is_err());

        mock_unlocked();
        assert_eq!(
            secp256k1_pubkey_hash160(keypath).unwrap(),
            *b"\xb5\x12\x5c\xec\xa0\xc1\xc8\x90\xda\x07\x9a\x12\x88\xdc\xf7\x7a\xa6\xac\xc4\x99"
        );

        mock_unlocked_using_mnemonic("sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man");
        assert_eq!(
            secp256k1_pubkey_hash160(&[44 + HARDENED, 0 + HARDENED, 0 + HARDENED, 1, 2]).unwrap(),
            *b"\xe5\xf8\x9a\xb6\x54\x37\x44\xf7\x8f\x15\x86\x7c\x43\x06\xee\x86\x6b\xb1\x1d\xf9"
        );
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
    fn test_secp256k1_pubkey_uncompressed() {
        let keypath = &[44 + HARDENED, 0 + HARDENED, 0 + HARDENED, 1, 2];

        keystore::lock();
        assert_eq!(secp256k1_pubkey_uncompressed(keypath), Err(()));

        mock_unlocked_using_mnemonic("sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man");
        assert_eq!(
            secp256k1_pubkey_uncompressed(keypath).unwrap(),
            *b"\x04\x77\xa4\x4a\xa9\xe8\xc8\xfb\x51\x05\xef\x5e\xe2\x39\x4e\x8a\xed\x89\xad\x73\xfc\x74\x36\x14\x25\xf0\x63\x47\xec\xfe\x32\x61\x31\xe1\x33\x93\x67\xee\x3c\xbe\x87\x71\x92\x85\xa0\x7f\x77\x4b\x17\xeb\x93\x3e\xcf\x0b\x9b\x82\xac\xeb\xc1\x95\x22\x6d\x63\x42\x44",
        );
    }
}
