// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

use bitcoin::secp256k1::{All, Secp256k1};

/// Length of a compressed secp256k1 pubkey.
const EC_PUBLIC_KEY_LEN: usize = 33;

pub struct SignResult {
    pub signature: [u8; 64],
    pub recid: u8,
}

pub fn _secp256k1_sign(
    secp: &Secp256k1<All>,
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_nonce: &[u8; 32],
) -> Result<SignResult, ()> {
    let mut signature = [0u8; 64];
    let mut recid: core::ffi::c_int = 0;
    match unsafe {
        bitbox02_sys::keystore_secp256k1_sign(
            secp.ctx().as_ptr().cast(),
            private_key.as_ptr(),
            msg.as_ptr(),
            host_nonce.as_ptr(),
            signature.as_mut_ptr(),
            &mut recid,
        )
    } {
        true => Ok(SignResult {
            signature,
            recid: recid.try_into().unwrap(),
        }),
        false => Err(()),
    }
}

pub fn _secp256k1_nonce_commit(
    secp: &Secp256k1<All>,
    private_key: &[u8; 32],
    msg: &[u8; 32],
    host_commitment: &[u8; 32],
) -> Result<[u8; EC_PUBLIC_KEY_LEN], ()> {
    let mut signer_commitment = [0u8; EC_PUBLIC_KEY_LEN];
    match unsafe {
        bitbox02_sys::keystore_secp256k1_nonce_commit(
            secp.ctx().as_ptr().cast(),
            private_key.as_ptr(),
            msg.as_ptr(),
            host_commitment.as_ptr(),
            signer_commitment.as_mut_ptr(),
        )
    } {
        true => Ok(signer_commitment),
        false => Err(()),
    }
}
