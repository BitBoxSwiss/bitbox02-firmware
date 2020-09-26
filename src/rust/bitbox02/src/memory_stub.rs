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

//! Stubs for testing.

pub fn set_device_name(name: &str) -> Result<(), ()> {
    let data = crate::testing::DATA.0.borrow();
    data.memory_set_device_name.as_ref().unwrap()(name)
}

pub fn is_initialized() -> bool {
    panic!("not implemented")
}

pub fn is_mnemonic_passphrase_enabled() -> bool {
    panic!("not implemented")
}

pub fn get_attestation_pubkey_and_certificate(
    _device_pubkey: &mut [u8; 64],
    _certificate: &mut [u8; 64],
    _root_pubkey_identifier: &mut [u8; 32],
) -> Result<(), ()> {
    panic!("not implemented")
}

pub fn bootloader_hash(_out: &mut [u8; 32]) {
    panic!("not implemented")
}

pub fn get_noise_static_private_key() -> Result<zeroize::Zeroizing<[u8; 32]>, ()> {
    panic!("not implemented")
}

pub fn check_noise_remote_static_pubkey(_pubkey: &[u8; 32]) -> bool {
    panic!("not implemented")
}

pub fn add_noise_remote_static_pubkey(_pubkey: &[u8; 32]) -> Result<(), ()> {
    panic!("not implemented")
}

pub fn set_mnemonic_passphrase_enabled(enabled: bool) -> Result<(), ()> {
    let data = crate::testing::DATA.0.borrow();
    data.memory_set_mnemonic_passphrase_enabled
        .as_ref()
        .unwrap()(enabled)
}
