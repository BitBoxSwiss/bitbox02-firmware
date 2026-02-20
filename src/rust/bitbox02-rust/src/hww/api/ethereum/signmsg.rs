// SPDX-License-Identifier: Apache-2.0

use super::Error;
use super::pb;

use crate::keystore;

use crate::workflow::verify_message;

use pb::eth_response::Response;

use alloc::vec::Vec;
use core::convert::TryInto;

use sha3::digest::Digest;

/// Process a sign message request.
///
/// The result contains a 65 byte signature. The first 64 bytes are the secp256k1 signature in
/// compact format (R and S values), and the last byte is the recoverable id (recid).
pub async fn process(
    hal: &mut impl crate::hal::Hal,
    request: &pb::EthSignMessageRequest,
) -> Result<Response, Error> {
    if request.msg.len() > 9999 {
        return Err(Error::InvalidInput);
    }

    let pub_request = pb::EthPubRequest {
        output_type: pb::eth_pub_request::OutputType::Address as _,
        keypath: request.keypath.clone(),
        coin: request.coin,
        display: true,
        contract_address: Vec::new(),
        chain_id: request.chain_id,
    };

    // Verify address. We don't need the actual result, but we have to propagate validation or user
    // abort errors.
    super::pubrequest::process(hal, &pub_request).await?;

    verify_message::verify(hal, "Sign message", "Sign", &request.msg, true).await?;

    // Construct message to be signed. There is no standard for this. We match what MyEtherWallet,
    // Trezor, etc. do, e.g.:
    // https://github.com/ethereumjs/ethereumjs-util/blob/dd2882d790c1d3b50b75bee6f88031433cbd5bef/src/signature.ts#L140
    let mut msg: Vec<u8> = Vec::new();
    msg.extend(b"\x19Ethereum Signed Message:\n");
    msg.extend(format!("{}", request.msg.len()).as_bytes());
    msg.extend(&request.msg);

    let sighash: [u8; 32] = sha3::Keccak256::digest(&msg).into();

    let host_nonce = match request.host_nonce_commitment {
        // Engage in the anti-klepto protocol if the host sends a host nonce commitment.
        Some(pb::AntiKleptoHostNonceCommitment { ref commitment }) => {
            let signer_commitment = crate::secp256k1::secp256k1_nonce_commit(
                keystore::secp256k1_get_private_key(hal, &request.keypath)?
                    .as_slice()
                    .try_into()
                    .unwrap(),
                &sighash,
                commitment
                    .as_slice()
                    .try_into()
                    .or(Err(Error::InvalidInput))?,
            )?;

            // Send signer commitment to host and wait for the host nonce from the host.
            super::antiklepto_get_host_nonce(signer_commitment).await?
        }

        // Return signature directly without the anti-klepto protocol, for backwards compatibility.
        None => [0; 32],
    };

    let sign_result = crate::secp256k1::secp256k1_sign(
        keystore::secp256k1_get_private_key(hal, &request.keypath)?
            .as_slice()
            .try_into()
            .unwrap(),
        &sighash,
        Some(&host_nonce),
    )?;
    let mut signature: Vec<u8> = sign_result.signature.to_vec();
    signature.push(sign_result.recid);

    Ok(Response::Sign(pb::EthSignResponse { signature }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use crate::hal::testing::ui::Screen;
    use crate::keystore::testing::mock_unlocked;
    use alloc::boxed::Box;
    use hex_lit::hex;
    use util::bb02_async::block_on;
    use util::bip32::HARDENED;

    const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];
    const MESSAGE: &str = "message";
    const EXPECTED_ADDRESS: &str = "0x773A77b9D32589be03f9132AF759e294f7851be9";

    #[test]
    pub fn test_process() {
        const SIGNATURE: [u8; 64] = [b'1'; 64];

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        assert_eq!(
            block_on(process(&mut mock_hal, &pb::EthSignMessageRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                msg: MESSAGE.as_bytes().to_vec(),
                host_nonce_commitment: None,
                chain_id: 0,
            })),
            Ok(Response::Sign(pb::EthSignResponse {
                signature: hex!(
                    "34885e9374375a12e8c5186ef9870b036b2bd251b3f20b979511912dd41894725c0a504a3419ae21d69e2243ca18e9c6eee75b2e16ea57b4f647fd106be83fd201"
                )
                .to_vec()
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Ethereum".into(),
                    body: EXPECTED_ADDRESS.into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: MESSAGE.into(),
                    longtouch: true,
                },
            ]
        );
    }

    #[test]
    pub fn test_process_warn_unusual_keypath() {
        const SIGNATURE: [u8; 64] = [b'1'; 64];

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        block_on(process(
            &mut mock_hal,
            &pb::EthSignMessageRequest {
                coin: pb::EthCoin::Eth as _,
                keypath: KEYPATH.to_vec(),
                msg: MESSAGE.as_bytes().to_vec(),
                host_nonce_commitment: None,
                chain_id: 11155111,
            },
        ))
        .unwrap();
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Sepolia".into(),
                    body: "Warning: unusual keypath m/44'/60'/0'/0/0. Proceed only if you know what you are doing.".into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sepolia".into(),
                    body: EXPECTED_ADDRESS.into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: MESSAGE.into(),
                    longtouch: true,
                },
            ]
        );
    }

    #[test]
    pub fn test_process_user_aborted() {
        let request = pb::EthSignMessageRequest {
            coin: pb::EthCoin::Eth as _,
            keypath: KEYPATH.to_vec(),
            msg: MESSAGE.as_bytes().to_vec(),
            host_nonce_commitment: None,
            chain_id: 0,
        };

        static mut CONFIRM_COUNTER: u32 = 0;

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        // User abort address verification.
        mock_hal.ui.abort_nth(0);
        assert_eq!(
            block_on(process(&mut mock_hal, &request)),
            Err(Error::UserAbort)
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![Screen::Confirm {
                title: "Ethereum".into(),
                body: EXPECTED_ADDRESS.into(),
                longtouch: false,
            }],
        );

        mock_unlocked();
        let mut mock_hal = TestingHal::new();
        // User abort message verification.
        mock_hal.ui.abort_nth(1);
        assert_eq!(
            block_on(process(&mut mock_hal, &request)),
            Err(Error::UserAbort)
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Ethereum".into(),
                    body: EXPECTED_ADDRESS.into(),
                    longtouch: false,
                },
                Screen::Confirm {
                    title: "Sign message".into(),
                    body: MESSAGE.into(),
                    longtouch: true,
                },
            ],
        );
    }

    #[test]
    pub fn test_process_failures() {
        const KEYPATH: &[u32] = &[44 + HARDENED, 60 + HARDENED, 0 + HARDENED, 0, 0];

        // Message too long
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::EthSignMessageRequest {
                    coin: pb::EthCoin::Eth as _,
                    keypath: KEYPATH.to_vec(),
                    msg: [0; 10000].to_vec(),
                    host_nonce_commitment: None,
                    chain_id: 0,
                }
            )),
            Err(Error::InvalidInput)
        );

        // Keystore locked.
        keystore::lock();
        assert_eq!(
            block_on(process(
                &mut TestingHal::new(),
                &pb::EthSignMessageRequest {
                    coin: pb::EthCoin::Eth as _,
                    keypath: KEYPATH.to_vec(),
                    msg: b"message".to_vec(),
                    host_nonce_commitment: None,
                    chain_id: 0,
                }
            )),
            Err(Error::InvalidInput)
        );
    }
}
