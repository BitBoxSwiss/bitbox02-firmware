//! This crate provides wrappers around pure rust implementations of the crypto
//! primitives used in `noise-protocol`.
//
//! The underlying implementations are:
//
//! * [`x25519-dalek`](https://crates.io/crates/x25519-dalek)
//! * [`chacha20poly1305`](https://crates.io/crates/chacha20poly1305)
//! * [`aes-gcm`](https://crates.io/crates/aes-gcm)
//! * [`sha2`](https://crates.io/crates/sha2)
//! * [`blake2`](https://crates.io/crates/blake2)

#![no_std]

pub mod sensitive;

use sensitive::Sensitive;

use noise_protocol::*;
#[cfg(feature = "x25519")]
use x25519_dalek::{PublicKey, StaticSecret};

#[cfg(feature = "x25519")]
pub enum X25519 {}

#[cfg(feature = "x25519")]
impl DH for X25519 {
    type Key = Sensitive<[u8; 32]>;
    type Pubkey = [u8; 32];
    type Output = Sensitive<[u8; 32]>;

    fn name() -> &'static str {
        "25519"
    }

    fn genkey() -> Self::Key {
        Self::Key::from_slice(StaticSecret::random().as_bytes())
    }

    fn pubkey(k: &Self::Key) -> Self::Pubkey {
        let static_secret = StaticSecret::from(**k);
        *PublicKey::from(&static_secret).as_bytes()
    }

    fn dh(k: &Self::Key, pk: &Self::Pubkey) -> Result<Self::Output, ()> {
        let k = StaticSecret::from(**k);
        let pk = PublicKey::from(*pk);
        Ok(Self::Output::from_slice(k.diffie_hellman(&pk).as_bytes()))
    }
}

#[cfg(feature = "use-chacha20poly1305")]
pub enum ChaCha20Poly1305 {}

#[cfg(feature = "use-chacha20poly1305")]
impl Cipher for ChaCha20Poly1305 {
    fn name() -> &'static str {
        "ChaChaPoly"
    }

    type Key = Sensitive<[u8; 32]>;

    fn encrypt(k: &Self::Key, nonce: u64, ad: &[u8], plaintext: &[u8], out: &mut [u8]) {
        assert!(plaintext.len().checked_add(16) == Some(out.len()));

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_le_bytes());

        let (in_out, tag_out) = out.split_at_mut(plaintext.len());
        in_out.copy_from_slice(plaintext);

        use chacha20poly1305::{AeadInPlace, KeyInit};
        let tag = chacha20poly1305::ChaCha20Poly1305::new(&(**k).into())
            .encrypt_in_place_detached(&full_nonce.into(), ad, in_out)
            .unwrap();

        tag_out.copy_from_slice(tag.as_ref())
    }

    fn encrypt_in_place(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        in_out: &mut [u8],
        plaintext_len: usize,
    ) -> usize {
        assert!(plaintext_len
            .checked_add(16)
            .map_or(false, |l| l <= in_out.len()));

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_le_bytes());

        let (in_out, tag_out) = in_out[..plaintext_len + 16].split_at_mut(plaintext_len);

        use chacha20poly1305::{AeadInPlace, KeyInit};
        let tag = chacha20poly1305::ChaCha20Poly1305::new(&(**k).into())
            .encrypt_in_place_detached(&full_nonce.into(), ad, in_out)
            .unwrap();
        tag_out.copy_from_slice(tag.as_ref());

        plaintext_len + 16
    }

    fn decrypt(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        ciphertext: &[u8],
        out: &mut [u8],
    ) -> Result<(), ()> {
        assert!(ciphertext.len().checked_sub(16) == Some(out.len()));

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_le_bytes());

        out.copy_from_slice(&ciphertext[..out.len()]);
        let tag = &ciphertext[out.len()..];

        use chacha20poly1305::{AeadInPlace, KeyInit};
        chacha20poly1305::ChaCha20Poly1305::new(&(**k).into())
            .decrypt_in_place_detached(&full_nonce.into(), ad, out, tag.into())
            .map_err(|_| ())
    }

    fn decrypt_in_place(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        in_out: &mut [u8],
        ciphertext_len: usize,
    ) -> Result<usize, ()> {
        assert!(ciphertext_len <= in_out.len());
        assert!(ciphertext_len >= 16);

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_le_bytes());

        let (in_out, tag) = in_out[..ciphertext_len].split_at_mut(ciphertext_len - 16);

        use chacha20poly1305::{AeadInPlace, KeyInit};
        chacha20poly1305::ChaCha20Poly1305::new(&(**k).into())
            .decrypt_in_place_detached(&full_nonce.into(), ad, in_out, tag.as_ref().into())
            .map_err(|_| ())?;

        Ok(in_out.len())
    }
}

#[cfg(feature = "use-aes-256-gcm")]
pub enum Aes256Gcm {}

