// Copyright 2021 Shift Crypto AG
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

use super::pb::CardanoNetwork;

/// Parameters for Cardano networks.
pub struct Params {
    pub name: &'static str,
    pub unit: &'static str,
    /// https://github.com/cardano-foundation/CIPs/blob/6c249ef48f8f5b32efc0ec768fadf4321f3173f2/CIP-0005/CIP-0005.md#miscellaneous
    pub bech32_hrp_payment: &'static str,
    /// https://github.com/cardano-foundation/CIPs/blob/6c249ef48f8f5b32efc0ec768fadf4321f3173f2/CIP-0019/CIP-0019.md#network-tag
    pub network_id: u8,
}

const PARAMS_MAINNET: Params = Params {
    name: "Cardano",
    unit: "ADA",
    bech32_hrp_payment: "addr",
    network_id: 1,
};

const PARAMS_TESTNET: Params = Params {
    name: "ADA testnet",
    unit: "TADA",
    bech32_hrp_payment: "addr_test",
    network_id: 0,
};

pub fn get(network: CardanoNetwork) -> &'static Params {
    match network {
        CardanoNetwork::CardanoMainnet => &PARAMS_MAINNET,
        CardanoNetwork::CardanoTestnet => &PARAMS_TESTNET,
    }
}
