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

use bech32::{ToBase32, Variant};

use pb::btc_script_config::SimpleType;
pub use pb::btc_sign_init_request::FormatUnit;
pub use pb::{BtcCoin, BtcOutputType};

use super::params::Params;

const HASH160_LEN: usize = 20;
const SHA256_LEN: usize = 32;

/// Converts a satoshi value to a string, suffixed with `unit`, e.g. 1234567890 -> "12.3456789 BTC".
pub fn format_amount(
    params: &Params,
    format_unit: FormatUnit,
    satoshi: u64,
) -> Result<String, Error> {
    let (decimals, unit) = match params.coin {
        BtcCoin::Btc => match format_unit {
            FormatUnit::Default => (8, "BTC"),
            FormatUnit::Sat => (0, "sat"),
        },
        BtcCoin::Tbtc => match format_unit {
            FormatUnit::Default => (8, "TBTC"),
            FormatUnit::Sat => (0, "tsat"),
        },
        BtcCoin::Ltc => match format_unit {
            FormatUnit::Default => (8, "LTC"),
            _ => return Err(Error::InvalidInput),
        },
        BtcCoin::Tltc => match format_unit {
            FormatUnit::Default => (8, "TLTC"),
            _ => return Err(Error::InvalidInput),
        },
    };
    let mut s = util::decimal::format(satoshi, decimals);
    s.push(' ');
    s.push_str(unit);
    Ok(s)
}

fn encode_segwit_addr(
    hrp: &str,
    witness_version: u8,
    witness_program: &[u8],
) -> Result<String, ()> {
    let variant = match witness_version {
        0 => Variant::Bech32,
        1 => Variant::Bech32m,
        _ => return Err(()),
    };
    let mut b32 = witness_program.to_base32();
    b32.insert(0, bech32::u5::try_from_u8(witness_version).or(Err(()))?);
    bech32::encode(hrp, &b32, variant).or(Err(()))
}

/// Converts a payload to an address. The payload can be obtained from `btc_common_payload_at_keypath()`.
pub fn address_from_payload(
    params: &Params,
    output_type: BtcOutputType,
    payload: &[u8],
) -> Result<String, ()> {
    match output_type {
        BtcOutputType::Unknown => Err(()),
        BtcOutputType::P2pkh => {
            if payload.len() != HASH160_LEN {
                return Err(());
            }
            Ok(bs58::encode(payload)
                .with_check_version(params.base58_version_p2pkh)
                .into_string())
        }
        BtcOutputType::P2sh => {
            if payload.len() != HASH160_LEN {
                return Err(());
            }
            Ok(bs58::encode(payload)
                .with_check_version(params.base58_version_p2sh)
                .into_string())
        }
        BtcOutputType::P2wpkh => {
            if payload.len() != HASH160_LEN {
                return Err(());
            }
            encode_segwit_addr(params.bech32_hrp, 0, payload)
        }
        BtcOutputType::P2wsh => {
            if payload.len() != SHA256_LEN {
                return Err(());
            }
            encode_segwit_addr(params.bech32_hrp, 0, payload)
        }
        BtcOutputType::P2tr => {
            if !params.taproot_support || payload.len() != 32 {
                return Err(());
            }
            encode_segwit_addr(params.bech32_hrp, 1, payload)
        }
    }
}

pub fn determine_output_type_from_simple_type(simple_type: SimpleType) -> BtcOutputType {
    match simple_type {
        SimpleType::P2wpkhP2sh => BtcOutputType::P2sh,
        SimpleType::P2wpkh => BtcOutputType::P2wpkh,
        SimpleType::P2tr => BtcOutputType::P2tr,
    }
}

pub fn determine_output_type_multisig(
    script_type: pb::btc_script_config::multisig::ScriptType,
) -> BtcOutputType {
    match script_type {
        pb::btc_script_config::multisig::ScriptType::P2wsh => BtcOutputType::P2wsh,
        pb::btc_script_config::multisig::ScriptType::P2wshP2sh => BtcOutputType::P2sh,
    }
}

