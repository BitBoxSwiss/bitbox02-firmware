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

use super::Error;
use crate::pb;

use pb::response::Response;

use crate::general::abort;
use crate::hal::Ui;
use crate::workflow::{confirm, mnemonic, password, unlock};

pub async fn from_file(
    hal: &mut impl crate::hal::Hal,
    request: &pb::RestoreBackupRequest,
) -> Result<Response, Error> {
    // This is a separate screen because 'Restore backup?' does not fit in the title field.
    hal.ui()
        .confirm(&confirm::Params {
            body: "Restore backup?",
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    let (data, metadata) = match crate::backup::load(&request.id) {
        Ok(d) => d,
        Err(_) => {
            hal.ui().status("Could not\nrestore backup", false).await;
            return Err(Error::Generic);
        }
    };

    hal.ui()
        .confirm(&confirm::Params {
            body: &format!("Name: {}. ID: {}", &metadata.name, &request.id),
            scrollable: true,
            accept_is_nextarrow: true,
            ..Default::default()
        })
        .await?;

    #[cfg(feature = "app-u2f")]
    {
        let datetime_string =
            bitbox02::format_datetime(request.timestamp, request.timezone_offset, false)
                .map_err(|_| Error::InvalidInput)?;
        let params = confirm::Params {
            title: "Is now?",
            body: &datetime_string,
            accept_is_nextarrow: true,
            ..Default::default()
        };
        hal.ui().confirm(&params).await?;
    }

    let password = password::enter_twice(hal).await?;
    if let Err(err) = bitbox02::keystore::encrypt_and_store_seed(data.get_seed(), &password) {
        hal.ui()
            .status(&format!("Could not\nrestore backup\n{:?}", err), false)
            .await;
        return Err(Error::Generic);
    }

    // Ignore error here. Missing birthdate should not abort an otherwise successful restore.
    let _ = bitbox02::memory::set_seed_birthdate(data.0.birthdate);

    #[cfg(feature = "app-u2f")]
    {
        // Ignore error - the U2f counter not being set can lead to problems with U2F, but it should
        // not fail the recovery, so the user can access their coins.
        let _ = bitbox02::securechip::u2f_counter_set(request.timestamp);
    }

    bitbox02::memory::set_initialized().or(Err(Error::Memory))?;
    if bitbox02::keystore::unlock(&password).is_err() {
        abort("restore_from_file: unlock failed");
    };

    // Ignore non-critical error.
    let _ = bitbox02::memory::set_device_name(&metadata.name);

    unlock::unlock_bip39(hal).await;
    Ok(Response::Success(pb::Success {}))
}

pub async fn from_mnemonic(
    hal: &mut impl crate::hal::Hal,
    #[cfg_attr(not(feature = "app-u2f"), allow(unused_variables))]
    &pb::RestoreFromMnemonicRequest {
        timestamp,
        timezone_offset,
    }: &pb::RestoreFromMnemonicRequest,
) -> Result<Response, Error> {
    #[cfg(feature = "app-u2f")]
    {
        let datetime_string = bitbox02::format_datetime(timestamp, timezone_offset, false)
            .map_err(|_| Error::InvalidInput)?;
        hal.ui()
            .confirm(&confirm::Params {
                title: "Is now?",
                body: &datetime_string,
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;
    }

    let mnemonic = mnemonic::get(hal).await?;
    let seed = match bitbox02::keystore::bip39_mnemonic_to_seed(&mnemonic) {
        Ok(seed) => seed,
        Err(()) => {
            hal.ui().status("Recovery words\ninvalid", false).await;
            return Err(Error::Generic);
        }
    };
    hal.ui().status("Recovery words\nvalid", true).await;

    // If entering password fails (repeat password does not match the first), we don't want to abort
    // the process immediately. We break out only if the user confirms.
    let password = loop {
        match password::enter_twice(hal).await {
            Err(password::EnterTwiceError::DoNotMatch) => {
                hal.ui()
                    .confirm(&confirm::Params {
                        title: "",
                        body: "Passwords\ndo not match.\nTry again?",
                        ..Default::default()
                    })
                    .await?;
            }
            Err(password::EnterTwiceError::Cancelled) => return Err(Error::UserAbort),
            Ok(password) => break password,
        }
    };

    if let Err(err) = bitbox02::keystore::encrypt_and_store_seed(&seed, &password) {
        hal.ui()
            .status(&format!("Could not\nrestore backup\n{:?}", err), false)
            .await;
        return Err(Error::Generic);
    };

    #[cfg(feature = "app-u2f")]
    {
        // Ignore error - the U2f counter not being set can lead to problems with U2F, but it should
        // not fail the recovery, so the user can access their coins.
        let _ = bitbox02::securechip::u2f_counter_set(timestamp);
    }

    bitbox02::memory::set_initialized().or(Err(Error::Memory))?;
    // This should never fail.
    if bitbox02::keystore::unlock(&password).is_err() {
        abort("restore_from_mnemonic: unlock failed");
    };

    unlock::unlock_bip39(hal).await;
    Ok(Response::Success(pb::Success {}))
}
