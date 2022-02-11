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

/// Determine the output type from the given an input script config.
pub fn determine_output_type(
    script_config: &pb::BtcScriptConfig,
) -> Result<pb::BtcOutputType, Error> {
    match script_config {
        pb::BtcScriptConfig {
            config: Some(pb::btc_script_config::Config::SimpleType(simple_type)),
        } => {
            let simple_type = pb::btc_script_config::SimpleType::from_i32(*simple_type)
                .ok_or(Error::InvalidInput)?;
            match simple_type {
                pb::btc_script_config::SimpleType::P2wpkhP2sh => Ok(pb::BtcOutputType::P2sh),
                pb::btc_script_config::SimpleType::P2wpkh => Ok(pb::BtcOutputType::P2wpkh),
            }
        }
        pb::BtcScriptConfig {
            config: Some(pb::btc_script_config::Config::Multisig(multisig)),
        } => {
            let script_type =
                pb::btc_script_config::multisig::ScriptType::from_i32(multisig.script_type)
                    .ok_or(Error::InvalidInput)?;
            match script_type {
                pb::btc_script_config::multisig::ScriptType::P2wsh => Ok(pb::BtcOutputType::P2wsh),
                pb::btc_script_config::multisig::ScriptType::P2wshP2sh => {
                    Ok(pb::BtcOutputType::P2sh)
                }
            }
        }
        _ => Err(Error::InvalidInput),
    }
}
