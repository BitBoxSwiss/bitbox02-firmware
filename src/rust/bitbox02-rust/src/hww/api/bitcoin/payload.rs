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

use super::multisig;

use bitbox02::keystore;

use pb::btc_script_config::SimpleType;

use alloc::vec::Vec;

pub fn compute_simple(simple_type: SimpleType, keypath: &[u32]) -> Result<Vec<u8>, Error> {
    match simple_type {
        SimpleType::P2wpkh => Ok(keystore::secp256k1_pubkey_hash160(keypath)?.to_vec()),
        simple_type => Ok(bitbox02::app_btc::payload_at_keypath(
            keypath,
            super::common::convert_simple_type(simple_type),
        )?),
    }
}

pub fn compute_multisig(
    multisig: &pb::btc_script_config::Multisig,
    keypath: &[u32],
) -> Result<Vec<u8>, Error> {
    let script_type = pb::btc_script_config::multisig::ScriptType::from_i32(multisig.script_type)
        .ok_or(Error::InvalidInput)?;
    Ok(bitbox02::app_btc::payload_from_multisig(
        &multisig::convert_multisig(multisig)?,
        multisig::convert_multisig_script_type(script_type),
        keypath[keypath.len() - 2],
        keypath[keypath.len() - 1],
    )?)
}

/// Computes the payload data from a script config. The payload can then be used generate a pkScript
/// or an address.
pub fn compute(
    keypath: &[u32],
    script_config_account: &pb::BtcScriptConfigWithKeypath,
) -> Result<Vec<u8>, Error> {
    match script_config_account {
        pb::BtcScriptConfigWithKeypath {
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(pb::btc_script_config::Config::SimpleType(simple_type)),
                }),
            ..
        } => {
            let simple_type = pb::btc_script_config::SimpleType::from_i32(*simple_type)
                .ok_or(Error::InvalidInput)?;
            compute_simple(simple_type, keypath)
        }
        pb::BtcScriptConfigWithKeypath {
            script_config:
                Some(pb::BtcScriptConfig {
                    config: Some(pb::btc_script_config::Config::Multisig(multisig)),
                }),
            ..
        } => compute_multisig(multisig, keypath),
        _ => Err(Error::InvalidInput),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use bitbox02::testing::mock_unlocked_using_mnemonic;
    use util::bip32::HARDENED;

    #[test]
    fn test_compute_simple() {
        mock_unlocked_using_mnemonic(
            "sudden tenant fault inject concert weather maid people chunk youth stumble grit",
        );

        // p2wpkh
        assert_eq!(
            compute_simple(
                SimpleType::P2wpkh,
                &[84 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0]
            )
            .unwrap()
            .as_slice(),
            b"\x3f\x0d\xc2\xe9\x14\x2d\x88\x39\xae\x9c\x90\xa1\x9c\xa8\x6c\x36\xd9\x23\xd8\xab"
        );

        //  p2wpkh-p2sh
        assert_eq!(
            compute_simple(
                SimpleType::P2wpkhP2sh,
                &[49 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0]
            )
            .unwrap()
            .as_slice(),
            b"\x8d\xd0\x9c\x25\xc9\x28\xbe\x67\x66\xf4\x50\x73\x87\x0c\xe3\xbb\x93\x1f\x2f\x55"
        );

        // p2tr
        assert_eq!(
            compute_simple(
                SimpleType::P2tr,
                &[86 + HARDENED, 0 + HARDENED, 0 + HARDENED, 0, 0]
            )
            .unwrap()
            .as_slice(),
            b"\x25\x0e\xc8\x02\xb6\xd3\xdb\x98\x42\xd1\xbd\xbe\x0e\xe4\x8d\x52\xf9\xa4\xb4\x6e\x60\xcb\xbb\xab\x3b\xcc\x4e\xe9\x15\x73\xfc\xe8"
        );
    }
}
