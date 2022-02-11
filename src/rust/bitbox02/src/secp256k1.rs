// Copyright 2022 Shift Crypto AG
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
