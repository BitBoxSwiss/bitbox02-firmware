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

use alloc::string::String;

/// Converts a satoshi value to a string, suffixed with `unit`, e.g. 1234567890 -> "12.3456789 BTC".
pub fn format_amount(satoshi: u64, unit: &str) -> String {
    let mut s = util::decimal::format(satoshi, 8);
    s.push(' ');
    s.push_str(unit);
    s
}

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
                pb::btc_script_config::SimpleType::P2tr => Ok(pb::BtcOutputType::P2tr),
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_format_amount() {
        let tests: Vec<(u64, &str)> = vec![
            (1234567890, "12.3456789 LOL"),
            (0, "0 LOL"),
            (1, "0.00000001 LOL"),
            (2, "0.00000002 LOL"),
            (10, "0.0000001 LOL"),
            (15, "0.00000015 LOL"),
            (20, "0.0000002 LOL"),
            (300, "0.000003 LOL"),
            (370, "0.0000037 LOL"),
            (371, "0.00000371 LOL"),
            (40000000000, "400 LOL"),
            (4000000000, "40 LOL"),
            (400000000, "4 LOL"),
            (40000000, "0.4 LOL"),
            (4000000, "0.04 LOL"),
            (400000, "0.004 LOL"),
            (40000, "0.0004 LOL"),
            (4000, "0.00004 LOL"),
            (400, "0.000004 LOL"),
            (40, "0.0000004 LOL"),
            (4, "0.00000004 LOL"),
            (5432345, "0.05432345 LOL"),
            (54323452, "0.54323452 LOL"),
            (543234527, "5.43234527 LOL"),
            (5432345270, "54.3234527 LOL"),
            (54323452708, "543.23452708 LOL"),
            (100000000, "1 LOL"),
            (1234567800000001, "12345678.00000001 LOL"),
            (0xffffffffffffffff, "184467440737.09551615 LOL"),
            (0xffffffffffffffff - 5, "184467440737.0955161 LOL"),
        ];
        for (satoshi, expected) in tests {
            assert_eq!(format_amount(satoshi, "LOL"), expected);
        }
    }
}
