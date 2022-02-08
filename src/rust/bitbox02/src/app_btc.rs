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
use alloc::{vec, vec::Vec};

pub use bitbox02_sys::app_btc_result_t as Error;
pub use bitbox02_sys::{BTCCoin, BTCScriptConfig_SimpleType};

pub fn address_simple(
    coin: BTCCoin,
    script_type: BTCScriptConfig_SimpleType,
    keypath: &[u32],
) -> Result<String, ()> {
    let mut address = [0u8; 500];
    match unsafe {
        bitbox02_sys::app_btc_address_simple(
            coin,
            script_type,
            keypath.as_ptr(),
            keypath.len() as _,
            address.as_mut_ptr(),
            address.len() as _,
        )
    } {
        true => Ok(crate::util::str_from_null_terminated(&address[..])
            .unwrap()
            .into()),
        false => Err(()),
    }
}

pub fn sign_init_wrapper(buffer_in: &[u8]) -> Result<(), Error> {
    unsafe {
        match bitbox02_sys::app_btc_sign_init_wrapper(bitbox02_sys::in_buffer_t {
            data: buffer_in.as_ptr(),
            len: buffer_in.len() as _,
        }) {
            Error::APP_BTC_OK => Ok(()),
            err => Err(err),
        }
    }
}

pub fn sign_payload_at_change_wrapper(buffer_in: &[u8]) -> Result<Vec<u8>, Error> {
    let mut payload = [0u8; 32];
    let mut payload_size: bitbox02_sys::size_t = 0;
    unsafe {
        match bitbox02_sys::app_btc_sign_payload_at_change_wrapper(
            bitbox02_sys::in_buffer_t {
                data: buffer_in.as_ptr(),
                len: buffer_in.len() as _,
            },
            payload.as_mut_ptr(),
            &mut payload_size,
        ) {
            Error::APP_BTC_OK => Ok(payload[..payload_size as usize].to_vec()),
            err => Err(err),
        }
    }
}

pub fn sign_output_wrapper(
    buffer_in: &[u8],
    last: bool,
    output_type: i32,
    payload: &[u8],
) -> Result<(), Error> {
    unsafe {
        match bitbox02_sys::app_btc_sign_output_wrapper(
            bitbox02_sys::in_buffer_t {
                data: buffer_in.as_ptr(),
                len: buffer_in.len() as _,
            },
            last,
            output_type as _,
            payload.as_ptr(),
            payload.len() as _,
        ) {
            Error::APP_BTC_OK => Ok(()),
            err => Err(err),
        }
    }
}

pub fn sign_input_pass2_wrapper(
    buffer_in: &[u8],
    hash_prevouts: &[u8],
    hash_sequence: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), Error> {
    let mut sig_out = vec![0u8; 64];
    let mut anti_klepto_signer_commitment_out = vec![0u8; 33];
    if hash_prevouts.len() != 32 || hash_sequence.len() != 32 {
        return Err(Error::APP_BTC_ERR_UNKNOWN);
    }
    unsafe {
        match bitbox02_sys::app_btc_sign_input_pass2_wrapper(
            bitbox02_sys::in_buffer_t {
                data: buffer_in.as_ptr(),
                len: buffer_in.len() as _,
            },
            hash_prevouts.as_ptr(),
            hash_sequence.as_ptr(),
            sig_out.as_mut_ptr(),
            anti_klepto_signer_commitment_out.as_mut_ptr(),
        ) {
            Error::APP_BTC_OK => Ok((sig_out, anti_klepto_signer_commitment_out)),
            err => Err(err),
        }
    }
}

pub fn sign_antiklepto_wrapper(buffer_in: &[u8]) -> Result<Vec<u8>, Error> {
    let mut sig_out = vec![0u8; 64];
    unsafe {
        match bitbox02_sys::app_btc_sign_antiklepto_wrapper(
            bitbox02_sys::in_buffer_t {
                data: buffer_in.as_ptr(),
                len: buffer_in.len() as _,
            },
            sig_out.as_mut_ptr(),
        ) {
            Error::APP_BTC_OK => Ok(sig_out),
            err => Err(err),
        }
    }
}

pub fn sign_reset() {
    unsafe { bitbox02_sys::app_btc_sign_reset() }
}
