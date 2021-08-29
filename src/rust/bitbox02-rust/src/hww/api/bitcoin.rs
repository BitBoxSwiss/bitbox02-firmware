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
mod script;
pub mod signmsg;

use super::pb;
use super::Error;

use crate::apps::bitcoin;
use crate::workflow::confirm;

use util::bip32::HARDENED;

use bitbox02::keystore::{encode_xpub_at_keypath, xpub_type_t};

use pb::btc_pub_request::{Output, XPubType};
use pb::btc_request::Request;
use pb::btc_script_config::{Config, SimpleType};
use pb::response::Response;
use pb::BtcCoin;
use pb::BtcScriptConfig;

use core::convert::TryInto;

/// Like `hww::next_request`, but for Bitcoin requests/responses.
pub async fn next_request(response: pb::btc_response::Response) -> Result<Request, Error> {
    let request = crate::hww::next_request(pb::response::Response::Btc(pb::BtcResponse {
        response: Some(response),
    }))
    .await?;
    match request {
        pb::request::Request::Btc(pb::BtcRequest {
            request: Some(request),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

/// Sends the `signer_nonce_commitment` to the host and waits for the next request, which has to be a
/// `AntiKleptoSignatureRequest` message containing the host nonce.
pub async fn antiklepto_get_host_nonce(
    signer_nonce_commitment: [u8; 33],
) -> Result<[u8; 32], Error> {
    let request = next_request(pb::btc_response::Response::AntikleptoSignerCommitment(
        pb::AntiKleptoSignerCommitment {
            commitment: signer_nonce_commitment.to_vec(),
        },
    ))
    .await?;
    match request {
        Request::AntikleptoSignature(pb::AntiKleptoSignatureRequest { host_nonce }) => {
            Ok(host_nonce
                .as_slice()
                .try_into()
                .or(Err(Error::InvalidInput))?)
        }
        _ => Err(Error::InvalidState),
    }
}

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

pub fn coin_name(coin: pb::BtcCoin) -> &'static str {
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
        confirm::confirm(&confirm_params).await?;
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
        confirm::confirm(&confirm_params).await?;
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

/// Handle a nexted Bitcoin protobuf api call.
///
/// Returns `None` if the call was not handled by Rust, in which case it should be handled by
/// the C commander.
pub async fn process_api(request: &Request) -> Option<Result<pb::btc_response::Response, Error>> {
    match request {
        Request::SignMessage(ref request) => Some(signmsg::process(request).await),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bb02_async::block_on;
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    use bitbox02::testing::{mock, mock_unlocked, Data, MUTEX};
    use util::bip32::HARDENED;

    #[test]
    pub fn test_address_simple() {
        let _guard = MUTEX.lock().unwrap();

        struct Test<'a> {
            coin: BtcCoin,
            keypath: &'a [u32],
            simple_type: SimpleType,
            expected_address: &'a str,
            expected_display_title: &'a str,
        }

        for test in vec![
            // BTC P2WPKH-P2SH
            Test {
                coin: BtcCoin::Btc,
                keypath: &[49 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
                simple_type: SimpleType::P2wpkhP2sh,
                expected_address: "3BaL6XecvLAidPToUDhXo1zxD99ZUrErpd",
                expected_display_title: "Bitcoin",
            },
            Test {
                coin: BtcCoin::Btc,
                keypath: &[49 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 1],
                simple_type: SimpleType::P2wpkhP2sh,
                expected_address: "3QRfCGEJVzvR1HN13kxB7xkuUtdEvG2orZ",
                expected_display_title: "Bitcoin",
            },
            Test {
                coin: BtcCoin::Btc,
                keypath: &[49 + HARDENED, 0 + HARDENED, 1 + HARDENED, 1, 100],
                simple_type: SimpleType::P2wpkhP2sh,
                expected_address: "39r7CFVo1wpb3mxQfkG6yYyxMAfqAmZMhA",
                expected_display_title: "Bitcoin",
            },
            // BTC P2WPKH
            Test {
                coin: BtcCoin::Btc,
                keypath: &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0],
                simple_type: SimpleType::P2wpkh,
                expected_address: "bc1qk5f9em9qc8yfpks8ngfg3h8h02n2e3yeqdyhpt",
                expected_display_title: "Bitcoin",
            },
            Test {
                coin: BtcCoin::Btc,
                keypath: &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 1],
                simple_type: SimpleType::P2wpkh,
                expected_address: "bc1qtn7feuj7juxkzf48zfxtngrcyqyns9f4ska7hg",
                expected_display_title: "Bitcoin",
            },
            Test {
                coin: BtcCoin::Btc,
                keypath: &[84 + HARDENED, 0 + HARDENED, 1 + HARDENED, 1, 100],
                simple_type: SimpleType::P2wpkh,
                expected_address: "bc1qarhxx6daqetewkjwz9p6y78a28ygxm2vndhdas",
                expected_display_title: "Bitcoin",
            },
            // TBTC P2WPKH-P2SH
            Test {
                coin: BtcCoin::Tbtc,
                keypath: &[49 + HARDENED, 1 + HARDENED, 0 + HARDENED, 0, 0],
                simple_type: SimpleType::P2wpkhP2sh,
                expected_address: "2N5Tjwx5Htk7gLbv7nWqXUgpg5K2Uf4TacQ",
                expected_display_title: "BTC Testnet",
            },
            // TBTC P2WPKH
            Test {
                coin: BtcCoin::Tbtc,
                keypath: &[84 + HARDENED, 1 + HARDENED, 0 + HARDENED, 0, 0],
                simple_type: SimpleType::P2wpkh,
                expected_address: "tb1qnlyrq9pshg0v0lsuudjgga4nvmjxhcvketqwdg",
                expected_display_title: "BTC Testnet",
            },
            // LTC P2WPKH-P2SH
            Test {
                coin: BtcCoin::Ltc,
                keypath: &[49 + HARDENED, 2 + HARDENED, 0 + HARDENED, 0, 0],
                simple_type: SimpleType::P2wpkhP2sh,
                expected_address: "MMmYgSH7fbTPnfdi1vTejMJyY7rKY4j9qv",
                expected_display_title: "Litecoin",
            },
            Test {
                coin: BtcCoin::Ltc,
                keypath: &[49 + HARDENED, 2 + HARDENED, 0 + HARDENED, 0, 1],
                simple_type: SimpleType::P2wpkhP2sh,
                expected_address: "M7wA8gBLL4SBiwQ1muQeKcG6naYqWcaUHg",
                expected_display_title: "Litecoin",
            },
            Test {
                coin: BtcCoin::Ltc,
                keypath: &[49 + HARDENED, 2 + HARDENED, 1 + HARDENED, 1, 100],
                simple_type: SimpleType::P2wpkhP2sh,
                expected_address: "MPBnihMP2JYjPtBnLxGydqvaALBsc5ALTG",
                expected_display_title: "Litecoin",
            },
            // LTC P2WPKH
            Test {
                coin: BtcCoin::Ltc,
                keypath: &[84 + HARDENED, 2 + HARDENED, 0 + HARDENED, 0, 0],
                simple_type: SimpleType::P2wpkh,
                expected_address: "ltc1q7598y6mzud5fka043vs4vkx7zktvppxffsf7e3",
                expected_display_title: "Litecoin",
            },
            Test {
                coin: BtcCoin::Ltc,
                keypath: &[84 + HARDENED, 2 + HARDENED, 0 + HARDENED, 0, 1],
                simple_type: SimpleType::P2wpkh,
                expected_address: "ltc1qtgjfu2ltg4slmksv27awmh6h2pccvsth4mw2w9",
                expected_display_title: "Litecoin",
            },
            Test {
                coin: BtcCoin::Ltc,
                keypath: &[84 + HARDENED, 2 + HARDENED, 1 + HARDENED, 1, 100],
                simple_type: SimpleType::P2wpkh,
                expected_address: "ltc1qwsz89auhpezjfllq9y9qegpfgdwpw5vesppsz0",
                expected_display_title: "Litecoin",
            },
            // TLTC P2WPKH-P2SH
            Test {
                coin: BtcCoin::Tltc,
                keypath: &[49 + HARDENED, 1 + HARDENED, 0 + HARDENED, 0, 0],
                simple_type: SimpleType::P2wpkhP2sh,
                expected_address: "2N5Tjwx5Htk7gLbv7nWqXUgpg5K2Uf4TacQ",
                expected_display_title: "LTC Testnet",
            },
            // TLTC P2WPKH
            Test {
                coin: BtcCoin::Tltc,
                keypath: &[84 + HARDENED, 1 + HARDENED, 0 + HARDENED, 0, 0],
                simple_type: SimpleType::P2wpkh,
                expected_address: "tltc1qnlyrq9pshg0v0lsuudjgga4nvmjxhcvkqrzsap",
                expected_display_title: "LTC Testnet",
            },
        ] {
            let mut req = pb::BtcPubRequest {
                coin: test.coin as _,
                keypath: test.keypath.to_vec(),
                display: false,
                output: Some(Output::ScriptConfig(BtcScriptConfig {
                    config: Some(Config::SimpleType(test.simple_type as _)),
                })),
            };

            // Without display.
            mock_unlocked();
            assert_eq!(
                block_on(process_pub(&req)),
                Some(Ok(Response::Pub(pb::PubResponse {
                    r#pub: test.expected_address.into(),
                }))),
            );

            // With display.
            req.display = true;
            let expected_display_title = test.expected_display_title.clone();
            let expected_address = test.expected_address.clone();
            mock(Data {
                ui_confirm_create: Some(Box::new(move |params| {
                    assert_eq!(params.title, expected_display_title);
                    assert_eq!(params.body, expected_address);
                    assert!(params.scrollable);
                    true
                })),
                ..Default::default()
            });
            mock_unlocked();
            assert_eq!(
                block_on(process_pub(&req)),
                Some(Ok(Response::Pub(pb::PubResponse {
                    r#pub: test.expected_address.into()
                }))),
            );
        }

        // --- Negative tests
        mock_unlocked();
        // First check a valid request:
        let req = pb::BtcPubRequest {
            coin: BtcCoin::Btc as _,
            keypath: [49 + HARDENED, 0 + HARDENED, 1 + HARDENED, 1, 100].to_vec(),
            display: false,
            output: Some(Output::ScriptConfig(BtcScriptConfig {
                config: Some(Config::SimpleType(SimpleType::P2wpkhP2sh as _)),
            })),
        };
        assert!(block_on(process_pub(&req)).unwrap().is_ok());
        // -- Wrong coin: MIN-1
        let mut req_invalid = req.clone();
        req_invalid.coin = BtcCoin::Btc as i32 - 1;
        assert!(block_on(process_pub(&req_invalid)).unwrap().is_err());
        // -- Wrong coin: MAX + 1
        let mut req_invalid = req.clone();
        req_invalid.coin = BtcCoin::Tltc as i32 + 1;
        assert!(block_on(process_pub(&req_invalid)).unwrap().is_err());
        // -- Wrong keypath
        let mut req_invalid = req.clone();
        req_invalid.keypath = [49 + HARDENED, 0 + HARDENED, 1 + HARDENED, 1, 10000].to_vec();
        assert!(block_on(process_pub(&req_invalid)).unwrap().is_err());
    }
}
