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
    use bitbox02::testing::{mock, mock_unlocked, Data, MUTEX};
    use std::boxed::Box;
    use util::bip32::HARDENED;

    const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
    const MESSAGE: &[u8] = b"message";
    const EXPECTED_ADDRESS: &str = "0x773A77b9D32589be03f9132AF759e294f7851be9";

    #[test]
    pub fn test_process() {
        let _guard = MUTEX.lock().unwrap();

        const EXPECTED_SIGHASH: &[u8; 32] = b"\x7f\x6c\x0e\x5c\x49\x7d\xed\x52\x46\x2e\xc1\x8d\xae\xb1\xc9\x4c\xef\xa1\x1c\xd6\x94\x9e\xbd\xb7\x07\x4b\x2a\x32\xca\xc1\x3f\xba";
        const SIGNATURE: [u8; 64] = [b'1'; 64];

        static mut CONFIRM_COUNTER: u32 = 0;

        mock(Data {
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
        mock_unlocked();
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

        let request = pb::EthSignMessageRequest {
            coin: pb::EthCoin::Eth as _,
            keypath: KEYPATH.to_vec(),
            msg: MESSAGE.to_vec(),
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        // User abort address verification.
        mock(Data {
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
        mock_unlocked();
        assert_eq!(block_on(process(&request)), Err(Error::UserAbort));

        // User abort message verification.
        unsafe {
            CONFIRM_COUNTER = 0;
        }
        mock(Data {
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
        mock_unlocked();
        assert_eq!(block_on(process(&request)), Err(Error::UserAbort));
    }

    #[test]
    pub fn test_process_failures() {
        let _guard = MUTEX.lock().unwrap();

        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

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
            ui_confirm_create: Some(Box::new(|_| true)),
            keystore_secp256k1_sign: Some(Box::new(|_, _, _| Err(()))),
            ..Default::default()
        });
        mock_unlocked();
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
