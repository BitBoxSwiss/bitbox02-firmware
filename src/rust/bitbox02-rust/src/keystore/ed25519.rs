// Copyright 2021, 2024 Shift Crypto AG
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

use alloc::vec::Vec;

use bip32_ed25519::{Xprv, Xpub, ED25519_EXPANDED_SECRET_KEY_SIZE};

/// Implements the digest traits for Sha512 backing it with the wally_sha512 C function. This is
/// done to avoid using a second sha512 implementation like `sha2::Sha512`, which bloats the binary
/// by an additional ~12.7kB (at the time of writing).
///
/// This implementation accumulates the data to be hashed in heap, it does **not** hash in a
/// streaming fashion, even when using `update()`. This is okay for the use within this module, as
/// bip32_ed25519 and sign_raw() do not hash a lot of data.
#[derive(Default, Clone)]
pub struct Sha512(Vec<u8>);

impl digest::HashMarker for Sha512 {}

impl digest::OutputSizeUser for Sha512 {
    type OutputSize = digest::typenum::U64;
}

impl digest::FixedOutput for Sha512 {
    fn finalize_into(self, out: &mut digest::Output<Self>) {
        // use digest::Digest;
        // out.copy_from_slice(&sha2::Sha512::digest(&self.0));
        out.copy_from_slice(&bitbox02::sha512(&self.0));
    }
}

impl digest::Update for Sha512 {
    fn update(&mut self, data: &[u8]) {
        self.0.extend(data);
    }
}

impl digest::Reset for Sha512 {
    fn reset(&mut self) {
        self.0 = vec![];
    }
}

impl digest::core_api::BlockSizeUser for Sha512 {
    type BlockSize = digest::typenum::U128;
}

fn get_seed() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    bitbox02::keystore::get_ed25519_seed()
}

fn get_xprv(keypath: &[u32]) -> Result<Xprv<Sha512>, ()> {
    let root = get_seed()?;
    Ok(Xprv::<Sha512>::from_normalize(
        &root[..ED25519_EXPANDED_SECRET_KEY_SIZE],
        &root[ED25519_EXPANDED_SECRET_KEY_SIZE..],
    )
    .derive_path(keypath))
}

pub fn get_xpub(keypath: &[u32]) -> Result<Xpub<Sha512>, ()> {
    Ok(get_xprv(keypath)?.public())
}

pub struct SignResult {
    pub signature: [u8; 64],
    pub public_key: ed25519_dalek::VerifyingKey,
}

