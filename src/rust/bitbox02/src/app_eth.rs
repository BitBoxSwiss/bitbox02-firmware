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

pub struct ERC20Params {
    pub unit: &'static str,
    pub contract_address: [u8; 20],
    pub decimals: u8,
}

pub fn erc20_params_get(chain_id: u64, contract_address: [u8; 20]) -> Option<ERC20Params> {
    let params = unsafe {
        bitbox02_sys::app_eth_erc20_params_get(chain_id, contract_address.as_ptr()).as_ref()?
    };
    Some(ERC20Params {
        unit: unsafe { crate::util::str_from_null_terminated_ptr(params.unit).unwrap() },
        contract_address: params.contract_address,
        decimals: params.decimals,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erc20_params_get() {
        // Not found for chainID 0.
        assert!(erc20_params_get(
            0,
            *b"\x0f\x72\x71\x4b\x35\xa3\x66\x28\x5d\xf8\x58\x86\xa2\xee\x17\x46\x01\x29\x2a\x17"
        )
        .is_none());

        // Contract address doesn't exist on chainID 1.
        assert!(erc20_params_get(
            1,
            *b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x17"
        )
        .is_none());

        let params = erc20_params_get(
            1,
            *b"\x00\x00\x00\x00\x00\x08\x5d\x47\x80\xb7\x31\x19\xb6\x44\xae\x5e\xcd\x22\xb3\x76",
        )
        .unwrap();
        assert_eq!(params.unit, "TUSD");
        assert_eq!(
            params.contract_address,
            *b"\x00\x00\x00\x00\x00\x08\x5d\x47\x80\xb7\x31\x19\xb6\x44\xae\x5e\xcd\x22\xb3\x76"
        );
        assert_eq!(params.decimals, 18);
    }
}
