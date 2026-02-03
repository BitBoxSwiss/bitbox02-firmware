// SPDX-License-Identifier: Apache-2.0

use bitcoin::secp256k1::{Message, PublicKey};

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
    bitbox02_rust::secp256k1::SECP256K1
        .verify_ecdsa(&message, &signature, &public_key)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::rust_secp256k1_verify;

    use bitcoin::secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

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
