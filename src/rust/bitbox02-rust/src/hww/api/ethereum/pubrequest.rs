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
