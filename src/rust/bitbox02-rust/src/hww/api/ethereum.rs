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
mod pubrequest;
mod sign;
mod signmsg;

use super::error::{Context, Error, ErrorKind};
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
        _ => Err(Error {
            msg: Some("expected Eth request".into()),
            kind: ErrorKind::InvalidState,
        }),
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
                .map_err(Error::err_invalid_input)
                .context("could not parse host nonce")?)
        }
        _ => Err(Error {
            msg: Some("expected AntikleptoSignature".into()),
            kind: ErrorKind::InvalidState,
        }),
    }
}

/// Handle a Ethereum protobuf api call.
///
/// Returns `None` if the call was not handled by Rust, in which case it should be handled by
/// the C commander.
pub async fn process_api(request: &Request) -> Option<Result<Response, Error>> {
    match request {
        Request::Pub(ref request) => Some(pubrequest::process(request).await),
        Request::SignMsg(ref request) => Some(signmsg::process(request).await),
        Request::Sign(ref request) => Some(sign::process(request).await),
        Request::AntikleptoSignature(_) => Some(Err(Error {
            msg: Some("unexpected AntikleptoSignature request".into()),
            kind: ErrorKind::InvalidInput,
        })),
    }
}