/// Determine the output type from the given an input script config.
pub fn determine_output_type(script_config: &pb::BtcScriptConfig) -> Result<BtcOutputType, Error> {
    match script_config {
        pb::BtcScriptConfig {
            config: Some(pb::btc_script_config::Config::SimpleType(simple_type)),
        } => {
            let simple_type = SimpleType::from_i32(*simple_type).ok_or(Error::InvalidInput)?;
            Ok(determine_output_type_from_simple_type(simple_type))
        }
        pb::BtcScriptConfig {
            config: Some(pb::btc_script_config::Config::Multisig(multisig)),
        } => {
            let script_type =
                pb::btc_script_config::multisig::ScriptType::from_i32(multisig.script_type)
                    .ok_or(Error::InvalidInput)?;
            Ok(determine_output_type_multisig(script_type))
        }
        _ => Err(Error::InvalidInput),
    }
}

/// Converts a Rust protobuf SimpleType to a representation suitable to be passed to C functions.
pub fn convert_simple_type(simple_type: SimpleType) -> bitbox02::app_btc::SimpleType {
    match simple_type {
        SimpleType::P2wpkhP2sh => bitbox02::app_btc::SimpleType::SIMPLE_TYPE_P2WPKH_P2SH,
        SimpleType::P2wpkh => bitbox02::app_btc::SimpleType::SIMPLE_TYPE_P2WPKH,
        SimpleType::P2tr => bitbox02::app_btc::SimpleType::SIMPLE_TYPE_P2TR,
    }
}

