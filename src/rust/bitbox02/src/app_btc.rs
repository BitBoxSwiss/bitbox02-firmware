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
use alloc::{vec, vec::Vec};

pub use bitbox02_sys::app_btc_result_t as Error;
pub use bitbox02_sys::multisig_t as Multisig;
pub use bitbox02_sys::{BTCCoin, BTCScriptConfig_Multisig_ScriptType, BTCScriptConfig_SimpleType};

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

pub fn sign_payload_at_keypath_wrapper(
    buffer_in: &[u8],
    keypath: &[u32],
) -> Result<Vec<u8>, Error> {
    let mut payload = [0u8; 32];
    let mut payload_size: bitbox02_sys::size_t = 0;
    unsafe {
        match bitbox02_sys::app_btc_sign_payload_at_keypath_wrapper(
            bitbox02_sys::in_buffer_t {
                data: buffer_in.as_ptr(),
                len: buffer_in.len() as _,
            },
            keypath.as_ptr(),
            keypath.len() as _,
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

pub fn payload_at_keypath(
    keypath: &[u32],
    script_type: BTCScriptConfig_SimpleType,
) -> Result<Vec<u8>, ()> {
    let mut out = [0u8; 32];
    let mut out_len: bitbox02_sys::size_t = 0;
    match unsafe {
        bitbox02_sys::btc_common_payload_at_keypath(
            keypath.as_ptr(),
            keypath.len() as _,
            script_type,
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
    script_type: BTCScriptConfig_Multisig_ScriptType,
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::testing::mock_unlocked_using_mnemonic;
    use util::bip32::HARDENED;

    #[test]
    fn test_payload_at_keypath() {
        mock_unlocked_using_mnemonic(
            "sudden tenant fault inject concert weather maid people chunk youth stumble grit",
        );
        assert_eq!(
            payload_at_keypath(
                &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
                bitbox02_sys::_BTCScriptConfig_SimpleType_BTCScriptConfig_SimpleType_P2WPKH,
            ),
            Ok(
                b"\x3f\x0d\xc2\xe9\x14\x2d\x88\x39\xae\x9c\x90\xa1\x9c\xa8\x6c\x36\xd9\x23\xd8\xab"
                    .to_vec()
            )
        );
    }
}
