// SPDX-License-Identifier: Apache-2.0

pub use bitcoin::secp256k1::constants::PUBLIC_KEY_SIZE;
use bitcoin::secp256k1::{All, Secp256k1};

use core::cell::OnceCell;
use core::ops::Deref;

pub use bitbox02::secp256k1::SignResult;

#[derive(Debug, Copy, Clone)]
pub struct GlobalContext {
    __private: (), // prevents direct init
}

/// Global context, initialized once.
///
/// Port of https://docs.rs/secp256k1/latest/secp256k1/global/struct.GlobalContext.html to no_std.
pub static SECP256K1: &GlobalContext = &GlobalContext { __private: () };

struct SyncWrapper(OnceCell<Secp256k1<All>>);

// SAFETY: Embedded single-threaded use only, can't use from an interrupt context.
unsafe impl Sync for SyncWrapper {}

impl Deref for GlobalContext {
    type Target = Secp256k1<All>;

    fn deref(&self) -> &Self::Target {
        static CONTEXT: SyncWrapper = SyncWrapper(OnceCell::new());

        CONTEXT.0.get_or_init(|| {
            // Initialized on first access
            Secp256k1::new()
        })
    }
}

/// Sign message with private key using the given private key.
///
/// Details about `host_nonce`, the host nonce contribution.  Instead of using plain rfc6979 to
/// generate the nonce in this signature, the following formula is used:
///
///     r = rfc6979(..., additional_data=Hash_d(host_nonce))
///     R = r * G (pubkey to secret r)
///     nonce = r + Hash_p(R, host_nonce)
/// `Hash_d(msg)` and `Hash_p(msg)` are tagged hashes: `sha256(sha256(tag)||sha256(tag)||msg)`.
/// Tag for `Hash_d`: "s2c/ecdsa/data".
/// Tag for `Hash_p`: "s2c/ecdsa/point".
/// This is part of the ECDSA Anti-Klepto protocol, preventing this function to leak any secrets via
/// the signatures (see the ecdsa-s2c module in secp256k1-zpk for more details).
///
/// # Arguments
/// * `private_key` - 32 byte private key
/// * `msg` - 32 byte message to sign
/// * `host_nonce` - 32 byte nonce contribution. Cannot be NULL.
///   Intended to be a contribution by the host. If there is none available, use 32 zero bytes.
///
/// # Returns
/// * `Ok(SignResult)` containing signature in compact format and recoverable id on success
/// * `Err(())` on error.
pub fn secp256k1_sign(
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_nonce: &[u8; 32],
) -> Result<SignResult, ()> {
    bitbox02::secp256k1::_secp256k1_sign(SECP256K1, private_key, msg, host_nonce)
}

/// Get a commitment to the original nonce before tweaking it with the host nonce. This is part of
/// the ECDSA Anti-Klepto Protocol. For more details, check the docs of
/// `secp256k1_ecdsa_anti_exfil_signer_commit`.
///
/// # Arguments
/// * `private_key` - 32 byte private key
/// * `msg` - 32 byte message which will be signed by `secp256k1_sign`
/// * `host_commitment` - must be `sha256(sha256(tag)||sha256(tag)||host_nonce)` where
///   host_nonce is passed to `secp256k1_sign()`. See `secp256k1_ecdsa_anti_exfil_host_commit()`.
///
/// # Returns
/// * `Ok([u8; PUBLIC_KEY_SIZE])` - PUBLIC_KEY_SIZE bytes compressed signer nonce pubkey on success
/// * `Err(())` on failure
pub fn secp256k1_nonce_commit(
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_commitment: &[u8; 32],
) -> Result<[u8; PUBLIC_KEY_SIZE], ()> {
    bitbox02::secp256k1::_secp256k1_nonce_commit(SECP256K1, private_key, msg, host_commitment)
}

#[cfg(test)]
mod tests {
    use super::*;

    use hex_lit::hex;

    use bitcoin::secp256k1;

    #[test]
    fn test_secp256k1_sign() {
        let private_key = hex!("a2d8cf543c60d65162b5a06f0cef9760c883f8aa09f31236859faa85d0b74c7c");
        let msg = [0x88u8; 32];
        let host_nonce = [0x56u8; 32];

        let sign_result = secp256k1_sign(&private_key, &msg, &host_nonce).unwrap();

        // Verify signature against expected pubkey.

        let expected_pubkey = {
            let pubkey = hex!("023ffb4a4e41444d40e4e1e4c6cc329bcba2be50d0ef380aea19d490c373be58fb");
            secp256k1::PublicKey::from_slice(&pubkey).unwrap()
        };
        let msg = secp256k1::Message::from_digest_slice(&msg).unwrap();
        // Test recid by recovering the public key from the signature and checking against the
        // expected public key.
        let recoverable_sig = secp256k1::ecdsa::RecoverableSignature::from_compact(
            &sign_result.signature,
            secp256k1::ecdsa::RecoveryId::from_i32(sign_result.recid as i32).unwrap(),
        )
        .unwrap();

        let recovered_pubkey = SECP256K1.recover_ecdsa(&msg, &recoverable_sig).unwrap();
        assert_eq!(recovered_pubkey, expected_pubkey);

        // Verify signature.
        assert!(
            SECP256K1
                .verify_ecdsa(&msg, &recoverable_sig.to_standard(), &expected_pubkey)
                .is_ok()
        );
    }

    #[test]
    fn test_secp256k1_nonce_commit() {
        let private_key = hex!("a2d8cf543c60d65162b5a06f0cef9760c883f8aa09f31236859faa85d0b74c7c");
        let msg = [0x88u8; 32];
        let host_commitment = [0xabu8; 32];

        let client_commitment =
            secp256k1_nonce_commit(&private_key, &msg, &host_commitment).unwrap();
        assert_eq!(
            hex::encode(client_commitment),
            "0381e4136251c87f2947b735159c6dd644a7b58d35b437e20c878e5129f1320e5e",
        );
    }
}
