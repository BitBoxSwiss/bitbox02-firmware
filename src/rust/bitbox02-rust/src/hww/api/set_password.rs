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

use super::pb;
use super::Error;

use crate::workflow::password;
use bitbox02::password::Password;
use core::convert::TryInto;
use pb::response::Response;

use bitbox02::keystore::Keystore;
use bitbox02::memory::Memory;
use bitbox02::ui::UI;

/// Handles the SetPassword api call. This has the user enter a password twice and creates the
/// seed/keystore. After this call is finished, the keystore is fully unlocked.
///
/// `entropy` must be exactly 32 bytes and provides additional entropy used when
/// creating the seed.
pub async fn process<K: Keystore, M: Memory, U: UI>(
    pb::SetPasswordRequest { entropy }: &pb::SetPasswordRequest,
) -> Result<Response, Error> {
    let entropy32: [u8; 32] = match entropy.as_slice().try_into() {
        Err(_) => return Err(Error::COMMANDER_ERR_INVALID_INPUT),
        Ok(e) => e,
    };
    let mut password = Password::new();
    if !password::enter_twice::<U>(&mut password).await {
        return Err(Error::COMMANDER_ERR_GENERIC);
    }
    if !K::create_and_store_seed(&password, &entropy32) {
        return Err(Error::COMMANDER_ERR_GENERIC);
    }
    if K::unlock(&password).is_err() {
        panic!("Unexpected error during restore: unlock failed.");
    }
    crate::workflow::unlock::unlock_bip39::<K, M, U>().await;
    Ok(Response::Success(pb::Success {}))
}
