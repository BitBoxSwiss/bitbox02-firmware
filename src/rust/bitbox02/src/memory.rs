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
use alloc::vec::Vec;

pub const MAX_UNLOCK_ATTEMPTS: u8 = bitbox02_sys::MAX_UNLOCK_ATTEMPTS as u8;

// deduct one for the null terminator.
pub const DEVICE_NAME_MAX_LEN: usize = bitbox02_sys::MEMORY_DEVICE_NAME_MAX_LEN as usize - 1;

// deduct one for the null terminator.
pub const MULTISIG_NAME_MAX_LEN: usize = bitbox02_sys::MEMORY_MULTISIG_NAME_MAX_LEN as usize - 1;

pub use bitbox02_sys::memory_ble_metadata_t as BleMetadata;

pub use bitbox02_sys::memory_result_t as MemoryError;

#[derive(Debug)]
pub struct Error;

pub fn get_device_name() -> String {
    let mut name = [0u8; DEVICE_NAME_MAX_LEN + 1];
    unsafe { bitbox02_sys::memory_get_device_name(name.as_mut_ptr().cast()) }
    crate::util::str_from_null_terminated(&name[..])
        .unwrap()
        .into()
}

pub fn set_device_name(name: &str) -> Result<(), Error> {
    match unsafe {
        bitbox02_sys::memory_set_device_name(
            crate::util::str_to_cstr_vec(name)
                .or(Err(Error))?
                .as_ptr()
                .cast(),
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

pub fn get_encrypted_seed_and_hmac() -> Result<alloc::vec::Vec<u8>, ()> {
    let mut out = vec![0u8; 96];
    let mut len = 0u8;
    match unsafe { bitbox02_sys::memory_get_encrypted_seed_and_hmac(out.as_mut_ptr(), &mut len) } {
        true => {
            out.truncate(len as _);
            Ok(out)
        }
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

pub fn set_encrypted_seed_and_hmac(data: &[u8]) -> Result<(), ()> {
    if data.len() > u8::MAX as usize {
        return Err(());
    }
    match unsafe {
        bitbox02_sys::memory_set_encrypted_seed_and_hmac(data.as_ptr(), data.len() as u8)
    } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn reset_hww() -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_reset_hww() } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn smarteeprom_get_unlock_attempts() -> u8 {
    unsafe { bitbox02_sys::bitbox02_smarteeprom_get_unlock_attempts() }
}

#[cfg(feature = "testing")]
pub fn smarteeprom_increment_unlock_attempts() {
    unsafe {
        bitbox02_sys::bitbox02_smarteeprom_increment_unlock_attempts();
    }
}

#[cfg(feature = "testing")]
pub fn smarteeprom_reset_unlock_attempts() {
    unsafe {
        bitbox02_sys::bitbox02_smarteeprom_reset_unlock_attempts();
    }
}

/// Testing helper to set the recorded number of unlock attempts to a precise value.
/// Panics if `attempts` exceeds the maximum supported by the firmware.
#[cfg(feature = "testing")]
pub fn set_unlock_attempts_for_testing(attempts: u8) {
    assert!(attempts <= MAX_UNLOCK_ATTEMPTS);
    smarteeprom_reset_unlock_attempts();
    for _ in 0..attempts {
        smarteeprom_increment_unlock_attempts();
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
                .as_ptr()
                .cast(),
        )
    } {
        MemoryError::MEMORY_OK => Ok(()),
        err => Err(err),
    }
}

pub fn multisig_get_by_hash(hash: &[u8]) -> Option<String> {
    let mut name = [0u8; MULTISIG_NAME_MAX_LEN + 1];
    match unsafe {
        bitbox02_sys::memory_multisig_get_by_hash(hash.as_ptr(), name.as_mut_ptr().cast())
    } {
        true => Some(
            crate::util::str_from_null_terminated(&name[..])
                .unwrap()
                .into(),
        ),
        false => None,
    }
}

pub enum Platform {
    BitBox02,
    BitBox02Plus,
}

pub fn get_platform() -> Result<Platform, ()> {
    match unsafe { bitbox02_sys::memory_get_platform() as u32 } {
        bitbox02_sys::MEMORY_PLATFORM_BITBOX02 => Ok(Platform::BitBox02),
        bitbox02_sys::MEMORY_PLATFORM_BITBOX02_PLUS => Ok(Platform::BitBox02Plus),
        _ => Err(()),
    }
}

pub enum SecurechipType {
    Atecc,
    Optiga,
}

pub fn get_securechip_type() -> Result<SecurechipType, ()> {
    match unsafe { bitbox02_sys::memory_get_securechip_type() as u32 } {
        bitbox02_sys::MEMORY_SECURECHIP_TYPE_ATECC => Ok(SecurechipType::Atecc),
        bitbox02_sys::MEMORY_SECURECHIP_TYPE_OPTIGA => Ok(SecurechipType::Optiga),
        _ => Err(()),
    }
}

pub fn get_ble_metadata() -> BleMetadata {
    let mut metadata = core::mem::MaybeUninit::uninit();

    unsafe {
        bitbox02_sys::memory_get_ble_metadata(metadata.as_mut_ptr());
        metadata.assume_init()
    }
}

pub fn set_ble_metadata(metadata: &BleMetadata) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_set_ble_metadata(metadata) } {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn ble_enabled() -> bool {
    unsafe { bitbox02_sys::memory_ble_enabled() }
}

pub fn ble_enable(enable: bool) -> Result<(), ()> {
    let res = unsafe { bitbox02_sys::memory_ble_enable(enable) };
    if res { Ok(()) } else { Err(()) }
}

pub fn get_salt_root() -> Result<zeroize::Zeroizing<Vec<u8>>, ()> {
    let mut salt_root = zeroize::Zeroizing::new(vec![0u8; 32]);
    if unsafe { bitbox02_sys::memory_get_salt_root(salt_root.as_mut_ptr()) } {
        Ok(salt_root)
    } else {
        Err(())
    }
}

#[cfg(feature = "testing")]
pub fn set_salt_root(salt_root: &[u8; 32]) -> Result<(), ()> {
    match unsafe { bitbox02_sys::memory_set_salt_root(salt_root.as_ptr()) } {
        true => Ok(()),
        false => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use hex_lit::hex;

    #[test]
    fn test_get_attestation_bootloader_hash() {
        let expected: [u8; 32] =
            hex!("713df0d58c717d4031787cdc8fa35b902582be6ab6a22e09de4477d30e2230fc");
        assert_eq!(get_attestation_bootloader_hash(), expected);
    }

    #[test]
    fn test_get_salt_root_roundtrip() {
        let original = get_salt_root().unwrap();

        let expected = hex!("00112233445566778899aabbccddeefffeeddccbbaa998877665544332211000");

        set_salt_root(expected.as_slice().try_into().unwrap()).unwrap();
        let salt_root = get_salt_root().unwrap();
        assert_eq!(salt_root.as_slice(), &expected);

        let erased = [0xffu8; 32];
        set_salt_root(&erased).unwrap();
        assert!(get_salt_root().is_err());

        set_salt_root(original.as_slice().try_into().unwrap()).unwrap();
    }
}
