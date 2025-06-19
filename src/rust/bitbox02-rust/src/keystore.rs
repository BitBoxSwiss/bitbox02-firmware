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

use alloc::string::String;
use alloc::vec::Vec;

use crate::bip32;
use bitbox02::keystore;

use util::bip32::HARDENED;

use crate::hash::Sha512;
use hmac::{digest::FixedOutput, Mac, SimpleHmac};

/// Returns the keystore's seed encoded as a BIP-39 mnemonic.
pub fn get_bip39_mnemonic() -> Result<zeroize::Zeroizing<String>, ()> {
    keystore::bip39_mnemonic_from_seed(&keystore::copy_seed()?)
}

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

fn bip85_entropy(keypath: &[u32]) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let priv_key = keystore::secp256k1_get_private_key(keypath)?;
    let mut mac = SimpleHmac::<Sha512>::new_from_slice(b"bip-entropy-from-k").unwrap();
    mac.update(&priv_key);
    let mut out = zeroize::Zeroizing::new(vec![0u8; 64]);
    let fixed_out: &mut [u8; 64] = out.as_mut_slice().try_into().unwrap();
    mac.finalize_into(fixed_out.into());
    Ok(out)
}

/// Computes a BIP39 mnemonic according to BIP-85:
/// https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip39
/// `words` must be 12, 18 or 24.
/// `index` must be smaller than `bip32::HARDENED`.
pub fn bip85_bip39(words: u32, index: u32) -> Result<zeroize::Zeroizing<String>, ()> {
    if index >= HARDENED {
        return Err(());
    }

    let seed_size: usize = match words {
        12 => 16,
        18 => 24,
        24 => 32,
        _ => return Err(()),
    };

    let keypath = [
        83696968 + HARDENED,
        39 + HARDENED,
        0 + HARDENED,
        words + HARDENED,
        index + HARDENED,
    ];

    let entropy = bip85_entropy(&keypath)?;
    keystore::bip39_mnemonic_from_seed(&entropy[..seed_size])
}

/// Computes a 16 byte deterministic seed specifically for Lightning hot wallets according to BIP-85.
/// It is the same as BIP-85 with app number 39', but instead using app number 19534' (= 0x4c4e =
/// 'LN'). https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip39
/// Restricted to 16 byte output entropy.
/// `index` must be smaller than `bip32::HARDENED`.
pub fn bip85_ln(index: u32) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    if index >= HARDENED {
        return Err(());
    }
    let keypath = [
        83696968 + HARDENED,
        19534 + HARDENED,
        0 + HARDENED,
        12 + HARDENED,
        index + HARDENED,
    ];

    let mut entropy = bip85_entropy(&keypath)?;
    entropy.truncate(16);
    Ok(entropy)
}

#[cfg(test)]
mod tests {
    use super::*;

    use bitbox02::testing::{
        mock_memory, mock_unlocked, mock_unlocked_using_mnemonic, TEST_MNEMONIC,
    };

    #[test]
    fn test_get_bip39_mnemonic() {
        keystore::lock();
        assert!(get_bip39_mnemonic().is_err());

        mock_unlocked();

        assert_eq!(get_bip39_mnemonic().unwrap().as_str(), TEST_MNEMONIC);
    }

    #[test]
    fn test_get_xpub() {
        let keypath = &[44 + HARDENED, 0 + HARDENED, 0 + HARDENED];

        keystore::lock();
        assert!(get_xpub(keypath).is_err());

        // 24 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
            "",
        );
        assert_eq!(
            get_xpub(&[]).unwrap().serialize_str(bip32::XPubType::Xpub).unwrap(),
            "xpub661MyMwAqRbcEhX8d9WJh78SZrxusAzWFoykz4n5CF75uYRzixw5FZPUSoWyhaaJ1bpiPFdzdHSQqJN38PcTkyrLmxT4J2JDYfoGJQ4ioE2",
        );
        assert_eq!(
            get_xpub(keypath).unwrap().serialize_str(bip32::XPubType::Xpub).unwrap(),
            "xpub6Cj6NNCGj2CRPHvkuEG1rbW3nrNCAnLjaoTg1P67FCGoahSsbg9WQ7YaMEEP83QDxt2kZ3hTPAPpGdyEZcfAC1C75HfR66UbjpAb39f4PnG",
        );

