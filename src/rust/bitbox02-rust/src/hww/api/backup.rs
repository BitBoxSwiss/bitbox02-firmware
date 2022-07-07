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

use super::Error;
use crate::pb;

use alloc::vec::Vec;

use pb::response::Response;

use crate::workflow::{confirm, status, unlock};
use bitbox02::backup;

pub async fn check(
    &pb::CheckBackupRequest { silent }: &pb::CheckBackupRequest,
) -> Result<Response, Error> {
    if !bitbox02::sd::sdcard_inserted() {
        return Err(Error::InvalidInput);
    }
    match backup::check() {
        Ok(backup::CheckData { id, name, .. }) => {
            if !silent {
                let params = confirm::Params {
                    title: "Name?",
                    body: &name,
                    scrollable: true,
                    ..Default::default()
                };

                confirm::confirm(&params).await?;

                let params = confirm::Params {
                    title: "ID?",
                    body: &id,
                    scrollable: true,
                    ..Default::default()
                };

                confirm::confirm(&params).await?;

                status::status("Backup valid", true).await;
            }
            Ok(Response::CheckBackup(pb::CheckBackupResponse { id }))
        }
        Err(backup::Error::BACKUP_ERR_CHECK) => {
            if !silent {
                status::status("Backup missing\nor invalid", false).await;
            }
            Err(Error::Generic)
        }
        Err(err) => {
            let msg = format!("Could not check\nbackup\n{:?}", err).replace("BACKUP_ERR_", "");
            status::status(&msg, false).await;
            Err(Error::Generic)
        }
    }
}

/// Creates a backup on the microsD card.
///
/// If the device is seeded but uninitialized, a backup is created with the passed `timestamp` as
/// backup creation time as well as seed birthdate.
///
/// If the device is initialized, an existing backup is overwritten, but the seed birthdate is
/// retained from the previous backup. If no backup existed, the seed birthdate is set to 0, meaning
/// it is unknown.
pub async fn create(
    &pb::CreateBackupRequest {
        timestamp,
        timezone_offset,
    }: &pb::CreateBackupRequest,
) -> Result<Response, Error> {
    const MAX_EAST_UTC_OFFSET: i32 = 50400; // 14 hours in seconds
    const MAX_WEST_UTC_OFFSET: i32 = -43200; // 12 hours in seconds

    if timezone_offset < MAX_WEST_UTC_OFFSET || timezone_offset > MAX_EAST_UTC_OFFSET {
        return Err(Error::InvalidInput);
    }

    // Wait for sd card
    super::sdcard::process(&pb::InsertRemoveSdCardRequest {
        action: pb::insert_remove_sd_card_request::SdCardAction::InsertCard as _,
    })
    .await?;

    let is_initialized = bitbox02::memory::is_initialized();

    if is_initialized {
        unlock::unlock_keystore("Unlock device", unlock::CanCancel::Yes).await?;
    }

    let seed_birthdate = if !is_initialized {
        let date_string = bitbox02::format_datetime(timestamp, timezone_offset, true);
        let params = confirm::Params {
            title: "Is today?",
            body: &date_string,
            ..Default::default()
        };
        confirm::confirm(&params).await?;
        if bitbox02::memory::set_seed_birthdate(timestamp).is_err() {
            return Err(Error::Memory);
        }
        timestamp
    } else if let Ok(backup::CheckData { birthdate, .. }) = backup::check() {
        // If adding new backup after initialized, we do not know the seed birthdate.
        // If re-creating it, we use the already existing one.
        birthdate
    } else {
        0
    };
    match backup::create(timestamp, seed_birthdate) {
        Ok(()) => {
            // The backup was created, so reporting an error here
            // could have bad consequences like replacing the sd card,
            // not safely disposing of the old one.  The issue fixes
            // itself after replugging and going through the backup
            // process again.
            let _ = bitbox02::memory::set_initialized();

            status::status("Backup created", true).await;
            Ok(Response::Success(pb::Success {}))
        }
        Err(err) => {
            let msg = format!("Backup not created\nPlease contact\nsupport ({:?})", err)
                .replace("BACKUP_ERR_", "");
            status::status(&msg, false).await;
            Err(Error::Generic)
        }
    }
}

