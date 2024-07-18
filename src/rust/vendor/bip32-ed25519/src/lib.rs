#![no_std]

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod arbitrary;

extern crate alloc;

mod bigint;

use alloc::boxed::Box;
use core::convert::TryInto;

use core::ops::{Deref, DerefMut};
use digest::{core_api::BlockSizeUser, typenum::U64, Digest};
use hmac::{Mac, SimpleHmac};
use zeroize::{Zeroize, Zeroizing};

use curve25519_dalek::{
    edwards::{CompressedEdwardsY, EdwardsPoint},
    scalar::Scalar,
};

const ED25519_SECRET_KEY_SIZE: usize = 32;

pub const ED25519_EXPANDED_SECRET_KEY_SIZE: usize = 64;
pub const CHAIN_CODE_SIZE: usize = 32;

pub const HARDENED_OFFSET: u32 = 0x80000000;

/// Compute the public key corresponding to an Ed25519 secret key. The result is a decoded point. Use `.compress()` to convert it into compressed form.
fn to_public_key(secret_key: &[u8; ED25519_SECRET_KEY_SIZE]) -> EdwardsPoint {
    Scalar::from_bytes_mod_order(*secret_key) * curve25519_dalek::constants::ED25519_BASEPOINT_POINT
}

/// The `D` digest type param must implement SHA512. Use `sha2::Sha512` if in doubt.
#[derive(Clone)]
pub struct Xpub<D: Digest<OutputSize = U64> + BlockSizeUser> {
    /// Ed25519 public key, 32 byte compressed encoded. Use
    /// `public_key.decompress()` to get an uncompressed public key for point
    /// operations.
    public_key: CompressedEdwardsY,
    chain_code: [u8; CHAIN_CODE_SIZE],
    marker: core::marker::PhantomData<D>,
}

impl<D: Digest<OutputSize = U64> + BlockSizeUser> core::cmp::PartialEq for Xpub<D> {
    fn eq(&self, other: &Self) -> bool {
        self.public_key == other.public_key && self.chain_code == other.chain_code
    }
}

/// Computes `(8 * sk[:28])*G` where `sk` is a little-endian encoded
/// int and `G` is the curve's base point.
fn point_of_trunc28_mul8(sk: &[u8]) -> EdwardsPoint {
    let mut copy = [0u8; 32];
    bigint::add_28_mul8(&[0u8; 32], sk, &mut copy);
    to_public_key(&copy)
}

#[derive(Debug)]
pub enum DerivationError {
    ExpectedSoftDerivation,
}

impl<D: Digest<OutputSize = U64> + BlockSizeUser> Xpub<D> {
    /// Creates an Xpub from a public key and a chain code.
    fn from(public_key: EdwardsPoint, chain_code: &[u8; CHAIN_CODE_SIZE]) -> Self {
        Xpub {
            public_key: public_key.compress(),
            chain_code: *chain_code,
            marker: Default::default(),
        }
    }

    pub fn pubkey_bytes(&self) -> &[u8; 32] {
        self.public_key.as_bytes()
    }

    pub fn chain_code(&self) -> &[u8; CHAIN_CODE_SIZE] {
        &self.chain_code
    }

    /// Derives a child extended public key. Only soft derivation is
    /// possible, i.e. `index` must be smaller than `HARDENED_OFFSET`.
    pub fn derive(&self, index: u32) -> Result<Xpub<D>, DerivationError> {
        if index >= HARDENED_OFFSET {
            return Err(DerivationError::ExpectedSoftDerivation);
        }
        let pk = &self.public_key;
        let pk_encoded = pk.as_bytes();

        let mut zmac = SimpleHmac::<D>::new_from_slice(&self.chain_code).unwrap();
        let mut imac = SimpleHmac::<D>::new_from_slice(&self.chain_code).unwrap();
        let i_serialized = index.to_le_bytes();
        zmac.update(&[0x2]);
        zmac.update(pk_encoded);
        zmac.update(&i_serialized);
        imac.update(&[0x3]);
        imac.update(pk_encoded);
        imac.update(&i_serialized);

        let zout = zmac.finalize().into_bytes();
        let zl = &zout[0..32];

        // left = kl + 8 * trunc28(zl)
        let left = pk.decompress().unwrap() + point_of_trunc28_mul8(zl);

        let iout = imac.finalize().into_bytes();
        let chain_code = &iout[32..];
        Ok(Xpub::from(left, chain_code.try_into().unwrap()))
    }
}

