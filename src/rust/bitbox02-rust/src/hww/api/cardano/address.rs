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

use bech32::{ToBase32, Variant};

use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};

use super::params;

/// Size of the Blake2b hash of payment keys and scripts.
const ADDRESS_HASH_SIZE: usize = 28;

/// Returns the hash of the pubkey at the keypath. Returns an error if the keystore is locked.
fn pubkey_hash_at_keypath(keypath: &[u32]) -> Result<[u8; ADDRESS_HASH_SIZE], ()> {
    let xpub = crate::keystore::ed25519::get_xpub(keypath)?;
    let pubkey_bytes = xpub.pubkey_bytes();
    let mut hasher = VarBlake2b::new(ADDRESS_HASH_SIZE).unwrap();
    hasher.update(pubkey_bytes);
    let mut out = [0u8; ADDRESS_HASH_SIZE];
    hasher.finalize_variable(|res| out.copy_from_slice(res));
    Ok(out)
}

fn address_header(params: &params::Params, script_config: &Config) -> u8 {
    let address_tag: u8 = match script_config {
        Config::PkhSkh(_) => 0,
    };

    address_tag << 4 | params.network_id
}

/// Encode the given address using bech32, validating that the keypaths are valid.
pub fn validate_and_encode_address(
    params: &params::Params,
    script_config: &Config,
) -> Result<String, Error> {
    let header = address_header(params, script_config);

    match script_config {
        Config::PkhSkh(config) => {
            super::keypath::validate_address_shelley(
                &config.keypath_payment,
                &config.keypath_stake,
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

    let encoded_address = validate_and_encode_address(params, script_config)?;

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
    use bitbox02::testing::{mock, mock_unlocked, Data, MUTEX};
    use util::bip32::HARDENED;

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
        let _guard = MUTEX.lock().unwrap();
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
        let _guard = MUTEX.lock().unwrap();

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
        let _guard = MUTEX.lock().unwrap();

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
        let _guard = MUTEX.lock().unwrap();

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
