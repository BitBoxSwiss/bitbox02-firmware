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

use crate::secp256k1::SECP256K1;

use bitcoin::hashes::{Hash, HashEngine, Hmac, HmacEngine, sha512};

/// Returns the keystore's seed encoded as a BIP-39 mnemonic.
pub fn get_bip39_mnemonic() -> Result<zeroize::Zeroizing<String>, ()> {
    keystore::bip39_mnemonic_from_seed(&keystore::copy_seed()?)
}

fn get_xprv(keypath: &[u32]) -> Result<bip32::Xprv, ()> {
    let bip39_seed = keystore::copy_bip39_seed()?;
    let xprv: bip32::Xprv =
        bitcoin::bip32::Xpriv::new_master(bitcoin::NetworkKind::Main, &bip39_seed)
            .map_err(|_| ())?
            .into();
    Ok(xprv
        .xprv
        .derive_priv(SECP256K1, &bip32::keypath_from_slice(keypath))
        .map_err(|_| ())?
        .into())
}

/// Get the private key at the keypath.
pub fn secp256k1_get_private_key(keypath: &[u32]) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let xprv = get_xprv(keypath)?;
    Ok(zeroize::Zeroizing::new(
        xprv.xprv.private_key.secret_bytes().to_vec(),
    ))
}

/// Get the private key at the keypath, computed twice to mitigate the risk of bitflips.
pub fn secp256k1_get_private_key_twice(keypath: &[u32]) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let privkey = secp256k1_get_private_key(keypath)?;
    if privkey == secp256k1_get_private_key(keypath)? {
        Ok(privkey)
    } else {
        Err(())
    }
}

/// Can be used only if the keystore is unlocked. Returns the derived xpub,
/// using bip32 derivation. Derivation is done from the xprv master, so hardened
/// derivation is allowed.
pub fn get_xpub_once(keypath: &[u32]) -> Result<bip32::Xpub, ()> {
    let xpriv = get_xprv(keypath)?;
    let xpub = bitcoin::bip32::Xpub::from_priv(SECP256K1, &xpriv.xprv);
    Ok(bip32::Xpub::from(xpub))
}

/// Can be used only if the keystore is unlocked. Returns the derived xpub,
/// using bip32 derivation. Derivation is done from the xprv master, so hardened
/// derivation is allowed.
pub fn get_xpub_twice(keypath: &[u32]) -> Result<bip32::Xpub, ()> {
    let res1 = get_xpub_once(keypath)?;
    let res2 = get_xpub_once(keypath)?;
    if res1 != res2 {
        return Err(());
    }
    Ok(res1)
}

/// Gets multiple xpubs at once. This is better than multiple calls to `get_xpub_twice()` as it only
/// uses two secure chip operations in total, instead of two per xpub.
pub fn get_xpubs_twice(keypaths: &[&[u32]]) -> Result<Vec<bip32::Xpub>, ()> {
    if keystore::is_locked() {
        return Err(());
    }
    if keypaths.is_empty() {
        return Ok(vec![]);
    }
    // We get the root xprv as a starting point (twice to mitigate bitflips), afterwards we don't
    // need the securechip anymore.
    let xprv = get_xprv(&[])?;
    let xprv2 = get_xprv(&[])?;

    let mut out = Vec::with_capacity(keypaths.len());
    for keypath in keypaths {
        if xprv != xprv2 {
            return Err(());
        }

        let derive_xpub = || -> Result<bip32::Xpub, ()> {
            let derived_xprv = xprv
                .xprv
                .derive_priv(SECP256K1, &bip32::keypath_from_slice(keypath))
                .map_err(|_| ())?;
            Ok(bip32::Xpub::from(bitcoin::bip32::Xpub::from_priv(
                SECP256K1,
                &derived_xprv,
            )))
        };
        let derived_xpub = derive_xpub()?;
        if derived_xpub != derive_xpub()? {
            return Err(());
        }
        out.push(derived_xpub);
    }

    Ok(out)
}

/// Returns fingerprint of the root public key at m/, which are the first four bytes of its hash160
/// according to:
/// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#serialization-format
pub fn root_fingerprint() -> Result<Vec<u8>, ()> {
    keystore::root_fingerprint()
}

