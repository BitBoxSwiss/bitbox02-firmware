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

use crate::backup;
use crate::hal::{Sd, Ui};
use crate::workflow::{confirm, unlock};

pub async fn check(
    hal: &mut impl crate::hal::Hal,
    &pb::CheckBackupRequest { silent }: &pb::CheckBackupRequest,
) -> Result<Response, Error> {
    if !hal.sd().sdcard_inserted().await {
        return Err(Error::InvalidInput);
    }

    let seed = crate::keystore::copy_seed(hal)?;
    let id = backup::id(&seed);
    let (backup_data, metadata) = backup::load(hal, &id).await?;
    if seed.as_slice() != backup_data.get_seed() {
        if !silent {
            hal.ui().status("Backup missing\nor invalid", false).await;
        }
        return Err(Error::Generic);
    }
    if !silent {
        hal.ui()
            .confirm(&confirm::Params {
                title: "Name?",
                body: &metadata.name,
                scrollable: true,
                accept_is_nextarrow: true,
                ..Default::default()
            })
            .await?;

        hal.ui()
            .confirm(&confirm::Params {
                title: "ID?",
                body: &id,
                scrollable: true,
                ..Default::default()
            })
            .await?;

        hal.ui().status("Backup valid", true).await;
    }
    Ok(Response::CheckBackup(pb::CheckBackupResponse { id }))
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
    hal: &mut impl crate::hal::Hal,
    &pb::CreateBackupRequest {
        timestamp,
        timezone_offset,
    }: &pb::CreateBackupRequest,
) -> Result<Response, Error> {
    hal.ui()
        .confirm(&confirm::Params {
            title: "Is today?",
            body: &bitbox02::format_datetime(timestamp, timezone_offset, true)
                .map_err(|_| Error::InvalidInput)?,
            ..Default::default()
        })
        .await?;

    // Wait for sd card
    super::sdcard::process(
        hal,
        &pb::InsertRemoveSdCardRequest {
            action: pb::insert_remove_sd_card_request::SdCardAction::InsertCard as _,
        },
    )
    .await?;

    let is_initialized = bitbox02::memory::is_initialized();

    let seed = if is_initialized {
        unlock::unlock_keystore(hal, "Unlock device", unlock::CanCancel::Yes).await?
    } else {
        let seed = crate::keystore::copy_seed(hal)?;
        // Yield now to give executor a chance to process USB/BLE communication, as copy_seed() causes
        // some delay.
        futures_lite::future::yield_now().await;
        seed
    };

    let seed_birthdate = if !is_initialized {
        if bitbox02::memory::set_seed_birthdate(timestamp).is_err() {
            return Err(Error::Memory);
        }
        timestamp
    } else if let Ok((data, _)) = backup::load(hal, &backup::id(&seed)).await {
        // If adding new backup after initialized, we do not know the seed birthdate.
        // If re-creating it, we use the already existing one.
        data.0.birthdate
    } else {
        0
    };
    match backup::create(
        hal,
        &seed,
        &bitbox02::memory::get_device_name(),
        timestamp,
        seed_birthdate,
    )
    .await
    {
        Ok(()) => {
            // The backup was created, so reporting an error here
            // could have bad consequences like replacing the sd card,
            // not safely disposing of the old one.  The issue fixes
            // itself after replugging and going through the backup
            // process again.
            let _ = bitbox02::memory::set_initialized();

            hal.ui().status("Backup created", true).await;
            Ok(Response::Success(pb::Success {}))
        }
        Err(err) => {
            let msg = format!("Backup not created\nPlease contact\nsupport ({:?})", err);
            hal.ui().status(&msg, false).await;
            Err(Error::Generic)
        }
    }
}

