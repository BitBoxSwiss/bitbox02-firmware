// Copyright 2025 Shift Crypto AG
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

use super::Error;
use super::params;
use super::pb;

use pb::BtcCoin;
use pb::btc_response::Response;
use pb::btc_xpubs_request::XPubType;

use alloc::string::String;
use alloc::vec::Vec;

/// Max number of xpubs that can be requested at once.
const MAX_XPUBS: usize = 20;

/// Retrieves up to 20 xpubs at once.
///
/// Only standard keypaths are allowed for now.
pub async fn process_xpubs(
    hal: &mut impl crate::hal::Hal,
    request: &pb::BtcXpubsRequest,
) -> Result<Response, Error> {
    let coin = BtcCoin::try_from(request.coin)?;
    super::coin_enabled(coin)?;

    let params = params::get(coin);
    if request.keypaths.len() > MAX_XPUBS {
        return Err(Error::InvalidInput);
    }
    let xpub_type: pb::btc_pub_request::XPubType = {
        let xpub_type = XPubType::try_from(request.xpub_type).map_err(|_| Error::InvalidInput)?;
        match xpub_type {
            XPubType::Unknown => return Err(Error::InvalidInput),
            XPubType::Xpub => pb::btc_pub_request::XPubType::Xpub,
            XPubType::Tpub => pb::btc_pub_request::XPubType::Tpub,
        }
    };
    let keypaths: Vec<&[u32]> = request
        .keypaths
        .iter()
        .map(|k| k.keypath.as_slice())
        .collect();

    for keypath in keypaths.iter() {
        super::keypath::validate_xpub(keypath, params.bip44_coin, params.taproot_support)
            .map_err(|_| Error::InvalidInput)?;
    }

    let xpubs = crate::keystore::get_xpubs_twice(hal, &keypaths)?;
    let xpub_strings: Vec<String> = xpubs
        .iter()
        .map(|xpub| xpub.serialize_str(xpub_type))
        .collect::<Result<_, _>>()?;
    Ok(Response::Pubs(pb::PubsResponse { pubs: xpub_strings }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::keystore::testing::{mock_unlocked, mock_unlocked_using_mnemonic};
    use bitbox02::testing::mock_memory;
    use util::bb02_async::block_on;
    use util::bip32::HARDENED;

    #[test]
    pub fn test_process_xpubs() {
        mock_unlocked_using_mnemonic(
            "sleep own lobster state clean thrive tail exist cactus bitter pass soccer clinic riot dream turkey before sport action praise tunnel hood donate man",
            "",
        );

        let mut mock_hal = crate::hal::testing::TestingHal::new();
        mock_hal.securechip.event_counter_reset();
        assert_eq!(
            block_on(process_xpubs(&mut mock_hal, &pb::BtcXpubsRequest {
                coin: BtcCoin::Btc as _,
                xpub_type: XPubType::Xpub as _,
                keypaths: vec![
                    pb::Keypath {
                        keypath: vec![84 + HARDENED, HARDENED, HARDENED],
                    },
                    pb::Keypath {
                        keypath: vec![86 + HARDENED, HARDENED, HARDENED],
                    },
                    pb::Keypath {
                        keypath: vec![49 + HARDENED, HARDENED, HARDENED],
                    },
                ],
            })),
            Ok(Response::Pubs(pb::PubsResponse {
                pubs: vec![
                    "xpub6CNbmcHwZDudAvCAZVE5kejUoFD63mbkRbRMA2HoF9oNWsCofni87gJKp31qZJ9FsCMQR2vK9AS51mT8dgUMGsHW6SfaAKb4eSzpqJn7zwK".into(),
                    "xpub6CGwpj8iQNuzSeeEKF4yuQt32fpLqfHj7sUfFH4uW34DoctWPksxAdjNYC9KwYgwA149B7SDdcLH1aFmucRcjBL4U6piN7HgaiFCBsToamH".into(),
                    "xpub6CKWKetFeZaZm76Tmzymyadg2Lc9njNZvCV7XaeePLbwPatyVTqS3k8iWeJNziZR6n1kUqkChCmaP7MxyED3KDsSUH7F5Lc9RFe9P4B78Uc".into(),
                ]
            })),
        );
        assert_eq!(mock_hal.securechip.get_event_counter(), 2);

        // Different output type
        assert_eq!(
            block_on(process_xpubs(&mut crate::hal::testing::TestingHal::new(),&pb::BtcXpubsRequest {
                coin: BtcCoin::Btc as _,
                xpub_type: XPubType::Tpub as _,
                keypaths: vec![
                    pb::Keypath {
                        keypath: vec![84 + HARDENED, HARDENED, HARDENED],
                    },
                ],
            })),
            Ok(Response::Pubs(pb::PubsResponse {
                pubs: vec![
                    "tpubDCkEHr7dGVs5SiP21gDAxa4r8NJk3A6oyE1eWaLwb4ZGG9sWk1ZDG7yA456d5o6Vf6tK2cSBgGG7hwwk2YKbAJjoA3QsqrFJEQbEbLKkt5w".into(),
                ]
            })),
        );

        // Different coin
        assert_eq!(
            block_on(process_xpubs(&mut crate::hal::testing::TestingHal::new(),&pb::BtcXpubsRequest {
                coin: BtcCoin::Ltc as _,
                xpub_type: XPubType::Xpub as _,
                keypaths: vec![
                    pb::Keypath {
                        keypath: vec![84 + HARDENED, 2+HARDENED, HARDENED],
                    },
                ],
            })),
            Ok(Response::Pubs(pb::PubsResponse {
                pubs: vec![
                    "xpub6DEKPXTV5HQNcJNGWcSCsdEc2zzoXUHy1L678r3ux3CN2iHqxwKgFaxnzs73nr33VR7SNTDaqFzeyMwHocBEa4j96LEoKacL38N6RAXS3hP".into(),
                ]
            })),
        );
    }

    // Can get up to 20 xpubs and not more..
    #[test]
    pub fn test_process_limit() {
        mock_unlocked();

        // At limit
        let result = block_on(process_xpubs(
            &mut crate::hal::testing::TestingHal::new(),
            &pb::BtcXpubsRequest {
                coin: BtcCoin::Btc as _,
                xpub_type: XPubType::Xpub as _,
                keypaths: (0..20)
                    .map(|i| pb::Keypath {
                        keypath: vec![86 + HARDENED, HARDENED, HARDENED + i],
                    })
                    .collect(),
            },
        ))
        .unwrap();
        match result {
            Response::Pubs(pubs) => assert_eq!(pubs.pubs.len(), 20),
            _ => panic!("unexpeced response"),
        };

        // Over limit
        assert_eq!(
            block_on(process_xpubs(
                &mut crate::hal::testing::TestingHal::new(),
                &pb::BtcXpubsRequest {
                    coin: BtcCoin::Btc as _,
                    xpub_type: XPubType::Xpub as _,
                    keypaths: (0..21)
                        .map(|i| pb::Keypath {
                            keypath: vec![86 + HARDENED, HARDENED, HARDENED + i],
                        })
                        .collect(),
                }
            )),
            Err(Error::InvalidInput)
        );
    }

    #[test]
    pub fn test_process_invalid_keypath() {
        mock_unlocked();
        assert_eq!(
            block_on(process_xpubs(
                &mut crate::hal::testing::TestingHal::new(),
                &pb::BtcXpubsRequest {
                    coin: BtcCoin::Ltc as _,
                    xpub_type: XPubType::Xpub as _,
                    keypaths: vec![pb::Keypath {
                        keypath: vec![84 + HARDENED, 0 + HARDENED, HARDENED],
                    },],
                }
            )),
            Err(Error::InvalidInput),
        );
    }
}
