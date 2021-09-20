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

pub struct Error;

pub fn get_device_name() -> String {
    let mut name = [0u8; DEVICE_NAME_MAX_LEN + 1];
    unsafe { bitbox02_sys::memory_get_device_name(name.as_mut_ptr()) }
    crate::util::str_from_null_terminated(&name[..])
        .unwrap()
        .into()
}

/// `name.as_bytes()` must be smaller or equal to
/// `DEVICE_NAME_MAX_LEN`, otherwise this function panics.
pub fn set_device_name(name: &str) -> Result<(), Error> {
    match unsafe {
        bitbox02_sys::memory_set_device_name(
            crate::str_to_cstr_force!(name, DEVICE_NAME_MAX_LEN).as_ptr(),
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

pub fn is_mnemonic_passphrase_enabled() -> bool {
    unsafe { bitbox02_sys::memory_is_mnemonic_passphrase_enabled() }
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

pub fn bootloader_hash(out: &mut [u8; 32]) {
    unsafe {
        bitbox02_sys::memory_bootloader_hash(out.as_mut_ptr());
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
