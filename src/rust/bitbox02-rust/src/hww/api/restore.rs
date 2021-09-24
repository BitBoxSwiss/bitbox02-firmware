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

use super::error::{Context, Error, ErrorKind};
use crate::pb;

use pb::response::Response;

use crate::workflow::{confirm, mnemonic, password, status, unlock};

pub async fn from_mnemonic(
    #[cfg_attr(not(feature = "app-u2f"), allow(unused_variables))]
    &pb::RestoreFromMnemonicRequest {
        timestamp,
        timezone_offset,
    }: &pb::RestoreFromMnemonicRequest,
) -> Result<Response, Error> {
    #[cfg(feature = "app-u2f")]
    {
        let datetime_string = bitbox02::format_datetime(timestamp, timezone_offset, false);
        let params = confirm::Params {
            title: "Is now?",
            body: &datetime_string,
            ..Default::default()
        };
        confirm::confirm(&params).await?;
    }

    let mnemonic = mnemonic::get()
        .await
        .map_err(Error::err)
        .context("mnemonic::get failed")?;
    let seed = match bitbox02::keystore::bip39_mnemonic_to_seed(&mnemonic) {
        Ok(seed) => seed,
        Err(()) => {
            status::status("Recovery words\ninvalid", false).await;
            return Err(Error {
                msg: Some("recovery words invalid".into()),
                kind: ErrorKind::Generic,
            });
        }
    };
    status::status("Recovery words\nvalid", true).await;

    // If entering password fails (repeat password does not match the first), we don't want to abort
    // the process immediately. We break out only if the user confirms.
    let password = loop {
        match password::enter_twice().await {
            Err(()) => {
                let params = confirm::Params {
                    title: "",
                    body: "Passwords\ndo not match.\nTry again?",
                    ..Default::default()
                };
                confirm::confirm(&params).await?;
            }
            Ok(password) => break password,
        }
    };

    if bitbox02::keystore::encrypt_and_store_seed(&seed, password.as_str()).is_err() {
        status::status("Could not\nrestore backup", false).await;
        return Err(Error {
            msg: Some("encrypt_and_store_seed failed".into()),
            kind: ErrorKind::Generic,
        });
    };

    #[cfg(feature = "app-u2f")]
    {
        // Ignore error - the U2f counter not being set can lead to problems with U2F, but it should
        // not fail the recovery, so the user can access their coins.
        let _ = bitbox02::securechip::u2f_counter_set(timestamp);
    }

    bitbox02::memory::set_initialized().map_err(Error::err_memory)?;

    // This should never fail.
    bitbox02::keystore::unlock(&password).expect("restore_from_mnemonic: unlock failed");
    unlock::unlock_bip39().await;
    Ok(Response::Success(pb::Success {}))
}
