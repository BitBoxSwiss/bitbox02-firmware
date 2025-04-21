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

use crate::bip32;
use crate::hal::Ui;
use crate::keystore;
use crate::workflow::confirm;

async fn process_address(
    hal: &mut impl crate::hal::Hal,
    request: &pb::EthPubRequest,
) -> Result<Response, Error> {
    let coin = pb::EthCoin::try_from(request.coin)?;

    let params = super::params::get_and_warn_unknown(hal, Some(coin), request.chain_id).await?;
    // If a contract_address is provided, it has to be a supported ERC20-token.
    let erc20_params: Option<erc20_params::Params> = if request.contract_address.is_empty() {
        None
    } else {
        let address: [u8; 20] = request
            .contract_address
            .as_slice()
            .try_into()
            .or(Err(Error::InvalidInput))?;
        Some(erc20_params::get(params.chain_id, address).ok_or(Error::InvalidInput)?)
    };

    if !super::keypath::is_valid_keypath_address(&request.keypath) {
        return Err(Error::InvalidInput);
    }
    let pubkey = crate::keystore::get_xpub(&request.keypath)
        .or(Err(Error::InvalidInput))?
        .pubkey_uncompressed()?;
    let address = super::address::from_pubkey(&pubkey);

    if request.display {
        let title = match erc20_params {
            Some(erc20_params) => format!("{}\n{}", params.name, erc20_params.unit),
            None => params.name.into(),
        };
        super::keypath::warn_unusual_keypath(hal, &params, &title, &request.keypath).await?;
        hal.ui()
            .confirm(&confirm::Params {
                title: &title,
                title_autowrap: true,
                body: &address,
                scrollable: true,
                ..Default::default()
            })
            .await?;
    }

    Ok(Response::Pub(pb::PubResponse { r#pub: address }))
}

fn process_xpub(request: &pb::EthPubRequest) -> Result<Response, Error> {
    if request.display {
        // No xpub user verification for now.
        return Err(Error::InvalidInput);
    }

    if !super::keypath::is_valid_keypath_xpub(&request.keypath) {
        return Err(Error::InvalidInput);
    }
    let xpub = keystore::get_xpub(&request.keypath)
        .or(Err(Error::InvalidInput))?
        .serialize_str(bip32::XPubType::Xpub)?;

    Ok(Response::Pub(pb::PubResponse { r#pub: xpub }))
}

pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::EthPubRequest,
) -> Result<Response, Error> {
    let output_type = OutputType::try_from(request.output_type)?;
    match output_type {
        OutputType::Address => process_address(hal, request).await,
        OutputType::Xpub => process_xpub(request),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bb02_async::block_on;
    use crate::hal::testing::TestingHal;
    use crate::workflow::testing::Screen;
    use alloc::boxed::Box;
    use bitbox02::testing::mock_unlocked;
    use util::bip32::HARDENED;

    #[test]
    pub fn test_process_xpub() {
        const EXPECTED_XPUB: &str = "xpub6FNKHYBc1HTwuwZcj4dz7xiG1kN7Hs3v7efYmgtzu1Gv6wJXxaCnFdQDRodbQpJKwdeVBf1RRNHARa6FsUMTCuRe2gKR7xCkSDdnppUp9oW";
        let request = pb::EthPubRequest {
            output_type: OutputType::Xpub as _,
            keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0].to_vec(),
            coin: pb::EthCoin::Eth as _,
            display: false,
            contract_address: b"".to_vec(),
            chain_id: 0,
        };

        // All good.
        mock_unlocked();
        assert_eq!(
            block_on(process(&mut TestingHal::new(), &request)),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: EXPECTED_XPUB.into()
            }))
        );

        // Wrong keypath (wrong expected coin)
        let mut invalid_request = request.clone();
        invalid_request.keypath[1] = 61 + HARDENED;
        assert_eq!(
            block_on(process(&mut TestingHal::new(), &invalid_request)),
            Err(Error::InvalidInput)
        );

        // xpub fetching/encoding failed.
        bitbox02::keystore::lock();
        assert_eq!(
            block_on(process(&mut TestingHal::new(), &request)),
            Err(Error::InvalidInput)
        );
    }

    #[test]
    pub fn test_process_address() {
        const ADDRESS: &str = "0x773A77b9D32589be03f9132AF759e294f7851be9";

        let request = pb::EthPubRequest {
            output_type: OutputType::Address as _,
            keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
            coin: pb::EthCoin::Eth as _,
            display: false,
            contract_address: b"".to_vec(),
            chain_id: 0,
        };

        // All good.
        mock_unlocked();
        assert_eq!(
            block_on(process(&mut TestingHal::new(), &request)),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: ADDRESS.into()
            }))
        );

        // All good, with display.
        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &pb::EthPubRequest {
                    output_type: OutputType::Address as _,
                    keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
                    coin: pb::EthCoin::Eth as _,
                    display: true,
                    contract_address: b"".to_vec(),
                    chain_id: 0,
                }
            )),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: ADDRESS.into()
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Ethereum".into(),
                body: ADDRESS.into(),
                longtouch: false,
            }]
        );

        // All good, with display, unusual keypath.
        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &pb::EthPubRequest {
                    output_type: OutputType::Address as _,
                    keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
                    coin: pb::EthCoin::Eth as _,
                    display: true,
                    contract_address: b"".to_vec(),
                    chain_id: 11155111,
                }
            )),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: ADDRESS.into()
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sepolia".into(),
                    body: "Warning: unusual keypath m/44'/60'/0'/0/0. Proceed only if you know what you are doing.".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sepolia".into(),
                    body: ADDRESS.into(),
                    longtouch: false,
                },
            ]
        );

        // Keystore locked.
        bitbox02::keystore::lock();
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::EthPubRequest {
                    output_type: OutputType::Address as _,
                    keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
                    coin: pb::EthCoin::Eth as _,
                    display: true,
                    contract_address: b"".to_vec(),
                    chain_id: 0,
                }
            )),
            Err(Error::InvalidInput)
        );

        // Params not found.
        let mut invalid_request = request.clone();
        invalid_request.coin = 100;
        assert_eq!(
            block_on(process(&mut TestingHal::new(), &invalid_request)),
            Err(Error::InvalidInput)
        );

        // Wrong keypath (wrong expected coin)
        let mut invalid_request = request.clone();
        invalid_request.keypath[1] = 61 + HARDENED;
        assert_eq!(
            block_on(process(&mut TestingHal::new(), &invalid_request)),
            Err(Error::InvalidInput)
        );

        // Wrong keypath (account too high)
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::EthPubRequest {
                    output_type: OutputType::Address as _,
                    keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 100].to_vec(),
                    coin: pb::EthCoin::Eth as _,
                    display: false,
                    contract_address: b"".to_vec(),
                    chain_id: 0,
                }
            )),
            Err(Error::InvalidInput)
        );
    }

    #[test]
    pub fn test_process_erc20_address() {
        const ADDRESS: &str = "0x773A77b9D32589be03f9132AF759e294f7851be9";
        const CONTRACT_ADDRESS: [u8; 20] =
            *b"\xda\xc1\x7f\x95\x8d\x2e\xe5\x23\xa2\x20\x62\x06\x99\x45\x97\xc1\x3d\x83\x1e\xc7";

        let request = pb::EthPubRequest {
            output_type: OutputType::Address as _,
            keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
            coin: pb::EthCoin::Eth as _,
            display: false,
            contract_address: CONTRACT_ADDRESS.to_vec(),
            chain_id: 0,
        };

        // All good.
        mock_unlocked();
        assert_eq!(
            block_on(process(&mut TestingHal::new(), &request)),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: ADDRESS.into()
            }))
        );

        // All good, with display.
        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(
                &mut mock_hal,
                &pb::EthPubRequest {
                    output_type: OutputType::Address as _,
                    keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
                    coin: pb::EthCoin::Eth as _,
                    display: true,
                    contract_address: CONTRACT_ADDRESS.to_vec(),
                    chain_id: 0,
                }
            )),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: ADDRESS.into()
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Ethereum\nUSDT".into(),
                body: ADDRESS.into(),
                longtouch: false,
            }]
        );

        // ERC20 params not found / invalid contract address.
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::EthPubRequest {
                    output_type: OutputType::Address as _,
                    keypath: [44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0].to_vec(),
                    coin: pb::EthCoin::Eth as _,
                    display: false,
                    contract_address: b"aaaaaaaaaaaaaaaaaaaa".to_vec(),
                    chain_id: 0,
                }
            )),
            Err(Error::InvalidInput)
        );
    }
}
