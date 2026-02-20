// SPDX-License-Identifier: Apache-2.0

#![no_std]

extern crate alloc;

pub use bitcoin::secp256k1::constants::PUBLIC_KEY_SIZE;
use bitcoin::secp256k1::ffi::CPtr;
use bitcoin::secp256k1::{All, Secp256k1};

use alloc::vec::Vec;
use core::cell::OnceCell;
use core::ffi::c_int;
use core::mem::MaybeUninit;
use core::ops::Deref;

mod ffi {
    use bitcoin::secp256k1::ffi::{Context, PublicKey, Signature};
    use core::ffi::{c_int, c_uchar};

    #[allow(non_camel_case_types)]
    #[repr(C)]
    pub struct secp256k1_ecdsa_s2c_opening {
        pub data: [c_uchar; 64],
    }

    unsafe extern "C" {
        pub fn secp256k1_anti_exfil_sign(
            ctx: *const Context,
            sig: *mut Signature,
            msg32: *const c_uchar,
            seckey: *const c_uchar,
            host_data32: *const c_uchar,
            recid: *mut c_int,
        ) -> c_int;

        #[cfg(feature = "testing")]
        pub fn secp256k1_anti_exfil_host_verify(
            ctx: *const Context,
            sig: *const Signature,
            msg32: *const c_uchar,
            pubkey: *const PublicKey,
            host_data32: *const c_uchar,
            opening: *const secp256k1_ecdsa_s2c_opening,
        ) -> c_int;

        #[cfg(feature = "testing")]
        pub fn secp256k1_ecdsa_anti_exfil_host_commit(
            ctx: *const Context,
            rand_commitment32: *mut c_uchar,
            rand32: *const c_uchar,
        ) -> c_int;

        pub fn secp256k1_ecdsa_anti_exfil_signer_commit(
            ctx: *const Context,
            s2c_opening: *mut secp256k1_ecdsa_s2c_opening,
            msg32: *const c_uchar,
            seckey32: *const c_uchar,
            rand_commitment32: *const c_uchar,
        ) -> c_int;

        #[cfg(feature = "testing")]
        pub fn secp256k1_ecdsa_s2c_opening_parse(
            ctx: *const Context,
            opening: *mut secp256k1_ecdsa_s2c_opening,
            input33: *const c_uchar,
        ) -> c_int;

        pub fn secp256k1_ecdsa_s2c_opening_serialize(
            ctx: *const Context,
            output33: *mut c_uchar,
            opening: *const secp256k1_ecdsa_s2c_opening,
        ) -> c_int;

        pub fn bitbox_secp256k1_dleq_prove(
            ctx: *const Context,
            s: *mut c_uchar,
            e: *mut c_uchar,
            sk: *const c_uchar,
            gen2: *const PublicKey,
            p1: *const PublicKey,
            p2: *const PublicKey,
        ) -> c_int;

        pub fn bitbox_secp256k1_dleq_verify(
            ctx: *const Context,
            s: *const c_uchar,
            e: *const c_uchar,
            p1: *const PublicKey,
            gen2: *const PublicKey,
            p2: *const PublicKey,
        ) -> c_int;
    }
}

pub struct SignResult {
    pub signature: [u8; 64],
    pub recid: u8,
}

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
/// If `host_nonce` is `Some`, the host nonce contribution is mixed into the nonce derivation.
/// Instead of using plain rfc6979 to generate the nonce in this signature, the following formula
/// is used:
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
/// * `host_nonce` - optional 32 byte nonce contribution.
///   If `Some`, anti-exfil signing is used.
///   If `None`, regular deterministic ECDSA signing (without host nonce contribution) is used.
///
/// # Returns
/// * `Ok(SignResult)` containing signature in compact format and recoverable id on success
/// * `Err(())` on error.
pub fn secp256k1_sign(
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_nonce: Option<&[u8; 32]>,
) -> Result<SignResult, ()> {
    let Some(host_nonce) = host_nonce else {
        let message = bitcoin::secp256k1::Message::from_digest_slice(msg).map_err(|_| ())?;
        let secret_key = bitcoin::secp256k1::SecretKey::from_slice(private_key).map_err(|_| ())?;
        let recoverable_sig = SECP256K1.sign_ecdsa_recoverable(&message, &secret_key);
        let (recid, signature) = recoverable_sig.serialize_compact();
        return Ok(SignResult {
            signature,
            recid: recid.to_i32().try_into().unwrap(),
        });
    };

    let mut sig = MaybeUninit::<bitcoin::secp256k1::ffi::Signature>::uninit();
    let mut recid: c_int = 0;
    if unsafe {
        ffi::secp256k1_anti_exfil_sign(
            SECP256K1.ctx().as_ptr(),
            sig.as_mut_ptr(),
            msg.as_ptr(),
            private_key.as_ptr(),
            host_nonce.as_ptr(),
            &mut recid,
        )
    } != 1
    {
        return Err(());
    }

    let sig = bitcoin::secp256k1::ecdsa::Signature::from(unsafe { sig.assume_init() });
    let signature = sig.serialize_compact();
    Ok(SignResult {
        signature,
        recid: recid.try_into().unwrap(),
    })
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
    let mut signer_commitment = MaybeUninit::<ffi::secp256k1_ecdsa_s2c_opening>::uninit();
    if unsafe {
        ffi::secp256k1_ecdsa_anti_exfil_signer_commit(
            SECP256K1.ctx().as_ptr(),
            signer_commitment.as_mut_ptr(),
            msg.as_ptr(),
            private_key.as_ptr(),
            host_commitment.as_ptr(),
        )
    } != 1
    {
        return Err(());
    }

    let mut out = [0u8; PUBLIC_KEY_SIZE];
    if unsafe {
        ffi::secp256k1_ecdsa_s2c_opening_serialize(
            SECP256K1.ctx().as_ptr(),
            out.as_mut_ptr(),
            signer_commitment.as_ptr(),
        )
    } != 1
    {
        return Err(());
    }
    Ok(out)
}

