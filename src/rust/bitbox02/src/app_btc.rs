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
use alloc::vec::Vec;

pub use bitbox02_sys::multisig_t as Multisig;

pub fn pkscript_from_multisig(
    multisig: &Multisig,
    keypath_change: u32,
    keypath_address: u32,
) -> Result<Vec<u8>, ()> {
    let mut out = [0u8; bitbox02_sys::MAX_PK_SCRIPT_SIZE as usize];
    let mut out_len: usize = out.len() as _;
    match unsafe {
        bitbox02_sys::btc_common_pkscript_from_multisig(
            multisig,
            keypath_change,
            keypath_address,
            out.as_mut_ptr(),
            &mut out_len,
        )
    } {
        true => Ok(out[..out_len].to_vec()),
        false => Err(()),
    }
}
