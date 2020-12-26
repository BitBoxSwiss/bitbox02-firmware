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

#[cfg(not(feature = "app-bitcoin"))]
compile_error!("Bitcoin code is being compiled even though the app-bitcoin feature is not enabled");

mod params;

use super::pb;
use super::Error;

use crate::apps::bitcoin;
use crate::workflow::confirm;

use util::bip32::HARDENED;

use bitbox02::keystore::{encode_xpub_at_keypath, xpub_type_t};

use pb::btc_pub_request::{Output, XPubType};
use pb::btc_script_config::{Config, SimpleType};
use pb::response::Response;
use pb::BtcCoin;
use pb::BtcScriptConfig;

/// Returns `Ok(())` if the coin is enabled in this edition of the firmware.
fn coin_enabled(coin: pb::BtcCoin) -> Result<(), Error> {
    use pb::BtcCoin::*;
    #[cfg(feature = "app-bitcoin")]
    if let Btc | Tbtc = coin {
        return Ok(());
    }
    #[cfg(feature = "app-litecoin")]
    if let Ltc | Tltc = coin {
        return Ok(());
    }
    Err(Error::Disabled)
}

fn coin_name(coin: pb::BtcCoin) -> &'static str {
    use pb::BtcCoin::*;
    match coin {
        Btc => "Bitcoin",
        Tbtc => "BTC Testnet",
        Ltc => "Litecoin",
        Tltc => "LTC Testnet",
    }
}

/// Processes an xpub api call.
async fn xpub(
    coin: BtcCoin,
    xpub_type: XPubType,
    keypath: &[u32],
    display: bool,
) -> Result<Response, Error> {
    let params = params::get(coin);
    bitcoin::keypath::validate_xpub(keypath, params.bip44_coin)?;
    let xpub_type = match xpub_type {
        XPubType::Tpub => xpub_type_t::TPUB,
        XPubType::Xpub => xpub_type_t::XPUB,
        XPubType::Ypub => xpub_type_t::YPUB,
        XPubType::Zpub => xpub_type_t::ZPUB,
        XPubType::Vpub => xpub_type_t::VPUB,
        XPubType::Upub => xpub_type_t::UPUB,
        XPubType::CapitalVpub => xpub_type_t::CAPITAL_VPUB,
        XPubType::CapitalZpub => xpub_type_t::CAPITAL_ZPUB,
        XPubType::CapitalUpub => xpub_type_t::CAPITAL_UPUB,
        XPubType::CapitalYpub => xpub_type_t::CAPITAL_YPUB,
    };
    let xpub = encode_xpub_at_keypath(keypath, xpub_type).or(Err(Error::InvalidInput))?;
    if display {
        let title = format!(
            "{}\naccount #{}",
            coin_name(coin),
            keypath[2] - HARDENED + 1,
        );
        let confirm_params = confirm::Params {
            title: &title,
            body: &xpub,
            scrollable: true,
            ..Default::default()
        };
        if !confirm::confirm(&confirm_params).await {
            return Err(Error::UserAbort);
        }
    }
    Ok(Response::Pub(pb::PubResponse { r#pub: xpub }))
}

/// Processes a SimpleType (single-sig) adress api call.
async fn address_simple(
    coin: BtcCoin,
    simple_type: SimpleType,
    keypath: &[u32],
    display: bool,
) -> Result<Response, Error> {
    let address = bitbox02::app_btc::address_simple(coin as _, simple_type as _, keypath)?;
    if display {
        let confirm_params = confirm::Params {
            title: coin_name(coin),
            body: &address,
            scrollable: true,
            ..Default::default()
        };
        if !confirm::confirm(&confirm_params).await {
            return Err(Error::UserAbort);
        }
    }
    Ok(Response::Pub(pb::PubResponse { r#pub: address }))
}

/// Handle a Bitcoin xpub/address protobuf api call.
///
/// Returns `None` if the call was not handled by Rust, in which case it should be handled by
/// the C commander.
pub async fn process_pub(request: &pb::BtcPubRequest) -> Option<Result<Response, Error>> {
    let coin = match BtcCoin::from_i32(request.coin) {
        Some(coin) => coin,
        None => return Some(Err(Error::InvalidInput)),
    };
    if let Err(err) = coin_enabled(coin) {
        return Some(Err(err));
    }
    match request.output {
        None => Some(Err(Error::InvalidInput)),
        Some(Output::XpubType(xpub_type)) => {
            let xpub_type = match XPubType::from_i32(xpub_type) {
                Some(xpub_type) => xpub_type,
                None => return Some(Err(Error::InvalidInput)),
            };
            Some(xpub(coin, xpub_type, &request.keypath, request.display).await)
        }
        Some(Output::ScriptConfig(BtcScriptConfig {
            config: Some(Config::SimpleType(simple_type)),
        })) => {
            let simple_type = match SimpleType::from_i32(simple_type) {
                Some(simple_type) => simple_type,
                None => return Some(Err(Error::InvalidInput)),
            };
            Some(address_simple(coin, simple_type, &request.keypath, request.display).await)
        }
        _ => None,
    }
}
