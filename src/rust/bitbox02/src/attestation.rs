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

pub use bitbox02_sys::PerformAttestationResponse;

/// Safelty wraps attestation_perform().
pub fn perform(host_challenge: [u8; 32]) -> Result<PerformAttestationResponse, ()> {
    let mut result = PerformAttestationResponse {
        bootloader_hash: [0; 32usize],
        device_pubkey: [0; 64usize],
        certificate: [0; 64usize],
        root_pubkey_identifier: [0; 32usize],
        challenge_signature: [0; 64usize],
    };
    match unsafe {
        bitbox02_sys::attestation_perform(host_challenge.as_ptr(), &mut result as *mut _)
    } {
        true => Ok(result),
        false => Err(()),
    }
}
