/// A trait for fixed size u8 array.

// Inspired by ArrayVec and SmallVec, but no unsafe.

// Use this trait so that we don't have to use [`Vec`] for some semi-fixed length buffers and
// input/output types.
pub trait U8Array: Sized {
    /// Create a new array filled with all zeros.
    fn new() -> Self;
    /// Create a new array filled with a same value.
    fn new_with(_: u8) -> Self;
    /// Create a new array from a slice.
    ///
    /// # Panics
    ///
    /// The slice must be of the same length.
    fn from_slice(_: &[u8]) -> Self;
    /// Length of the array.
    fn len() -> usize;
    /// As slice.
    fn as_slice(&self) -> &[u8];
    /// As mutable slice.
    fn as_mut(&mut self) -> &mut [u8];
    // Cannot just impl [`Clone`], that will conflict with [u8; 32].
    /// Clone.
    fn clone(&self) -> Self {
        Self::from_slice(self.as_slice())
    }
}

macro_rules! impl_array {
    ($len:expr) => {
        impl U8Array for [u8; $len] {
            fn new() -> Self {
                [0u8; $len]
            }
            fn new_with(x: u8) -> Self {
                [x; $len]
            }
            fn from_slice(data: &[u8]) -> Self {
                let mut a = [0u8; $len];
                a.copy_from_slice(data);
                a
            }
            fn len() -> usize {
                $len
            }
            fn as_slice(&self) -> &[u8] {
                self
            }
            fn as_mut(&mut self) -> &mut [u8] {
                self
            }
        }
    };
}

impl_array!(32);
impl_array!(64);
impl_array!(128);

/// A DH.
pub trait DH {
    /// Type of private key.
    type Key: U8Array;
    /// Type of pubkey key.
    type Pubkey: U8Array;
    /// Type of output.
    type Output: U8Array;

    /// Name of this DH function, e.g., “25519”.
    fn name() -> &'static str;

    /// Randomly generate a new private key.
    fn genkey() -> Self::Key;

    /// Calculate public key from a private key.
    fn pubkey(_: &Self::Key) -> Self::Pubkey;

    /// Perform DH key exchange.
    fn dh(_: &Self::Key, _: &Self::Pubkey) -> Result<Self::Output, ()>;
}

/// An AEAD.
pub trait Cipher {
    /// Name of this cipher function.
    fn name() -> &'static str;
    /// Type of key.
    type Key: U8Array;

    /// Length of key.
    fn key_len() -> usize {
        Self::Key::len()
    }

    /// Length of auth tag.
    ///
    /// All ciphers specified in the spec has tag length 16.
    fn tag_len() -> usize {
        16
    }

    /// AEAD encryption.
    ///
    /// # Panics
    ///
    /// If `out.len() != plaintext.len() + Self::tag_len()`
    fn encrypt(k: &Self::Key, nonce: u64, ad: &[u8], plaintext: &[u8], out: &mut [u8]);

    /// AEAD encryption, but encrypt on one buffer.
    /// return the length of ciphertext.
    ///
    /// # Panics
    ///
    /// If `in_out.len() < plaintext_len + Self::tag_len()`
    fn encrypt_in_place(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        in_out: &mut [u8],
        plaintext_len: usize,
    ) -> usize;

    /// AEAD decryption.
    ///
    /// # Panics
    ///
    /// If `out.len() != ciphertext.len() - Self::tag_len()`
    fn decrypt(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        ciphertext: &[u8],
        out: &mut [u8],
    ) -> Result<(), ()>;

    /// AEAD decryption, but decrypt on one buffer.
    /// return the length of plaintext.
    ///
    /// # Panics
    ///
    /// If `in_out.len() < ciphertext_len` or `ciphertext_len < Self::tag_len()`
    fn decrypt_in_place(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        in_out: &mut [u8],
        ciphertext_len: usize,
    ) -> Result<usize, ()>;

    /// Rekey. Returns a new cipher key as a pseudorandom function of `k`.
    fn rekey(k: &Self::Key) -> Self::Key {
        // XXX: `k1` is not zeroed.
        let mut k1 = [0u8; 48];
        Self::encrypt(k, 0u64.wrapping_sub(1), &[], &[0; 32], &mut k1);
        Self::Key::from_slice(&k1[..32])
    }
}

/// A hash function.
pub trait Hash: Default {
    /// Name of the hash function.
    fn name() -> &'static str;

    /// Type of a block.
    type Block: U8Array;
    /// Type of output.
    type Output: U8Array;

    /// Length of block.
    fn block_len() -> usize {
        Self::Block::len()
    }

    /// Length of hash output, in number of bytes.
    fn hash_len() -> usize {
        Self::Output::len()
    }

    /// Reset state of hash context.
    fn reset(&mut self) {
        *self = Default::default();
    }

    /// Update hash context with some input.
    fn input(&mut self, data: &[u8]);

    /// Get hash result.
    fn result(&mut self) -> Self::Output;

    /// Calculate hash of some data.
    fn hash(data: &[u8]) -> Self::Output {
        let mut h: Self = Default::default();
        h.input(data);
        h.result()
    }

    /// Calculate HMAC-THIS-HASH, with some `key` and several messages.
    fn hmac_many(key: &[u8], data: &[&[u8]]) -> Self::Output {
        assert!(key.len() <= Self::block_len());

        let mut ipad = Self::Block::new_with(0x36u8);
        let mut opad = Self::Block::new_with(0x5cu8);

        let ipad = ipad.as_mut();
        let opad = opad.as_mut();

        for (i, b) in key.iter().enumerate() {
            ipad[i] ^= b;
            opad[i] ^= b;
        }

        let mut hasher: Self = Default::default();
        hasher.input(ipad);
        for d in data {
            hasher.input(d);
        }
        let inner_output = hasher.result();

        hasher.reset();
        hasher.input(opad);
        hasher.input(inner_output.as_slice());
        hasher.result()
    }

    /// Calculate HMAC-THIS-HASH, with some `key` and a message.
    fn hmac(key: &[u8], data: &[u8]) -> Self::Output {
        Self::hmac_many(key, &[data])
    }

    /// Calculate HKDF, as specified in the noise spec.
    fn hkdf(chaining_key: &[u8], input_key_material: &[u8]) -> (Self::Output, Self::Output) {
        let temp_key = Self::hmac(chaining_key, input_key_material);
        let out1 = Self::hmac(temp_key.as_slice(), &[1u8]);
        let out2 = Self::hmac_many(temp_key.as_slice(), &[out1.as_slice(), &[2u8]]);
        (out1, out2)
    }

    /// Triple output HKDF.
    fn hkdf3(
        chaining_key: &[u8],
        input_key_material: &[u8],
    ) -> (Self::Output, Self::Output, Self::Output) {
        let temp_key = Self::hmac(chaining_key, input_key_material);
        let out1 = Self::hmac(temp_key.as_slice(), &[1u8]);
        let out2 = Self::hmac_many(temp_key.as_slice(), &[out1.as_slice(), &[2u8]]);
        let out3 = Self::hmac_many(temp_key.as_slice(), &[out2.as_slice(), &[3u8]]);
        (out1, out2, out3)
    }
}
