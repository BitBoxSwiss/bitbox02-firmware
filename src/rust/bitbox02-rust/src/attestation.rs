// Copyright 2020 Shift Cryptosecurity AG
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

use sha2::{Digest, Sha256};

pub struct Data {
    pub bootloader_hash: [u8; 32],
    pub device_pubkey: [u8; 64],
    pub certificate: [u8; 64],
    pub root_pubkey_identifier: [u8; 32],
    pub challenge_signature: [u8; 64],
}

pub fn perform(host_challenge: [u8; 32]) -> Result<Data, ()> {
    let mut result = Data {
        bootloader_hash: [0; 32],
        device_pubkey: [0; 64],
        certificate: [0; 64],
        root_pubkey_identifier: [0; 32],
        challenge_signature: [0; 64],
    };
    bitbox02::memory::get_attestation_pubkey_and_certificate(
        &mut result.device_pubkey,
        &mut result.certificate,
        &mut result.root_pubkey_identifier,
    )?;
    let hash: [u8; 32] = Sha256::digest(host_challenge).into();
    result.bootloader_hash = bitbox02::memory::get_attestation_bootloader_hash();
    bitbox02::securechip::attestation_sign(&hash, &mut result.challenge_signature)?;
    Ok(result)
}