pub fn sign(keypath: &[u32], msg: &[u8; 32]) -> Result<SignResult, ()> {
    let xprv = get_xprv(keypath)?;
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

    use bip32_ed25519::HARDENED_OFFSET;
    use bitbox02::testing::{mock_unlocked, mock_unlocked_using_mnemonic};
    use digest::Digest;

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

        mock_unlocked_using_mnemonic(
            "recall grace sport punch exhibit mad harbor stand obey short width stem awkward used stairs wool ugly trap season stove worth toward congress jaguar",
            "",
        );
        assert_eq!(
            get_seed().unwrap().as_slice(),
            b"\xa0\x8c\xf8\x5b\x56\x4e\xcf\x3b\x94\x7d\x8d\x43\x21\xfb\x96\xd7\x0e\xe7\xbb\x76\x08\x77\xe3\x71\x89\x9b\x14\xe2\xcc\xf8\x86\x58\x10\x4b\x88\x46\x82\xb5\x7e\xfd\x97\xde\xcb\xb3\x18\xa4\x5c\x05\xa5\x27\xb9\xcc\x5c\x2f\x64\xf7\x35\x29\x35\xa0\x49\xce\xea\x60\x68\x0d\x52\x30\x81\x94\xcc\xef\x2a\x18\xe6\x81\x2b\x45\x2a\x58\x15\xfb\xd7\xf5\xba\xbc\x08\x38\x56\x91\x9a\xaf\x66\x8f\xe7\xe4",
        );

        // Multiple loop iterations.
        mock_unlocked_using_mnemonic(
            "correct cherry mammal bubble want mandate polar hazard crater better craft exotic choice fun tourist census gap lottery neglect address glow carry old business",
            "",
        );
        assert_eq!(
            get_seed().unwrap().as_slice(),
            b"\x58\x7c\x67\x74\x35\x7e\xcb\xf8\x40\xd4\xdb\x64\x04\xff\x7a\xf0\x16\xda\xce\x04\x00\x76\x97\x51\xad\x2a\xbf\xc7\x7b\x9a\x38\x44\xcc\x71\x70\x25\x20\xef\x1a\x4d\x1b\x68\xb9\x11\x87\x78\x7a\x9b\x8f\xaa\xb0\xa9\xbb\x6b\x16\x0d\xe5\x41\xb6\xee\x62\x46\x99\x01\xfc\x0b\xed\xa0\x97\x5f\xe4\x76\x3b\xea\xbd\x83\xb7\x05\x1a\x5f\xd5\xcb\xce\x5b\x88\xe8\x2c\x4b\xba\xca\x26\x50\x14\xe5\x24\xbd",
        );

        mock_unlocked_using_mnemonic(
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art",
            "foo",
        );
        assert_eq!(
            get_seed().unwrap().as_slice(),
            b"\xf0\x53\xa1\xe7\x52\xde\x5c\x26\x19\x7b\x60\xf0\x32\xa4\x80\x9f\x08\xbb\x3e\x5d\x90\x48\x4f\xe4\x20\x24\xbe\x31\xef\xcb\xa7\x57\x8d\x91\x4d\x3f\xf9\x92\xe2\x16\x52\xfe\xe6\xa4\xd9\x9f\x60\x91\x00\x69\x38\xfa\xc2\xc0\xc0\xf9\xd2\xde\x0b\xa6\x4b\x75\x4e\x92\xa4\xf3\x72\x3f\x23\x47\x20\x77\xaa\x4c\xd4\xdd\x8a\x8a\x17\x5d\xba\x07\xea\x18\x52\xda\xd1\xcf\x26\x8c\x61\xa2\x67\x9c\x38\x90",
        );
    }

    #[test]
    fn test_get_xpub() {
        bitbox02::keystore::lock();
        assert!(get_xpub(&[]).is_err());

        mock_unlocked();

        let xpub = get_xpub(&[]).unwrap();
        assert_eq!(xpub.pubkey_bytes(), b"\x1c\xc2\xc8\x0d\x6f\xb0\x3e\xc0\x9e\x8a\x26\x8b\xaa\x45\xd4\xca\x2a\xfe\x5c\x5a\xc4\xdb\x3e\xe2\x9c\x7a\xd2\x37\x55\xab\xdc\x14");
        assert_eq!(xpub.chain_code(), b"\xf0\xa5\x91\x06\x42\xd0\x77\x98\x17\x40\x2e\x5e\x7a\x75\x54\x95\xe7\x44\xf5\x5c\xf1\x1e\x49\xee\xfd\x22\xa4\x60\xe9\xb2\xf7\x53");

        let xpub = get_xpub(&[10 + HARDENED_OFFSET, 10]).unwrap();
        assert_eq!(xpub.pubkey_bytes(), b"\xab\x58\xbd\x94\x7e\x2b\xf6\x64\xa7\xc0\x66\xde\x2e\xf0\x24\x0e\xfc\x24\xf3\x6e\xfd\x50\x2d\xf8\x83\x93\xe1\x96\xaf\x3c\x91\x8e");
        assert_eq!(xpub.chain_code(), b"\xf2\x00\x13\x38\x58\x02\xa6\xf9\xc0\x5e\xe7\xb0\x36\x16\xad\xf6\x9f\x5f\x9e\xc4\x32\x53\xa5\xd0\x8b\xe9\x65\x79\x81\x90\x83\xbb");
    }

    #[test]
    fn test_get_xprv() {
        bitbox02::keystore::lock();
        assert!(get_xprv(&[]).is_err());

        mock_unlocked();
        let xprv = get_xprv(&[]).unwrap();
        assert_eq!(xprv.expanded_secret_key().as_slice(), b"\xf8\xcb\x28\x85\x37\x60\x2b\x90\xd1\x29\x75\x4b\xdd\x0e\x4b\xed\xf9\xe2\x92\x3a\x04\xb6\x86\x7e\xdb\xeb\xc7\x93\xa7\x17\x6f\x5d\xca\xc5\xc9\x5d\x5f\xd2\x3a\x8e\x01\x6c\x95\x57\x69\x0e\xad\x1f\x00\x2b\x0f\x35\xd7\x06\xff\x8e\x59\x84\x1c\x09\xe0\xb6\xbb\x23");

        let xprv = get_xprv(&[10 + HARDENED_OFFSET, 10]).unwrap();
        assert_eq!(xprv.expanded_secret_key().as_slice(), b"\x00\x28\x46\xb1\xeb\x06\x66\xff\x4e\xf1\x66\xde\x37\x80\xdf\xe1\x95\xed\x6f\xfd\xce\x41\x18\x09\x9d\x9d\x80\x85\xaa\x17\x6f\x5d\x1f\xcf\xf9\x55\x2e\xe4\xc0\xcb\x03\xaa\x42\x1a\xe8\x2f\x98\xa0\x0a\xfc\x65\xb6\x84\x66\x31\xaa\x41\x8e\x6d\x5a\x62\x6e\x75\xf4");
    }

    #[test]
    fn test_sign() {
        let msg = &[0u8; 32];
        bitbox02::keystore::lock();
        assert!(sign(&[10 + HARDENED_OFFSET, 10], msg).is_err());

        mock_unlocked();
        let sig = sign(&[10 + HARDENED_OFFSET, 10], msg).unwrap();
        assert_eq!(sig.public_key.as_ref(), b"\xab\x58\xbd\x94\x7e\x2b\xf6\x64\xa7\xc0\x66\xde\x2e\xf0\x24\x0e\xfc\x24\xf3\x6e\xfd\x50\x2d\xf8\x83\x93\xe1\x96\xaf\x3c\x91\x8e");
        assert_eq!(
            sig.signature,
            *b"\x6c\x9b\xc4\x0e\x34\xe2\xa9\xb7\x88\x5e\xec\x72\xc0\x60\xba\x76\x9f\xe3\xa7\x4c\x9b\x14\x4b\xbf\x63\xf4\xd5\x4e\xa6\x66\x04\x31\x34\x25\x0e\xb2\x7d\xd3\x42\x28\x47\x5d\x7c\x6b\x54\x32\xd7\x37\x42\xf4\xb5\xa0\x98\xf4\x65\xba\x10\x1e\x90\xd1\x00\x35\x68\x01"
        );
    }
}
