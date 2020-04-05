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

extern crate alloc;
use alloc::vec::Vec;

const OP_UNLOCK: u8 = b'u';
const OP_ATTESTATION: u8 = b'a';

const OP_STATUS_SUCCESS: u8 = 0;
const OP_STATUS_FAILURE: u8 = 1;

/// Process OP_UNLOCK.
async fn api_unlock() -> Vec<u8> {
    match crate::workflow::unlock::unlock().await {
        Ok(()) => [OP_STATUS_SUCCESS].to_vec(),
        Err(()) => [OP_STATUS_FAILURE].to_vec(),
    }
}

/// Process OP_ATTESTATION.
///
/// On failure, returns < 1 >.
///
/// On success, returns < 0 | bootloader_hash 32 | device_pubkey 64 |
/// certificate 64 | root_pubkey_identifier 32 | challenge_signature 64>
fn api_attestation(usb_in: &[u8]) -> Vec<u8> {
    use core::convert::TryInto;

    let usb_in: [u8; 32] = match usb_in.try_into() {
        Ok(usb_in) => usb_in,
        Err(_) => return [OP_STATUS_FAILURE].to_vec(),
    };

    let result = match crate::attestation::perform(usb_in) {
        Ok(result) => result,
        Err(()) => return [OP_STATUS_FAILURE].to_vec(),
    };

    let mut out = Vec::with_capacity(257);
    out.push(OP_STATUS_SUCCESS);
    out.extend_from_slice(&result.bootloader_hash[..]);
    out.extend_from_slice(&result.device_pubkey[..]);
    out.extend_from_slice(&result.certificate[..]);
    out.extend_from_slice(&result.root_pubkey_identifier[..]);
    out.extend_from_slice(&result.challenge_signature[..]);
    out
}

/// Async HWW api processing main entry point.
/// `usb_in` - api request bytes.
/// Returns the usb response bytes.
pub async fn process_packet(usb_in: Vec<u8>) -> Vec<u8> {
    match usb_in.split_first() {
        Some((&OP_UNLOCK, b"")) => return api_unlock().await,
        Some((&OP_ATTESTATION, rest)) => return api_attestation(rest),
        _ => (),
    }

    // -- Process anything not ported to Rust yet. --
    // This function is blocking.
    bitbox02::hww::process_packet(usb_in)
}
