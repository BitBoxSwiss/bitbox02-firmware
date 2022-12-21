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

use super::pb;
use super::Error;

use alloc::string::String;
use alloc::vec::Vec;

use crate::workflow::confirm;

use pb::cardano_response::Response;
use pb::cardano_script_config::Config;
use pb::CardanoNetwork;

use bech32::{FromBase32, ToBase32, Variant};

use blake2::{
    digest::{Update, VariableOutput},
    Blake2bVar,
};

use super::params;

/// Size of the Blake2b hash of payment keys and scripts.
pub const ADDRESS_HASH_SIZE: usize = 28;

/// Decodes a bech32 Shelley payment address and validates that it was encoded for the right
/// network.
///
/// These address tags are accepted:
/// https://github.com/cardano-foundation/CIPs/blob/0081c890995ff94618145ae5beb7f288c029a86a/CIP-0019/CIP-0019.md#shelley-addresses
/// See also: https://github.com/input-output-hk/cardano-ledger-specs/blob/d0aa86ded0b973b09b629e5aa62aa1e71364d088/eras/alonzo/test-suite/cddl-files/alonzo.cddl#L119-L127
fn decode_shelley_payment_address(params: &params::Params, address: &str) -> Result<Vec<u8>, ()> {
    let (hrp, data, variant) = bech32::decode(address).or(Err(()))?;
    if variant != Variant::Bech32 || hrp != params.bech32_hrp_payment {
        return Err(());
    }
    let data = Vec::from_base32(&data).or(Err(()))?;
    if data.is_empty() {
        return Err(());
    }
    let header = data[0];
    if header & 0b0000_1111 != params.network_id {
        return Err(());
    }
    let address_tag = header >> 4;
    if address_tag > 7 {
        return Err(());
    }
    Ok(data)
}

/// Decode a base58-encoded Byron payment address, validate it's checksum and that it was encoded for the right network.
///
/// A byron address is cbor encoded data: https://raw.githubusercontent.com/cardano-foundation/CIPs/0081c890995ff94618145ae5beb7f288c029a86a/CIP-0019/CIP-0019-byron-addresses.cddl
///
/// See also:
/// - https://github.com/input-output-hk/cardano-ledger/blob/d0aa86ded0b973b09b629e5aa62aa1e71364d088/eras/alonzo/test-suite/cddl-files/alonzo.cddl#L134-L135
/// - https://github.com/input-output-hk/technical-docs/blob/8d4f08bc05ec611f3943cdc09a4ae18e72a0eb3c/cardano-components/cardano-wallet/doc/About-Address-Format---Byron.md
fn decode_byron_payment_address(params: &params::Params, address: &str) -> Result<Vec<u8>, ()> {
    let base58_decoded = bs58::decode(address).into_vec().or(Err(()))?;
    let payload = {
        let mut decoder = minicbor::Decoder::new(&base58_decoded);
        if decoder.array().or(Err(()))?.ok_or(())? != 2 {
            return Err(());
        }
        if decoder.tag().or(Err(()))? != minicbor::data::Tag::Cbor {
            return Err(());
        }
        let payload = decoder.bytes().or(Err(()))?;
        let address_crc = decoder.u32().or(Err(()))?;
        if crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(payload) != address_crc {
            return Err(());
        }
        payload
    };

    let mut decoder = minicbor::Decoder::new(payload);
    // Array with three elements.
    if decoder.array().or(Err(()))?.ok_or(())? != 3 {
        return Err(());
    }
    // First element: address root digest.
    if decoder.bytes().or(Err(()))?.len() != 28 {
        return Err(());
    }
    // Second element: address attributes map.  Item with key 2 is the network magic. If absent, it
    // is mainnet. If present, must not be mainnet.
    let mut magic: Option<u32> = None;
    for item in decoder
        .map_iter::<u32, minicbor::bytes::ByteVec>()
        .or(Err(()))?
    {
        let (key, value) = item.or(Err(()))?;
        if key == 2 {
            magic = Some(minicbor::decode(&value).or(Err(()))?);
            break;
        }
    }
    match magic {
        None => {
            if params.network != CardanoNetwork::CardanoMainnet {
                return Err(());
            }
        }
        Some(magic) => {
            if params.network == CardanoNetwork::CardanoMainnet
                || magic != params.protocol_magic.ok_or(())?
            {
                return Err(());
            }
        }
    }
    // Third element: address type
    let typ = decoder.u32().or(Err(()))?;
    if typ != 0 && typ != 2 {
        return Err(());
    }
    Ok(base58_decoded)
}

