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

use super::pb;
use super::Error;

use crate::workflow::confirm;

use pb::btc_pub_request::Output;
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

/// Processes a SimpleType (single-sig) adress api call.
async fn address_simple(
    coin: BtcCoin,
    simple_type: SimpleType,
    keypath: &[u32],
    display: bool,
) -> Result<Response, Error> {
    let address = bitbox02::app_btc::address_simple(coin as _, simple_type as _, keypath)?;
    if display {
        let params = confirm::Params {
            title: coin_name(coin),
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
