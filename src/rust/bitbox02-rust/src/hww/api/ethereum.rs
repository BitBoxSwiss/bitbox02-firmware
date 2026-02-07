// SPDX-License-Identifier: Apache-2.0

#[cfg(not(feature = "app-ethereum"))]
compile_error!(
    "Ethereum code is being compiled even though the app-ethereum feature is not enabled"
);

mod address;
mod amount;
mod keypath;
mod params;
mod pubrequest;
mod sighash;
mod sign;
mod sign_typed_msg;
mod signmsg;

use super::Error;
use super::pb;

use pb::eth_request::Request;
use pb::eth_response::Response;

use core::convert::TryInto;

/// Like `hww::next_request`, but for Ethereum requests/responses.
pub async fn next_request(response: Response) -> Result<Request, Error> {
    let request = crate::hww::next_request(pb::response::Response::Eth(pb::EthResponse {
        response: Some(response),
    }))
    .await?;
    match request {
        pb::request::Request::Eth(pb::EthRequest {
            request: Some(request),
        }) => Ok(request),
        _ => Err(Error::InvalidState),
    }
}

/// Sends the `signer_nonce_commitment` to the host and waits for the next request, which has to be a
/// `AntiKleptoSignatureRequest` message containing the host nonce.
pub async fn antiklepto_get_host_nonce(
    signer_nonce_commitment: [u8; 33],
) -> Result<[u8; 32], Error> {
    let request = next_request(Response::AntikleptoSignerCommitment(
        pb::AntiKleptoSignerCommitment {
            commitment: signer_nonce_commitment.to_vec(),
        },
    ))
    .await?;
    match request {
        Request::AntikleptoSignature(pb::AntiKleptoSignatureRequest { host_nonce }) => {
            Ok(host_nonce
                .as_slice()
                .try_into()
                .or(Err(Error::InvalidInput))?)
        }
        _ => Err(Error::InvalidState),
    }
}

/// Handle a Ethereum protobuf api call.
///
/// Returns `None` if the call was not handled by Rust, in which case it should be handled by
/// the C commander.
pub async fn process_api(
    hal: &mut impl crate::hal::Hal,
    request: &Request,
) -> Result<Response, Error> {
    match request {
        Request::Pub(request) => pubrequest::process(hal, request).await,
        Request::SignMsg(request) => signmsg::process(hal, request).await,
        Request::Sign(request) => sign::process(hal, &sign::Transaction::Legacy(request)).await,
        Request::SignEip1559(request) => {
            sign::process(hal, &sign::Transaction::Eip1559(request)).await
        }
        Request::AntikleptoSignature(_) => Err(Error::InvalidInput),
        Request::SignTypedMsg(request) => sign_typed_msg::process(hal, request).await,
        Request::TypedMsgValue(_) => Err(Error::InvalidInput),
        Request::DataChunk(_) => Err(Error::InvalidInput),
    }
}
