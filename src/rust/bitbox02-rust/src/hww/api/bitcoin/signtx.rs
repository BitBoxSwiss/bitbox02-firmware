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

use alloc::vec::Vec;

use pb::request::Request;
use pb::response::Response;

use prost::Message;

use pb::btc_sign_next_response::Type as NextType;

fn encode<M: Message>(msg: &M) -> Vec<u8> {
    let mut serialized = Vec::<u8>::new();
    msg.encode(&mut serialized).unwrap();
    serialized
}

/// After each request from the host, we send a `BtcSignNextResponse` response back to the host,
/// containing information which request we want next, and containing additional metadata if
/// available (e.g. a signature after signing an input).
struct NextResponse {
    next: pb::BtcSignNextResponse,
    /// If true, `next` is wrapped in the `BTCResponse` protobuf message, otherwise it is sent
    /// directly in a `Response` message.
    wrap: bool,
}

impl NextResponse {
    fn to_protobuf(&self) -> Response {
        if self.wrap {
            Response::Btc(pb::BtcResponse {
                response: Some(pb::btc_response::Response::SignNext(self.next.clone())),
            })
        } else {
            Response::BtcSignNext(self.next.clone())
        }
    }
}

/// Wait for the next request sent by the host. Since host<->device communication is a
/// request/response pattern, we have to send a response (to the previous request) before getting
/// the next request.
///
/// In BTC signing, the response is always a `BtcSignNextResponse`, but depending on the previous
/// request, it is either a direct response result (hww.proto:Response), or a a result wrapped in a
/// `BTCResponse` (which was introduced latter, hence the messages are scattered). `response.wrap`
/// is set so the next call to this function wraps the response correctly.
///
/// The NextResponse contains information for the host as to which request we need, but also
/// additional results, e.g. a signature after an input is signed. The response is reset to default
/// values after this call so that this additional data is only sent once.
async fn get_request(
    typ: NextType,
    index: u32,
    prev_index: Option<u32>,
    response: &mut NextResponse,
) -> Result<Request, Error> {
    response.next.r#type = typ as _;
    response.next.index = index;
    if let Some(prev_index) = prev_index {
        response.next.prev_index = prev_index;
    }
    let request = crate::hww::next_request(response.to_protobuf()).await?;
    response.next = pb::BtcSignNextResponse {
        r#type: 0,
        index: 0,
        has_signature: false,
        signature: vec![],
        prev_index: 0,
        anti_klepto_signer_commitment: None,
    };
    Ok(request)
}

