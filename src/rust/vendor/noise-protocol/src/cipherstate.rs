use crate::traits::{Cipher, U8Array};

#[cfg(feature = "use_alloc")]
use alloc::vec::Vec;

/// A `CipherState` can encrypt and decrypt data.
///
/// Mostly like `CipherState` in the spec, but must be created with a key.
///
/// # Panics
///
/// Encryption and decryption methods will panic if nonce reaches maximum u64, i.e., 2 ^ 64 - 1.
pub struct CipherState<C: Cipher> {
    key: C::Key,
    n: u64,
}

impl<C> Clone for CipherState<C>
where
    C: Cipher,
{
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            n: self.n,
        }
    }
}

impl<C> CipherState<C>
where
    C: Cipher,
{
    /// Name of cipher, e.g. “ChaChaPoly”.
    pub fn name() -> &'static str {
        C::name()
    }

    /// Create a new `CipherState` with a `key` and a nonce `n`.
    pub fn new(key: &[u8], n: u64) -> Self {
        CipherState {
            key: C::Key::from_slice(key),
            n,
        }
    }

    /// Rekey. Set our key to `REKEY(old key)`.
    pub fn rekey(&mut self) {
        self.key = C::rekey(&self.key);
    }

    /// AEAD encryption.
    pub fn encrypt_ad(&mut self, authtext: &[u8], plaintext: &[u8], out: &mut [u8]) {
        C::encrypt(&self.key, self.n, authtext, plaintext, out);
        #[cfg(feature = "use_std")]
        if option_env!("NOISE_RUST_TEST_IN_PLACE").is_some() {
            let mut inout = plaintext.to_vec();
            inout.extend_from_slice(&[0; 16]);
            let l = C::encrypt_in_place(&self.key, self.n, authtext, &mut inout, plaintext.len());
            assert_eq!(inout, out);
            assert_eq!(l, out.len());
        }
        // This will fail when n == 2 ^ 64 - 1, complying to the spec.
        self.n = self.n.checked_add(1).unwrap();
    }

    /// AEAD encryption in place.
    pub fn encrypt_ad_in_place(
        &mut self,
        authtext: &[u8],
        in_out: &mut [u8],
        plaintext_len: usize,
    ) -> usize {
        let size = C::encrypt_in_place(&self.key, self.n, authtext, in_out, plaintext_len);
        // This will fail when n == 2 ^ 64 - 1, complying to the spec.
        self.n = self.n.checked_add(1).unwrap();
        size
    }

    /// AEAD decryption.
    pub fn decrypt_ad(
        &mut self,
        authtext: &[u8],
        ciphertext: &[u8],
        out: &mut [u8],
    ) -> Result<(), ()> {
        let r = C::decrypt(&self.key, self.n, authtext, ciphertext, out);
        #[cfg(feature = "use_std")]
        if option_env!("NOISE_RUST_TEST_IN_PLACE").is_some() {
            let mut inout = ciphertext.to_vec();
            let r2 = C::decrypt_in_place(&self.key, self.n, authtext, &mut inout, ciphertext.len());
            assert_eq!(r.map(|_| out.len()), r2);
            if r.is_ok() {
                assert_eq!(&inout[..out.len()], out);
            }
        }
        r?;
        self.n = self.n.checked_add(1).unwrap();
        Ok(())
    }

    /// AEAD decryption in place.
    pub fn decrypt_ad_in_place(
        &mut self,
        authtext: &[u8],
        in_out: &mut [u8],
        ciphertext_len: usize,
    ) -> Result<usize, ()> {
        let size = C::decrypt_in_place(&self.key, self.n, authtext, in_out, ciphertext_len)?;
        self.n = self.n.checked_add(1).unwrap();
        Ok(size)
    }

    /// Encryption.
    pub fn encrypt(&mut self, plaintext: &[u8], out: &mut [u8]) {
        self.encrypt_ad(&[0u8; 0], plaintext, out)
    }

    /// Encryption in place.
    pub fn encrypt_in_place(&mut self, in_out: &mut [u8], plaintext_len: usize) -> usize {
        self.encrypt_ad_in_place(&[0u8; 0], in_out, plaintext_len)
    }

    /// Encryption, returns ciphertext as `Vec<u8>`.
    #[cfg(any(feature = "use_std", feature = "use_alloc"))]
    pub fn encrypt_vec(&mut self, plaintext: &[u8]) -> Vec<u8> {
        let mut out = vec![0u8; plaintext.len() + 16];
        self.encrypt(plaintext, &mut out);
        out
    }

    /// Decryption.
    pub fn decrypt(&mut self, ciphertext: &[u8], out: &mut [u8]) -> Result<(), ()> {
        self.decrypt_ad(&[0u8; 0], ciphertext, out)
    }

    /// Decryption in place.
    pub fn decrypt_in_place(
        &mut self,
        in_out: &mut [u8],
        ciphertext_len: usize,
    ) -> Result<usize, ()> {
        self.decrypt_ad_in_place(&[0u8; 0], in_out, ciphertext_len)
    }

    /// Decryption, returns plaintext as `Vec<u8>`.
    #[cfg(any(feature = "use_std", feature = "use_alloc"))]
    pub fn decrypt_vec(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>, ()> {
        if ciphertext.len() < 16 {
            return Err(());
        }
        let mut out = vec![0u8; ciphertext.len() - 16];
        self.decrypt(ciphertext, &mut out)?;
        Ok(out)
    }

    /// Get the next value of `n`. Could be used to decide on whether to re-key, etc.
    pub fn get_next_n(&self) -> u64 {
        self.n
    }

    /// Get underlying cipher and nonce.
    ///
    /// This is useful for e.g. WireGuard. Because packets may be lost or arrive out of order,
    /// they would likely want to deal with nonces themselves.
    pub fn extract(self) -> (C::Key, u64) {
        (self.key, self.n)
    }
}
