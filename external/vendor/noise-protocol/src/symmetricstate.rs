use crate::cipherstate::CipherState;
use crate::traits::{Cipher, Hash, U8Array};

pub struct SymmetricState<C: Cipher, H: Hash> {
    // Instead of `has_key`, use an `Option`.
    cipherstate: Option<CipherState<C>>,
    h: H::Output,
    ck: H::Output,
}

impl<C, H> Clone for SymmetricState<C, H>
where
    C: Cipher,
    H: Hash,
{
    fn clone(&self) -> Self {
        Self {
            cipherstate: self.cipherstate.clone(),
            h: self.h.clone(),
            ck: self.ck.clone(),
        }
    }
}

impl<C, H> SymmetricState<C, H>
where
    C: Cipher,
    H: Hash,
{
    /// Initialize a `SymmetricState` with a handshake name.
    pub fn new(handshake_name: &[u8]) -> SymmetricState<C, H> {
        let mut h = H::Output::new();

        if handshake_name.len() <= H::hash_len() {
            h.as_mut()[..handshake_name.len()].copy_from_slice(handshake_name);
        } else {
            h = H::hash(handshake_name);
        }

        SymmetricState {
            cipherstate: None,
            ck: h.clone(),
            h,
        }
    }

    pub fn mix_key(&mut self, data: &[u8]) {
        let (k1, k2) = H::hkdf(self.ck.as_slice(), data);
        self.ck = k1;
        self.cipherstate = Some(CipherState::new(&k2.as_slice()[..C::key_len()], 0));
    }

    pub fn mix_hash(&mut self, data: &[u8]) {
        let mut h: H = Default::default();
        h.input(self.h.as_slice());
        h.input(data);
        self.h = h.result();
    }

    pub fn mix_key_and_hash(&mut self, input_key_material: &[u8]) {
        let (ck, temp_h, temp_k) = H::hkdf3(self.ck.as_slice(), input_key_material);
        self.ck = ck;
        self.mix_hash(temp_h.as_slice());
        self.cipherstate = Some(CipherState::new(&temp_k.as_slice()[..C::key_len()], 0));
    }

    pub fn has_key(&self) -> bool {
        self.cipherstate.is_some()
    }

    pub fn encrypt_and_hash(&mut self, plaintext: &[u8], out: &mut [u8]) {
        if let Some(ref mut c) = self.cipherstate {
            c.encrypt_ad(self.h.as_slice(), plaintext, out);
        } else {
            out.copy_from_slice(plaintext);
        };
        self.mix_hash(out);
    }

    pub fn decrypt_and_hash(&mut self, data: &[u8], out: &mut [u8]) -> Result<(), ()> {
        if let Some(ref mut c) = self.cipherstate {
            c.decrypt_ad(self.h.as_slice(), data, out)?;
        } else {
            out.copy_from_slice(data)
        }
        self.mix_hash(data);
        Ok(())
    }

    pub fn split(&self) -> (CipherState<C>, CipherState<C>) {
        let (k1, k2) = H::hkdf(self.ck.as_slice(), &[]);
        let c1 = CipherState::new(&k1.as_slice()[..C::key_len()], 0);
        let c2 = CipherState::new(&k2.as_slice()[..C::key_len()], 0);
        (c1, c2)
    }

    pub fn get_hash(&self) -> &[u8] {
        self.h.as_slice()
    }
}