#[derive(Zeroize, Clone, Debug, PartialEq)]
#[zeroize(drop)]
struct XprvData {
    // An xprv consists of an expanded Ed25519 secret key and a chain
    // code.
    //
    // An expanded Ed5519 secret key is the SHA512 of a Ed25519 secret
    // key, with some bits tweaked (see `normalize()`).It is used to
    // sign messages in Ed25515. The left half is the secret key used
    // when signing, and the right half is used to derive the nonce.
    //
    // We keep track of them in two halves as that is more convenient
    // for the child derivation functions and requires fewer copies.
    expanded_secret_key_left: [u8; ED25519_SECRET_KEY_SIZE],
    expanded_secret_key_right: [u8; ED25519_SECRET_KEY_SIZE],
    chain_code: [u8; CHAIN_CODE_SIZE],
}

/// The `D` digest type param must implement SHA512. Use `sha2::Sha512` if in doubt.
#[derive(Clone, Debug)]
pub struct Xprv<D: Digest<OutputSize = U64> + BlockSizeUser + Clone>(
    // The data is boxed so that moving an `Xprv` does not accidentally
    // leave copies of the data on the stack.
    Box<XprvData>,
    core::marker::PhantomData<D>,
);

impl<D: Digest<OutputSize = U64> + BlockSizeUser + Clone> core::cmp::PartialEq for Xprv<D> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<D: Digest<OutputSize = U64> + BlockSizeUser + Clone> Xprv<D> {
    /// Creates an Xprv from the raw components.
    ///
    /// NOTE: the caller is responsible for ensuring the secret key is
    /// valid. See `normalize()` and `from_normalize()`.
    ///
    /// `expanded_secret_key_left` and `expanded_secret_key_right`
    /// must be of size `ED25519_SECRET_KEY_SIZE`.
    /// `chain_code` must be of size `CHAIN_CODE_SIZE`.
    fn from(
        expanded_secret_key_left: &[u8],
        expanded_secret_key_right: &[u8],
        chain_code: &[u8],
    ) -> Self {
        let mut result = Xprv(
            Box::new(XprvData {
                expanded_secret_key_left: [0; ED25519_SECRET_KEY_SIZE],
                expanded_secret_key_right: [0; ED25519_SECRET_KEY_SIZE],
                chain_code: [0; CHAIN_CODE_SIZE],
            }),
            Default::default(),
        );

        result
            .0
            .expanded_secret_key_left
            .copy_from_slice(expanded_secret_key_left);
        result
            .0
            .expanded_secret_key_right
            .copy_from_slice(expanded_secret_key_right);
        result.0.chain_code.copy_from_slice(chain_code);

        result
    }

    /// Creates an Xrpv from an expanded secret key and a chain code,
    /// normalizing see secret key. See `normalize()`.
    ///
    /// `expanded_secret_key` must be of size `ED25519_EXPANDED_SECRET_KEY_SIZE`.
    /// `chain_code` mus be of size `CHAIN_CODE_SIZE`.
    pub fn from_normalize(expanded_secret_key: &[u8], chain_code: &[u8]) -> Self {
        let mut result = Self::from(
            &expanded_secret_key[..32],
            &expanded_secret_key[32..],
            chain_code,
        );
        result.normalize();
        result
    }

    /// Clears the bits according to the Ed25519 specification:
    /// - Clear the lowest three bits.
    /// - Clear highest bit
    /// - Set second highest bit.
    ///
    /// In addition, it also clears the 3rd highest bit according the
    /// BIP32-Ed25519 paper. This ensures that the 2nd highest bit is
    /// always set in derived keys.
    fn normalize(&mut self) {
        // Clear lowest three bits.
        self.0.expanded_secret_key_left[0] &= 0b1111_1000;
        // Clear highest bit.
        self.0.expanded_secret_key_left[31] &= 0b0011_1111;
        // Set second highest bit.
        self.0.expanded_secret_key_left[31] |= 0b0100_0000;

        // Clear the 3rd highest bit according the BIP32-Ed25519 paper.
        self.0.expanded_secret_key_left[31] &= 0b1101_1111;
    }

    /// Computes the corresponding extended public key.
    pub fn public(&self) -> Xpub<D> {
        Xpub::from(
            to_public_key(&self.0.expanded_secret_key_left),
            &self.0.chain_code,
        )
    }

