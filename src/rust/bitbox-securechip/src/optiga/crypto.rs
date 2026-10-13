// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, vec};
use ccm::{
    Ccm, Nonce, Tag,
    aead::{AeadInPlace, KeyInit},
    consts::U8,
};
use util::{
    bytes::{Bytes, BytesMut},
    sha2::{hmac_sha256, hmac_sha256_overwrite},
};
use zeroize::{Zeroize, Zeroizing};

type Aes128Ccm = Ccm<bitbox_aes::Aes128, U8, U8>;

fn tls_prf_sha256(secret: &[u8], label: &[u8], seed: &[u8], out: &mut [u8]) -> bool {
    if label
        .len()
        .checked_add(seed.len())
        .is_none_or(|len| len > 96)
    {
        out.zeroize();
        return false;
    }
    let mut message = Zeroizing::new(vec![0; 32 + label.len() + seed.len()]);
    let (a, label_seed) = message.split_at_mut(32);
    label_seed[..label.len()].copy_from_slice(label);
    label_seed[label.len()..].copy_from_slice(seed);
    hmac_sha256(secret, label_seed, a.try_into().unwrap());
    let mut block = Box::new(Zeroizing::new([0; 32]));
    for chunk in out.chunks_mut(32) {
        hmac_sha256(secret, &message, &mut block);
        chunk.copy_from_slice(&block[..chunk.len()]);
        hmac_sha256_overwrite(secret, (&mut message[..32]).try_into().unwrap());
    }
    true
}

fn encrypt_aes128_ccm(key: &[u8], nonce: &[u8], associated_data: &[u8], data: &mut [u8]) -> bool {
    if nonce.len() != 8 || data.len() < 8 {
        return false;
    }
    let Ok(cipher) = Aes128Ccm::new_from_slice(key) else {
        return false;
    };
    let (plain, tag_out) = data.split_at_mut(data.len() - 8);
    let Ok(tag) =
        cipher.encrypt_in_place_detached(Nonce::<U8>::from_slice(nonce), associated_data, plain)
    else {
        return false;
    };
    tag_out.copy_from_slice(&tag);
    true
}

fn decrypt_aes128_ccm(
    key: &[u8],
    nonce: &[u8],
    associated_data: &[u8],
    tag: &[u8],
    data: &mut [u8],
) -> bool {
    if nonce.len() != 8 || tag.len() != 8 {
        return false;
    }
    let Ok(cipher) = Aes128Ccm::new_from_slice(key) else {
        return false;
    };
    if cipher
        .decrypt_in_place_detached(
            Nonce::<U8>::from_slice(nonce),
            associated_data,
            data,
            Tag::<U8>::from_slice(tag),
        )
        .is_err()
    {
        data.zeroize();
        return false;
    }
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_optiga_tls_prf_sha256(
    secret: Bytes,
    label: Bytes,
    seed: Bytes,
    mut out: BytesMut,
) -> bool {
    tls_prf_sha256(secret.as_ref(), label.as_ref(), seed.as_ref(), out.as_mut())
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_optiga_encrypt_aes128_ccm(
    key: Bytes,
    nonce: Bytes,
    associated_data: Bytes,
    mut data: BytesMut,
) -> bool {
    encrypt_aes128_ccm(
        key.as_ref(),
        nonce.as_ref(),
        associated_data.as_ref(),
        data.as_mut(),
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_optiga_decrypt_aes128_ccm(
    key: Bytes,
    nonce: Bytes,
    associated_data: Bytes,
    tag: Bytes,
    mut data: BytesMut,
) -> bool {
    decrypt_aes128_ccm(
        key.as_ref(),
        nonce.as_ref(),
        associated_data.as_ref(),
        tag.as_ref(),
        data.as_mut(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_lit::hex;

    #[test]
    fn test_tls_prf_sha256() {
        // TLS 1.2 PRF vector from Mbed TLS's test_suite_ssl.data.
        let secret = hex!("9bbe436ba940f017b17652849a71db35");
        let seed = hex!("a0ba9f936cda311827a6f796ffd5198c");
        let expected = hex!(
            "e3f229ba727be17b8d122620557cd453c2aab21d07c3d495329b52d4e61edb5a6b301791e90d35c9c9a46b4e14baf9af0fa022f7077def17abfd3797c0564bab4fbc91666e9def9b97fce34f796789baa48082d122ee42c5a72e5a5110fff70187347b66"
        );
        for len in [0, 1, 31, 32, 33, 40, 64, 100] {
            let mut out = vec![0; len];
            assert!(tls_prf_sha256(&secret, b"test label", &seed, &mut out));
            assert_eq!(out.as_slice(), &expected[..len]);
        }
    }

    #[test]
    fn test_tls_prf_sha256_invalid_length() {
        let mut out = [0xa5; 40];
        assert!(!tls_prf_sha256(&[0; 32], &[0; 65], &[0; 32], &mut out));
        assert_eq!(out, [0; 40]);
    }

    #[test]
    fn test_aes128_ccm() {
        // Checked independently with Python cryptography's AESCCM (8-byte nonce and tag).
        let key = hex!("c0c1c2c3c4c5c6c7c8c9cacbcccdcecf");
        let nonce = hex!("00000003020100a0");
        let associated_data = hex!("0001020304050607");
        let plain = hex!("08090a0b0c0d0e0f101112131415161718191a1b1c1d1e");
        let expected = hex!("0a8ab97677c1a82ccf49dc936e75a965d4ddf2088d6b94ab8139a9cbf2967a");
        let mut data = vec![0; plain.len() + 8];
        data[..plain.len()].copy_from_slice(&plain);
        assert!(encrypt_aes128_ccm(
            &key,
            &nonce,
            &associated_data,
            &mut data
        ));
        assert_eq!(data.as_slice(), expected);
        let (data, tag) = data.split_at_mut(plain.len());
        assert!(decrypt_aes128_ccm(
            &key,
            &nonce,
            &associated_data,
            tag,
            data
        ));
        assert_eq!(data, plain);
        data.copy_from_slice(&expected[..plain.len()]);
        tag[0] ^= 1;
        assert!(!decrypt_aes128_ccm(
            &key,
            &nonce,
            &associated_data,
            tag,
            data
        ));
        assert_eq!(data, [0; 23]);
    }

    #[test]
    fn test_aes128_ccm_invalid_lengths() {
        let mut data = [0; 16];
        assert!(!encrypt_aes128_ccm(&[0; 15], &[0; 8], &[], &mut data));
        assert!(!encrypt_aes128_ccm(&[0; 16], &[0; 7], &[], &mut data));
        assert!(!encrypt_aes128_ccm(&[0; 16], &[0; 8], &[], &mut data[..7]));
        assert!(!decrypt_aes128_ccm(
            &[0; 16],
            &[0; 8],
            &[],
            &[0; 7],
            &mut data
        ));
    }
}