async fn get_tx_input(
    index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcSignInputRequest, Error> {
    let request = get_request(NextType::Input, index, None, response).await?;
    response.wrap = false;
    match request {
        Request::BtcSignInput(request) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_prevtx_init(
    index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcPrevTxInitRequest, Error> {
    response.next.r#type = NextType::PrevtxInit as _;
    response.next.index = index;
    let request = get_request(NextType::PrevtxInit, index, None, response).await?;
    response.wrap = true;
    match request {
        Request::Btc(pb::BtcRequest {
            request: Some(pb::btc_request::Request::PrevtxInit(request)),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_prevtx_input(
    input_index: u32,
    prevtx_input_index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcPrevTxInputRequest, Error> {
    let request = get_request(
        NextType::PrevtxInput,
        input_index,
        Some(prevtx_input_index),
        response,
    )
    .await?;
    response.wrap = true;
    match request {
        Request::Btc(pb::BtcRequest {
            request: Some(pb::btc_request::Request::PrevtxInput(request)),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_prevtx_output(
    output_index: u32,
    prevtx_output_index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcPrevTxOutputRequest, Error> {
    let request = get_request(
        NextType::PrevtxOutput,
        output_index,
        Some(prevtx_output_index),
        response,
    )
    .await?;
    response.wrap = true;
    match request {
        Request::Btc(pb::BtcRequest {
            request: Some(pb::btc_request::Request::PrevtxOutput(request)),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_tx_output(
    index: u32,
    response: &mut NextResponse,
) -> Result<pb::BtcSignOutputRequest, Error> {
    let request = get_request(NextType::Output, index, None, response).await?;
    response.wrap = false;
    match request {
        Request::BtcSignOutput(request) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

async fn get_antiklepto_host_nonce(
    index: u32,
    response: &mut NextResponse,
) -> Result<pb::AntiKleptoSignatureRequest, Error> {
    let request = get_request(NextType::HostNonce, index, None, response).await?;
    response.wrap = true;
    match request {
        Request::Btc(pb::BtcRequest {
            request: Some(pb::btc_request::Request::AntikleptoSignature(request)),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

pub async fn process(request: &pb::BtcSignInitRequest) -> Result<Response, Error> {
    bitbox02::app_btc::sign_init_wrapper(encode(request).as_ref())?;
    let mut next_response = NextResponse {
        next: pb::BtcSignNextResponse {
            r#type: 0,
            index: 0,
            has_signature: false,
            signature: vec![],
            prev_index: 0,
            anti_klepto_signer_commitment: None,
        },
        wrap: false,
    };
    for input_index in 0..request.num_inputs {
        let tx_input = get_tx_input(input_index, &mut next_response).await?;
        bitbox02::app_btc::sign_input_pass1_wrapper(encode(&tx_input).as_ref())?;

        let prevtx_init = get_prevtx_init(input_index, &mut next_response).await?;
        bitbox02::app_btc::sign_prevtx_init_wrapper(encode(&prevtx_init).as_ref())?;
        for prevtx_input_index in 0..prevtx_init.num_inputs {
            let prevtx_input =
                get_prevtx_input(input_index, prevtx_input_index, &mut next_response).await?;
            bitbox02::app_btc::sign_prevtx_input_wrapper(encode(&prevtx_input).as_ref())?;
        }
        for prevtx_output_index in 0..prevtx_init.num_outputs {
            let prevtx_output =
                get_prevtx_output(input_index, prevtx_output_index, &mut next_response).await?;
            bitbox02::app_btc::sign_prevtx_output_wrapper(encode(&prevtx_output).as_ref())?;
        }
    }
    for output_index in 0..request.num_outputs {
        let tx_output = get_tx_output(output_index, &mut next_response).await?;
        bitbox02::app_btc::sign_output_wrapper(encode(&tx_output).as_ref())?;
    }
    for input_index in 0..request.num_inputs {
        let tx_input = get_tx_input(input_index, &mut next_response).await?;
        let (signature, anti_klepto_signer_commitment) =
            bitbox02::app_btc::sign_input_pass2_wrapper(encode(&tx_input).as_ref())?;
        // Engage in the Anti-Klepto protocol if the host sends a host nonce commitment.
        if tx_input.host_nonce_commitment.is_some() {
            next_response.next.anti_klepto_signer_commitment =
                Some(pb::AntiKleptoSignerCommitment {
                    commitment: anti_klepto_signer_commitment,
                });

            let antiklepto_host_nonce =
                get_antiklepto_host_nonce(input_index, &mut next_response).await?;

            next_response.next.has_signature = true;
            next_response.next.signature = bitbox02::app_btc::sign_antiklepto_wrapper(
                encode(&antiklepto_host_nonce).as_ref(),
            )?;
        } else {
            next_response.next.has_signature = true;
            next_response.next.signature = signature;
        }
    }
    next_response.next.r#type = NextType::Done as _;
    Ok(next_response.to_protobuf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bb02_async::block_on;
    use alloc::boxed::Box;
    use bitbox02::testing::{mock, mock_unlocked, Data, MUTEX};
    use util::bip32::HARDENED;

    struct TxInput {
        input: pb::BtcSignInputRequest,
        prevtx_version: u32,
        prevtx_inputs: Vec<pb::BtcPrevTxInputRequest>,
        prevtx_outputs: Vec<pb::BtcPrevTxOutputRequest>,
        prevtx_locktime: u32,
    }

    struct Transaction {
        version: u32,
        inputs: Vec<TxInput>,
        outputs: Vec<pb::BtcSignOutputRequest>,
        locktime: u32,
    }

    impl Transaction {
        /// An arbitrary test transaction with some inputs and outputs.
        fn new() -> Self {
            Transaction {
                version: 1,
                inputs: vec![
                    TxInput {
                        input: pb::BtcSignInputRequest {
                            prev_out_hash: vec![
                                0x45, 0x17, 0x74, 0x50, 0x1b, 0xaf, 0xdf, 0xf7, 0x46, 0x9, 0xe,
                                0x6, 0x16, 0xd9, 0x5e, 0xd0, 0x80, 0xd7, 0x82, 0x9a, 0xfe, 0xa2,
                                0xbd, 0x97, 0x8a, 0xf8, 0x11, 0xf4, 0x5e, 0x43, 0x81, 0x39,
                            ],
                            prev_out_index: 1,
                            prev_out_value: 1010000000,
                            sequence: 0xffffffff,
                            keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED, 0, 5],
                            script_config_index: 0,
                            host_nonce_commitment: None,
                        },
                        prevtx_version: 1,
                        prevtx_inputs: vec![
                            pb::BtcPrevTxInputRequest {
                                prev_out_hash: vec![
                                    0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                    0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                    0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                    0x74, 0x74,
                                ],
                                prev_out_index: 3,
                                signature_script: b"signature script".to_vec(),
                                sequence: 0xffffffff - 2,
                            },
                            pb::BtcPrevTxInputRequest {
                                prev_out_hash: vec![
                                    0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
                                    0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
                                    0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75,
                                    0x75, 0x75,
                                ],
                                prev_out_index: 23,
                                signature_script: b"signature script 2".to_vec(),
                                sequence: 123456,
                            },
                        ],
                        prevtx_outputs: vec![
                            pb::BtcPrevTxOutputRequest {
                                value: 101000000, // btc 1.01
                                pubkey_script: b"pubkey script".to_vec(),
                            },
                            pb::BtcPrevTxOutputRequest {
                                value: 1010000000, // btc 10.1
                                pubkey_script: b"pubkey script 2".to_vec(),
                            },
                        ],
                        prevtx_locktime: 0,
                    },
                    TxInput {
                        input: pb::BtcSignInputRequest {
                            prev_out_hash: vec![
                                0x40, 0x9b, 0x4f, 0x56, 0xca, 0x9f, 0x6, 0xcb, 0x88, 0x28, 0x3,
                                0xad, 0x55, 0x4b, 0xeb, 0x1d, 0x9e, 0xf8, 0x78, 0x7, 0xf0, 0x52,
                                0x29, 0xe7, 0x55, 0x15, 0xe4, 0xb2, 0xaa, 0x87, 0x69, 0x1d,
                            ],
                            prev_out_index: 0,
                            prev_out_value: 1020000000, // btc 10.2, matches prevout tx output at index 0.
                            sequence: 0xffffffff,
                            keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED, 0, 7],
                            script_config_index: 0,
                            host_nonce_commitment: None,
                        },
                        prevtx_version: 2,
                        prevtx_inputs: vec![pb::BtcPrevTxInputRequest {
                            prev_out_hash: vec![
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                                0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74, 0x74,
                            ],
                            prev_out_index: 3,
                            signature_script: b"signature script".to_vec(),
                            sequence: 0xffffffff - 2,
                        }],
                        prevtx_outputs: vec![pb::BtcPrevTxOutputRequest {
                            value: 1020000000, // btc 10.2
                            pubkey_script: b"pubkey script".to_vec(),
                        }],
                        prevtx_locktime: 87654,
                    },
                ],
                outputs: vec![
                    pb::BtcSignOutputRequest {
                        ours: false,
                        r#type: pb::BtcOutputType::P2pkh as _,
                        value: 100000000, // btc 1,
                        payload: vec![
                            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                            0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                        ],
                        keypath: vec![],
                        script_config_index: 0,
                    },
                    pb::BtcSignOutputRequest {
                        ours: false,
                        r#type: pb::BtcOutputType::P2sh as _,
                        value: 1234567890, // btc 12.3456789,
                        payload: vec![
                            0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                            0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                        ],
                        keypath: vec![],
                        script_config_index: 0,
                    },
                    pb::BtcSignOutputRequest {
                        ours: false,
                        r#type: pb::BtcOutputType::P2wpkh as _,
                        value: 6000, // btc .00006
                        payload: vec![
                            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
                            0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
                        ],
                        keypath: vec![],
                        script_config_index: 0,
                    },
                    pb::BtcSignOutputRequest {
                        ours: false,
                        r#type: pb::BtcOutputType::P2wsh as _,
                        value: 7000, // btc .00007
                        payload: vec![
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                            0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 0x44,
                        ],
                        keypath: vec![],
                        script_config_index: 0,
                    },
                    pb::BtcSignOutputRequest {
                        // change
                        ours: true,
                        r#type: 0,
                        value: 690000000, // btc 6.9
                        payload: vec![],
                        keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED, 1, 3],
                        script_config_index: 0,
                    },
                    pb::BtcSignOutputRequest {
                        // change #2
                        ours: true,
                        r#type: 0,
                        value: 100,
                        payload: vec![],
                        keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED, 1, 30],
                        script_config_index: 0,
                    },
                ],
                locktime: 0,
            }
        }

        fn init_request(&self) -> pb::BtcSignInitRequest {
            pb::BtcSignInitRequest {
                coin: pb::BtcCoin::Btc as _,
                script_configs: vec![pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::SimpleType(
                            pb::btc_script_config::SimpleType::P2wpkh as _,
                        )),
                    }),
                    keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED],
                }],
                version: self.version,
                num_inputs: self.inputs.len() as _,
                num_outputs: self.outputs.len() as _,
                locktime: self.locktime,
            }
        }
    }

    fn mock_host_responder(tx: alloc::rc::Rc<core::cell::RefCell<Transaction>>) {
        *crate::hww::MOCK_NEXT_REQUEST.0.borrow_mut() =
            Some(Box::new(move |response: Response| {
                let tx = tx.borrow();
                let next: pb::BtcSignNextResponse = match response {
                    Response::BtcSignNext(next) => next,
                    Response::Btc(pb::BtcResponse {
                        response: Some(pb::btc_response::Response::SignNext(next)),
                    }) => next,
                    _ => panic!("wrong response type"),
                };
                match NextType::from_i32(next.r#type).unwrap() {
                    NextType::Input => Ok(Request::BtcSignInput(
                        tx.inputs[next.index as usize].input.clone(),
                    )),
                    NextType::Output => Ok(Request::BtcSignOutput(
                        tx.outputs[next.index as usize].clone(),
                    )),
                    NextType::PrevtxInit => Ok(Request::Btc(pb::BtcRequest {
                        request: Some(pb::btc_request::Request::PrevtxInit(
                            pb::BtcPrevTxInitRequest {
                                version: tx.inputs[next.index as usize].prevtx_version,
                                num_inputs: tx.inputs[next.index as usize].prevtx_inputs.len() as _,
                                num_outputs: tx.inputs[next.index as usize].prevtx_outputs.len()
                                    as _,
                                locktime: tx.inputs[next.index as usize].prevtx_locktime,
                            },
                        )),
                    })),
                    NextType::PrevtxInput => Ok(Request::Btc(pb::BtcRequest {
                        request: Some(pb::btc_request::Request::PrevtxInput(
                            tx.inputs[next.index as usize].prevtx_inputs[next.prev_index as usize]
                                .clone(),
                        )),
                    })),
                    NextType::PrevtxOutput => Ok(Request::Btc(pb::BtcRequest {
                        request: Some(pb::btc_request::Request::PrevtxOutput(
                            tx.inputs[next.index as usize].prevtx_outputs[next.prev_index as usize]
                                .clone(),
                        )),
                    })),
                    _ => panic!("unexpected next response"),
                }
            }));
    }

    #[test]
    pub fn test_sign_init_fail() {
        let init_req_valid = pb::BtcSignInitRequest {
            coin: pb::BtcCoin::Btc as _,
            script_configs: vec![pb::BtcScriptConfigWithKeypath {
                script_config: Some(pb::BtcScriptConfig {
                    config: Some(pb::btc_script_config::Config::SimpleType(
                        pb::btc_script_config::SimpleType::P2wpkh as _,
                    )),
                }),
                keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED],
            }],
            version: 1,
            num_inputs: 1,
            num_outputs: 1,
            locktime: 0,
        };

        {
            // test invalid version
            let mut init_req_invalid = init_req_valid.clone();
            for version in 3..10 {
                init_req_invalid.version = version;
                assert_eq!(
                    block_on(process(&init_req_invalid)),
                    Err(Error::InvalidInput)
                );
            }
        }
        {
            // test invalid locktime
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.locktime = 500000000;
            assert_eq!(
                block_on(process(&init_req_invalid)),
                Err(Error::InvalidInput)
            );
        }
        {
            // test invalid inputs
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.num_inputs = 0;
            assert_eq!(
                block_on(process(&init_req_invalid)),
                Err(Error::InvalidInput)
            );
        }
        {
            // test invalid outputs
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.num_outputs = 0;
            assert_eq!(
                block_on(process(&init_req_invalid)),
                Err(Error::InvalidInput)
            );
        }
        {
            // test invalid coin
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.coin = 4; // BtcCoin is defined from 0 to 3.
            assert_eq!(
                block_on(process(&init_req_invalid)),
                Err(Error::InvalidInput)
            );
        }
        {
            // test invalid account keypath
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.script_configs[0].keypath[2] = HARDENED + 100;
            assert_eq!(
                block_on(process(&init_req_invalid)),
                Err(Error::InvalidInput)
            );
        }
        {
            // no script configs is invalid
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.script_configs = vec![];
            assert_eq!(
                block_on(process(&init_req_invalid)),
                Err(Error::InvalidInput)
            );
        }
        {
            // can't mix script configs from different bip44 accounts
            // (mixing input scripts is allowed, but only from the same bip44 account).
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.script_configs = vec![
                pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::SimpleType(
                            pb::btc_script_config::SimpleType::P2wpkh as _,
                        )),
                    }),
                    keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED],
                },
                pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::SimpleType(
                            pb::btc_script_config::SimpleType::P2wpkhP2sh as _,
                        )),
                    }),
                    keypath: vec![49 + HARDENED, 0 + HARDENED, 0 + HARDENED],
                },
            ];
            assert_eq!(
                block_on(process(&init_req_invalid)),
                Err(Error::InvalidInput)
            );
        }

        {
            // can't mix simple type (singlesig) and multisig configs in one tx
            let mut init_req_invalid = init_req_valid.clone();
            init_req_invalid.script_configs = vec![
                pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::SimpleType(
                            pb::btc_script_config::SimpleType::P2wpkh as _,
                        )),
                    }),
                    keypath: vec![84 + HARDENED, 0 + HARDENED, 10 + HARDENED],
                },
                pb::BtcScriptConfigWithKeypath {
                    script_config: Some(pb::BtcScriptConfig {
                        config: Some(pb::btc_script_config::Config::Multisig(
                            pb::btc_script_config::Multisig {
                                threshold: 1,
                                xpubs: vec![
                                    pb::XPub {
                                        ..Default::default()
                                    },
                                    pb::XPub {
                                        ..Default::default()
                                    },
                                ],
                                our_xpub_index: 0,
                                script_type: pb::btc_script_config::multisig::ScriptType::P2wsh
                                    as _,
                            },
                        )),
                    }),
                    keypath: vec![49 + HARDENED, 0 + HARDENED, 0 + HARDENED],
                },
            ];
            assert_eq!(
                block_on(process(&init_req_invalid)),
                Err(Error::InvalidInput)
            );
        }
    }

    #[test]
    pub fn test_process() {
        let _guard = MUTEX.lock().unwrap();

        let transaction = alloc::rc::Rc::new(core::cell::RefCell::new(Transaction::new()));

        let tx = transaction.clone();
        mock_host_responder(tx);
        mock_unlocked();
        static mut UI_COUNTER: u32 = 0;
        bitbox02::app_btc_sign_ui::mock(bitbox02::app_btc_sign_ui::Ui {
            verify_recipient: Box::new(|recipient, amount| {
                match unsafe {
                    UI_COUNTER += 1;
                    UI_COUNTER
                } {
                    1 => {
                        assert_eq!(recipient, "12ZEw5Hcv1hTb6YUQJ69y1V7uhcoDz92PH");
                        assert_eq!(amount, "1 BTC");
                        true
                    }
                    2 => {
                        assert_eq!(recipient, "34oVnh4gNviJGMnNvgquMeLAxvXJuaRVMZ");
                        assert_eq!(amount, "12.3456789 BTC");
                        true
                    }
                    3 => {
                        assert_eq!(recipient, "bc1qxvenxvenxvenxvenxvenxvenxvenxven2ymjt8");
                        assert_eq!(amount, "0.00006 BTC");
                        true
                    }
                    4 => {
                        assert_eq!(
                            recipient,
                            "bc1qg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zyg3zqd8sxw4"
                        );
                        assert_eq!(amount, "0.00007 BTC");
                        true
                    }
                    _ => panic!("unexpected UI dialog"),
                }
            }),
            confirm: Box::new(|title, body| {
                match unsafe {
                    UI_COUNTER += 1;
                    UI_COUNTER
                } {
                    5 => {
                        assert_eq!(title, "Warning");
                        assert_eq!(body, "There are 2\nchange outputs.\nProceed?");
                        true
                    }
                    _ => panic!("unexpected UI dialog"),
                }
            }),
            verify_total: Box::new(|total, fee| {
                match unsafe {
                    UI_COUNTER += 1;
                    UI_COUNTER
                } {
                    6 => {
                        assert_eq!(total, "13.399999 BTC");
                        assert_eq!(fee, "0.0541901 BTC");
                        true
                    }
                    _ => panic!("unexpected UI dialog"),
                }
            }),
            status: Box::new(|_msg, _status_success| {}),
        });
        let tx = transaction.borrow();
        let result = block_on(process(&tx.init_request()));
        match result {
            Ok(Response::BtcSignNext(next)) => {
                assert!(next.has_signature);
                assert_eq!(&next.signature, b"\x2e\x08\x4a\x0a\x5f\x9b\xab\xb3\x5d\xf6\xec\x3a\x89\x72\x0b\xcf\xc0\x88\xd4\xba\x6a\xee\x47\x97\x3c\x55\xfe\xc3\xb3\xdd\xaa\x60\x07\xc7\xb1\x1c\x8b\x5a\x1a\x68\x20\xca\x74\xa8\x5a\xeb\x4c\xf5\x45\xc1\xb3\x37\x53\x70\xf4\x4f\x24\xd5\x3d\x61\xfe\x67\x6e\x4c");
            }
            _ => panic!("wrong result"),
        }
        assert_eq!(unsafe { UI_COUNTER }, 6);
    }
}
