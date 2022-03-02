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
use pb::EthCoin;

use util::bip32::HARDENED;

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
        coin: Some(EthCoin::RopstenEth),
        bip44_coin: 1 + HARDENED,
        chain_id: 3,
        name: "Ropsten",
        unit: "TETH",
    },
    Params {
        coin: Some(EthCoin::RinkebyEth),
        bip44_coin: 1 + HARDENED,
        chain_id: 4,
        name: "Rinkeby",
        unit: "TETH",
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
        unit: "OETH",
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
        unit: "AETH",
    },
];

/// Get the chain parameters by `coin` or `chain_id`. If `chain_id` is non-zero, `coin` is ignored.
pub fn get(coin: EthCoin, chain_id: u64) -> Option<&'static Params> {
    PARAMS.iter().find(|p| {
        if chain_id > 0 {
            p.chain_id == chain_id
        } else {
            p.coin == Some(coin)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get() {
        assert_eq!(get(EthCoin::Eth, 0).unwrap().name, "Ethereum");
        assert_eq!(get(EthCoin::Eth, 1).unwrap().name, "Ethereum");
        assert_eq!(get(EthCoin::RopstenEth, 0).unwrap().name, "Ropsten");
        assert_eq!(get(EthCoin::Eth, 3).unwrap().name, "Ropsten");
        assert_eq!(get(EthCoin::RinkebyEth, 0).unwrap().name, "Rinkeby");
        assert_eq!(get(EthCoin::Eth, 4).unwrap().name, "Rinkeby");
        assert_eq!(get(EthCoin::Eth, 56).unwrap().name, "Binance Smart Chain");

        // Unknown chain id.
        assert!(get(EthCoin::Eth, 2).is_none());
    }
}