fn bip85_entropy(keypath: &[u32]) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let priv_key = secp256k1_get_private_key_twice(keypath)?;

    let mut engine = HmacEngine::<sha512::Hash>::new(b"bip-entropy-from-k");
    engine.input(&priv_key);
    Ok(zeroize::Zeroizing::new(
        Hmac::from_engine(engine).to_byte_array().to_vec(),
    ))
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

/// Sign a message using the private key at the keypath, which is optionally tweaked with the given
/// tweak.
pub fn secp256k1_schnorr_sign(
    keypath: &[u32],
    msg: &[u8; 32],
    tweak: Option<&[u8; 32]>,
) -> Result<[u8; 64], ()> {
    let private_key = secp256k1_get_private_key(keypath)?;
    let mut keypair =
        bitcoin::secp256k1::Keypair::from_seckey_slice(SECP256K1, &private_key).map_err(|_| ())?;

    if let Some(tweak) = tweak {
        keypair = keypair
            .add_xonly_tweak(
                SECP256K1,
                &bitcoin::secp256k1::Scalar::from_be_bytes(*tweak).map_err(|_| ())?,
            )
            .map_err(|_| ())?;
    }

    let sig = SECP256K1.sign_schnorr_with_aux_rand(
        &bitcoin::secp256k1::Message::from_digest(*msg),
        &keypair,
        &bitbox02::random::random_32_bytes(),
    );
    Ok(sig.serialize())
}

/// Get the seed to be used for u2f
#[cfg(feature = "app-u2f")]
pub fn get_u2f_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let bip39_seed = keystore::copy_bip39_seed()?;

    let mut engine = HmacEngine::<bitcoin::hashes::sha256::Hash>::new(&bip39_seed);
    // Null-terminator for backwards compatibility from the time when this was coded in C.
    engine.input(b"u2f\0");
    Ok(zeroize::Zeroizing::new(
        Hmac::from_engine(engine).to_byte_array().to_vec(),
    ))
}

/// # Safety
///
/// keypath pointer has point to a buffer of length `keypath_len` uint32 elements.
#[cfg(feature = "c-unit-testing")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_secp256k1_get_private_key(
    keypath: *const u32,
    keypath_len: usize,
    mut out: util::bytes::BytesMut,
) -> bool {
    match unsafe { secp256k1_get_private_key(core::slice::from_raw_parts(keypath, keypath_len)) } {
        Ok(private_key) => {
            out.as_mut().copy_from_slice(&private_key);
            true
        }
        Err(()) => false,
    }
}