    /// Returns a copy of the 64 byte expanded secret key.
    pub fn expanded_secret_key(&self) -> Box<Zeroizing<[u8; ED25519_EXPANDED_SECRET_KEY_SIZE]>> {
        let mut result = Box::new(Zeroizing::new([0; ED25519_EXPANDED_SECRET_KEY_SIZE]));
        result[..32].copy_from_slice(&self.0.expanded_secret_key_left);
        result[32..].copy_from_slice(&self.0.expanded_secret_key_right);
        result
    }

    pub fn chain_code(&self) -> &[u8; CHAIN_CODE_SIZE] {
        &self.0.chain_code
    }

    /// Derives a child extended private key.
    pub fn derive(&self, index: u32) -> Xprv<D> {
        let kl = &self.0.expanded_secret_key_left;
        let kr = &self.0.expanded_secret_key_right;
        let mut zmac = SimpleHmac::<D>::new_from_slice(&self.0.chain_code).unwrap();
        let mut imac = SimpleHmac::<D>::new_from_slice(&self.0.chain_code).unwrap();
        let i_serialized = index.to_le_bytes();
        if index >= HARDENED_OFFSET {
            zmac.update(&[0x0]);
            zmac.update(kl);
            zmac.update(kr);
            zmac.update(&i_serialized);
            imac.update(&[0x1]);
            imac.update(kl);
            imac.update(kr);
            imac.update(&i_serialized);
        } else {
            let pk = to_public_key(kl).compress();
            let pk_encoded = pk.as_bytes();
            zmac.update(&[0x2]);
            zmac.update(pk_encoded);
            zmac.update(&i_serialized);
            imac.update(&[0x3]);
            imac.update(pk_encoded);
            imac.update(&i_serialized);
        }

        let zout = zmac.finalize().into_bytes();
        let zl = &zout[0..32];
        let zr = &zout[32..64];
        // left = kl + 8 * trunc28(zl)
        let mut left = Zeroizing::new([0u8; 32]);
        bigint::add_28_mul8(kl, zl, left.deref_mut());
        // right = zr + kr
        let mut right = [0u8; 32];
        bigint::add_256bits(kr, zr, &mut right);

        // note: we don't perform the check for curve order divisibility because it will not happen:
        // 1. all keys are in the range K=2^254 .. 2^255 (actually the even smaller range 2^254+2^253)
        // 2. all keys are also multiple of 8
        // 3. all existing multiple of the curve order n in the range of K are not multiple of 8
        // n = 2^252 + 27742317777372353535851937790883648493
        // The only multiple of the order n which is also a multiple of 8 in 0..2^256 is 8*n, and 8*n >= 2^255.

        let iout = imac.finalize().into_bytes();
        let chain_code = &iout[32..];
        Xprv::from(left.deref(), &right, chain_code)
    }

