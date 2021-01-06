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
use super::Error;

use crate::workflow::verify_message;

use pb::eth_response::Response;

use alloc::vec::Vec;
use core::convert::TryInto;

use sha3::digest::Digest;

/// Process a sign message request.
///
/// The result contains a 65 byte signature. The first 64 bytes are the secp256k1 signature in
/// compact format (R and S values), and the last byte is the recoverable id (recid).
pub async fn process(request: &pb::EthSignMessageRequest) -> Result<Response, Error> {
    if request.msg.len() > 9999 {
        return Err(Error::InvalidInput);
    }
    let pub_request = pb::EthPubRequest {
        output_type: pb::eth_pub_request::OutputType::Address as _,
        keypath: request.keypath.clone(),
        coin: request.coin,
        display: true,
        contract_address: Vec::new(),
    };

    // Verify address. We don't need the actual result, but we have to propagate validation or user
    // abort errors.
    super::pubrequest::process(&pub_request).await?;

    verify_message::verify(&request.msg).await?;

    // Construct message to be signed. There is no standard for this. We match what MyEtherWallet,
    // Trezor, etc. do, e.g.:
    // https://github.com/ethereumjs/ethereumjs-util/blob/dd2882d790c1d3b50b75bee6f88031433cbd5bef/src/signature.ts#L140
    let mut msg: Vec<u8> = Vec::new();
    msg.extend(b"\x19Ethereum Signed Message:\n");
    msg.extend(format!("{}", request.msg.len()).as_bytes());
    msg.extend(&request.msg);

    let sighash: [u8; 32] = sha3::Keccak256::digest(&msg).as_slice().try_into().unwrap();
    let host_nonce = [0; 32]; // TODO: get nonce contribution from host.
    let sign_result = bitbox02::keystore::secp256k1_sign(&request.keypath, &sighash, &host_nonce)?;

    let mut signature: Vec<u8> = sign_result.signature.to_vec();
    signature.push(sign_result.recid);

    Ok(Response::Sign(pb::EthSignResponse { signature }))
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    use crate::bb02_async::block_on;
    use bitbox02::testing::{mock, Data, MUTEX};
    use std::boxed::Box;
    use util::bip32::HARDENED;

    const PUBKEY: [u8; 65] = [
        0x04, 0xd8, 0xae, 0xa8, 0x0d, 0x2d, 0xbc, 0xeb, 0xbe, 0x10, 0xfd, 0xfa, 0xc2, 0xd2, 0xdb,
        0x19, 0x64, 0x15, 0x5b, 0xa9, 0x9e, 0x0d, 0xd7, 0xbf, 0xd5, 0xcf, 0xfe, 0xd9, 0x7a, 0x1c,
        0xae, 0xf7, 0xd0, 0xb9, 0x07, 0x2d, 0x9c, 0x0f, 0x50, 0x49, 0x30, 0xef, 0x59, 0xb7, 0x52,
        0xd4, 0xfe, 0xa0, 0xcb, 0xde, 0x3e, 0x27, 0x3e, 0xe9, 0x54, 0xd8, 0xda, 0xc8, 0xee, 0x03,
        0x1a, 0x4e, 0xd1, 0x71, 0xfd,
    ];
    const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
    const MESSAGE: &[u8] = b"message";
    const EXPECTED_ADDRESS: &str = "0xF4C21710Ef8b5a5Ec4bd3780A687FE083446e67B";

    #[test]
    pub fn test_process() {
        let _guard = MUTEX.lock().unwrap();

        const EXPECTED_SIGHASH: &[u8; 32] = b"\x7f\x6c\x0e\x5c\x49\x7d\xed\x52\x46\x2e\xc1\x8d\xae\xb1\xc9\x4c\xef\xa1\x1c\xd6\x94\x9e\xbd\xb7\x07\x4b\x2a\x32\xca\xc1\x3f\xba";
        const SIGNATURE: [u8; 64] = [b'1'; 64];

        static mut CONFIRM_COUNTER: u32 = 0;

        mock(Data {
            eth_params_get: Some(Box::new(|coin| {
                assert_eq!(coin, pb::EthCoin::Eth as _);
                Some(bitbox02::app_eth::Params {
                    bip44_coin: 60 + HARDENED,
                    chain_id: 1,
                    unit: "ETH",
                })
            })),
            keystore_secp256k1_pubkey_uncompressed: Some(Box::new(|_| Ok(PUBKEY))),
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Ethereum");
                        assert_eq!(params.body, EXPECTED_ADDRESS);
                        true
                    }
                    2 => {
                        assert_eq!(params.title, "Sign message");
                        assert_eq!(params.body.as_bytes(), MESSAGE);
                        true
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            keystore_secp256k1_sign: Some(Box::new(|keypath, sighash, _host_nonce| {
                assert_eq!(keypath, KEYPATH);
                assert_eq!(sighash, EXPECTED_SIGHASH);
                Ok(bitbox02::keystore::SignResult {
                    signature: [b'1'; 64],
                    recid: 3,
                })
            })),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&pb::EthSignMessageRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                msg: MESSAGE.to_vec(),
            })),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: b"1111111111111111111111111111111111111111111111111111111111111111\x03"
                    .to_vec()
            }))
        );
    }

    #[test]
    pub fn test_process_user_aborted() {
        let _guard = MUTEX.lock().unwrap();

        const ETH_PARAMS: Option<bitbox02::app_eth::Params> = Some(bitbox02::app_eth::Params {
            bip44_coin: 60 + HARDENED,
            chain_id: 1,
            unit: "ETH",
        });
        let request = pb::EthSignMessageRequest {
            coin: pb::EthCoin::Eth as _,
            keypath: KEYPATH.to_vec(),
            msg: MESSAGE.to_vec(),
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        // User abort address verification.
        mock(Data {
            eth_params_get: Some(Box::new(|_| ETH_PARAMS)),
            keystore_secp256k1_pubkey_uncompressed: Some(Box::new(|_| Ok(PUBKEY))),
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => {
                        assert_eq!(params.title, "Ethereum");
                        assert_eq!(params.body, EXPECTED_ADDRESS);
                        false
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ..Default::default()
        });
        assert_eq!(block_on(process(&request)), Err(Error::UserAbort));

        // User abort message verification.
        unsafe {
            CONFIRM_COUNTER = 0;
        }
        mock(Data {
            eth_params_get: Some(Box::new(|_| ETH_PARAMS)),
            keystore_secp256k1_pubkey_uncompressed: Some(Box::new(|_| Ok(PUBKEY))),
            ui_confirm_create: Some(Box::new(|params| {
                match unsafe {
                    CONFIRM_COUNTER += 1;
                    CONFIRM_COUNTER
                } {
                    1 => true,
                    2 => {
                        assert_eq!(params.title, "Sign message");
                        assert_eq!(params.body.as_bytes(), MESSAGE);
                        false
                    }
                    _ => panic!("too many user confirmations"),
                }
            })),
            ..Default::default()
        });
        assert_eq!(block_on(process(&request)), Err(Error::UserAbort));
    }

    #[test]
    pub fn test_process_failures() {
        let _guard = MUTEX.lock().unwrap();

        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
        const ETH_PARAMS: Option<bitbox02::app_eth::Params> = Some(bitbox02::app_eth::Params {
            bip44_coin: 60 + HARDENED,
            chain_id: 1,
            unit: "ETH",
        });

        // Message too long
        assert_eq!(
            block_on(process(&pb::EthSignMessageRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                msg: [0; 10000].to_vec(),
            })),
            Err(Error::InvalidInput)
        );

        // Signing failed.
        mock(Data {
            eth_params_get: Some(Box::new(|_| ETH_PARAMS)),
            keystore_secp256k1_pubkey_uncompressed: Some(Box::new(|_| Ok(PUBKEY))),
            ui_confirm_create: Some(Box::new(|_| true)),
            keystore_secp256k1_sign: Some(Box::new(|_, _, _| Err(()))),
            ..Default::default()
        });
        assert_eq!(
            block_on(process(&&pb::EthSignMessageRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                msg: b"message".to_vec(),
            })),
            Err(Error::Generic)
        );
    }
}
