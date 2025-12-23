// SPDX-License-Identifier: Apache-2.0

#[cfg(not(feature = "app-cardano"))]
compile_error!("Cardano code is being compiled even though the app-cardano feature is not enabled");

mod address;
pub mod keypath;
mod params;
mod sign_transaction;
mod xpubs;

use super::Error;
use super::pb;

use pb::cardano_request::Request;
use pb::cardano_response::Response;

/// Handle a Cardano protobuf api call.
pub async fn process_api(
    hal: &mut impl crate::hal::Hal,
    request: &Request,
) -> Result<Response, Error> {
    match request {
        Request::Xpubs(request) => xpubs::process(hal, request),
        Request::Address(request) => address::process(hal, request).await,
        Request::SignTransaction(request) => sign_transaction::process(hal, request).await,
    }
}
