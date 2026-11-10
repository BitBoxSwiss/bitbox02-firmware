// SPDX-License-Identifier: Apache-2.0

// Adapter derived from noise-rust-crypto 0.6.2, originally licensed under the Unlicense.
// https://github.com/sopium/noise-rust/blob/36bc85bd05db04b0d469843275ccafd848cddb99/noise-rust-crypto/src/lib.rs

use chacha20poly1305::{AeadInOut, KeyInit};
use noise_protocol::Cipher;

use crate::sensitive::Sensitive;

pub enum ChaCha20Poly1305 {}

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

        let tag = chacha20poly1305::ChaCha20Poly1305::new(&(**k).into())
            .encrypt_inout_detached(&full_nonce.into(), ad, in_out.into())
            .unwrap();

        tag_out.copy_from_slice(tag.as_ref());
    }

    fn encrypt_in_place(
        k: &Self::Key,
        nonce: u64,
        ad: &[u8],
        in_out: &mut [u8],
        plaintext_len: usize,
    ) -> usize {
        assert!(
            plaintext_len
                .checked_add(16)
                .is_some_and(|len| len <= in_out.len())
        );

        let mut full_nonce = [0u8; 12];
        full_nonce[4..].copy_from_slice(&nonce.to_le_bytes());

        let (in_out, tag_out) = in_out[..plaintext_len + 16].split_at_mut(plaintext_len);

        let tag = chacha20poly1305::ChaCha20Poly1305::new(&(**k).into())
            .encrypt_inout_detached(&full_nonce.into(), ad, in_out.into())
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

        chacha20poly1305::ChaCha20Poly1305::new(&(**k).into())
            .decrypt_inout_detached(&full_nonce.into(), ad, out.into(), tag.try_into().unwrap())
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

        chacha20poly1305::ChaCha20Poly1305::new(&(**k).into())
            .decrypt_inout_detached(
                &full_nonce.into(),
                ad,
                in_out.into(),
                tag.as_ref().try_into().unwrap(),
            )
            .map_err(|_| ())?;

        Ok(in_out.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_lit::hex;
    use noise_protocol::U8Array;

    const KEY: [u8; 32] = hex!("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    const NONCE: u64 = 0x0807060504030201;
    const AD: &[u8] = b"noise associated data";
    const PLAINTEXT: &[u8] = b"noise transport payload";
    // Generated independently with Python cryptography's ChaCha20Poly1305 using
    // the 96-bit nonce 000000000102030405060708.
    const CIPHERTEXT: [u8; 39] =
        hex!("8189cc8bc0acdcee71d19ba6e59e537ee5ed1d3d732c4c77e7124b0242f252da7de1b8cc4a2b90");

    #[test]
    fn test_encrypt_decrypt() {
        let key = Sensitive::from_slice(&KEY);
        let cases: &[(&[u8], &[u8])] = &[
            (b"", &hex!("a3bea751a894a825060dbce0756618d2")),
            (PLAINTEXT, &CIPHERTEXT),
        ];
        for &(plaintext, ciphertext) in cases {
            let mut encrypted = vec![0; ciphertext.len()];
            ChaCha20Poly1305::encrypt(&key, NONCE, AD, plaintext, &mut encrypted);
            assert_eq!(encrypted, ciphertext);

            let mut decrypted = vec![0; plaintext.len()];
            ChaCha20Poly1305::decrypt(&key, NONCE, AD, ciphertext, &mut decrypted).unwrap();
            assert_eq!(decrypted, plaintext);

            let mut in_out = vec![0xa5; ciphertext.len() + 8];
            in_out[..plaintext.len()].copy_from_slice(plaintext);
            assert_eq!(
                ChaCha20Poly1305::encrypt_in_place(&key, NONCE, AD, &mut in_out, plaintext.len()),
                ciphertext.len()
            );
            assert_eq!(&in_out[..ciphertext.len()], ciphertext);
            assert_eq!(&in_out[ciphertext.len()..], &[0xa5; 8]);
            assert_eq!(
                ChaCha20Poly1305::decrypt_in_place(&key, NONCE, AD, &mut in_out, ciphertext.len())
                    .unwrap(),
                plaintext.len()
            );
            assert_eq!(&in_out[..plaintext.len()], plaintext);
            assert_eq!(&in_out[ciphertext.len()..], &[0xa5; 8]);
        }
    }

    #[test]
    fn test_decrypt_rejects_tampering() {
        let key = Sensitive::from_slice(&KEY);
        let mut decrypted = vec![0; PLAINTEXT.len()];
        let mut changed_payload = CIPHERTEXT;
        changed_payload[0] ^= 1;
        let mut changed_tag = CIPHERTEXT;
        changed_tag[CIPHERTEXT.len() - 1] ^= 1;
        for (nonce, ad, ciphertext) in [
            (NONCE, AD, changed_payload),
            (NONCE, AD, changed_tag),
            (NONCE + 1, AD, CIPHERTEXT),
            (NONCE, b"changed".as_slice(), CIPHERTEXT),
        ] {
            assert!(
                ChaCha20Poly1305::decrypt(&key, nonce, ad, &ciphertext, &mut decrypted).is_err()
            );
            let mut in_out = ciphertext;
            assert!(
                ChaCha20Poly1305::decrypt_in_place(&key, nonce, ad, &mut in_out, ciphertext.len())
                    .is_err()
            );
        }
    }

    #[test]
    #[should_panic]
    fn test_decrypt_in_place_short_ciphertext() {
        let _ = ChaCha20Poly1305::decrypt_in_place(
            &Sensitive::from_slice(&KEY),
            NONCE,
            AD,
            &mut [0; 16],
            15,
        );
    }

    #[test]
    #[should_panic]
    fn test_encrypt_in_place_length_overflow() {
        ChaCha20Poly1305::encrypt_in_place(
            &Sensitive::from_slice(&KEY),
            NONCE,
            AD,
            &mut [0; 16],
            usize::MAX,
        );
    }
}
