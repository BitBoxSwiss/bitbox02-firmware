// Copyright 2022-2024 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use bitcoin::secp256k1::ffi::CPtr;

use alloc::vec::Vec;

pub fn ecdsa_anti_exfil_host_commit(rand32: &[u8]) -> Result<Vec<u8>, ()> {
    let mut out = [0u8; 32];
    match unsafe {
        bitbox02_sys::secp256k1_ecdsa_anti_exfil_host_commit(
            bitbox02_sys::wally_get_secp_context(),
            out.as_mut_ptr(),
            rand32.as_ptr(),
        )
    } {
        1 => Ok(out.to_vec()),
        _ => Err(()),
    }
}

pub fn dleq_prove(
    sk: &[u8; 32],
    gen2: &bitcoin::secp256k1::PublicKey,
    p1: &bitcoin::secp256k1::PublicKey,
    p2: &bitcoin::secp256k1::PublicKey,
) -> Result<Vec<u8>, ()> {
    let mut s = [0u8; 32];
    let mut e = [0u8; 32];
    let result = unsafe {
        bitbox02_sys::bitbox_secp256k1_dleq_prove(
            bitbox02_sys::wally_get_secp_context(),
            s.as_mut_ptr(),
            e.as_mut_ptr(),
            sk.as_ptr(),
            gen2.as_c_ptr() as _,
            p1.as_c_ptr() as _,
            p2.as_c_ptr() as _,
        )
    };
    if result == 1 {
        let mut result = s.to_vec();
        result.extend(&e);
        Ok(result)
    } else {
        Err(())
    }
}

pub fn dleq_verify(
    proof: [u8; 64],
    gen2: &bitcoin::secp256k1::PublicKey,
    p1: &bitcoin::secp256k1::PublicKey,
    p2: &bitcoin::secp256k1::PublicKey,
) -> Result<(), ()> {
    let result = unsafe {
        bitbox02_sys::bitbox_secp256k1_dleq_verify(
            bitbox02_sys::wally_get_secp_context(),
            proof[..32].as_ptr(),
            proof[32..].as_ptr(),
            p1.as_c_ptr() as _,
            gen2.as_c_ptr() as _,
            p2.as_c_ptr() as _,
        )
    };
    if result == 1 {
        Ok(())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::secp256k1::{PublicKey, Secp256k1, SecretKey};

    #[test]
    fn test_dleq() {
        let secp = Secp256k1::new();
        let seckey_bytes = b"\x07\x7e\xb7\x5a\x52\xec\xa2\x4c\xde\xdf\x05\x8c\x92\xf1\xca\x8b\x9d\x48\x41\x77\x1f\xd6\xba\xa3\xd2\x78\x85\xfb\x5b\x49\xfb\xa2";
        let seckey = SecretKey::from_slice(seckey_bytes).unwrap();

        let pubkey = seckey.public_key(&secp);

        let other_base_bytes = b"\x03\x89\x14\x0f\x7b\xb8\x52\xf0\x20\xf1\x54\xe5\x59\x08\xfe\x36\x99\xdc\x9f\x65\x15\x3e\x68\x15\x27\xf0\xd5\x5a\xab\xed\x93\x7f\x4b";
        let other_base = PublicKey::from_slice(other_base_bytes).unwrap();

        let other_pubkey = other_base;
        let other_pubkey = other_pubkey.mul_tweak(&secp, &seckey.into()).unwrap();
        let proof = dleq_prove(seckey_bytes, &other_base, &pubkey, &other_pubkey).unwrap();
        // Check against fixture so potential upstream changes in the DLEQ implementation get
        // caught.  Incompatible changes can break BitBox client libraries that rely on this
        // specific DLEQ implementation.
        assert_eq!(
            hex::encode(&proof),
            "6c885f825f6ce7565bc6d0bfda90506b11e2682dfe943f5a85badf1c8a96edc5f5e03f5ee2c58bf979646fbada920f9f1c5bd92805fb5b01534b42d26a550f79",
        );
        dleq_verify(
            proof.try_into().unwrap(),
            &other_base,
            &pubkey,
            &other_pubkey,
        )
        .unwrap();
    }
}
