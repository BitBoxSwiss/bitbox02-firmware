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
