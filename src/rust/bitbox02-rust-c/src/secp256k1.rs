// SPDX-License-Identifier: Apache-2.0

use bitcoin::secp256k1::{
    Message, PublicKey,
    ffi::{self, CPtr},
};

unsafe extern "C" {
    fn secp256k1_selftest();
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_secp256k1_selftest() {
    // SAFETY: This function takes no arguments and either returns successfully or aborts.
    unsafe { secp256k1_selftest() }
}

/// Computes the attestation root identifier from the uncompressed SEC1 encoding, regardless of
/// the input encoding. Returns false for an invalid public key or an output buffer not 32 bytes long.
#[unsafe(no_mangle)]
pub extern "C" fn rust_secp256k1_pubkey_identifier(
    pubkey: util::bytes::Bytes,
    mut out: util::bytes::BytesMut,
) -> bool {
    let Ok(public_key) = PublicKey::from_slice(pubkey.as_ref()) else {
        return false;
    };
    let Ok(out) = out.as_mut().try_into() else {
        return false;
    };
    util::sha2::sha256(&public_key.serialize_uncompressed(), out);
    true
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_secp256k1_verify(
    signature_compact: util::bytes::Bytes,
    msg32: util::bytes::Bytes,
    pubkey: util::bytes::Bytes,
) -> bool {
    let Ok(signature) =
        bitcoin::secp256k1::ecdsa::Signature::from_compact(signature_compact.as_ref())
    else {
        return false;
    };
    let Ok(message) = Message::from_digest_slice(msg32.as_ref()) else {
        return false;
    };
    let Ok(public_key) = PublicKey::from_slice(pubkey.as_ref()) else {
        return false;
    };
    // Signature verification does not need a dynamically allocated signing context. Using the
    // static context also avoids linking the signing precomputation table into factory setup,
    // saving roughly 35 kB in the image.
    unsafe {
        ffi::secp256k1_ecdsa_verify(
            ffi::secp256k1_context_no_precomp,
            signature.as_c_ptr(),
            message.as_c_ptr(),
            public_key.as_c_ptr(),
        ) == 1
    }
}

#[cfg(test)]
mod tests {
    use super::{rust_secp256k1_pubkey_identifier, rust_secp256k1_verify};

    use bitcoin::secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

    fn pubkey_identifier(pubkey: &[u8], out: &mut [u8]) -> bool {
        rust_secp256k1_pubkey_identifier(
            unsafe { util::bytes::rust_util_bytes(pubkey.as_ptr(), pubkey.len()) },
            unsafe { util::bytes::rust_util_bytes_mut(out.as_mut_ptr(), out.len()) },
        )
    }

    #[test]
    fn test_rust_secp256k1_pubkey_identifier() {
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&[0x11u8; 32]).unwrap();
        let pk = PublicKey::from_secret_key(&secp, &sk);

        let mut expected = [0; 32];
        util::sha2::sha256(&pk.serialize_uncompressed(), &mut expected);
        let mut out = [0; 32];
        assert!(pubkey_identifier(&pk.serialize(), &mut out));
        assert_eq!(out, expected);
        assert!(pubkey_identifier(&pk.serialize_uncompressed(), &mut out));
        assert_eq!(out, expected);
    }

    #[test]
    fn test_rust_secp256k1_pubkey_identifier_invalid_pubkey() {
        let mut out = [0x55; 32];
        assert!(!pubkey_identifier(&[], &mut out));
        assert!(!pubkey_identifier(&[0; 33], &mut out));
        assert!(!pubkey_identifier(&[0; 65], &mut out));
        assert_eq!(out, [0x55; 32]);
    }

    #[test]
    fn test_rust_secp256k1_pubkey_identifier_invalid_output() {
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&[0x11u8; 32]).unwrap();
        let pk = PublicKey::from_secret_key(&secp, &sk);

        let mut out = [0x55; 33];
        assert!(!pubkey_identifier(&pk.serialize(), &mut out[..31]));
        assert!(!pubkey_identifier(&pk.serialize(), &mut out));
        assert_eq!(out, [0x55; 33]);
    }

    fn verify(signature_compact: &[u8], msg32: &[u8], pubkey: &[u8]) -> bool {
        rust_secp256k1_verify(
            unsafe {
                util::bytes::rust_util_bytes(signature_compact.as_ptr(), signature_compact.len())
            },
            unsafe { util::bytes::rust_util_bytes(msg32.as_ptr(), msg32.len()) },
            unsafe { util::bytes::rust_util_bytes(pubkey.as_ptr(), pubkey.len()) },
        )
    }

    #[test]
    fn test_rust_secp256k1_verify() {
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&[0x11u8; 32]).unwrap();
        let pk = PublicKey::from_secret_key(&secp, &sk);

        let msg32 = [0x22u8; 32];
        let msg = Message::from_digest_slice(&msg32).unwrap();
        let sig64 = secp.sign_ecdsa(&msg, &sk).serialize_compact();

        assert!(verify(&sig64, &msg32, &pk.serialize_uncompressed()));
        assert!(verify(&sig64, &msg32, &pk.serialize()));
    }

    #[test]
    fn test_rust_secp256k1_verify_invalid_signature() {
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&[0x11u8; 32]).unwrap();
        let pk = PublicKey::from_secret_key(&secp, &sk);

        let msg32 = [0x22u8; 32];
        let msg = Message::from_digest_slice(&msg32).unwrap();
        let sig64 = secp.sign_ecdsa(&msg, &sk).serialize_compact();

        assert!(!verify(&sig64[..63], &msg32, &pk.serialize_uncompressed()));
    }

    #[test]
    fn test_rust_secp256k1_verify_invalid_message() {
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&[0x11u8; 32]).unwrap();
        let pk = PublicKey::from_secret_key(&secp, &sk);

        let msg32 = [0x22u8; 32];
        let msg = Message::from_digest_slice(&msg32).unwrap();
        let sig64 = secp.sign_ecdsa(&msg, &sk).serialize_compact();

        assert!(!verify(&sig64, &msg32[..31], &pk.serialize_uncompressed()));
    }

    #[test]
    fn test_rust_secp256k1_verify_invalid_pubkey() {
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&[0x11u8; 32]).unwrap();

        let msg32 = [0x22u8; 32];
        let msg = Message::from_digest_slice(&msg32).unwrap();
        let sig64 = secp.sign_ecdsa(&msg, &sk).serialize_compact();

        assert!(!verify(&sig64, &msg32, &[0u8; 65]));
    }

    #[test]
    fn test_rust_secp256k1_verify_mismatch() {
        let secp = Secp256k1::new();
        let sk = SecretKey::from_slice(&[0x11u8; 32]).unwrap();
        let pk = PublicKey::from_secret_key(&secp, &sk);

        let msg32 = [0x22u8; 32];
        let msg = Message::from_digest_slice(&msg32).unwrap();
        let sig64 = secp.sign_ecdsa(&msg, &sk).serialize_compact();

        let mut other_msg32 = msg32;
        other_msg32[0] ^= 1;
        assert!(!verify(&sig64, &other_msg32, &pk.serialize_uncompressed()));
    }
}
