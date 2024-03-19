// Copyright 2020 Shift Cryptosecurity AG
// Copyright 2020 Shift Crypto AG
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

extern crate alloc;
use alloc::string::String;

// deduct one for the null terminator.
pub const DEVICE_NAME_MAX_LEN: usize = bitbox02_sys::MEMORY_DEVICE_NAME_MAX_LEN as usize - 1;

// deduct one for the null terminator.
pub const MULTISIG_NAME_MAX_LEN: usize = bitbox02_sys::MEMORY_MULTISIG_NAME_MAX_LEN as usize - 1;

pub use bitbox02_sys::memory_result_t as MemoryError;

#[derive(Debug)]
pub struct Error;

pub fn get_device_name() -> String {
    let mut name = [0u8; DEVICE_NAME_MAX_LEN + 1];
    unsafe { bitbox02_sys::memory_get_device_name(name.as_mut_ptr()) }
    crate::util::str_from_null_terminated(&name[..])
        .unwrap()
        .into()
}

pub fn set_device_name(name: &str) -> Result<(), Error> {
    match unsafe {
        bitbox02_sys::memory_set_device_name(
            crate::util::str_to_cstr_vec(name).or(Err(Error))?.as_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(Error),
    }
}

pub fn is_initialized() -> bool {
    unsafe { bitbox02_sys::memory_is_initialized() }
}

pub fn set_initialized() -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_set_initialized() } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn is_seeded() -> bool {
    unsafe { bitbox02_sys::memory_is_seeded() }
}

pub fn is_mnemonic_passphrase_enabled() -> bool {
    unsafe { bitbox02_sys::memory_is_mnemonic_passphrase_enabled() }
}

pub fn get_attestation_bootloader_hash() -> [u8; 32] {
    let mut hash = [0u8; 32];
    unsafe {
        bitbox02_sys::memory_get_attestation_bootloader_hash(hash.as_mut_ptr());
    }
    hash
}

pub fn get_attestation_pubkey_and_certificate(
    device_pubkey: &mut [u8; 64],
    certificate: &mut [u8; 64],
    root_pubkey_identifier: &mut [u8; 32],
) -> Result<(), ()> {
    match unsafe {
        bitbox02_sys::memory_get_attestation_pubkey_and_certificate(
            device_pubkey.as_mut_ptr(),
            certificate.as_mut_ptr(),
            root_pubkey_identifier.as_mut_ptr(),
        )
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn get_noise_static_private_key() -> Result<zeroize::Zeroizing<[u8; 32]>, ()> {
    let mut out = zeroize::Zeroizing::new([0u8; 32]);
    match unsafe { bitbox02_sys::memory_get_noise_static_private_key(out.as_mut_ptr()) } {
        true => Ok(out),
        false => Err(()),
    }
}

pub fn check_noise_remote_static_pubkey(pubkey: &[u8; 32]) -> bool {
    unsafe { bitbox02_sys::memory_check_noise_remote_static_pubkey(pubkey.as_ptr()) }
}

pub fn add_noise_remote_static_pubkey(pubkey: &[u8; 32]) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_add_noise_remote_static_pubkey(pubkey.as_ptr()) } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn set_mnemonic_passphrase_enabled(enabled: bool) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_set_mnemonic_passphrase_enabled(enabled) } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn set_seed_birthdate(timestamp: u32) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_set_seed_birthdate(timestamp) } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn get_seed_birthdate() -> u32 {
    let mut timestamp = core::mem::MaybeUninit::uninit();
    unsafe {
        bitbox02_sys::memory_get_seed_birthdate(timestamp.as_mut_ptr());
        timestamp.assume_init()
    }
}

pub fn multisig_set_by_hash(hash: &[u8], name: &str) -> Result<(), MemoryError> {
    if hash.len() != 32 {
        return Err(MemoryError::MEMORY_ERR_INVALID_INPUT);
    }
    match unsafe {
        bitbox02_sys::memory_multisig_set_by_hash(
            hash.as_ptr(),
            crate::util::str_to_cstr_vec(name)
                .or(Err(MemoryError::MEMORY_ERR_INVALID_INPUT))?
                .as_ptr(),
        )
    } {
        MemoryError::MEMORY_OK => Ok(()),
        err => Err(err),
    }
}

pub fn multisig_get_by_hash(hash: &[u8]) -> Option<String> {
    let mut name = [0u8; MULTISIG_NAME_MAX_LEN + 1];
    match unsafe { bitbox02_sys::memory_multisig_get_by_hash(hash.as_ptr(), name.as_mut_ptr()) } {
        true => Some(
            crate::util::str_from_null_terminated(&name[..])
                .unwrap()
                .into(),
        ),
        false => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_attestation_bootloader_hash() {
        let expected: [u8; 32] = *b"\x71\x3d\xf0\xd5\x8c\x71\x7d\x40\x31\x78\x7c\xdc\x8f\xa3\x5b\x90\x25\x82\xbe\x6a\xb6\xa2\x2e\x09\xde\x44\x77\xd3\x0e\x22\x30\xfc";
        assert_eq!(get_attestation_bootloader_hash(), expected);
    }
}
