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

use super::error::{Context, Error};
use super::pb;

use pb::response::Response;

use bitbox02::keystore;

/// Returns the keystore's root fingerprint, which is the first 32
/// bits of the hash160 of the pubkey at the keypath m/.
/// https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#key-identifiers
pub fn process() -> Result<Response, Error> {
    let fingerprint = keystore::root_fingerprint()
        .map_err(Error::err)
        .context("root_fingerprint failed")?;
    Ok(Response::Fingerprint(pb::RootFingerprintResponse {
        fingerprint: fingerprint.to_vec(),
    }))
}
