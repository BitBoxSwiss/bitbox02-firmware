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

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
pub use bitbox02_sys::backup_error_t as Error;
pub use bitbox02_sys::restore_error_t as RestoreError;

pub struct CheckData {
    pub id: String,
    pub name: String,
    // unix timestamp, UTC.
    pub birthdate: u32,
}

pub fn create(backup_create_timestamp: u32, seed_birthdate_timestamp: u32) -> Result<(), Error> {
    match unsafe { bitbox02_sys::backup_create(backup_create_timestamp, seed_birthdate_timestamp) }
    {
        Error::BACKUP_OK => Ok(()),
        err => Err(err),
    }
}

/// Returns the backup id, name and birthdate of the backup that matches the current keystore seed.
/// If none matches, `Err()` is returned.
pub fn check() -> Result<CheckData, Error> {
    let mut id = [0u8; 65];
    let mut name = [0u8; bitbox02_sys::MEMORY_DEVICE_NAME_MAX_LEN as _];
    let mut birthdate = 0u32;
    match unsafe { bitbox02_sys::backup_check(id.as_mut_ptr(), name.as_mut_ptr(), &mut birthdate) }
    {
        Error::BACKUP_OK => Ok(CheckData {
            id: crate::util::str_from_null_terminated(&id[..])
                .unwrap()
                .into(),
            name: crate::util::str_from_null_terminated(&name[..])
                .unwrap()
                .into(),
            birthdate,
        }),
        err => Err(err),
    }
}

#[derive(Copy, Clone, Default)]
struct Backup(bitbox02_sys::Backup);

#[derive(Copy, Clone, Default)]
struct BackupData(bitbox02_sys::BackupData);

impl zeroize::DefaultIsZeroes for Backup {}
impl zeroize::DefaultIsZeroes for BackupData {}

pub struct RestoreData {
    pub seed: zeroize::Zeroizing<Vec<u8>>,
    // unix timestamp, UTC.
    pub birthdate: u32,
    // unix timestamp, UTC.
    pub timestamp: u32,
    pub name: String,
}

pub fn restore_from_directory(dir: &str) -> Result<RestoreData, RestoreError> {
    let mut backup = zeroize::Zeroizing::new(Backup {
        ..Default::default()
    });
    let mut backup_data = zeroize::Zeroizing::new(BackupData {
        ..Default::default()
    });

    match unsafe {
        bitbox02_sys::restore_from_directory(
            crate::util::str_to_cstr_vec(dir).unwrap().as_ptr(),
            &mut backup.0,
            &mut backup_data.0,
        )
    } {
        RestoreError::RESTORE_OK => Ok(RestoreData {
            seed: zeroize::Zeroizing::new(
                backup_data.0.seed[..backup_data.0.seed_length as _].to_vec(),
            ),
            birthdate: backup_data.0.birthdate,
            timestamp: backup.0.backup_v1.content.metadata.timestamp,
            name: crate::util::str_from_null_terminated(&backup.0.backup_v1.content.metadata.name)
                .or(Err(RestoreError::RESTORE_ERR_DECODE))?
                .into(),
        }),
        err => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{mock_memory, mock_sd, mock_unlocked};

    #[test]
    pub fn test_restore_from_backup() {
        mock_memory();
        mock_sd();
        mock_unlocked();
        const BIRTHDATE: u32 = 0;
        const TIMESTAMP: u32 = 0;
        create(TIMESTAMP, BIRTHDATE).unwrap();

        let data = restore_from_directory(
            "41233dfbad010723dbbb93514b7b81016b73f8aa35c5148e1b478f60d5750dce",
        )
        .unwrap();
        assert_eq!(data.birthdate, BIRTHDATE);
        assert_eq!(data.timestamp, TIMESTAMP);
        assert_eq!(data.name, "My BitBox");
        assert_eq!(
            &data.seed[..],
            b"\xae\x45\xd4\x02\x3a\xfa\x4a\x48\x68\x77\x51\x69\xfe\xa5\xf5\xe4\x97\xf7\xa1\xa4\xd6\x22\x9a\xd0\x23\x9e\x68\x9b\x48\x2e\xd3\x5e",
        );

        assert!(restore_from_directory("invalid").is_err());
    }
}
