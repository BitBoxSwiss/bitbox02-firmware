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
const OP_STATUS_SUCCESS: u8 = 0;
const OP_STATUS_FAILURE: u8 = 1;

/// Process OP_UNLOCK.
async fn api_unlock() -> Vec<u8> {
    match crate::workflow::unlock::unlock().await {
        Ok(()) => [OP_STATUS_SUCCESS].to_vec(),
        Err(()) => [OP_STATUS_FAILURE].to_vec(),
    }
}

/// Async HWW api processing main entry point.
/// `usb_in` - api request bytes.
/// Returns the usb response bytes.
pub async fn process_packet(usb_in: Vec<u8>) -> Vec<u8> {
    if let &[OP_UNLOCK] = &usb_in[..] {
        return api_unlock().await;
    }

    // -- Process anything not ported to Rust yet. --
    // This function is blocking.
    bitbox02::hww::process_packet(usb_in)
}