pub fn list() -> Result<Response, Error> {
    let mut info: Vec<pb::BackupInfo> = Vec::new();
    for dir in bitbox02::sd::list_subdir(None)? {
        let data = match bitbox02::backup::restore_from_directory(&dir) {
            Ok(data) => data,
            Err(_) => continue,
        };
        info.push(pb::BackupInfo {
            id: dir,
            timestamp: data.timestamp,
            name: data.name,
        })
    }
    Ok(Response::ListBackups(pb::ListBackupsResponse { info }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::bb02_async::block_on;
    use alloc::boxed::Box;
    use bitbox02::testing::{
        mock, mock_memory, mock_sd, mock_unlocked, mock_unlocked_using_mnemonic, Data,
    };

    /// Test backup creation on a uninitialized keystore.
    #[test]
    pub fn test_create_uninitialized() {
        const EXPECTED_TIMESTMAP: u32 = 1601281809;

        // All good.
        mock(Data {
            sdcard_inserted: Some(true),
            ui_confirm_create: Some(Box::new(|params| {
                assert_eq!(params.body, "<date>");
                true
            })),
            ..Default::default()
        });
        mock_sd();
        mock_unlocked();
        mock_memory();
        assert_eq!(
            block_on(create(&pb::CreateBackupRequest {
                timestamp: EXPECTED_TIMESTMAP,
                timezone_offset: 18000,
            })),
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(EXPECTED_TIMESTMAP, bitbox02::memory::get_seed_birthdate());
    }

    #[test]
    pub fn test_list() {
        const EXPECTED_TIMESTMAP: u32 = 1601281809;

        const DEVICE_NAME_1: &str = "test device name";
        const DEVICE_NAME_2: &str = "another test device name";

        mock_sd();

        // No backups yet.
        assert_eq!(
            list(),
            Ok(Response::ListBackups(pb::ListBackupsResponse {
                info: vec![]
            }))
        );

        // Create one backup.
        mock(Data {
            sdcard_inserted: Some(true),
            ui_confirm_create: Some(Box::new(|_params| true)),
            ..Default::default()
        });
        mock_unlocked_using_mnemonic("purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay");
        mock_memory();
        bitbox02::memory::set_device_name(DEVICE_NAME_1).unwrap();
        assert!(block_on(create(&pb::CreateBackupRequest {
            timestamp: EXPECTED_TIMESTMAP,
            timezone_offset: 18000,
        }))
        .is_ok());

        assert_eq!(
            list(),
            Ok(Response::ListBackups(pb::ListBackupsResponse {
                info: vec![pb::BackupInfo {
                    id: "41233dfbad010723dbbb93514b7b81016b73f8aa35c5148e1b478f60d5750dce".into(),
                    timestamp: EXPECTED_TIMESTMAP,
                    name: DEVICE_NAME_1.into(),
                }]
            }))
        );

        // Create another backup.
        mock(Data {
            sdcard_inserted: Some(true),
            ui_confirm_create: Some(Box::new(|_params| true)),
            ..Default::default()
        });
        mock_unlocked_using_mnemonic("goddess item rack improve shaft occur actress rib emerge salad rich blame model glare lounge stable electric height scrub scrub oyster now dinner oven");
        mock_memory();
        bitbox02::memory::set_device_name(DEVICE_NAME_2).unwrap();
        assert!(block_on(create(&pb::CreateBackupRequest {
            timestamp: EXPECTED_TIMESTMAP,
            timezone_offset: 18000,
        }))
        .is_ok());

        assert_eq!(
            list(),
            Ok(Response::ListBackups(pb::ListBackupsResponse {
                info: vec![
                    pb::BackupInfo {
                        id: "41233dfbad010723dbbb93514b7b81016b73f8aa35c5148e1b478f60d5750dce"
                            .into(),
                        timestamp: EXPECTED_TIMESTMAP,
                        name: DEVICE_NAME_1.into(),
                    },
                    pb::BackupInfo {
                        id: "4c7005846ffc09f31850201a6fdfff084191164eb318db2c6fe5a39df4a97ba0"
                            .into(),
                        timestamp: EXPECTED_TIMESTMAP,
                        name: DEVICE_NAME_2.into(),
                    }
                ]
            }))
        )
    }
}
