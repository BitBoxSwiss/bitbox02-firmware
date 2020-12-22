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

use super::pb;
use super::Error;

use pb::eth_pub_request::OutputType;
use pb::eth_response::Response;

use crate::workflow::confirm;
use bitbox02::keystore;

extern crate alloc;
use alloc::string::String;
use core::convert::TryInto;

fn coin_title(coin: pb::EthCoin) -> &'static str {
    match coin {
        pb::EthCoin::Eth => "Ethereum",
        pb::EthCoin::RopstenEth => "Ropsten",
        pb::EthCoin::RinkebyEth => "Rinkeby",
    }
}

async fn process_address(request: &pb::EthPubRequest) -> Result<Response, Error> {
    let coin = pb::EthCoin::from_i32(request.coin).ok_or(Error::InvalidInput)?;
    let params = bitbox02::app_eth::params_get(request.coin as _).ok_or(Error::InvalidInput)?;
    // If a contract_address is provided, it has to be a supported ERC20-token.
    let erc20_params: Option<bitbox02::app_eth::ERC20Params> =
        if request.contract_address.is_empty() {
            None
        } else {
            let address: [u8; 20] = request
                .contract_address
                .as_slice()
                .try_into()
                .or(Err(Error::InvalidInput))?;
            Some(
                bitbox02::app_eth::erc20_params_get(request.coin as _, address)
                    .ok_or(Error::InvalidInput)?,
            )
        };

    if !ethereum::keypath::is_valid_keypath_address(&request.keypath, params.bip44_coin) {
        return Err(Error::InvalidInput);
    }
    let pubkey = bitbox02::keystore::secp256k1_pubkey_uncompressed(&request.keypath)
        .or(Err(Error::InvalidInput))?;
    let mut address = String::new();
    ethereum::address::from_pubkey(&pubkey, &mut address).unwrap();

    if request.display {
        let title = match erc20_params {
            Some(erc20_params) => erc20_params.name,
            None => coin_title(coin),
        };
        let params = confirm::Params {
            title,
            title_autowrap: true,
            body: &address,
            scrollable: true,
            ..Default::default()
        };
        if !confirm::confirm(&params).await {
            return Err(Error::UserAbort);
        }
    }

    Ok(Response::Pub(pb::PubResponse { r#pub: address }))
}

fn process_xpub(request: &pb::EthPubRequest) -> Result<Response, Error> {
    if request.display {
        // No xpub user verification for now.
        return Err(Error::InvalidInput);
    }

    let params = bitbox02::app_eth::params_get(request.coin as _).ok_or(Error::InvalidInput)?;
    if !ethereum::keypath::is_valid_keypath_xpub(&request.keypath, params.bip44_coin) {
        return Err(Error::InvalidInput);
    }
    let xpub = keystore::encode_xpub_at_keypath(&request.keypath, keystore::xpub_type_t::XPUB)
        .or(Err(Error::InvalidInput))?;

    Ok(Response::Pub(pb::PubResponse { r#pub: xpub }))
}

pub async fn process(request: &pb::EthPubRequest) -> Result<Response, Error> {
    let output_type = OutputType::from_i32(request.output_type).ok_or(Error::InvalidInput)?;
    match output_type {
        OutputType::Address => process_address(request).await,
        OutputType::Xpub => process_xpub(request),
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use crate::bb02_async::block_on;
    use bitbox02::testing::{mock, Data, MUTEX};
    use std::boxed::Box;
    use util::bip32::HARDENED;

    #[test]
    pub fn test_process_xpub() {
        let _guard = MUTEX.lock().unwrap();

        const EXPECTED_XPUB: &str = "xpub";
        let request = pb::EthPubRequest {
            output_type: OutputType::Xpub as _,
            keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0].to_vec(),
            coin: pb::EthCoin::Eth as _,
            display: false,
            contract_address: b"".to_vec(),
        };

        // All good.
        mock(Data {
            eth_params_get: Some(Box::new(|coin| {
                assert_eq!(coin, pb::EthCoin::Eth as _);
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 60 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            keystore_encode_xpub_at_keypath: Some(Box::new(|_, xpub_type| {
                assert_eq!(xpub_type, keystore::xpub_type_t::XPUB);
                Ok(EXPECTED_XPUB.into())
            })),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&request)),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: EXPECTED_XPUB.into()
            }))
        );

        // Params not found.
        mock(Data {
            eth_params_get: Some(Box::new(|_| None)),
            ..Default::default()
        });
        assert_eq!(block_on(process(&request)), Err(Error::InvalidInput));

        // Wrong keypath (wrong expected coin)
        mock(Data {
            eth_params_get: Some(Box::new(|_| {
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 61 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            ..Default::default()
        });
        assert_eq!(block_on(process(&request)), Err(Error::InvalidInput));

        // xpub fetching/encoding failed.
        mock(Data {
            eth_params_get: Some(Box::new(|_| {
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 60 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            keystore_encode_xpub_at_keypath: Some(Box::new(|_, _| Err(()))),
            ..Default::default()
        });
        assert_eq!(block_on(process(&request)), Err(Error::InvalidInput));
    }

    #[test]
    pub fn test_process_address() {
        let _guard = MUTEX.lock().unwrap();

        const PUBKEY: [u8; 65] = [
            0x04, 0xd8, 0xae, 0xa8, 0x0d, 0x2d, 0xbc, 0xeb, 0xbe, 0x10, 0xfd, 0xfa, 0xc2, 0xd2,
            0xdb, 0x19, 0x64, 0x15, 0x5b, 0xa9, 0x9e, 0x0d, 0xd7, 0xbf, 0xd5, 0xcf, 0xfe, 0xd9,
            0x7a, 0x1c, 0xae, 0xf7, 0xd0, 0xb9, 0x07, 0x2d, 0x9c, 0x0f, 0x50, 0x49, 0x30, 0xef,
            0x59, 0xb7, 0x52, 0xd4, 0xfe, 0xa0, 0xcb, 0xde, 0x3e, 0x27, 0x3e, 0xe9, 0x54, 0xd8,
            0xda, 0xc8, 0xee, 0x03, 0x1a, 0x4e, 0xd1, 0x71, 0xfd,
        ];
        const ADDRESS: &str = "0xF4C21710Ef8b5a5Ec4bd3780A687FE083446e67B";

        let request = &pb::EthPubRequest {
            output_type: OutputType::Address as _,
            keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
            coin: pb::EthCoin::Eth as _,
            display: false,
            contract_address: b"".to_vec(),
        };

        // All good.
        mock(Data {
            eth_params_get: Some(Box::new(|coin| {
                assert_eq!(coin, pb::EthCoin::Eth as _);
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 60 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            keystore_secp256k1_pubkey_uncompressed: Some(Box::new(|_| Ok(PUBKEY))),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&request)),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: ADDRESS.into()
            }))
        );

        // All good, with display.
        mock(Data {
            eth_params_get: Some(Box::new(|_| {
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 60 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            keystore_secp256k1_pubkey_uncompressed: Some(Box::new(|_| Ok(PUBKEY))),
            ui_confirm_create: Some(Box::new(|params| {
                assert_eq!(params.title, "Ethereum");
                assert_eq!(params.body, "0xF4C21710Ef8b5a5Ec4bd3780A687FE083446e67B");
                true
            })),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::EthPubRequest {
                output_type: OutputType::Address as _,
                keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
                coin: pb::EthCoin::Eth as _,
                display: true,
                contract_address: b"".to_vec(),
            })),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: ADDRESS.into()
            }))
        );

        // Params not found.
        mock(Data {
            eth_params_get: Some(Box::new(|_| None)),
            ..Default::default()
        });
        assert_eq!(block_on(process(&request)), Err(Error::InvalidInput));

        // Wrong keypath (wrong expected coin)
        mock(Data {
            eth_params_get: Some(Box::new(|_| {
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 61 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            ..Default::default()
        });
        assert_eq!(block_on(process(&request)), Err(Error::InvalidInput));

        // Wrong keypath (account too high)
        mock(Data {
            eth_params_get: Some(Box::new(|_| {
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 60 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::EthPubRequest {
                output_type: OutputType::Address as _,
                keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 100].to_vec(),
                coin: pb::EthCoin::Eth as _,
                display: false,
                contract_address: b"".to_vec(),
            })),
            Err(Error::InvalidInput)
        );
    }

    #[test]
    pub fn test_process_erc20_address() {
        let _guard = MUTEX.lock().unwrap();

        const PUBKEY: [u8; 65] = [
            0x04, 0xd8, 0xae, 0xa8, 0x0d, 0x2d, 0xbc, 0xeb, 0xbe, 0x10, 0xfd, 0xfa, 0xc2, 0xd2,
            0xdb, 0x19, 0x64, 0x15, 0x5b, 0xa9, 0x9e, 0x0d, 0xd7, 0xbf, 0xd5, 0xcf, 0xfe, 0xd9,
            0x7a, 0x1c, 0xae, 0xf7, 0xd0, 0xb9, 0x07, 0x2d, 0x9c, 0x0f, 0x50, 0x49, 0x30, 0xef,
            0x59, 0xb7, 0x52, 0xd4, 0xfe, 0xa0, 0xcb, 0xde, 0x3e, 0x27, 0x3e, 0xe9, 0x54, 0xd8,
            0xda, 0xc8, 0xee, 0x03, 0x1a, 0x4e, 0xd1, 0x71, 0xfd,
        ];
        const ADDRESS: &str = "0xF4C21710Ef8b5a5Ec4bd3780A687FE083446e67B";
        const CONTRACT_ADDRESS: [u8; 20] = *b"aaaaaaaaaaaaaaaaaaaa";

        let request = &pb::EthPubRequest {
            output_type: OutputType::Address as _,
            keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
            coin: pb::EthCoin::Eth as _,
            display: false,
            contract_address: CONTRACT_ADDRESS.to_vec(),
        };

        // All good.
        mock(Data {
            eth_params_get: Some(Box::new(|coin| {
                assert_eq!(coin, pb::EthCoin::Eth as _);
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 60 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            eth_erc20_params_get: Some(Box::new(|coin, contract_address| {
                assert_eq!(coin, pb::EthCoin::Eth as _);
                assert_eq!(contract_address, CONTRACT_ADDRESS);
                Some(bitbox02::app_eth::ERC20Params {
                    unit: "ETH",
                    name: "ERC20 token",
                    contract_address: CONTRACT_ADDRESS,
                    decimals: 6,
                })
            })),
            keystore_secp256k1_pubkey_uncompressed: Some(Box::new(|_| Ok(PUBKEY))),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&request)),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: ADDRESS.into()
            }))
        );

        const TOKEN_NAME: &str = "ERC20 token";
        // All good, with display.
        mock(Data {
            eth_params_get: Some(Box::new(|_| {
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 60 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            eth_erc20_params_get: Some(Box::new(|_, _| {
                Some(bitbox02::app_eth::ERC20Params {
                    unit: "ETH",
                    name: TOKEN_NAME,
                    contract_address: CONTRACT_ADDRESS,
                    decimals: 6,
                })
            })),
            keystore_secp256k1_pubkey_uncompressed: Some(Box::new(|_| Ok(PUBKEY))),
            ui_confirm_create: Some(Box::new(|params| {
                assert_eq!(params.title, TOKEN_NAME);
                assert_eq!(params.body, "0xF4C21710Ef8b5a5Ec4bd3780A687FE083446e67B");
                true
            })),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::EthPubRequest {
                output_type: OutputType::Address as _,
                keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
                coin: pb::EthCoin::Eth as _,
                display: true,
                contract_address: CONTRACT_ADDRESS.to_vec(),
            })),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: ADDRESS.into()
            }))
        );

        // ERC20 params not found / invalid contract address.
        mock(Data {
            eth_params_get: Some(Box::new(|_| {
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 60 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            eth_erc20_params_get: Some(Box::new(|_, _| None)),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::EthPubRequest {
                output_type: OutputType::Address as _,
                keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
                coin: pb::EthCoin::Eth as _,
                display: false,
                contract_address: CONTRACT_ADDRESS.to_vec(),
            })),
            Err(Error::InvalidInput)
        );
    }
}
