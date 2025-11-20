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

    let (data, metadata) = match crate::backup::load(hal, &request.id).await {
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
    let seed = data.get_seed();
    if let Err(err) = crate::keystore::encrypt_and_store_seed(hal, seed, &password) {
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

    // Ignore non-critical error.
    let _ = bitbox02::memory::set_device_name(&metadata.name);

    unlock::unlock_bip39(hal, seed).await;
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
    let seed = match crate::bip39::mnemonic_to_seed(&mnemonic) {
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
            Err(err @ password::EnterTwiceError::EnterError(_)) => return Err(err.into()),
            Ok(password) => break password,
        }
    };

    if let Err(err) = crate::keystore::encrypt_and_store_seed(hal, &seed, &password) {
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

    unlock::unlock_bip39(hal, &seed).await;
    Ok(Response::Success(pb::Success {}))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use bitbox02::testing::mock_memory;
    use bitbox02::{keystore, memory};
    use util::bb02_async::block_on;

    use alloc::boxed::Box;

    #[test]
    fn test_from_mnemonic() {
        mock_memory();
        crate::keystore::lock();
        let mut counter = 0u32;
        let mut mock_hal = TestingHal::new();
        mock_hal.ui.set_enter_string(Box::new(|params| {
            counter += 1;
            match counter {
                1 => assert_eq!(params.title, "Set password"),
                2 => assert_eq!(params.title, "Repeat password"),
                _ => panic!("too many user inputs"),
            }
            Ok("password".into())
        }));

        mock_hal.securechip.event_counter_reset();
        assert_eq!(
            block_on(from_mnemonic(
                &mut mock_hal,
                &pb::RestoreFromMnemonicRequest {
                    timestamp: 0,
                    timezone_offset: 0,
                }
            )),
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(mock_hal.securechip.get_event_counter(), 8);
        drop(mock_hal); // to remove mutable borrow of counter
        assert_eq!(counter, 2);
        assert!(!crate::keystore::is_locked());
        assert!(memory::is_initialized());
        // Seed of hardcoded phrase used in unit tests:
        // boring mistake dish oyster truth pigeon viable emerge sort crash wire portion cannon couple enact box walk height pull today solid off enable tide
        assert_eq!(
            hex::encode(crate::keystore::copy_seed().unwrap()),
            "19f1bcfccf3e9d497cd245cf864ff0d42216625258d4f68d56b571aceb329257"
        );
        assert_eq!(
            hex::encode(crate::keystore::copy_bip39_seed().unwrap()),
            "257724bccc8858cfe565b456b01263a4a6a45184fab4531f5c199649207a74e74c399a01d4f957258c05cee818369b31404c884a4b7a29ff6886bae6700fb56a"
        );
    }
}
