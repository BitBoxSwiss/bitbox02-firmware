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

use bip32_ed25519::{Xprv, Xpub, ED25519_EXPANDED_SECRET_KEY_SIZE};

fn get_xprv(keypath: &[u32]) -> Result<Xprv, ()> {
    let root = bitbox02::keystore::get_ed25519_seed()?;
    Ok(Xprv::from_normalize(
        &root[..ED25519_EXPANDED_SECRET_KEY_SIZE],
        &root[ED25519_EXPANDED_SECRET_KEY_SIZE..],
    )
    .derive_path(keypath))
}

pub fn get_xpub(keypath: &[u32]) -> Result<Xpub, ()> {
    Ok(get_xprv(keypath)?.public())
}

pub struct SignResult {
    pub signature: [u8; 64],
    pub public_key: ed25519_dalek::PublicKey,
}

pub fn sign(keypath: &[u32], msg: &[u8; 32]) -> Result<SignResult, ()> {
    let xprv = get_xprv(keypath)?;
    let secret_key = ed25519_dalek::ExpandedSecretKey::from_bytes(&xprv.expanded_secret_key()[..])
        .or(Err(()))?;
    let public_key = ed25519_dalek::PublicKey::from(&secret_key);
    Ok(SignResult {
        signature: secret_key.sign(msg, &public_key).to_bytes(),
        public_key,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use bip32_ed25519::HARDENED_OFFSET;
    use bitbox02::testing::{mock_unlocked, MUTEX};

    #[test]
    fn test_get_xpub() {
        let _guard = MUTEX.lock().unwrap();

        assert!(get_xpub(&[]).is_err());

        mock_unlocked();

        let xpub = get_xpub(&[]).unwrap();
        assert_eq!(xpub.pubkey_bytes(), b"\x1c\xc2\xc8\x0d\x6f\xb0\x3e\xc0\x9e\x8a\x26\x8b\xaa\x45\xd4\xca\x2a\xfe\x5c\x5a\xc4\xdb\x3e\xe2\x9c\x7a\xd2\x37\x55\xab\xdc\x14");
        assert_eq!(xpub.chain_code(), b"\xf0\xa5\x91\x06\x42\xd0\x77\x98\x17\x40\x2e\x5e\x7a\x75\x54\x95\xe7\x44\xf5\x5c\xf1\x1e\x49\xee\xfd\x22\xa4\x60\xe9\xb2\xf7\x53");

        let xpub = get_xpub(&[10 + HARDENED_OFFSET, 10]).unwrap();
        assert_eq!(xpub.pubkey_bytes(), b"\xab\x58\xbd\x94\x7e\x2b\xf6\x64\xa7\xc0\x66\xde\x2e\xf0\x24\x0e\xfc\x24\xf3\x6e\xfd\x50\x2d\xf8\x83\x93\xe1\x96\xaf\x3c\x91\x8e");
        assert_eq!(xpub.chain_code(), b"\xf2\x00\x13\x38\x58\x02\xa6\xf9\xc0\x5e\xe7\xb0\x36\x16\xad\xf6\x9f\x5f\x9e\xc4\x32\x53\xa5\xd0\x8b\xe9\x65\x79\x81\x90\x83\xbb");
    }
}