#[cfg(feature = "use-aes-256-gcm")]
impl Cipher for Aes256Gcm {
    fn name() -> &'static str {
        "AESGCM"
    }

    type Key = Sensitive<[u8; 32]>;

    fn encrypt(k: &Self::Key, nonce: u64, ad: &[u8], plaintext: &[u8], out: &mut [u8]) {
        assert!(plaintext.len().checked_add(16) == Some(out.len()));

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_be_bytes());

        let (in_out, tag_out) = out.split_at_mut(plaintext.len());
        in_out.copy_from_slice(plaintext);

        use aes_gcm::{AeadInPlace, KeyInit};
        let tag = aes_gcm::Aes256Gcm::new(&(**k).into())
            .encrypt_in_place_detached(&full_nonce.into(), ad, in_out)
            .unwrap();

        tag_out.copy_from_slice(tag.as_ref())
    }

    fn encrypt_in_place(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        in_out: &mut [u8],
        plaintext_len: usize,
    ) -> usize {
        assert!(plaintext_len
            .checked_add(16)
            .map_or(false, |l| l <= in_out.len()));

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_be_bytes());

        let (in_out, tag_out) = in_out[..plaintext_len + 16].split_at_mut(plaintext_len);

        use aes_gcm::{AeadInPlace, KeyInit};
        let tag = aes_gcm::Aes256Gcm::new(&(**k).into())
            .encrypt_in_place_detached(&full_nonce.into(), ad, in_out)
            .unwrap();
        tag_out.copy_from_slice(tag.as_ref());

        plaintext_len + 16
    }

    fn decrypt(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        ciphertext: &[u8],
        out: &mut [u8],
    ) -> Result<(), ()> {
        assert!(ciphertext.len().checked_sub(16) == Some(out.len()));

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_be_bytes());

        out.copy_from_slice(&ciphertext[..out.len()]);
        let tag = &ciphertext[out.len()..];

        use aes_gcm::{AeadInPlace, KeyInit};
        aes_gcm::Aes256Gcm::new(&(**k).into())
            .decrypt_in_place_detached(&full_nonce.into(), ad, out, tag.into())
            .map_err(|_| ())
    }

    fn decrypt_in_place(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        in_out: &mut [u8],
        ciphertext_len: usize,
    ) -> Result<usize, ()> {
        assert!(ciphertext_len <= in_out.len());
        assert!(ciphertext_len >= 16);

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_be_bytes());

        let (in_out, tag) = in_out[..ciphertext_len].split_at_mut(ciphertext_len - 16);

        use aes_gcm::{AeadInPlace, KeyInit};
        aes_gcm::Aes256Gcm::new(&(**k).into())
            .decrypt_in_place_detached(&full_nonce.into(), ad, in_out, tag.as_ref().into())
            .map_err(|_| ())?;

        Ok(in_out.len())
    }
}

#[cfg(feature = "use-sha2")]
#[derive(Default, Clone)]
pub struct Sha256(sha2::Sha256);

#[cfg(feature = "use-sha2")]
impl Hash for Sha256 {
    fn name() -> &'static str {
        "SHA256"
    }

    type Block = [u8; 64];
    type Output = Sensitive<[u8; 32]>;

    fn input(&mut self, data: &[u8]) {
        use sha2::Digest;
        self.0.update(data);
    }

    fn result(&mut self) -> Self::Output {
        use sha2::Digest;
        Self::Output::from_slice(self.0.finalize_reset().as_ref())
    }
}

#[cfg(feature = "use-sha2")]
#[derive(Default, Clone)]
pub struct Sha512(sha2::Sha512);

#[cfg(feature = "use-sha2")]
impl Hash for Sha512 {
    fn name() -> &'static str {
        "SHA512"
    }

    type Block = [u8; 128];
    type Output = Sensitive<[u8; 64]>;

    fn input(&mut self, data: &[u8]) {
        use sha2::Digest;
        self.0.update(data);
    }

    fn result(&mut self) -> Self::Output {
        use sha2::Digest;
        Self::Output::from_slice(self.0.finalize_reset().as_ref())
    }
}

#[cfg(feature = "use-blake2")]
#[derive(Default, Clone)]
pub struct Blake2s(blake2::Blake2s256);

#[cfg(feature = "use-blake2")]
impl Hash for Blake2s {
    fn name() -> &'static str {
        "BLAKE2s"
    }

    type Block = [u8; 64];
    type Output = Sensitive<[u8; 32]>;

    fn input(&mut self, data: &[u8]) {
        use blake2::Digest;
        self.0.update(data);
    }

    fn result(&mut self) -> Self::Output {
        use blake2::Digest;
        Self::Output::from_slice(self.0.finalize_reset().as_ref())
    }
}

#[cfg(feature = "use-blake2")]
#[derive(Default, Clone)]
pub struct Blake2b(blake2::Blake2b512);

#[cfg(feature = "use-blake2")]
impl Hash for Blake2b {
    fn name() -> &'static str {
        "BLAKE2b"
    }

    type Block = [u8; 128];
    type Output = Sensitive<[u8; 64]>;

    fn input(&mut self, data: &[u8]) {
        use blake2::Digest;
        self.0.update(data);
    }

    fn result(&mut self) -> Self::Output {
        use blake2::Digest;
        Self::Output::from_slice(self.0.finalize_reset().as_ref())
    }
}
