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
    pub coin: EthCoin,
    pub bip44_coin: u32,
    /// https://github.com/ethereum/EIPs/blob/master/EIPS/eip-155.md#list-of-chain-ids
    pub chain_id: u8,
    pub name: &'static str,
    pub unit: &'static str,
}

const PARAMS: &[Params] = &[
    Params {
        coin: EthCoin::Eth,
        bip44_coin: 60 + HARDENED,
        chain_id: 1,
        name: "Ethereum",
        unit: "ETH",
    },
    Params {
        coin: EthCoin::RopstenEth,
        bip44_coin: 1 + HARDENED,
        chain_id: 3,
        name: "Ropsten",
        unit: "TETH",
    },
    Params {
        coin: EthCoin::RinkebyEth,
        bip44_coin: 1 + HARDENED,
        chain_id: 4,
        name: "Rinkeby",
        unit: "TETH",
    },
];

pub fn get(coin: EthCoin) -> Option<&'static Params> {
    PARAMS.iter().find(|p| p.coin == coin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get() {
        assert_eq!(get(EthCoin::Eth).unwrap().name, "Ethereum");
        assert_eq!(get(EthCoin::RopstenEth).unwrap().name, "Ropsten");
        assert_eq!(get(EthCoin::RinkebyEth).unwrap().name, "Rinkeby");
    }
}