/// Converts a Rust protobuf OutputType to a representation suitable to be passed to C functions.
pub fn convert_output_type(simple_type: BtcOutputType) -> bitbox02::app_btc::OutputType {
    match simple_type {
        BtcOutputType::Unknown => bitbox02::app_btc::OutputType::OUTPUT_TYPE_UNKNOWN,
        BtcOutputType::P2pkh => bitbox02::app_btc::OutputType::OUTPUT_TYPE_P2PKH,
        BtcOutputType::P2sh => bitbox02::app_btc::OutputType::OUTPUT_TYPE_P2SH,
        BtcOutputType::P2wpkh => bitbox02::app_btc::OutputType::OUTPUT_TYPE_P2WPKH,
        BtcOutputType::P2wsh => bitbox02::app_btc::OutputType::OUTPUT_TYPE_P2WSH,
        BtcOutputType::P2tr => bitbox02::app_btc::OutputType::OUTPUT_TYPE_P2TR,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::vec::Vec;

    #[test]
    fn test_address_from_payload() {
        let params_btc = super::super::params::get(pb::BtcCoin::Btc);
        let params_ltc = super::super::params::get(pb::BtcCoin::Ltc);
        {
            // BTC & LTC p2pkh

            let payload =
                b"\x67\xfe\x0b\xdd\xe7\x98\x46\x71\xf2\xed\x59\xbb\x68\xa9\x7d\x9c\xc6\x8a\x02\xe0";
            assert_eq!(
                address_from_payload(params_btc, BtcOutputType::P2pkh, payload),
                Ok("1AUrwD77AL5ax5zj2BhZQ1x43wA5NLsYg1".into())
            );
            assert_eq!(
                address_from_payload(params_ltc, BtcOutputType::P2pkh, payload),
                Ok("LUhpCRQwEzKeCtgtCKgrg31pG9XMZLm6qX".into())
            );
        }
        {
            // BTC & LTC p2wpkh

            let payload =
                b"\x3f\x0d\xc2\xe9\x14\x2d\x88\x39\xae\x9c\x90\xa1\x9c\xa8\x6c\x36\xd9\x23\xd8\xab";
            assert_eq!(
                address_from_payload(params_btc, BtcOutputType::P2wpkh, payload),
                Ok("bc1q8uxu96g59kyrnt5ujzsee2rvxmvj8k9trg5ltx".into())
            );
            assert_eq!(
                address_from_payload(params_ltc, BtcOutputType::P2wpkh, payload),
                Ok("ltc1q8uxu96g59kyrnt5ujzsee2rvxmvj8k9t85wmnk".into())
            );
        }

        {
            // BTC & LTC p2sh

            let payload =
                b"\x8d\xd0\x9c\x25\xc9\x28\xbe\x67\x66\xf4\x50\x73\x87\x0c\xe3\xbb\x93\x1f\x2f\x55";
            assert_eq!(
                address_from_payload(params_btc, BtcOutputType::P2sh, payload),
                Ok("3Ecs74kCeeAc6EKWMGe7RXupUoeeXPdyj7".into())
            );
            assert_eq!(
                address_from_payload(params_ltc, BtcOutputType::P2sh, payload),
                Ok("MLq1QxAAbm22tjbQT9dTFBADoWF6UwYB7R".into())
            );
        }

        {
            // BTC & LTC p2wsh

            let payload = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
            assert_eq!(
                address_from_payload(params_btc, BtcOutputType::P2wsh, payload),
                Ok("bc1qv9skzctpv9skzctpv9skzctpv9skzctpv9skzctpv9skzctpv9ss52vqes".into())
            );
            assert_eq!(
                address_from_payload(params_ltc, BtcOutputType::P2wsh, payload),
                Ok("ltc1qv9skzctpv9skzctpv9skzctpv9skzctpv9skzctpv9skzctpv9sshwzsr4".into())
            );
        }

        // Taproot addresses, test vectors from
        // https://github.com/bitcoin/bips/blob/fb5bd37d0cdec14b47c45fda7aba4f7e8f801690/bip-0086.mediawiki#Test_vectors
        {
            // First receiving address

            let payload = b"\xa6\x08\x69\xf0\xdb\xcf\x1d\xc6\x59\xc9\xce\xcb\xaf\x80\x50\x13\x5e\xa9\xe8\xcd\xc4\x87\x05\x3f\x1d\xc6\x88\x09\x49\xdc\x68\x4c";
            assert!(address_from_payload(params_ltc, BtcOutputType::P2tr, payload).is_err());
            assert_eq!(
                address_from_payload(params_btc, BtcOutputType::P2tr, payload),
                Ok("bc1p5cyxnuxmeuwuvkwfem96lqzszd02n6xdcjrs20cac6yqjjwudpxqkedrcr".into())
            );
        }
        {
            // Second receiving address

            let payload = b"\xa8\x2f\x29\x94\x4d\x65\xb8\x6a\xe6\xb5\xe5\xcc\x75\xe2\x94\xea\xd6\xc5\x93\x91\xa1\xed\xc5\xe0\x16\xe3\x49\x8c\x67\xfc\x7b\xbb";
            assert!(address_from_payload(params_ltc, BtcOutputType::P2tr, payload).is_err());
            assert_eq!(
                address_from_payload(params_btc, BtcOutputType::P2tr, payload),
                Ok("bc1p4qhjn9zdvkux4e44uhx8tc55attvtyu358kutcqkudyccelu0was9fqzwh".into())
            );
        }
        {
            // First change address

            let payload = b"\x88\x2d\x74\xe5\xd0\x57\x2d\x5a\x81\x6c\xef\x00\x41\xa9\x6b\x6c\x1d\xe8\x32\xf6\xf9\x67\x6d\x96\x05\xc4\x4d\x5e\x9a\x97\xd3\xdc";
            assert!(address_from_payload(params_ltc, BtcOutputType::P2tr, payload).is_err());
            assert_eq!(
                address_from_payload(params_btc, BtcOutputType::P2tr, payload),
                Ok("bc1p3qkhfews2uk44qtvauqyr2ttdsw7svhkl9nkm9s9c3x4ax5h60wqwruhk7".into())
            );
        }
    }

    #[test]
    fn test_format_amount() {
        let params = super::super::params::get(pb::BtcCoin::Btc);
        let params_tbtc = super::super::params::get(pb::BtcCoin::Tbtc);
        let params_ltc = super::super::params::get(pb::BtcCoin::Ltc);
        let params_tltc = super::super::params::get(pb::BtcCoin::Tltc);
        let tests: Vec<(&Params, FormatUnit, u64, Result<&str, Error>)> = vec![
            (
                params,
                FormatUnit::Default,
                1234567890,
                Ok("12.3456789 BTC"),
            ),
            (params, FormatUnit::Default, 0, Ok("0 BTC")),
            (params, FormatUnit::Sat, 0, Ok("0 sat")),
            (params, FormatUnit::Default, 1, Ok("0.00000001 BTC")),
            (params, FormatUnit::Sat, 1, Ok("1 sat")),
            (params, FormatUnit::Default, 2, Ok("0.00000002 BTC")),
            (params, FormatUnit::Sat, 2, Ok("2 sat")),
            (params, FormatUnit::Default, 10, Ok("0.0000001 BTC")),
            (params, FormatUnit::Sat, 10, Ok("10 sat")),
            (params, FormatUnit::Default, 15, Ok("0.00000015 BTC")),
            (params, FormatUnit::Default, 20, Ok("0.0000002 BTC")),
            (params, FormatUnit::Default, 300, Ok("0.000003 BTC")),
            (params, FormatUnit::Default, 370, Ok("0.0000037 BTC")),
            (params, FormatUnit::Default, 371, Ok("0.00000371 BTC")),
            (params, FormatUnit::Sat, 371, Ok("371 sat")),
            (params, FormatUnit::Default, 40000000000, Ok("400 BTC")),
            (params, FormatUnit::Sat, 40000000000, Ok("40000000000 sat")),
            (params, FormatUnit::Default, 4000000000, Ok("40 BTC")),
            (params, FormatUnit::Default, 400000000, Ok("4 BTC")),
            (params, FormatUnit::Default, 40000000, Ok("0.4 BTC")),
            (params, FormatUnit::Default, 4000000, Ok("0.04 BTC")),
            (params, FormatUnit::Default, 400000, Ok("0.004 BTC")),
            (params, FormatUnit::Default, 40000, Ok("0.0004 BTC")),
            (params, FormatUnit::Default, 4000, Ok("0.00004 BTC")),
            (params, FormatUnit::Default, 400, Ok("0.000004 BTC")),
            (params, FormatUnit::Default, 40, Ok("0.0000004 BTC")),
            (params, FormatUnit::Default, 4, Ok("0.00000004 BTC")),
            (params, FormatUnit::Default, 5432345, Ok("0.05432345 BTC")),
            (params, FormatUnit::Default, 54323452, Ok("0.54323452 BTC")),
            (params, FormatUnit::Default, 543234527, Ok("5.43234527 BTC")),
            (params, FormatUnit::Sat, 543234527, Ok("543234527 sat")),
            (
                params,
                FormatUnit::Default,
                5432345270,
                Ok("54.3234527 BTC"),
            ),
            (
                params,
                FormatUnit::Default,
                54323452708,
                Ok("543.23452708 BTC"),
            ),
            (params, FormatUnit::Default, 100000000, Ok("1 BTC")),
            (
                params,
                FormatUnit::Default,
                1234567800000001,
                Ok("12345678.00000001 BTC"),
            ),
            (
                params,
                FormatUnit::Sat,
                1234567800000001,
                Ok("1234567800000001 sat"),
            ),
            (
                params,
                FormatUnit::Default,
                0xffffffffffffffff,
                Ok("184467440737.09551615 BTC"),
            ),
            (
                params,
                FormatUnit::Default,
                0xffffffffffffffff - 5,
                Ok("184467440737.0955161 BTC"),
            ),
            // TBTC
            (
                params_tbtc,
                FormatUnit::Default,
                40001000000,
                Ok("400.01 TBTC"),
            ),
            // LTC
            (
                params_ltc,
                FormatUnit::Default,
                40001000000,
                Ok("400.01 LTC"),
            ),
            // TLTC
            (
                params_tltc,
                FormatUnit::Default,
                40001000000,
                Ok("400.01 TLTC"),
            ),
            // Failures
            // No sats in LTC
            (
                params_ltc,
                FormatUnit::Sat,
                40001000000,
                Err(Error::InvalidInput),
            ),
            // No sats in TLTC
            (
                params_tltc,
                FormatUnit::Sat,
                40001000000,
                Err(Error::InvalidInput),
            ),
        ];
        for (params, format_unit, satoshi, expected) in tests {
            assert_eq!(
                format_amount(params, format_unit, satoshi),
                expected.map(|s| s.into())
            );
        }
    }
}