/// Decode a Byron or Shelley payment address string and check that it was encoded for the right network.
pub fn decode_payment_address(params: &params::Params, address: &str) -> Result<Vec<u8>, Error> {
    if let Ok(address) = decode_shelley_payment_address(params, address) {
        return Ok(address);
    }
    if let Ok(address) = decode_byron_payment_address(params, address) {
        return Ok(address);
    }
    Err(Error::InvalidInput)
}

/// Returns the hash of the pubkey at the keypath. Returns an error if the keystore is locked.
pub fn pubkey_hash_at_keypath(keypath: &[u32]) -> Result<[u8; ADDRESS_HASH_SIZE], ()> {
    let xpub = crate::keystore::ed25519::get_xpub(keypath)?;
    let pubkey_bytes = xpub.pubkey_bytes();
    let mut hasher = Blake2bVar::new(ADDRESS_HASH_SIZE).unwrap();
    hasher.update(pubkey_bytes);
    let mut out = [0u8; ADDRESS_HASH_SIZE];
    hasher.finalize_variable(&mut out).or(Err(()))?;
    Ok(out)
}

/// See https://github.com/input-output-hk/cardano-ledger-specs/blob/d0aa86ded0b973b09b629e5aa62aa1e71364d088/eras/alonzo/test-suite/cddl-files/alonzo.cddl#L119-L127
fn address_header(params: &params::Params, script_config: &Config) -> u8 {
    let address_tag: u8 = match script_config {
        Config::PkhSkh(_) => 0,
    };

    address_tag << 4 | params.network_id
}

/// Encode the given address using bech32, validating that the keypaths are valid. If
/// `keypath_prefix` is provided, it is also validated that the address keypaths start with this
/// prefix.
pub fn validate_and_encode_payment_address(
    params: &params::Params,
    script_config: &Config,
    bip44_account: Option<u32>,
) -> Result<String, Error> {
    let header = address_header(params, script_config);

    match script_config {
        Config::PkhSkh(config) => {
            super::keypath::validate_address_shelley(
                &config.keypath_payment,
                &config.keypath_stake,
                bip44_account,
            )?;

            let payment_key_hash = pubkey_hash_at_keypath(&config.keypath_payment)?;
            let stake_key_hash = pubkey_hash_at_keypath(&config.keypath_stake)?;

            let mut bytes: Vec<u8> = Vec::with_capacity(1 + 2 * ADDRESS_HASH_SIZE);
            bytes.push(header);
            bytes.extend_from_slice(&payment_key_hash);
            bytes.extend_from_slice(&stake_key_hash);

            Ok(bech32::encode(
                params.bech32_hrp_payment,
                bytes.to_base32(),
                Variant::Bech32,
            )
            .unwrap())
        }
    }
}

