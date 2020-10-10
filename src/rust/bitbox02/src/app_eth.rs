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

pub struct Params {
    pub bip44_coin: u32,
    pub chain_id: u8,
    pub unit: &'static str,
}

pub fn params_get(coin: bitbox02_sys::ETHCoin) -> Option<Params> {
    let params = unsafe { bitbox02_sys::app_eth_params_get(coin).as_ref()? };
    Some(Params {
        bip44_coin: params.bip44_coin,
        chain_id: params.chain_id,
        unit: {
            let s = unsafe {
                let len = crate::util::strlen_ptr(params.unit);
                core::slice::from_raw_parts(params.unit, len as _)
            };
            core::str::from_utf8(&s[..]).unwrap()
        },
    })
}

pub struct ERC20Params {
    pub unit: &'static str,
    pub name: &'static str,
    pub contract_address: [u8; 20],
    pub decimals: u8,
}

pub fn erc20_params_get(
    coin: bitbox02_sys::ETHCoin,
    contract_address: [u8; 20],
) -> Option<ERC20Params> {
    let params = unsafe {
        bitbox02_sys::app_eth_erc20_params_get(coin, contract_address.as_ptr()).as_ref()?
    };
    Some(ERC20Params {
        unit: {
            let s = unsafe {
                let len = crate::util::strlen_ptr(params.unit);
                core::slice::from_raw_parts(params.unit, len as _)
            };
            core::str::from_utf8(&s[..]).unwrap()
        },
        name: {
            let s = unsafe {
                let len = crate::util::strlen_ptr(params.name);
                core::slice::from_raw_parts(params.name, len as _)
            };
            core::str::from_utf8(&s[..]).unwrap()
        },
        contract_address: params.contract_address,
        decimals: params.decimals,
    })
}
