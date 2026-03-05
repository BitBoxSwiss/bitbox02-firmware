// SPDX-License-Identifier: Apache-2.0

use alloc::vec::Vec;

use crate::hash::Sha512;
use bip32_ed25519::{ED25519_EXPANDED_SECRET_KEY_SIZE, Xprv, Xpub};

use bitcoin::hashes::{Hash, HashEngine, Hmac, HmacEngine, sha256, sha512};

fn hmac_sha512(key: &[u8], msg: &[u8]) -> [u8; 64] {
    let mut engine = HmacEngine::<sha512::Hash>::new(key);
    engine.input(msg);
    Hmac::from_engine(engine).to_byte_array()
}

/// Get the seed to be used for ed25519 applications such as Cardano. The output is the root key to
/// BIP32-ED25519.
/// This implements a derivation compatible with Ledger according to
/// https://github.com/LedgerHQ/orakolo/blob/0b2d5e669ec61df9a824df9fa1a363060116b490/src/python/orakolo/HDEd25519.py.
/// Returns 96 bytes. It will contain a 64 byte expanded ed25519 private key followed by a 32 byte chain code.
fn get_seed(hal: &mut impl crate::hal::Hal) -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let bip39_seed = crate::keystore::copy_bip39_seed(hal)?;
    let mut seed_out = zeroize::Zeroizing::new(vec![0u8; 96]);
    let first64: &mut [u8] = &mut seed_out.as_mut_slice()[..64];
    first64.copy_from_slice(&bip39_seed);

    let key = b"ed25519 seed";

    loop {
        first64.copy_from_slice(&hmac_sha512(key, first64));
        if first64[31] & 0x20 == 0 {
            break;
        }
    }

    seed_out[0] &= 248;
    seed_out[31] &= 127;
    seed_out[31] |= 64;

    // Compute chain code and put it into seed_out at offset 64.
    let mut engine = HmacEngine::<sha256::Hash>::new(key);
    engine.input(&[0x01]);
    engine.input(&bip39_seed);
    let hmac_result: [u8; 32] = Hmac::from_engine(engine).to_byte_array();
    seed_out[64..].copy_from_slice(&hmac_result);
    Ok(seed_out)
}

fn get_xprv(hal: &mut impl crate::hal::Hal, keypath: &[u32]) -> Result<Xprv<Sha512>, ()> {
    let root = get_seed(hal)?;
    Ok(Xprv::<Sha512>::from_normalize(
        &root[..ED25519_EXPANDED_SECRET_KEY_SIZE],
        &root[ED25519_EXPANDED_SECRET_KEY_SIZE..],
    )
    .derive_path(keypath))
}

pub fn get_xpub(hal: &mut impl crate::hal::Hal, keypath: &[u32]) -> Result<Xpub<Sha512>, ()> {
    Ok(get_xprv(hal, keypath)?.public())
}

pub fn get_xpub_twice(hal: &mut impl crate::hal::Hal, keypath: &[u32]) -> Result<Xpub<Sha512>, ()> {
    let xpub = get_xpub(hal, keypath)?;
    let xpub2 = get_xpub(hal, keypath)?;
    if xpub.pubkey_bytes() == xpub2.pubkey_bytes() && xpub.chain_code() == xpub2.chain_code() {
        Ok(xpub)
    } else {
        Err(())
    }
}

pub struct SignResult {
    pub signature: [u8; 64],
    pub public_key: ed25519_dalek::VerifyingKey,
}

