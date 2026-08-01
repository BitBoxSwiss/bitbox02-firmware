// SPDX-License-Identifier: Apache-2.0

//! This module implements the X25519 trait needed by noise_protocol
//! by using the x25519_dalek crate. It is adapted from
//! https://github.com/sopium/noise-rust/blob/76fb694f06b429879c264087f496958a99710356/noise-rust-crypto/src/lib.rs#L31,
//! but uses the HAL random source to generate keys.

use bitbox_hal::Random;
use core::ops::Deref;
use noise_protocol::U8Array;
use noise_rust_crypto::sensitive::Sensitive;

pub struct X25519;

pub type PrivateKey = [u8; 32];
pub type PublicKey = [u8; 32];

/// Turn 32 bytes of entropy into an x25519 private key by applying the standard clamping.
///
/// The caller is responsible for the quality of `entropy`. For long-lived keys, prefer
/// `bitbox_core_utils::random::random_32_bytes()`, which mixes the MCU TRNG, the secure chip
/// TRNG and the factory randomness.
pub fn genkey_from_entropy(entropy: &[u8; 32]) -> Sensitive<PrivateKey> {
    let mut k: Sensitive<PrivateKey> = Sensitive::new();
    k.copy_from_slice(entropy);

    // Copied from: https://github.com/sopium/noise-rust/blob/76fb694f06b429879c264087f496958a99710356/noise-rust-crypto/src/lib.rs#L49-L51
    // which in turn copied it from:
    // https://github.com/dalek-cryptography/x25519-dalek/blob/ecd6be674850a99ad26404f6aa29b0cf79642b97/src/x25519.rs#L162-L164
    // which is also in our vendored deps: `vendor/x25519-dalek/src/x25519.rs`.
    k[0] &= 248;
    k[31] &= 127;
    k[31] |= 64;

    k
}

/// Generate a fresh x25519 private key by reading 32 random bytes from the MCU TRNG and applying
/// the standard clamping.
///
/// This is used for per-session ephemeral keys, where a synchronous, cheap source is wanted.
/// Long-lived keys should be built with `genkey_from_entropy()` instead.
pub fn genkey(random: &mut impl Random) -> Sensitive<PrivateKey> {
    let mut entropy: Sensitive<PrivateKey> = Sensitive::new();
    random.mcu_32_bytes(&mut entropy);
    genkey_from_entropy(&entropy)
}

impl noise_protocol::DH for X25519 {
    type Key = Sensitive<PrivateKey>;
    type Pubkey = PublicKey;
    type Output = [u8; 32];

    fn name() -> &'static str {
        "25519"
    }

    fn genkey() -> Self::Key {
        panic!("implicit X25519 key generation is unsupported; generate keys explicitly")
    }

    fn pubkey(k: &Self::Key) -> Self::Pubkey {
        let static_secret = x25519_dalek::StaticSecret::from(*k.deref());
        *x25519_dalek::PublicKey::from(&static_secret).as_bytes()
    }

    fn dh(k: &Self::Key, pk: &Self::Pubkey) -> Result<Self::Output, ()> {
        let k = x25519_dalek::StaticSecret::from(*k.deref());
        let pk = x25519_dalek::PublicKey::from(*pk);
        Ok(*k.diffie_hellman(&pk).as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ops::Deref;

    /// RFC 7748 clamping: bottom three bits cleared, top bit cleared, second-highest bit set.
    fn assert_clamped(k: &[u8; 32]) {
        assert_eq!(k[0] & 0b0000_0111, 0);
        assert_eq!(k[31] & 0b1000_0000, 0);
        assert_eq!(k[31] & 0b0100_0000, 0b0100_0000);
    }

    #[test]
    fn test_genkey_from_entropy_clamps() {
        for entropy in [[0x00u8; 32], [0xffu8; 32], [0xaau8; 32]] {
            assert_clamped(genkey_from_entropy(&entropy).deref());
        }
    }

    #[test]
    fn test_genkey_from_entropy_preserves_unclamped_bits() {
        // Only the clamped bits may differ from the supplied entropy.
        let entropy = [0xffu8; 32];
        let k = genkey_from_entropy(&entropy);
        assert_eq!(k[0], 0xff & 248);
        assert_eq!(k[31], (0xff & 127) | 64);
        for i in 1..31 {
            assert_eq!(k[i], entropy[i]);
        }
    }

    #[test]
    fn test_genkey_matches_genkey_from_entropy() {
        // genkey() must be exactly genkey_from_entropy() over the MCU TRNG, so that the
        // ephemeral-key path is unchanged by the refactor.
        struct FixedRandom([u8; 32]);
        impl Random for FixedRandom {
            fn factory_randomness(&mut self) -> &'static [u8; 32] {
                &[0; 32]
            }
            fn mcu_32_bytes(&mut self, out: &mut [u8; 32]) {
                for (o, v) in out.iter_mut().zip(self.0.iter()) {
                    *o ^= *v;
                }
            }
        }

        let entropy = [0x5au8; 32];
        let from_genkey = genkey(&mut FixedRandom(entropy));
        let from_entropy = genkey_from_entropy(&entropy);
        assert_eq!(from_genkey.deref(), from_entropy.deref());
    }
}