#[cfg(feature = "testing")]
pub fn ecdsa_anti_exfil_host_commit(secp: &Secp256k1<All>, rand32: &[u8]) -> Result<Vec<u8>, ()> {
    let mut out = [0u8; 32];
    match unsafe {
        ffi::secp256k1_ecdsa_anti_exfil_host_commit(
            secp.ctx().as_ptr(),
            out.as_mut_ptr(),
            rand32.as_ptr(),
        )
    } {
        1 => Ok(out.to_vec()),
        _ => Err(()),
    }
}

#[cfg(feature = "testing")]
pub fn anti_exfil_host_verify(
    secp: &Secp256k1<All>,
    signature: &bitcoin::secp256k1::ecdsa::Signature,
    msg: &[u8; 32],
    pubkey: &bitcoin::secp256k1::PublicKey,
    host_nonce: &[u8; 32],
    signer_commitment: &[u8; 33],
) -> Result<(), ()> {
    let mut opening = MaybeUninit::<ffi::secp256k1_ecdsa_s2c_opening>::uninit();
    if unsafe {
        ffi::secp256k1_ecdsa_s2c_opening_parse(
            secp.ctx().as_ptr(),
            opening.as_mut_ptr(),
            signer_commitment.as_ptr(),
        )
    } != 1
    {
        return Err(());
    }
    let opening = unsafe { opening.assume_init() };
    if unsafe {
        ffi::secp256k1_anti_exfil_host_verify(
            secp.ctx().as_ptr(),
            signature.as_c_ptr(),
            msg.as_ptr(),
            pubkey.as_c_ptr(),
            host_nonce.as_ptr(),
            &opening,
        )
    } == 1
    {
        Ok(())
    } else {
        Err(())
    }
}

pub fn dleq_prove(
    secp: &Secp256k1<All>,
    sk: &[u8; 32],
    gen2: &bitcoin::secp256k1::PublicKey,
    p1: &bitcoin::secp256k1::PublicKey,
    p2: &bitcoin::secp256k1::PublicKey,
) -> Result<Vec<u8>, ()> {
    let mut s = [0u8; 32];
    let mut e = [0u8; 32];
    if unsafe {
        ffi::bitbox_secp256k1_dleq_prove(
            secp.ctx().as_ptr(),
            s.as_mut_ptr(),
            e.as_mut_ptr(),
            sk.as_ptr(),
            gen2.as_c_ptr(),
            p1.as_c_ptr(),
            p2.as_c_ptr(),
        )
    } == 1
    {
        let mut result = s.to_vec();
        result.extend(&e);
        Ok(result)
    } else {
        Err(())
    }
}

