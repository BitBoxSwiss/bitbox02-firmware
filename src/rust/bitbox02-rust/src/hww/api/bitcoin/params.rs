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
use pb::BtcCoin;

use util::bip32::HARDENED;

/// Parameters for BTC-like coins. See also:
/// https://en.bitcoin.it/wiki/List_of_address_prefixes
pub struct Params {
    pub coin: BtcCoin,
    /// https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    pub bip44_coin: u32,
    pub base58_version_p2pkh: u8,
    pub base58_version_p2sh: u8,
    pub bech32_hrp: &'static str,
    pub name: &'static str,
    pub rbf_support: bool,
    pub taproot_support: bool,
}

impl Params {
    /// Returns the SLIP44 coin type:
    /// https://github.com/satoshilabs/slips/blob/master/slip-0044.md
    pub fn slip44(&self) -> u32 {
        self.bip44_coin - HARDENED
    }
}

const PARAMS_BTC: Params = Params {
    coin: BtcCoin::Btc,
    bip44_coin: 0 + HARDENED,
    base58_version_p2pkh: 0x00, // starts with 1
    base58_version_p2sh: 0x05,  // starts with 3
    bech32_hrp: "bc",
    name: "Bitcoin",
    rbf_support: true,
    taproot_support: true,
};

const PARAMS_TBTC: Params = Params {
    coin: BtcCoin::Tbtc,
    bip44_coin: 1 + HARDENED,
    base58_version_p2pkh: 0x6f, // starts with m or n
    base58_version_p2sh: 0xc4,  // starts with 2
    bech32_hrp: "tb",
    name: "BTC Testnet",
    rbf_support: true,
    taproot_support: true,
};
const PARAMS_RBTC: Params = Params {
    coin: BtcCoin::Rbtc,
    bip44_coin: 1 + HARDENED,
    base58_version_p2pkh: 0x6f, // starts with m or n
    base58_version_p2sh: 0xc4,  // starts with 2
    bech32_hrp: "bcrt",
    name: "BTC Regtest",
    rbf_support: true,
    taproot_support: true,
};

const PARAMS_LTC: Params = Params {
    coin: BtcCoin::Ltc,
    bip44_coin: 2 + HARDENED,
    base58_version_p2pkh: 0x30, // starts with L
    base58_version_p2sh: 0x32,  // starts with M
    bech32_hrp: "ltc",
    name: "Litecoin",
    rbf_support: false,
    taproot_support: false,
};

const PARAMS_TLTC: Params = Params {
    coin: BtcCoin::Tltc,
    bip44_coin: 1 + HARDENED,
    base58_version_p2pkh: 0x6f, // starts with m or n
    base58_version_p2sh: 0xc4,  // starts with 2
    bech32_hrp: "tltc",
    name: "LTC Testnet",
    rbf_support: false,
    taproot_support: false,
};

pub fn get(coin: BtcCoin) -> &'static Params {
    use BtcCoin::*;
    match coin {
        Btc => &PARAMS_BTC,
        Tbtc => &PARAMS_TBTC,
        Rbtc => &PARAMS_RBTC,
        Ltc => &PARAMS_LTC,
        Tltc => &PARAMS_TLTC,
    }
}
