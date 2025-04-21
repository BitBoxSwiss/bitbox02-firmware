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

#[cfg(not(feature = "app-cardano"))]
compile_error!("Cardano code is being compiled even though the app-cardano feature is not enabled");

mod address;
pub mod keypath;
mod params;
mod sign_transaction;
mod xpubs;

use super::pb;
use super::Error;

use pb::cardano_request::Request;
use pb::cardano_response::Response;

/// Handle a Cardano protobuf api call.
pub async fn process_api(
    hal: &mut impl crate::hal::Hal,
    request: &Request,
) -> Result<Response, Error> {
    match request {
        Request::Xpubs(ref request) => xpubs::process(request),
        Request::Address(ref request) => address::process(hal, request).await,
        Request::SignTransaction(ref request) => sign_transaction::process(hal, request).await,
    }
}