pub fn dleq_verify(
    secp: &Secp256k1<All>,
    proof: [u8; 64],
    gen2: &bitcoin::secp256k1::PublicKey,
    p1: &bitcoin::secp256k1::PublicKey,
    p2: &bitcoin::secp256k1::PublicKey,
) -> Result<(), ()> {
    if unsafe {
        ffi::bitbox_secp256k1_dleq_verify(
            secp.ctx().as_ptr(),
            proof[..32].as_ptr(),
            proof[32..].as_ptr(),
            p1.as_c_ptr(),
            gen2.as_c_ptr(),
            p2.as_c_ptr(),
        )
    } == 1
    {
        Ok(())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hex_lit::hex;

    use bitcoin::secp256k1;
    use bitcoin::secp256k1::{PublicKey, Secp256k1, SecretKey};

    #[test]
    fn test_dleq() {
        let secp = Secp256k1::new();
        let seckey_bytes = hex!("077eb75a52eca24cdedf058c92f1ca8b9d4841771fd6baa3d27885fb5b49fba2");
        let seckey = SecretKey::from_slice(&seckey_bytes).unwrap();

        let pubkey = seckey.public_key(&secp);

        let other_base_bytes =
            hex!("0389140f7bb852f020f154e55908fe3699dc9f65153e681527f0d55aabed937f4b");
        let other_base = PublicKey::from_slice(&other_base_bytes).unwrap();

        let other_pubkey = other_base;
        let other_pubkey = other_pubkey.mul_tweak(&secp, &seckey.into()).unwrap();
        let proof = dleq_prove(&secp, &seckey_bytes, &other_base, &pubkey, &other_pubkey).unwrap();
        // Check against fixture so potential upstream changes in the DLEQ implementation get
        // caught.  Incompatible changes can break BitBox client libraries that rely on this
        // specific DLEQ implementation.
        assert_eq!(
            proof,
            hex!("6c885f825f6ce7565bc6d0bfda90506b11e2682dfe943f5a85badf1c8a96edc5f5e03f5ee2c58bf979646fbada920f9f1c5bd92805fb5b01534b42d26a550f79")
                .to_vec(),
        );
        dleq_verify(
            &secp,
            proof.try_into().unwrap(),
            &other_base,
            &pubkey,
            &other_pubkey,
        )
        .unwrap();
    }

    #[test]
    fn test_secp256k1_sign() {
        let private_key = hex!("a2d8cf543c60d65162b5a06f0cef9760c883f8aa09f31236859faa85d0b74c7c");
        let msg = [0x88u8; 32];
        let host_nonce = [0x56u8; 32];

        let sign_result = secp256k1_sign(&private_key, &msg, Some(&host_nonce)).unwrap();

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
    fn test_secp256k1_sign_zero_host_nonce_differs_from_no_host_nonce_sign() {
        let private_key = hex!("a2d8cf543c60d65162b5a06f0cef9760c883f8aa09f31236859faa85d0b74c7c");
        let msg = [0x88u8; 32];
        let host_nonce = [0u8; 32];

        let anti_exfil_signature = secp256k1_sign(&private_key, &msg, Some(&host_nonce))
            .unwrap()
            .signature;

        let no_host_nonce_signature = secp256k1_sign(&private_key, &msg, None).unwrap().signature;

        assert_ne!(anti_exfil_signature, no_host_nonce_signature);
    }

    #[test]
    fn test_secp256k1_sign_no_host_nonce_deterministic() {
        let private_key = hex!("a2d8cf543c60d65162b5a06f0cef9760c883f8aa09f31236859faa85d0b74c7c");
        let msg = [0x88u8; 32];

        let sign_result_1 = secp256k1_sign(&private_key, &msg, None).unwrap();
        let sign_result_2 = secp256k1_sign(&private_key, &msg, None).unwrap();
        let message = secp256k1::Message::from_digest_slice(&msg).unwrap();
        let secret_key = SecretKey::from_slice(&private_key).unwrap();
        let expected_pubkey = secret_key.public_key(SECP256K1);
        let recoverable_sig = secp256k1::ecdsa::RecoverableSignature::from_compact(
            &sign_result_1.signature,
            secp256k1::ecdsa::RecoveryId::from_i32(sign_result_1.recid as i32).unwrap(),
        )
        .unwrap();

        assert_eq!(sign_result_1.signature, sign_result_2.signature);
        assert_eq!(
            sign_result_1.signature,
            hex!(
                "a58eaaad54d6af33e3844b1c59b70aa9a0ad5bb9e072e5d006a4cd3b27694fc12d38d8488c07586207bf0ade93af18fc7d28e0242df938ff1f495d489111192a"
            ),
        );
        assert_eq!(
            sign_result_1.signature,
            SECP256K1
                .sign_ecdsa(&message, &secret_key)
                .serialize_compact(),
        );
        assert_eq!(
            SECP256K1.recover_ecdsa(&message, &recoverable_sig).unwrap(),
            expected_pubkey,
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
            client_commitment,
            hex!("0381e4136251c87f2947b735159c6dd644a7b58d35b437e20c878e5129f1320e5e"),
        );
    }
}