pub async fn list(hal: &mut impl crate::hal::Hal) -> Result<Response, Error> {
    let mut info: Vec<pb::BackupInfo> = Vec::new();
    for dir in hal.sd().list_subdir(None).await? {
        let (_, metadata) = match backup::load(hal, &dir).await {
            Ok(d) => d,
            Err(_) => continue,
        };
        info.push(pb::BackupInfo {
            id: dir,
            timestamp: metadata.timestamp,
            name: metadata.name,
        })
    }
    Ok(Response::ListBackups(pb::ListBackupsResponse { info }))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::hal::testing::TestingHal;
    use crate::keystore::testing::{mock_unlocked, mock_unlocked_using_mnemonic};
    use crate::workflow::testing::Screen;
    use alloc::boxed::Box;
    use bitbox02::testing::mock_memory;
    use util::bb02_async::block_on;

    /// Test backup creation on a uninitialized keystore.
    #[test]
    pub fn test_create_uninitialized() {
        const EXPECTED_TIMESTMAP: u32 = 1601281809;

        // All good.
        mock_memory();
        mock_unlocked();

        let mut mock_hal = TestingHal::new();
        mock_hal.sd.inserted = Some(true);
        mock_hal.securechip.event_counter_reset();
        assert_eq!(
            block_on(create(
                &mut mock_hal,
                &pb::CreateBackupRequest {
                    timestamp: EXPECTED_TIMESTMAP,
                    timezone_offset: 18000,
                }
            )),
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(mock_hal.securechip.get_event_counter(), 1);
        assert_eq!(EXPECTED_TIMESTMAP, bitbox02::memory::get_seed_birthdate());
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Is today?".into(),
                    body: "Mon 2020-09-28".into(),
                    longtouch: false
                },
                Screen::Status {
                    title: "Backup created".into(),
                    success: true
                }
            ]
        );

        assert_eq!(
            block_on(check(
                &mut mock_hal,
                &pb::CheckBackupRequest { silent: true }
            )),
            Ok(Response::CheckBackup(pb::CheckBackupResponse {
                id: "41233dfbad010723dbbb93514b7b81016b73f8aa35c5148e1b478f60d5750dce".into()
            }))
        );
    }

    /// Test backup creation on a initialized keystore. The sdcard does not contain the backup yet.
    #[test]
    pub fn test_create_initialized_new() {
        const TIMESTMAP: u32 = 1601281809;

        mock_memory();

        let seed = hex::decode("cb33c20cea62a5c277527e2002da82e6e2b37450a755143a540a54cea8da9044")
            .unwrap();
        crate::keystore::encrypt_and_store_seed(&mut TestingHal::new(), &seed, "password").unwrap();
        bitbox02::memory::set_initialized().unwrap();

        let mut password_entered: bool = false;

        let mut mock_hal = TestingHal::new();
        mock_hal.sd.inserted = Some(true);
        mock_hal.ui.set_enter_string(Box::new(|_params| {
            password_entered = true;
            Ok("password".into())
        }));
        mock_hal.securechip.event_counter_reset();
        assert_eq!(
            block_on(create(
                &mut mock_hal,
                &pb::CreateBackupRequest {
                    timestamp: TIMESTMAP,
                    timezone_offset: 18000,
                }
            )),
            Ok(Response::Success(pb::Success {}))
        );
        assert_eq!(mock_hal.securechip.get_event_counter(), 5);
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Is today?".into(),
                    body: "Mon 2020-09-28".into(),
                    longtouch: false
                },
                Screen::Status {
                    title: "Backup created".into(),
                    success: true
                }
            ]
        );

        mock_hal.ui.remove_enter_string(); // no more password entry needed
        assert_eq!(
            block_on(check(
                &mut mock_hal,
                &pb::CheckBackupRequest { silent: true }
            )),
            Ok(Response::CheckBackup(pb::CheckBackupResponse {
                id: backup::id(&seed),
            }))
        );

        drop(mock_hal); // to remove mutable borrow of `password_entered`
        assert!(password_entered);
    }

    /// Use backup file fixtures generated using firmware v9.12.0 and perform tests on it. This
    /// should catch regressions when changing backup loading/verification in the firmware code.
    #[test]
    fn test_fixture() {
        const EXPECTED_ID: &str =
            "577782fdfffbe314b23acaeefc39ad5e8641fba7e7dbe418a35956a879a67dd2";
        mock_memory();
        mock_unlocked_using_mnemonic(
            "memory raven era cave phone system dice come mechanic split moon repeat",
            "",
        );

        let mut mock_hal = TestingHal::new();
        mock_hal.sd.inserted = Some(true);

        // Create the three files using the a fixture in the directory with the backup ID of the
        // above seed.
        let backup_fixture_v9_12_0: Vec<u8> = hex::decode("0a6c0a6a0a2017834e53e17370800c0bc49b49ef3f1309df104d7239db5bbd093c90eefc995112110891bec6fb0512094d7920426974426f782233081012208af64d31126a39b98f59708a3a463e5b000000000000000000000000000000001891bec6fb05220776392e31332e30").unwrap();
        for i in 0..3 {
            block_on(mock_hal.sd.write_bin(
                &format!("backup_Mon_2020-09-28T08-30-09Z_{}.bin", i),
                EXPECTED_ID,
                &backup_fixture_v9_12_0,
            ))
            .unwrap();
        }
        // Check that the loaded seed matches the backup.
        assert_eq!(
            block_on(check(
                &mut mock_hal,
                &pb::CheckBackupRequest { silent: false }
            )),
            Ok(Response::CheckBackup(pb::CheckBackupResponse {
                id: EXPECTED_ID.into()
            }))
        );
        assert_eq!(
            mock_hal.ui.screens,
            vec![
                Screen::Confirm {
                    title: "Name?".into(),
                    body: "My BitBox".into(),
                    longtouch: false
                },
                Screen::Confirm {
                    title: "ID?".into(),
                    body: EXPECTED_ID.into(),
                    longtouch: false
                },
                Screen::Status {
                    title: "Backup valid".into(),
                    success: true
                },
            ]
        );
    }

    #[test]
    pub fn test_list() {
        const EXPECTED_TIMESTAMP: u32 = 1601281809;

        const DEVICE_NAME_1: &str = "test device name";
        const DEVICE_NAME_2: &str = "another test device name";

        let mut mock_hal = TestingHal::new();
        mock_hal.sd.inserted = Some(true);

        // No backups yet.
        assert_eq!(
            block_on(list(&mut mock_hal)),
            Ok(Response::ListBackups(pb::ListBackupsResponse {
                info: vec![]
            }))
        );

        // Create one backup.
        mock_memory();
        mock_unlocked_using_mnemonic(
            "purity concert above invest pigeon category peace tuition hazard vivid latin since legal speak nation session onion library travel spell region blast estate stay",
            "",
        );

        bitbox02::memory::set_device_name(DEVICE_NAME_1).unwrap();
        assert!(
            block_on(create(
                &mut mock_hal,
                &pb::CreateBackupRequest {
                    timestamp: EXPECTED_TIMESTAMP,
                    timezone_offset: 18000,
                }
            ))
            .is_ok()
        );

        assert_eq!(
            block_on(list(&mut mock_hal)),
            Ok(Response::ListBackups(pb::ListBackupsResponse {
                info: vec![pb::BackupInfo {
                    id: "41233dfbad010723dbbb93514b7b81016b73f8aa35c5148e1b478f60d5750dce".into(),
                    timestamp: EXPECTED_TIMESTAMP,
                    name: DEVICE_NAME_1.into(),
                }]
            }))
        );

        // Create another backup.
        mock_memory();
        mock_unlocked_using_mnemonic(
            "goddess item rack improve shaft occur actress rib emerge salad rich blame model glare lounge stable electric height scrub scrub oyster now dinner oven",
            "",
        );
        bitbox02::memory::set_device_name(DEVICE_NAME_2).unwrap();
        assert!(
            block_on(create(
                &mut mock_hal,
                &pb::CreateBackupRequest {
                    timestamp: EXPECTED_TIMESTAMP,
                    timezone_offset: 18000,
                }
            ))
            .is_ok()
        );

        assert_eq!(
            block_on(list(&mut mock_hal)),
            Ok(Response::ListBackups(pb::ListBackupsResponse {
                info: vec![
                    pb::BackupInfo {
                        id: "41233dfbad010723dbbb93514b7b81016b73f8aa35c5148e1b478f60d5750dce"
                            .into(),
                        timestamp: EXPECTED_TIMESTAMP,
                        name: DEVICE_NAME_1.into(),
                    },
                    pb::BackupInfo {
                        id: "4c7005846ffc09f31850201a6fdfff084191164eb318db2c6fe5a39df4a97ba0"
                            .into(),
                        timestamp: EXPECTED_TIMESTAMP,
                        name: DEVICE_NAME_2.into(),
                    }
                ]
            }))
        )
    }
}