pub fn sign(
    hal: &mut impl crate::hal::Hal,
    keypath: &[u32],
    msg: &[u8; 32],
) -> Result<SignResult, ()> {
    let xprv = get_xprv(hal, keypath)?;
    let secret_key =
        ed25519_dalek::hazmat::ExpandedSecretKey::from_bytes(&xprv.expanded_secret_key());
    let public_key = ed25519_dalek::VerifyingKey::from(&secret_key);
    Ok(SignResult {
        signature: ed25519_dalek::hazmat::raw_sign::<Sha512>(&secret_key, msg, &public_key)
            .to_bytes(),
        public_key,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::keystore::testing::{mock_unlocked, mock_unlocked_using_mnemonic};
    use bip32_ed25519::HARDENED_OFFSET;
    use digest::Digest;
    use hex_lit::hex;

    #[test]
    fn test_sha512() {
        assert_eq!(Sha512::digest(b"foobar"), sha2::Sha512::digest(b"foobar"));

        let mut hasher: Sha512 = Default::default();
        hasher.update(b"foo");
        hasher.update(b"bar");
        assert_eq!(hasher.finalize(), sha2::Sha512::digest(b"foobar"));

        hasher = Default::default();
        hasher.update(b"foo");
        hasher.update(b"bar");
        hasher.reset();
        hasher.update(b"baz");
        assert_eq!(hasher.finalize(), sha2::Sha512::digest(b"baz"));
    }

    #[test]
    fn test_get_seed() {
        // Test vectors taken from:
        // https://github.com/cardano-foundation/CIPs/blob/6c249ef48f8f5b32efc0ec768fadf4321f3173f2/CIP-0003/Ledger.md#test-vectors
        // See also: https://github.com/cardano-foundation/CIPs/pull/132

        let mut mock_hal = crate::hal::testing::TestingHal::new();

        mock_unlocked_using_mnemonic(
            "recall grace sport punch exhibit mad harbor stand obey short width stem awkward used stairs wool ugly trap season stove worth toward congress jaguar",
            "",
        );
        assert_eq!(
            get_seed(&mut mock_hal).unwrap().as_slice(),
            &hex!(
                "a08cf85b564ecf3b947d8d4321fb96d70ee7bb760877e371899b14e2ccf88658104b884682b57efd97decbb318a45c05a527b9cc5c2f64f7352935a049ceea60680d52308194ccef2a18e6812b452a5815fbd7f5babc083856919aaf668fe7e4"
            ),
        );

        // Multiple loop iterations.
        mock_unlocked_using_mnemonic(
            "correct cherry mammal bubble want mandate polar hazard crater better craft exotic choice fun tourist census gap lottery neglect address glow carry old business",
            "",
        );
        assert_eq!(
            get_seed(&mut mock_hal).unwrap().as_slice(),
            &hex!(
                "587c6774357ecbf840d4db6404ff7af016dace0400769751ad2abfc77b9a3844cc71702520ef1a4d1b68b91187787a9b8faab0a9bb6b160de541b6ee62469901fc0beda0975fe4763beabd83b7051a5fd5cbce5b88e82c4bbaca265014e524bd"
            ),
        );

        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art",
            "foo",
        );
        assert_eq!(
            get_seed(&mut mock_hal).unwrap().as_slice(),
            &hex!(
                "f053a1e752de5c26197b60f032a4809f08bb3e5d90484fe42024be31efcba7578d914d3ff992e21652fee6a4d99f6091006938fac2c0c0f9d2de0ba64b754e92a4f3723f23472077aa4cd4dd8a8a175dba07ea1852dad1cf268c61a2679c3890"
            ),
        );
    }

    #[test]
    fn test_get_xpub() {
        crate::keystore::lock();

        let mut mock_hal = crate::hal::testing::TestingHal::new();

        assert!(get_xpub(&mut mock_hal, &[]).is_err());

        mock_unlocked();

        let xpub = get_xpub(&mut mock_hal, &[]).unwrap();
        assert_eq!(
            xpub.pubkey_bytes(),
            &hex!("1cc2c80d6fb03ec09e8a268baa45d4ca2afe5c5ac4db3ee29c7ad23755abdc14")
        );
        assert_eq!(
            xpub.chain_code(),
            &hex!("f0a5910642d0779817402e5e7a755495e744f55cf11e49eefd22a460e9b2f753")
        );

        let xpub = get_xpub(&mut mock_hal, &[10 + HARDENED_OFFSET, 10]).unwrap();
        assert_eq!(
            xpub.pubkey_bytes(),
            &hex!("ab58bd947e2bf664a7c066de2ef0240efc24f36efd502df88393e196af3c918e")
        );
        assert_eq!(
            xpub.chain_code(),
            &hex!("f20013385802a6f9c05ee7b03616adf69f5f9ec43253a5d08be96579819083bb")
        );
    }

    #[test]
    fn test_get_xpub_twice() {
        crate::keystore::lock();
        let mut mock_hal = crate::hal::testing::TestingHal::new();

        assert!(get_xpub_twice(&mut mock_hal, &[]).is_err());

        mock_unlocked();
        let xpub = get_xpub_twice(&mut mock_hal, &[]).unwrap();
        assert_eq!(
            xpub.pubkey_bytes(),
            &hex!("1cc2c80d6fb03ec09e8a268baa45d4ca2afe5c5ac4db3ee29c7ad23755abdc14")
        );
        assert_eq!(
            xpub.chain_code(),
            &hex!("f0a5910642d0779817402e5e7a755495e744f55cf11e49eefd22a460e9b2f753")
        );
    }

    #[test]
    fn test_get_xprv() {
        crate::keystore::lock();

        let mut mock_hal = crate::hal::testing::TestingHal::new();

        assert!(get_xprv(&mut mock_hal, &[]).is_err());

        mock_unlocked();
        let xprv = get_xprv(&mut mock_hal, &[]).unwrap();
        assert_eq!(
            xprv.expanded_secret_key().as_slice(),
            &hex!(
                "f8cb288537602b90d129754bdd0e4bedf9e2923a04b6867edbebc793a7176f5dcac5c95d5fd23a8e016c9557690ead1f002b0f35d706ff8e59841c09e0b6bb23"
            )
        );

        let xprv = get_xprv(&mut mock_hal, &[10 + HARDENED_OFFSET, 10]).unwrap();
        assert_eq!(
            xprv.expanded_secret_key().as_slice(),
            &hex!(
                "002846b1eb0666ff4ef166de3780dfe195ed6ffdce4118099d9d8085aa176f5d1fcff9552ee4c0cb03aa421ae82f98a00afc65b6846631aa418e6d5a626e75f4"
            )
        );
    }

    #[test]
    fn test_sign() {
        let msg = &[0u8; 32];
        crate::keystore::lock();
        assert!(
            sign(
                &mut crate::hal::testing::TestingHal::new(),
                &[10 + HARDENED_OFFSET, 10],
                msg
            )
            .is_err()
        );

        mock_unlocked();
        let sig = sign(
            &mut crate::hal::testing::TestingHal::new(),
            &[10 + HARDENED_OFFSET, 10],
            msg,
        )
        .unwrap();
        assert_eq!(
            sig.public_key.as_ref(),
            &hex!("ab58bd947e2bf664a7c066de2ef0240efc24f36efd502df88393e196af3c918e")
        );
        assert_eq!(
            sig.signature,
            hex!(
                "6c9bc40e34e2a9b7885eec72c060ba769fe3a74c9b144bbf63f4d54ea666043134250eb27dd34228475d7c6b5432d73742f4b5a098f465ba101e90d100356801"
            )
        );
    }
}