        // 18 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before subject",
            "",
        );
        assert_eq!(
            get_xpub(keypath).unwrap().serialize_str(bip32::XPubType::Xpub).unwrap(),
            "xpub6C7fKxGtTzEVxCC22U2VHx4GpaVy77DzU6KdZ1CLuHgoUGviBMWDc62uoQVxqcRa5RQbMPnffjpwxve18BG81VJhJDXnSpRe5NGKwVpXiAb",
        );

        // 12 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass sniff",
            "",
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
            "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay",
            "",
        );
        assert_eq!(root_fingerprint(), Ok(vec![0x02, 0x40, 0xe9, 0x2a]));

        mock_unlocked_using_mnemonic(
            "small agent wife animal marine cloth exit thank stool idea steel frame",
            "",
        );
        assert_eq!(root_fingerprint(), Ok(vec![0xf4, 0x0b, 0x46, 0x9a]));
    }

    #[test]
    fn test_bip85_bip39() {
        keystore::lock();
        assert!(bip85_bip39(12, 0).is_err());

        // Test fixtures generated using:
        // `docker build -t bip85 .`
        // `podman run --rm bip85 --index 0 --bip39-mnemonic "virtual weapon code laptop defy cricket vicious target wave leopard garden give" bip39 --num-words 12`
        // `podman run --rm bip85 --index 1 --bip39-mnemonic "virtual weapon code laptop defy cricket vicious target wave leopard garden give" bip39 --num-words 12`
        // `podman run --rm bip85 --index 2147483647 --bip39-mnemonic "virtual weapon code laptop defy cricket vicious target wave leopard garden give" bip39 --num-words 12`
        // `podman run --rm bip85 --index 0 --bip39-mnemonic "virtual weapon code laptop defy cricket vicious target wave leopard garden give" bip39 --num-words 18`
        // `podman run --rm bip85 --index 0 --bip39-mnemonic "virtual weapon code laptop defy cricket vicious target wave leopard garden give" bip39 --num-words 24`
        // in  https://github.com/ethankosakovsky/bip85/tree/435a0589746c1036735d0a5081167e08abfa7413.

        mock_unlocked_using_mnemonic(
            "virtual weapon code laptop defy cricket vicious target wave leopard garden give",
            "",
        );

        assert_eq!(
            bip85_bip39(12, 0).unwrap().as_ref() as &str,
            "slender whip place siren tissue chaos ankle door only assume tent shallow",
        );
        assert_eq!(
            bip85_bip39(12, 1).unwrap().as_ref() as &str,
            "income soft level reunion height pony crane use unfold win keen satisfy",
        );
        assert_eq!(
            bip85_bip39(12, HARDENED - 1).unwrap().as_ref() as &str,
            "carry build nerve market domain energy mistake script puzzle replace mixture idea",
        );
        assert_eq!(
            bip85_bip39(18, 0).unwrap().as_ref() as &str,
            "enact peasant tragic habit expand jar senior melody coin acid logic upper soccer later earn napkin planet stereo",
        );
        assert_eq!(
            bip85_bip39(24, 0).unwrap().as_ref() as &str,
            "cabbage wink october add anchor mean tray surprise gasp tomorrow garbage habit beyond merge where arrive beef gentle animal office drop panel chest size",
        );

        // Invalid number of words.
        assert!(bip85_bip39(10, 0).is_err());
        // Index too high.
        assert!(bip85_bip39(12, HARDENED).is_err());
    }

    #[test]
    fn test_bip85_ln() {
        keystore::lock();
        assert!(bip85_ln(0).is_err());

        mock_unlocked_using_mnemonic(
            "virtual weapon code laptop defy cricket vicious target wave leopard garden give",
            "",
        );

        assert_eq!(
            bip85_ln(0).unwrap().as_slice(),
            b"\x3a\x5f\x3b\x88\x8a\xab\x88\xe2\xa9\xab\x99\x1b\x60\xa0\x3e\xd8",
        );
        assert_eq!(
            bip85_ln(1).unwrap().as_slice(),
            b"\xe7\xd9\xce\x75\xf8\xcb\x17\x57\x0e\x66\x54\x17\xb4\x7f\xa0\xbe",
        );
        assert_eq!(
            bip85_ln(HARDENED - 1).unwrap().as_slice(),
            b"\x1f\x3b\x75\xea\x25\x27\x49\x70\x0a\x1e\x45\x34\x69\x14\x8c\xa6",
        );

        // Index too high.
        assert!(bip85_ln(HARDENED).is_err());
    }

    #[test]
    fn test_fixtures() {
        struct Test {
            seed_len: usize,
            mnemonic_passphrase: &'static str,
            expected_mnemonic: &'static str,
            expected_xpub: &'static str,
            expected_u2f_seed_hex: &'static str,
        }
        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();

        let tests = [
            Test {
                seed_len: 32,
                mnemonic_passphrase: "",
                expected_mnemonic: "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
                expected_xpub: "xpub6Cj6NNCGj2CRPHvkuEG1rbW3nrNCAnLjaoTg1P67FCGoahSsbg9WQ7YaMEEP83QDxt2kZ3hTPAPpGdyEZcfAC1C75HfR66UbjpAb39f4PnG",
                expected_u2f_seed_hex: "4f464a6667ad88eebcd0f02982761e474ee0dd16253160320f49d1d6681745e9",
            },
            Test {
                seed_len: 32,
                mnemonic_passphrase: "abc",
                expected_mnemonic: "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
                expected_xpub: "xpub6DXBP3HhFdhUTafatEULxfTXUUxDVuCxfa9RAiBU5r6aRgKiABbeBDyqwWWjmKPP1BZvpvVNMbVR5LeHzhQphtLcPZ8jk3MdLBgc2sACJwR",
                expected_u2f_seed_hex: "d599da991ad83baaf449c789e2dff1539dd66983b47a1dec1c00ff3f352cccbc",
            },
            Test {
                seed_len: 24,
                mnemonic_passphrase: "",
                expected_mnemonic: "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before subject",
                expected_xpub: "xpub6C7fKxGtTzEVxCC22U2VHx4GpaVy77DzU6KdZ1CLuHgoUGviBMWDc62uoQVxqcRa5RQbMPnffjpwxve18BG81VJhJDXnSpRe5NGKwVpXiAb",
                expected_u2f_seed_hex: "fb9dc3fb0a17390776df5c3d8f9261bc5fd5df9f00414cee1393e37e0efda7ef",
            },
            Test {
                seed_len: 16,
                mnemonic_passphrase: "",
                expected_mnemonic: "sleep own lobster state clean thrive tail exist cactus bitter pass sniff",
                expected_xpub: "xpub6DLvpzjKpJ8k4xYrWYPmZQkUe9dkG1eRig2v6Jz4iYgo8hcpHWx87gGoCGDaB2cHFZ3ExUfe1jDiMu7Ch6gA4ULCBhvwZj29mHCPYSux3YV",
                expected_u2f_seed_hex: "20d68b206aff9667b623a460ce61fc94762de67561d6855ca9a6df7b409b2a54",
            },
        ];

        for test in tests {
            mock_memory();
            keystore::lock();
            let seed = &seed[..test.seed_len];
            assert!(keystore::unlock_bip39(test.mnemonic_passphrase).is_err());
            assert!(keystore::encrypt_and_store_seed(seed, "foo").is_ok());
            assert!(keystore::unlock_bip39(test.mnemonic_passphrase).is_err());
            assert!(keystore::is_locked());
            assert!(keystore::unlock("foo").is_ok());
            assert!(keystore::is_locked());
            assert!(keystore::unlock_bip39(test.mnemonic_passphrase).is_ok());
            assert!(!keystore::is_locked());
            assert_eq!(
                get_bip39_mnemonic().unwrap().as_str(),
                test.expected_mnemonic,
            );
            let keypath = &[44 + HARDENED, 0 + HARDENED, 0 + HARDENED];
            let xpub = get_xpub(keypath).unwrap();
            assert_eq!(
                xpub.serialize_str(crate::bip32::XPubType::Xpub).unwrap(),
                test.expected_xpub,
            );
            assert_eq!(
                hex::encode(keystore::get_u2f_seed().unwrap()),
                test.expected_u2f_seed_hex,
            );
        }
    }
}
