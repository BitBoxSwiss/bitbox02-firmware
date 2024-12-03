// Copyright 2024 Shift Crypto AG
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

/// Derive the public key of a ECC NIST-P256 private key.
/// private_key must be 32 bytes, public_key_out must be 64 bytes.
#[no_mangle]
pub extern "C" fn rust_p256_pubkey(
    private_key: crate::util::Bytes,
    mut public_key_out: crate::util::BytesMut,
) {
    use p256::elliptic_curve::sec1::ToEncodedPoint;
    let secret_key = p256::SecretKey::from_slice(private_key.as_ref()).unwrap();
    let public_key = secret_key.public_key();
    let encoded_point = public_key.to_encoded_point(false);
    public_key_out.as_mut()[..32].copy_from_slice(encoded_point.x().unwrap());
    public_key_out.as_mut()[32..].copy_from_slice(encoded_point.y().unwrap());
}

/// Signs a msg using ECC-DSA NIST-P256.
/// private_key must be 32 bytes.
/// msg must be 32 bytes digest and is signed directly without further hashing.
/// sig_out must be 64 bytes.
#[no_mangle]
pub extern "C" fn rust_p256_sign(
    private_key: crate::util::Bytes,
    msg: crate::util::Bytes,
    mut sig_out: crate::util::BytesMut,
) {
    use p256::ecdsa::{signature::hazmat::PrehashSigner, Signature, SigningKey};
    let signing_key = SigningKey::from_slice(private_key.as_ref()).unwrap();
    let (signature, _): (Signature, _) = signing_key.sign_prehash(msg.as_ref()).unwrap();
    sig_out.as_mut().copy_from_slice(&signature.to_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_rust_p256_pubkey() {
        // Same in Python:
        // import ecdsa
        // privkey = bytes.fromhex("503e32eeb9cab8673f7847c047fa57ad2be0485d0759948413cc8c002b529fe4")
        // ecdsa.SigningKey.from_string(privkey, curve=ecdsa.curves.NIST256p).verifying_key.to_string().hex()

        let privkey = b"\x50\x3e\x32\xee\xb9\xca\xb8\x67\x3f\x78\x47\xc0\x47\xfa\x57\xad\x2b\xe0\x48\x5d\x07\x59\x94\x84\x13\xcc\x8c\x00\x2b\x52\x9f\xe4";
        let mut pubkey = [0u8; 64];
        rust_p256_pubkey(
            unsafe { crate::util::rust_util_bytes(privkey.as_ptr(), privkey.len()) },
            unsafe { crate::util::rust_util_bytes_mut(pubkey.as_mut_ptr(), pubkey.len()) },
        );
    }

    #[test]
    fn test_rust_p256_sign() {
        // Same in Python:
        // import ecdsa
        // privkey = bytes.fromhex("503e32eeb9cab8673f7847c047fa57ad2be0485d0759948413cc8c002b529fe4")
        // sig = bytes.fromhex("5a1f4c00a5104edd39fc8f55863b04a82c662c56448a10c3a35fd4d0decb9d8a8240cbcdd251c6d76d78981b14092c01466c90ad7e1c699f2c5e11523ec6df8c")
        // ecdsa.SigningKey.from_string(privkey, curve=ecdsa.curves.NIST256p).verifying_key.verify(sig, b"msg", hashfunc=hashlib.sha256)

        let privkey = b"\x50\x3e\x32\xee\xb9\xca\xb8\x67\x3f\x78\x47\xc0\x47\xfa\x57\xad\x2b\xe0\x48\x5d\x07\x59\x94\x84\x13\xcc\x8c\x00\x2b\x52\x9f\xe4";
        let msg = Sha256::digest(b"msg");
        let mut sig = [0u8; 64];
        rust_p256_sign(
            unsafe { crate::util::rust_util_bytes(privkey.as_ptr(), privkey.len()) },
            unsafe { crate::util::rust_util_bytes(msg.as_ptr(), msg.len()) },
            unsafe { crate::util::rust_util_bytes_mut(sig.as_mut_ptr(), sig.len()) },
        );
        assert_eq!(
            hex::encode(sig),
            "5a1f4c00a5104edd39fc8f55863b04a82c662c56448a10c3a35fd4d0decb9d8a8240cbcdd251c6d76d78981b14092c01466c90ad7e1c699f2c5e11523ec6df8c",
        );
    }
}
