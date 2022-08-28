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
pub use bitbox02_sys::{multisig_script_type_t as MultisigScriptType, output_type_t as OutputType};

pub fn pkscript_from_payload(
    taproot_support: bool,
    output_type: OutputType,
    payload: &[u8],
) -> Result<Vec<u8>, ()> {
    // current expected max pk script size is a m-of-15 multisig. 700 is also enough for m-of-20, which
    // is technically possible to extend to if needed.
    const MAX_PK_SCRIPT_SIZE: usize = 700;
    let mut out = [0u8; MAX_PK_SCRIPT_SIZE];
    let mut out_len: bitbox02_sys::size_t = out.len() as _;
    match unsafe {
        bitbox02_sys::btc_common_pkscript_from_payload(
            taproot_support,
            output_type,
            payload.as_ptr(),
            payload.len() as _,
            out.as_mut_ptr(),
            &mut out_len,
        )
    } {
        true => Ok(out[..out_len as usize].to_vec()),
        false => Err(()),
    }
}

pub fn pkscript_from_multisig(
    multisig: &Multisig,
    keypath_change: u32,
    keypath_address: u32,
) -> Result<Vec<u8>, ()> {
    let mut out = [0u8; bitbox02_sys::MAX_PK_SCRIPT_SIZE as usize];
    let mut out_len: bitbox02_sys::size_t = out.len() as _;
    match unsafe {
        bitbox02_sys::btc_common_pkscript_from_multisig(
            multisig,
            keypath_change,
            keypath_address,
            out.as_mut_ptr(),
            &mut out_len,
        )
    } {
        true => Ok(out[..out_len as usize].to_vec()),
        false => Err(()),
    }
}

pub fn payload_from_multisig(
    multisig: &Multisig,
    script_type: MultisigScriptType,
    keypath_change: u32,
    keypath_address: u32,
) -> Result<Vec<u8>, ()> {
    let mut out = [0u8; 32];
    let mut out_len: bitbox02_sys::size_t = 0;
    match unsafe {
        bitbox02_sys::btc_common_payload_from_multisig(
            multisig,
            script_type,
            keypath_change,
            keypath_address,
            out.as_mut_ptr(),
            &mut out_len,
        )
    } {
        true => Ok(out[..out_len as usize].to_vec()),
        false => Err(()),
    }
}

pub fn hash160(data: &[u8]) -> [u8; 20] {
    let mut out = [0u8; 20];
    unsafe {
        bitbox02_sys::wally_hash160(
            data.as_ptr(),
            data.len() as _,
            out.as_mut_ptr(),
            out.len() as _,
        );
    }
    out
}