#[cfg(feature = "app-u2f")]
#[unsafe(no_mangle)]
pub extern "C" fn rust_keystore_get_u2f_seed(mut seed_out: util::bytes::BytesMut) -> bool {
    match get_u2f_seed() {
        Ok(seed) => {
            seed_out.as_mut().copy_from_slice(&seed);
            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bitbox02::testing::{
        TEST_MNEMONIC, mock_memory, mock_unlocked, mock_unlocked_using_mnemonic,
    };

    use bitcoin::secp256k1;

    #[test]
    fn test_secp256k1_get_private_key() {
        keystore::lock();
        let keypath = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        assert!(secp256k1_get_private_key(keypath).is_err());

        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "",
        );

        bitbox02::securechip::fake_event_counter_reset();
        assert_eq!(
            hex::encode(secp256k1_get_private_key(keypath).unwrap()),
            "4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3"
        );
        assert_eq!(bitbox02::securechip::fake_event_counter(), 1);
    }

    #[test]
    fn test_secp256k1_get_private_key_twice() {
        keystore::lock();
        let keypath = &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        assert!(secp256k1_get_private_key_twice(keypath).is_err());

        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "",
        );

        bitbox02::securechip::fake_event_counter_reset();
        assert_eq!(
            hex::encode(secp256k1_get_private_key_twice(keypath).unwrap()),
            "4604b4b710fe91f584fff084e1a9159fe4f8408fff380596a604948474ce4fa3"
        );
        assert_eq!(bitbox02::securechip::fake_event_counter(), 2);
    }

    #[test]
    fn test_get_bip39_mnemonic() {
        keystore::lock();
        assert!(get_bip39_mnemonic().is_err());

        mock_unlocked();

        assert_eq!(get_bip39_mnemonic().unwrap().as_str(), TEST_MNEMONIC);
    }

    #[test]
    fn test_get_xpub_twice() {
        let keypath = &[44 + HARDENED, 0 + HARDENED, 0 + HARDENED];
        // Also test with unhardened and non-zero elements.
        let keypath_5 = &[44 + HARDENED, 1 + HARDENED, 10 + HARDENED, 1, 100];

        keystore::lock();
        assert!(get_xpub_twice(keypath).is_err());

        // 24 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
            "",
        );

        bitbox02::securechip::fake_event_counter_reset();

        assert_eq!(
            get_xpub_twice(&[])
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub661MyMwAqRbcEhX8d9WJh78SZrxusAzWFoykz4n5CF75uYRzixw5FZPUSoWyhaaJ1bpiPFdzdHSQqJN38PcTkyrLmxT4J2JDYfoGJQ4ioE2",
        );

        assert_eq!(bitbox02::securechip::fake_event_counter(), 2);

        assert_eq!(
            get_xpub_twice(keypath)
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6Cj6NNCGj2CRPHvkuEG1rbW3nrNCAnLjaoTg1P67FCGoahSsbg9WQ7YaMEEP83QDxt2kZ3hTPAPpGdyEZcfAC1C75HfR66UbjpAb39f4PnG",
        );
        assert_eq!(
            get_xpub_twice(keypath_5)
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6HHn1zdtf1RjePopiTV5nxf8jY2xwbJicTQ91jV4cUJZ5EnbvXyBGDhqWt8B9JxxBt9vExi4pdWzrbrM43qSFs747VCGmSy2DPWAhg9MkUg",
        );

        // 18 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before subject",
            "",
        );
        assert_eq!(
            get_xpub_twice(keypath)
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6C7fKxGtTzEVxCC22U2VHx4GpaVy77DzU6KdZ1CLuHgoUGviBMWDc62uoQVxqcRa5RQbMPnffjpwxve18BG81VJhJDXnSpRe5NGKwVpXiAb",
        );

        // 12 words
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass sniff",
            "",
        );
        assert_eq!(
            get_xpub_twice(keypath)
                .unwrap()
                .serialize_str(bip32::XPubType::Xpub)
                .unwrap(),
            "xpub6DLvpzjKpJ8k4xYrWYPmZQkUe9dkG1eRig2v6Jz4iYgo8hcpHWx87gGoCGDaB2cHFZ3ExUfe1jDiMu7Ch6gA4ULCBhvwZj29mHCPYSux3YV",
        )
    }

    #[test]
    fn test_get_xpubs_twice() {
        keystore::lock();
        assert!(get_xpubs_twice(&[]).is_err());

        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
            "",
        );

        // Helper to convert to strings.
        let get = |keypaths| -> Vec<String> {
            get_xpubs_twice(keypaths)
                .unwrap()
                .iter()
                .map(|xpub| xpub.serialize_str(bip32::XPubType::Xpub).unwrap())
                .collect()
        };

        bitbox02::securechip::fake_event_counter_reset();
        assert!(get_xpubs_twice(&[]).unwrap().is_empty());
        assert_eq!(bitbox02::securechip::fake_event_counter(), 0);

        bitbox02::securechip::fake_event_counter_reset();
        assert_eq!(
            get(&[
                &[84 + HARDENED, HARDENED, HARDENED],
                &[86 + HARDENED, HARDENED, HARDENED],
            ]),
            vec![
                "xpub6CNbmcHwZDudAvCAZVE5kejUoFD63mbkRbRMA2HoF9oNWsCofni87gJKp31qZJ9FsCMQR2vK9AS51mT8dgUMGsHW6SfaAKb4eSzpqJn7zwK",
                "xpub6CGwpj8iQNuzSeeEKF4yuQt32fpLqfHj7sUfFH4uW34DoctWPksxAdjNYC9KwYgwA149B7SDdcLH1aFmucRcjBL4U6piN7HgaiFCBsToamH",
            ],
        );
        assert_eq!(bitbox02::securechip::fake_event_counter(), 2);
    }

    #[test]
    fn test_root_fingerprint() {
        keystore::lock();
        assert_eq!(root_fingerprint(), Err(()));

        mock_unlocked_using_mnemonic(
            "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay",
            "",
        );

        bitbox02::securechip::fake_event_counter_reset();
        assert_eq!(root_fingerprint(), Ok(vec![0x02, 0x40, 0xe9, 0x2a]));
        // fingerprint is precomputed during bip39 unlock, so takes no securechip events.
        assert_eq!(bitbox02::securechip::fake_event_counter(), 0);

        mock_unlocked_using_mnemonic(
            "small agent wife animal marine cloth exit thank stool idea steel frame",
            "",
        );
        assert_eq!(root_fingerprint(), Ok(vec![0xf4, 0x0b, 0x46, 0x9a]));

        keystore::lock();
        assert_eq!(root_fingerprint(), Err(()));
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

            assert!(keystore::unlock_bip39(seed, test.mnemonic_passphrase).is_err());

            bitbox02::securechip::fake_event_counter_reset();
            assert!(keystore::encrypt_and_store_seed(seed, "foo").is_ok());
            assert_eq!(bitbox02::securechip::fake_event_counter(), 12);

            assert!(keystore::is_locked());

            bitbox02::securechip::fake_event_counter_reset();
            assert!(keystore::unlock_bip39(seed, test.mnemonic_passphrase).is_ok());
            assert_eq!(bitbox02::securechip::fake_event_counter(), 1);

            assert!(!keystore::is_locked());
            assert_eq!(
                get_bip39_mnemonic().unwrap().as_str(),
                test.expected_mnemonic,
            );
            let keypath = &[44 + HARDENED, 0 + HARDENED, 0 + HARDENED];

            bitbox02::securechip::fake_event_counter_reset();
            let xpub = get_xpub_once(keypath).unwrap();
            assert_eq!(bitbox02::securechip::fake_event_counter(), 1);

            assert_eq!(
                xpub.serialize_str(crate::bip32::XPubType::Xpub).unwrap(),
                test.expected_xpub,
            );
            assert_eq!(
                hex::encode(get_u2f_seed().unwrap()),
                test.expected_u2f_seed_hex,
            );
        }
    }

    #[test]
    fn test_secp256k1_schnorr_sign() {
        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "",
        );
        let keypath = [86 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0];
        let msg = [0x88u8; 32];

        let expected_pubkey = {
            let pubkey =
                hex::decode("cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115")
                    .unwrap();
            secp256k1::XOnlyPublicKey::from_slice(&pubkey).unwrap()
        };

        // Test without tweak
        bitbox02::random::fake_reset();

        bitbox02::securechip::fake_event_counter_reset();
        let sig = secp256k1_schnorr_sign(&keypath, &msg, None).unwrap();
        assert_eq!(bitbox02::securechip::fake_event_counter(), 1);

        assert!(
            SECP256K1
                .verify_schnorr(
                    &secp256k1::schnorr::Signature::from_slice(&sig).unwrap(),
                    &secp256k1::Message::from_digest_slice(&msg).unwrap(),
                    &expected_pubkey
                )
                .is_ok()
        );

        // Test with tweak
        bitbox02::random::fake_reset();
        let tweak = {
            let tweak =
                hex::decode("a39fb163dbd9b5e0840af3cc1ee41d5b31245c5dd8d6bdc3d026d09b8964997c")
                    .unwrap();
            secp256k1::Scalar::from_be_bytes(tweak.try_into().unwrap()).unwrap()
        };
        let (tweaked_pubkey, _) = expected_pubkey.add_tweak(SECP256K1, &tweak).unwrap();
        let sig = secp256k1_schnorr_sign(&keypath, &msg, Some(&tweak.to_be_bytes())).unwrap();
        assert!(
            SECP256K1
                .verify_schnorr(
                    &secp256k1::schnorr::Signature::from_slice(&sig).unwrap(),
                    &secp256k1::Message::from_digest(msg),
                    &tweaked_pubkey
                )
                .is_ok()
        );
    }
}
