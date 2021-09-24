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

use super::error::{Context, Error, ErrorKind};
use crate::pb;

use crate::workflow::password;
use pb::response::Response;

/// Handles the SetPassword api call. This has the user enter a password twice and creates the
/// seed/keystore. After this call is finished, the keystore is fully unlocked.
///
/// `entropy` must be exactly 16 or 32 bytes and provides additional entropy used when creating the
/// seed. If 16 bytes are provided, the seed will also be 16 bytes long, corresponding to 12 BIP39
/// recovery words. If 32 bytes are provided, the seed will also be 32 bytes long, corresponding to
/// 24 BIP39 recovery words.
pub async fn process(
    pb::SetPasswordRequest { entropy }: &pb::SetPasswordRequest,
) -> Result<Response, Error> {
    if entropy.len() != 16 && entropy.len() != 32 {
        return Err(Error {
            msg: Some("host entropy len should be 16 or 32".into()),
            kind: ErrorKind::InvalidInput,
        });
    }
    let password = password::enter_twice()
        .await
        .map_err(Error::err)
        .context("password::enter_twice")?;
    if !bitbox02::keystore::create_and_store_seed(&password, &entropy) {
        return Err(Error {
            msg: Some("create_and_store_seed failed".into()),
            kind: ErrorKind::Generic,
        });
    }
    if bitbox02::keystore::unlock(&password).is_err() {
        panic!("Unexpected error during restore: unlock failed.");
    }
    crate::workflow::unlock::unlock_bip39().await;
    Ok(Response::Success(pb::Success {}))
}