pub async fn process(request: &pb::CardanoAddressRequest) -> Result<Response, Error> {
    let network = CardanoNetwork::from_i32(request.network).ok_or(Error::InvalidInput)?;
    let params = params::get(network);
    let script_config: &Config = request
        .script_config
        .as_ref()
        .ok_or(Error::InvalidInput)?
        .config
        .as_ref()
        .ok_or(Error::InvalidInput)?;

    let encoded_address = validate_and_encode_payment_address(params, script_config, None)?;

    if request.display {
        confirm::confirm(&confirm::Params {
            title: params.name,
            body: &encoded_address,
            scrollable: true,
            ..Default::default()
        })
        .await?;
    }

    Ok(Response::Pub(pb::PubResponse {
        r#pub: encoded_address,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bb02_async::block_on;
    use alloc::boxed::Box;
    use bitbox02::testing::{mock, mock_unlocked, Data};
    use util::bip32::HARDENED;

    #[test]
    fn test_decode_payment_address() {
        // See https://github.com/cardano-foundation/CIPs/blob/0081c890995ff94618145ae5beb7f288c029a86a/CIP-0019/CIP-0019.md#test-vectors
        // One for each Shelley address type, except for stake addresses.
        //
        // Apart from the above, some Byron addresses are added.

        let valid_addresses_mainnet = vec![
            "addr1qx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgse35a3x",
            "addr1z8phkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gten0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgs9yc0hh",
            "addr1yx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzerkr0vd4msrxnuwnccdxlhdjar77j6lg0wypcc9uar5d2shs2z78ve",
            "addr1x8phkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gt7r0vd4msrxnuwnccdxlhdjar77j6lg0wypcc9uar5d2shskhj42g",
            "addr1gx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer5pnz75xxcrzqf96k",
            "addr128phkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gtupnz75xxcrtw79hu",
            "addr1vx2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzers66hrl8",
            "addr1w8phkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gtcyjy7wx",

            // Byron addresses:
            "Ae2tdPwUPEZFRbyhz3cpfC2CumGzNkFBN2L42rcUc2yjQpEkxDbkPodpMAi", // Yoroi style
            "DdzFFzCqrhtC3C4UY8YFaEyDALJmFAwhx4Kggk3eae3BT9PhymMjzCVYhQE753BH1Rp3LXfVkVaD1FHT4joSBq7Y8rcXbbVWoxkqB7gy", // Daedalus style
        ];

        for address in &valid_addresses_mainnet {
            assert!(
                decode_payment_address(params::get(CardanoNetwork::CardanoMainnet), address)
                    .is_ok()
            );
            assert!(
                decode_payment_address(params::get(CardanoNetwork::CardanoTestnet), address)
                    .is_err()
            );
        }

        let valid_addresses_testnet = vec![
            "addr_test1qz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer3n0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgs68faae",
            "addr_test1zrphkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gten0d3vllmyqwsx5wktcd8cc3sq835lu7drv2xwl2wywfgsxj90mg",
            "addr_test1yz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzerkr0vd4msrxnuwnccdxlhdjar77j6lg0wypcc9uar5d2shsf5r8qx",
            "addr_test1xrphkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gt7r0vd4msrxnuwnccdxlhdjar77j6lg0wypcc9uar5d2shs4p04xh",
            "addr_test1gz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzer5pnz75xxcrdw5vky",
            "addr_test12rphkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gtupnz75xxcryqrvmw",
            "addr_test1vz2fxv2umyhttkxyxp8x0dlpdt3k6cwng5pxj3jhsydzerspjrlsz",
            "addr_test1wrphkx6acpnf78fuvxn0mkew3l0fd058hzquvz7w36x4gtcl6szpr",

            // Byron addresses:
            "37btjrVyb4KEB2STADSsj3MYSAdj52X5FrFWpw2r7Wmj2GDzXjFRsHWuZqrw7zSkwopv8Ci3VWeg6bisU9dgJxW5hb2MZYeduNKbQJrqz3zVBsu9nT", // Daedalus style

        ];

        for address in &valid_addresses_testnet {
            assert!(
                decode_payment_address(params::get(CardanoNetwork::CardanoTestnet), address)
                    .is_ok()
            );
            assert!(
                decode_payment_address(params::get(CardanoNetwork::CardanoMainnet), address)
                    .is_err()
            );
        }
    }

    fn make_pkh_skh(keypath_payment: &[u32], keypath_stake: &[u32]) -> pb::CardanoScriptConfig {
        pb::CardanoScriptConfig {
            config: Some(Config::PkhSkh(pb::cardano_script_config::PkhSkh {
                keypath_payment: keypath_payment.to_vec(),
                keypath_stake: keypath_stake.to_vec(),
            })),
        }
    }

    fn do_pkh_skh(keypath_payment: &[u32], keypath_stake: &[u32]) -> Result<Response, Error> {
        block_on(process(&pb::CardanoAddressRequest {
            network: CardanoNetwork::CardanoMainnet as _,
            display: false,
            script_config: Some(make_pkh_skh(keypath_payment, keypath_stake)),
        }))
    }

    #[test]
    fn test_pubkey_hash_at_keypath() {
        bitbox02::keystore::lock();
        assert!(
            pubkey_hash_at_keypath(&[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0]).is_err()
        );

        mock_unlocked();
        assert_eq!(
            pubkey_hash_at_keypath(&[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0]),
            Ok(*b"\x5e\xbf\xc2\xcd\xae\xef\x4b\x4f\x1b\xe7\xfc\xc3\x1c\xfe\x94\x5e\xb9\x2d\x28\x67\x43\x49\xbd\x0f\x1a\x4a\x00\x63")
        );
    }

    #[test]
    fn test_process_failures() {
        // All good
        mock_unlocked();
        assert_eq!(
            do_pkh_skh(
                &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
            ),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs".into()
            }))
        );

        // Keystore locked
        bitbox02::keystore::lock();
        assert_eq!(
            do_pkh_skh(
                &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
            ),
            Err(Error::Generic)
        );

        // Wrong keypath purpose
        mock_unlocked();
        assert_eq!(
            do_pkh_skh(
                &[1815 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                &[1815 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
            ),
            Err(Error::InvalidInput),
        );

        // Payment and staking keypath on different accounts
        mock_unlocked();
        assert_eq!(
            do_pkh_skh(
                &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                &[1852 + HARDENED, 1815 + HARDENED, HARDENED + 1, 2, 0],
            ),
            Err(Error::InvalidInput),
        );

        // Invalid staking key
        mock_unlocked();
        assert_eq!(
            do_pkh_skh(
                &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 1, 0],
            ),
            Err(Error::InvalidInput),
        );
        assert_eq!(
            do_pkh_skh(
                &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 1],
            ),
            Err(Error::InvalidInput),
        );
    }

    #[test]
    fn test_process_confirm() {
        const EXPECTED: &str = "addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs";

        mock(Data {
            ui_confirm_create: Some(Box::new(|params| {
                assert_eq!(params.title, "Cardano");
                assert_eq!(params.body, EXPECTED);
                true
            })),
            ..Default::default()
        });
        mock_unlocked();

        assert_eq!(
            block_on(process(&pb::CardanoAddressRequest {
                network: CardanoNetwork::CardanoMainnet as _,
                display: true,
                script_config: Some(make_pkh_skh(
                    &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                    &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0]
                )),
            })),
            Ok(Response::Pub(pb::PubResponse {
                r#pub: EXPECTED.into()
            }))
        );
    }

    #[test]
    fn test_process_table() {
        struct Test<'a> {
            keypath_payment: &'a [u32],
            keypath_stake: &'a [u32],
            expected_address: &'a str,
        }
        let tests = &[
            Test {
                keypath_payment: &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 0],
                keypath_stake: &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                expected_address:"addr1q90tlskd4mh5kncmul7vx887j30tjtfgvap5n0g0rf9qqc7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqmvu6hs",
            },
            Test {
                keypath_payment: &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 0, 10],
                keypath_stake: &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                expected_address:"addr1qxgr8vtpxq6tzghua0ye8tz869y8w5vs3xr6qk83vzmpy2xznmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqatkd04",
            },
            Test {
                keypath_payment: &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 1, 10],
                keypath_stake: &[1852 + HARDENED, 1815 + HARDENED, HARDENED, 2, 0],
                expected_address:"addr1qy6wl9mazd7w8s303a3t6hjx9k3qqjxzcyfrqjug8wu5uw7znmndrdhe7rwvqkw5c7mqnp4a3yflnvu6kff7l5dungvqvlsgvu",
            },
            Test {
                keypath_payment: &[1852 + HARDENED, 1815 + HARDENED, HARDENED+50, 1, 10],
                keypath_stake: &[1852 + HARDENED, 1815 + HARDENED, HARDENED+50, 2, 0],
                expected_address:"addr1q9t8qctl2mg55fvxrlgnlctf70hww5gtj9cgzrane7nj0amdad2jzalmf2zvjnw9x4z8e5emcqklue3gz85vadsgfutq96mqmx",
            },
        ];

        mock_unlocked();
        for test in tests {
            assert_eq!(
                do_pkh_skh(test.keypath_payment, test.keypath_stake),
                Ok(Response::Pub(pb::PubResponse {
                    r#pub: test.expected_address.into()
                }))
            );
        }
    }
}
