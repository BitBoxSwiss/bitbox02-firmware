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

use alloc::vec::Vec;

use pb::cardano_response::Response;

use super::keypath::validate_account_shelley;

/// Return the xpub at the request keypath.
///
/// 64 bytes: 32 bytes public key + 32 bytes chain code.
pub fn process(request: &pb::CardanoXpubsRequest) -> Result<Response, Error> {
    let mut xpubs: Vec<Vec<u8>> = Vec::with_capacity(request.keypaths.len());
    for pb::Keypath { keypath } in &request.keypaths {
        validate_account_shelley(keypath)?;

        let xpub = crate::keystore::ed25519::get_xpub(keypath)?;
        let mut xpub_bytes = Vec::with_capacity(64);
        xpub_bytes.extend_from_slice(xpub.pubkey_bytes());
        xpub_bytes.extend_from_slice(xpub.chain_code());
        xpubs.push(xpub_bytes);
    }
    Ok(Response::Xpubs(pb::CardanoXpubsResponse { xpubs }))
}
