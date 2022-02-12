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

pub fn sign_sighash_script_wrapper(buffer_in: &[u8]) -> Result<Vec<u8>, Error> {
    let mut sighash_script_out = vec![
        0u8;
        bitbox02_sys::MAX_PK_SCRIPT_SIZE as usize
            + bitbox02_sys::MAX_VARINT_SIZE as usize
    ];
    let mut sighash_script_out_size: bitbox02_sys::size_t = sighash_script_out.len() as _;
    unsafe {
        match bitbox02_sys::app_btc_sign_sighash_script_wrapper(
            bitbox02_sys::in_buffer_t {
                data: buffer_in.as_ptr(),
                len: buffer_in.len() as _,
            },
            sighash_script_out.as_mut_ptr(),
            &mut sighash_script_out_size,
        ) {
            Error::APP_BTC_OK => Ok(sighash_script_out[..sighash_script_out_size as _].to_vec()),
            err => Err(err),
        }
    }
}

pub fn sign_reset() {
    unsafe { bitbox02_sys::app_btc_sign_reset() }
}

pub fn address_from_payload(
    coin: bitbox02_sys::BTCCoin,
    output_type: bitbox02_sys::BTCOutputType,
    payload: &[u8],
) -> Result<String, ()> {
    let mut out = [0u8; 100];
    match unsafe {
        bitbox02_sys::btc_common_address_from_payload(
            bitbox02_sys::app_btc_params_get(coin),
            output_type,
            payload.as_ptr(),
            payload.len() as _,
            out.as_mut_ptr(),
            out.len() as _,
        )
    } {
        true => Ok(crate::util::str_from_null_terminated(&out[..])
            .unwrap()
            .into()),
        false => Err(()),
    }
}

pub fn pkscript_from_payload(
    coin: bitbox02_sys::BTCCoin,
    output_type: bitbox02_sys::BTCOutputType,
    payload: &[u8],
) -> Result<Vec<u8>, ()> {
    // current expected max pk script size is a m-of-15 multisig. 700 is also enough for m-of-20, which
    // is technically possible to extend to if needed.
    const MAX_PK_SCRIPT_SIZE: usize = 700;
    let mut out = [0u8; MAX_PK_SCRIPT_SIZE];
    let mut out_len: bitbox02_sys::size_t = out.len() as _;
    match unsafe {
        bitbox02_sys::btc_common_pkscript_from_payload(
            bitbox02_sys::app_btc_params_get(coin),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_from_payload() {
        assert_eq!(
            address_from_payload(
                bitbox02_sys::_BTCCoin_BTCCoin_BTC,
                bitbox02_sys::_BTCOutputType_BTCOutputType_P2TR,
                b"\xa6\x08\x69\xf0\xdb\xcf\x1d\xc6\x59\xc9\xce\xcb\xaf\x80\x50\x13\x5e\xa9\xe8\xcd\xc4\x87\x05\x3f\x1d\xc6\x88\x09\x49\xdc\x68\x4c",
            ),
            Ok("bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr".into())
        )
    }
}
