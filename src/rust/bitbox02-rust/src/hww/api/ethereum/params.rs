// Copyright 2022 Shift Crypto AG
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
use pb::EthCoin;

use crate::hal::Ui;
use crate::workflow::confirm;

use util::bip32::HARDENED;

#[derive(Copy, Clone)]
pub struct Params {
    /// Used until v9.10.0 and kept for backwards compatibility. From v9.10.0, `chain_id` is used to
    /// identify the network.
    pub coin: Option<EthCoin>,
    pub bip44_coin: u32,
    /// https://github.com/ethereum/EIPs/blob/master/EIPS/eip-155.md#list-of-chain-ids
    pub chain_id: u64,
    pub name: &'static str,
    pub unit: &'static str,
}

// If there should ever be two networks with the same chain ID, the `get()` function should prompt
// the user to choose the network they want to interact with.
const PARAMS: &[Params] = &[
    Params {
        coin: Some(EthCoin::Eth),
        bip44_coin: 60 + HARDENED,
        chain_id: 1,
        name: "Ethereum",
        unit: "ETH",
    },
    Params {
        coin: None,
        bip44_coin: 1 + HARDENED,
        chain_id: 11155111,
        name: "Sepolia",
        unit: "SEPETH",
    },
    Params {
        coin: None,
        bip44_coin: 60 + HARDENED,
        chain_id: 56,
        name: "Binance Smart Chain",
        unit: "BNB",
    },
    Params {
        coin: None,
        bip44_coin: 60 + HARDENED,
        chain_id: 10,
        name: "Optimism",
        unit: "ETH",
    },
    Params {
        coin: None,
        bip44_coin: 60 + HARDENED,
        chain_id: 137,
        name: "Polygon",
        unit: "MATIC",
    },
    Params {
        coin: None,
        bip44_coin: 60 + HARDENED,
        chain_id: 250,
        name: "Fanton Opera",
        unit: "FTM",
    },
    Params {
        coin: None,
        bip44_coin: 60 + HARDENED,
        chain_id: 42161,
        name: "Arbitrum One",
        unit: "ETH",
    },
    Params {
        coin: None,
        bip44_coin: 60 + HARDENED,
        chain_id: 8453,
        name: "Base",
        unit: "ETH",
    },
    Params {
        coin: None,
        bip44_coin: 60 + HARDENED,
        chain_id: 100,
        name: "Gnosis Chain",
        unit: "xDAI",
    },
];

/// Get the chain parameters by `coin` or `chain_id`. If `chain_id` is non-zero, `coin` is ignored.
fn get(coin: Option<EthCoin>, chain_id: u64) -> Option<&'static Params> {
    PARAMS.iter().find(|p| {
        if chain_id > 0 {
            p.chain_id == chain_id
        } else if coin.is_some() {
            p.coin == coin
        } else {
            false
        }
    })
}

/// Check if the chain_id corresponds to a known network (to show an additional confirmations for).
pub fn is_known_network(coin: Option<EthCoin>, chain_id: u64) -> bool {
    get(coin, chain_id).is_some()
}

/// Get the chain parameters by `coin` or `chain_id`. If `chain_id` is non-zero, `coin` is
/// ignored. If `coin` is None. `chain_id` alone is used.
///
/// If no params could be found and `chain_id` is non-zero, the user is asked to confirm the chain
/// ID, and params with this chain ID and "UNKNOWN" name is returned. The main reason for this is
/// that users can rescue funds sent on an unsupported network.
pub async fn get_and_warn_unknown(
    hal: &mut impl crate::hal::Hal,
    coin: Option<EthCoin>,
    chain_id: u64,
) -> Result<Params, Error> {
    match get(coin, chain_id) {
        Some(params) => Ok(*params),
        None => {
            if chain_id == 0 {
                Err(Error::InvalidInput)
            } else {
                hal.ui()
                    .confirm(&confirm::Params {
                        title: "Warning",
                        body: &format!("Unknown network\nwith chain ID:\n{}", chain_id),
                        accept_is_nextarrow: true,
                        ..Default::default()
                    })
                    .await?;
                hal.ui()
                    .confirm(&confirm::Params {
                        title: "Warning",
                        body: "Only proceed if\nyou recognize\nthis chain ID.",
                        accept_is_nextarrow: true,
                        ..Default::default()
                    })
                    .await?;
                Ok(Params {
                    coin: None,
                    bip44_coin: 60 + HARDENED,
                    chain_id,
                    name: "UNKNOWN",
                    unit: "",
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get() {
        assert_eq!(get(Some(EthCoin::Eth), 0).unwrap().name, "Ethereum");
        assert_eq!(get(Some(EthCoin::Eth), 1).unwrap().name, "Ethereum");
        assert_eq!(get(None, 1).unwrap().name, "Ethereum");
        assert_eq!(get(Some(EthCoin::Eth), 11155111).unwrap().name, "Sepolia");
        assert_eq!(get(None, 11155111).unwrap().name, "Sepolia");
        assert_eq!(
            get(Some(EthCoin::Eth), 56).unwrap().name,
            "Binance Smart Chain"
        );
        assert_eq!(get(None, 56).unwrap().name, "Binance Smart Chain");

        // Unknown chain id.
        assert!(get(Some(EthCoin::Eth), 2).is_none());
        assert!(get(None, 2).is_none());
        assert!(get(None, 0).is_none());
    }
}
