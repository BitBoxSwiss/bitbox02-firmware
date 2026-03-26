// SPDX-License-Identifier: Apache-2.0

use super::pb::SolanaNetwork;

pub struct Params {
    pub name: &'static str,
    pub unit: &'static str,
}

const PARAMS_MAINNET: Params = Params {
    name: "Solana",
    unit: "SOL",
};

const PARAMS_DEVNET: Params = Params {
    name: "Solana Devnet",
    unit: "SOL",
};

pub fn get(network: SolanaNetwork) -> &'static Params {
    match network {
        SolanaNetwork::SolanaMainnet => &PARAMS_MAINNET,
        SolanaNetwork::SolanaDevnet => &PARAMS_DEVNET,
    }
}