    // Derives a child extended private key.
    pub fn derive_path(&self, path: &[u32]) -> Xprv<D> {
        let mut k = self.clone();
        for index in path {
            k = k.derive(*index);
        }
        k
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arbitrary::{Arbitrary32, Arbitrary64};
    use sha2::Sha512;

    #[quickcheck]
    fn xpub_matches_xprv(key: Arbitrary64, chain_code: Arbitrary32) -> bool {
        let mut xprv = Xprv::<Sha512>::from_normalize(&key.0, &chain_code.0);
        let mut xpub = xprv.public();
        for index in 0..10 {
            xprv = xprv.derive(index);
            xpub = xpub.derive(index).unwrap();
            if xpub != xprv.public() {
                return false;
            }
        }
        true
    }

    const KEY: &[u8;ED25519_EXPANDED_SECRET_KEY_SIZE] = b"\x17\x6b\xdb\xc8\x4f\xa5\x42\xc1\xb4\xc4\x9c\x48\x47\xb5\x89\x4c\x5a\x11\x89\x2f\xbc\x44\x7f\x73\x9e\x53\xac\x64\x70\x7d\x57\xc1\x0e\x06\x18\xa6\xb7\xab\x8c\xf9\x04\xa9\xf4\x97\x5c\xa9\xb8\x84\xf3\x4e\x16\xe4\x09\xd3\x50\xbb\x9d\xa9\xc3\xc4\x1c\xe1\x57\xf4";

    const CHAIN_CODE: &[u8; CHAIN_CODE_SIZE] = b"\x10\x51\x75\xc4\x52\x9c\x2a\xbb\x4d\x7a\x8a\xf8\x1e\x87\x4c\x1d\x8d\xb2\xf1\x90\x14\x1a\x6f\xaa\x6f\x57\x83\x07\x6b\x52\xa9\x64";

    // This checks if this is a valid Ed25519 private keys with regards to the special bit requirements:
    // - Highest bit is cleared
    // - Second-highest bit is set
    // - Three lowest bits are cleared
    fn ed25519_bits_correct(xprv: &Xprv<Sha512>) -> bool {
        let s = xprv.expanded_secret_key();
        s[31] & 0b1000_0000 == 0 && s[31] & 0b0100_0000 != 0 && s[0] & 0b0000_0111 == 0
    }

    #[test]
    fn ed25519_secret_key_bits_correct() {
        // We start with a private key which, after clearing the bits,
        // is 0b0101_1111_1111_...._1000.  If the third-highest bit
        // was not set, any addition to the private key (any
        // derivation) would clear the second-highest bit, which would
        // be an invalid Ed25519 private key.
        let xprv = Xprv::from_normalize(
            &[0xff; ED25519_EXPANDED_SECRET_KEY_SIZE],
            &[0xff; CHAIN_CODE_SIZE],
        );
        assert!(ed25519_bits_correct(&xprv));
        assert!(ed25519_bits_correct(&xprv.derive(0)));
    }

    #[test]
    fn test_derive_path() {
        let xprv = Xprv::<Sha512>::from_normalize(KEY, CHAIN_CODE);

        let derived = xprv.derive_path(&[]);
        assert_eq!(derived.0, xprv.0);

        let derived = xprv.derive_path(&[
            44 + HARDENED_OFFSET,
            HARDENED_OFFSET,
            HARDENED_OFFSET,
            1,
            10,
        ]);
        let expected_derived = xprv
            .derive(44 + HARDENED_OFFSET)
            .derive(HARDENED_OFFSET)
            .derive(HARDENED_OFFSET)
            .derive(1)
            .derive(10);
        assert_eq!(derived, expected_derived);
    }

    #[test]
    fn xpub_hard_derivation_fails() {
        assert!(Xprv::<Sha512>::from_normalize(KEY, CHAIN_CODE)
            .public()
            .derive(HARDENED_OFFSET - 1)
            .is_ok());
        assert!(Xprv::<Sha512>::from_normalize(KEY, CHAIN_CODE)
            .public()
            .derive(HARDENED_OFFSET)
            .is_err());
        assert!(Xprv::<Sha512>::from_normalize(KEY, CHAIN_CODE)
            .public()
            .derive(u32::MAX)
            .is_err());
    }

    #[test]
    fn ed25519_sign_verify() {
        let mut xprv = Xprv::<Sha512>::from_normalize(KEY, CHAIN_CODE)
            .derive(HARDENED_OFFSET)
            .derive(0);
        let sk = ed25519_dalek::hazmat::ExpandedSecretKey::from_bytes(&xprv.expanded_secret_key());
        let pk = ed25519_dalek::VerifyingKey::from(&sk);

        let signature = ed25519_dalek::hazmat::raw_sign::<Sha512>(&sk, b"message", &pk);
        assert_eq!(
            *b"\xe1\xe2\x87\xc0\xc3\x92\x66\x40\xfa\x1a\x30\xf6\x87\x76\x52\x8f\x0c\x3d\x1e\xf2\x5e\xc0\xf6\x18\x92\x50\xd9\x77\xbb\x83\x32\xd8\x22\x5d\xe9\x9f\x33\xb0\xdf\x77\x96\x48\xb9\x5b\x9f\xb2\x2b\xf3\x53\x66\x18\x8f\x17\x94\x53\x62\x87\xac\x86\x71\xba\xba\xd3\x02",
            signature.to_bytes(),
        );

        assert!(ed25519_dalek::Verifier::verify(&pk, b"message", &signature).is_ok());

        for index in 0..300 {
            xprv = xprv.derive(HARDENED_OFFSET + index);
            let sk =
                ed25519_dalek::hazmat::ExpandedSecretKey::from_bytes(&xprv.expanded_secret_key());
            let pk = ed25519_dalek::VerifyingKey::from(&sk);
            let signature = ed25519_dalek::hazmat::raw_sign::<Sha512>(&sk, b"message", &pk);
            assert!(ed25519_dalek::Verifier::verify(&pk, b"message", &signature).is_ok());
        }
    }
}
