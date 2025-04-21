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

use super::pb;
use super::Error;

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
        Request::Pub(ref request) => pubrequest::process(hal, request).await,
        Request::SignMsg(ref request) => signmsg::process(hal, request).await,
        Request::Sign(ref request) => sign::process(hal, &sign::Transaction::Legacy(request)).await,
        Request::SignEip1559(ref request) => {
            sign::process(hal, &sign::Transaction::Eip1559(request)).await
        }
        Request::AntikleptoSignature(_) => Err(Error::InvalidInput),
        Request::SignTypedMsg(ref request) => sign_typed_msg::process(hal, request).await,
        Request::TypedMsgValue(_) => Err(Error::InvalidInput),
    }
}
