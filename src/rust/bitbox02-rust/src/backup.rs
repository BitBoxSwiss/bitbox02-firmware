// Copyright 2022 Shift Crypto AG
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

use prost::Message;

use crate::pb_backup;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use zeroize::{Zeroize, Zeroizing};

#[derive(Debug)]
pub enum Error {
    Generic,
    Stale,
    SdList,
    SdRead,
    SdWrite,
    Check,
}

#[derive(Default)]
pub struct BackupData(pub Box<pb_backup::BackupData>);

impl BackupData {
    /// The seed field is always 32 bytes, a historical "accident" as it was easier to fix the size
    /// when using C (and nanopb protobuf), with the seed_length indicating the actual length.
    pub fn get_seed(&self) -> &[u8] {
        &self.0.seed[..self.0.seed_length as _]
    }
}

impl Zeroize for BackupData {
    fn zeroize(&mut self) {
        self.0.seed_length.zeroize();
        self.0.seed.zeroize();
        self.0.birthdate.zeroize();
        self.0.generator.zeroize();
    }
}

/// Pad a seed with zeroes to the right up to 32 bytes.  That the seed field in backups is always 32
/// bytes is a historical "accident" as it was easier to fix the size when using C (and nanopb
/// protobuf), with the seed_length indicating the actual length.
fn padded_seed(seed: &[u8]) -> Zeroizing<[u8; 32]> {
    let mut result = Zeroizing::new([0u8; 32]);
    result[..seed.len()].copy_from_slice(seed);
    result
}

pub fn id(seed: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(b"backup").unwrap();
    mac.update(padded_seed(seed).as_ref());
    hex::encode(mac.finalize_reset().into_bytes())
}

fn compute_checksum(
    metadata: &pb_backup::BackupMetaData,
    data: &pb_backup::BackupData,
    data_length: u32,
) -> Result<Vec<u8>, ()> {
    let mut hasher = Sha256::new();
    hasher.update(metadata.timestamp.to_le_bytes());
    let mode: u8 = metadata.mode as _;
    hasher.update(mode.to_le_bytes());
    let padded_name: [u8; 64] = {
        let mut result = [0u8; 64];
        let name = metadata.name.as_bytes();
        if name.len() > 64 {
            return Err(());
        }
        result[..name.len()].copy_from_slice(name);
        result
    };
    hasher.update(padded_name);
    hasher.update(data.seed_length.to_le_bytes());
    if data.seed.len() != 32 {
        return Err(());
    }
    hasher.update(padded_seed(&data.seed));
    hasher.update(data.birthdate.to_le_bytes());
    let padded_generator: [u8; 20] = {
        let mut result = [0u8; 20];
        let generator = data.generator.as_bytes();
        if generator.len() > 20 {
            return Err(());
        }
        result[..generator.len()].copy_from_slice(generator);
        result
    };
    hasher.update(padded_generator);
    hasher.update(data_length.to_le_bytes());
    Ok(hasher.finalize().to_vec())
}

fn load_from_buffer(buf: &[u8]) -> Result<(Zeroizing<BackupData>, pb_backup::BackupMetaData), ()> {
    let backup = pb_backup::Backup::decode(buf).or(Err(()))?;
    match backup.backup_version {
        Some(pb_backup::backup::BackupVersion::BackupV1(pb_backup::BackupV1 {
            content: Some(content),
        })) => {
            let mut backup_data: Zeroizing<BackupData> = Default::default();
            backup_data.0.merge(content.data.as_slice()).or(Err(()))?;

            let checksum = compute_checksum(
                content.metadata.as_ref().unwrap(),
                &backup_data.0,
                content.length,
            )?;
            if checksum != content.checksum {
                Err(())
            } else {
                Ok((backup_data, content.metadata.unwrap()))
            }
        }
        _ => Err(()),
    }
}

/// Does a bitwise majority vote to recover the contents of potentially corrupted data. All three
/// buffers be of the same length.
fn bitwise_recovery(buf1: &[u8], buf2: &[u8], buf3: &[u8]) -> Result<Zeroizing<Vec<u8>>, ()> {
    let len = buf1.len();
    if len != buf2.len() || len != buf3.len() {
        return Err(());
    }
    let mut recovered_contents = Zeroizing::new(vec![0u8; len]);
    for i in 0..len {
        for bit in 0..8 {
            let bit1 = buf1[i] & (1 << bit);
            let bit2 = buf2[i] & (1 << bit);
            let bit3 = buf3[i] & (1 << bit);
            let winner = if bit1 == bit2 || bit1 == bit3 {
                bit1
            } else {
                bit2
            };
            recovered_contents[i] |= winner;
        }
    }
    Ok(recovered_contents)
}

pub fn load(dir: &str) -> Result<(Zeroizing<BackupData>, pb_backup::BackupMetaData), ()> {
    let files = bitbox02::sd::list_subdir(Some(dir))?;
    if files.len() != 3 {
        return Err(());
    }
    let file_contents: [Zeroizing<Vec<u8>>; 3] = [
        bitbox02::sd::load_bin(&files[0], dir)?,
        bitbox02::sd::load_bin(&files[1], dir)?,
        bitbox02::sd::load_bin(&files[2], dir)?,
    ];
    for contents in file_contents.iter() {
        if let o @ Ok(_) = load_from_buffer(contents) {
            return o;
        }
    }
    // If we arrived here, it means all three copies of the backup are corrupted (couldn't decode or
    // failed the checksum verification). We try to recover with a bit-wise majority vote using the
    // three copies of the backup. This only works if all three files have the same size.
    load_from_buffer(&bitwise_recovery(
        &file_contents[0],
        &file_contents[1],
        &file_contents[2],
    )?)
}

pub fn create(
    seed: &[u8],
    name: &str,
    backup_create_timestamp: u32,
    seed_birthdate_timestamp: u32,
) -> Result<(), Error> {
    let backup_data = zeroize::Zeroizing::new(BackupData(Box::new(pb_backup::BackupData {
        seed_length: seed.len() as _,
        seed: padded_seed(seed).to_vec(),
        birthdate: seed_birthdate_timestamp,
        generator: crate::version::FIRMWARE_VERSION_SHORT.into(),
    })));
    let length: u32 = {
        // See the documentation in the backup.proto file - the length field is obsolete, but for
        // backwards compatbility still set, as it is part of the checksum.
        0
    };
    let metadata = pb_backup::BackupMetaData {
        timestamp: backup_create_timestamp,
        name: name.into(),
        mode: pb_backup::BackupMode::Plaintext as _,
    };

    // We cap at 19/63 because the previous implementation in C/nanopb used null terminated strings
    // with a buffer of 20/64 bytes when decoding the protobuf message. This allows the backup to be
    // restored on older firmware.
    if backup_data.0.generator.len() > 19 || metadata.name.len() > 63 {
        return Err(Error::Generic);
    }

    let checksum = compute_checksum(&metadata, &backup_data.0, length).or(Err(Error::Generic))?;
    let backup = pb_backup::Backup {
        backup_version: Some(pb_backup::backup::BackupVersion::BackupV1(
            pb_backup::BackupV1 {
                content: Some(pb_backup::BackupContent {
                    checksum,
                    metadata: Some(metadata),
                    length,
                    data: backup_data.0.encode_to_vec(),
                }),
            },
        )),
    };
    let backup_encoded = backup.encode_to_vec();
    let dir = id(seed);
    let files = bitbox02::sd::list_subdir(Some(&dir)).or(Err(Error::SdList))?;

    let filename_datetime = {
        let tm = bitbox02::get_datetime(backup_create_timestamp).map_err(|_| Error::Generic)?;
        format!(
            "{}_{}T{}-{}-{}Z",
            tm.weekday(),
            tm.date(),
            tm.hour(),
            tm.minute(),
            tm.second()
        )
    };

    for i in 0..3 {
        let filename = format!("backup_{}_{}.bin", filename_datetime, i,);
        // Timestamp must be different from an existing backup when recreating a backup, otherwise
        // we might end up corrupting the existing backup.
        if files.contains(&filename) {
            return Err(Error::Generic);
        }
        bitbox02::sd::write_bin(&filename, &dir, &backup_encoded).or(Err(Error::SdWrite))?;
        if bitbox02::sd::load_bin(&filename, &dir)
            .or(Err(Error::SdRead))?
            .as_slice()
            != backup_encoded.as_slice()
        {
            return Err(Error::Check);
        }
    }
    let mut stale = false;
    for file in files {
        if bitbox02::sd::erase_file_in_subdir(&file, &dir).is_err() {
            stale = true
        }
    }
    if stale {
        return Err(Error::Stale);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use core::convert::TryInto;

    use bitbox02::testing::mock_sd;

    #[test]
    fn test_id() {
        // Seeds of different lengths (16, 24, 32 bytes)
        let seed_16 = [1u8; 16];
        let seed_24 = [2u8; 24];
        let seed_32 = [3u8; 32];

        // Expected outputs for each seed length
        let expected_output_16 = "2ca3c18f988437806217db3bbee04913cbfaaa7152d4a9c00c80cf3802d0ef6f";
        let expected_output_24 = "57c24fa271ba9c6f4d5f2519c26f52dae8f491423daffb87d1c1002ca8a2b172";
        let expected_output_32 = "89910bac6d7f82e4e97ea2b6449e201dfd793adb75b342a90314d178866c89eb";

        // Assert that id function produces expected outputs
        assert_eq!(id(&seed_16), expected_output_16);
        assert_eq!(id(&seed_24), expected_output_24);
        assert_eq!(id(&seed_32), expected_output_32);
    }

    fn _test_create_load(seed: &[u8]) {
        mock_sd();
        let timestamp = 1601281809;
        let birthdate = timestamp - 32400;
        assert!(create(seed, "test name", timestamp, birthdate).is_ok());
        let dir = id(seed);
        assert_eq!(bitbox02::sd::list_subdir(None), Ok(vec![dir.clone()]));
        assert_eq!(
            bitbox02::sd::list_subdir(Some(&dir)),
            Ok(vec![
                "backup_Mon_2020-09-28T08-30-09Z_0.bin".into(),
                "backup_Mon_2020-09-28T08-30-09Z_1.bin".into(),
                "backup_Mon_2020-09-28T08-30-09Z_2.bin".into()
            ])
        );

        // Recreating using same timestamp is not allowed and doesn't change the backups.
        assert!(create(seed, "new name", timestamp, birthdate).is_err());
        assert_eq!(
            bitbox02::sd::list_subdir(Some(&dir)),
            Ok(vec![
                "backup_Mon_2020-09-28T08-30-09Z_0.bin".into(),
                "backup_Mon_2020-09-28T08-30-09Z_1.bin".into(),
                "backup_Mon_2020-09-28T08-30-09Z_2.bin".into()
            ])
        );

        let contents: [zeroize::Zeroizing<Vec<u8>>; 3] = bitbox02::sd::list_subdir(Some(&dir))
            .unwrap()
            .iter()
            .map(|file| bitbox02::sd::load_bin(file, &dir).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        assert!(
            contents[0].as_slice() == contents[1].as_slice()
                && contents[0].as_slice() == contents[2].as_slice()
        );

        // Recreating the backup removes the previous files.
        assert!(create(seed, "new name", timestamp + 1, birthdate).is_ok());
        assert_eq!(
            bitbox02::sd::list_subdir(Some(&dir)),
            Ok(vec![
                "backup_Mon_2020-09-28T08-30-10Z_0.bin".into(),
                "backup_Mon_2020-09-28T08-30-10Z_1.bin".into(),
                "backup_Mon_2020-09-28T08-30-10Z_2.bin".into()
            ])
        );

        let (backup_data, metadata) = load(&dir).unwrap();
        assert_eq!(backup_data.get_seed(), seed);
        assert_eq!(backup_data.0.birthdate, birthdate);
        assert_eq!(metadata.name.as_str(), "new name");
        assert_eq!(metadata.timestamp, timestamp + 1);
    }

    #[test]
    fn test_create_load() {
        // Test for seeds of different size.
        _test_create_load(&b"\x52\x20\xa4\xe9\xce\xea\xc6\x80\x5d\xf2\x36\x09\xf6\xb4\x78\xbb\x28\xca\x69\xb5\x16\x95\xed\x7c\x03\xbf\x74\x3a\xa5\xde\xe3\x7e"[..]);
        _test_create_load(&b"\x52\x20\xa4\xe9\xce\xea\xc6\x80\x5d\xf2\x36\x09\xf6\xb4\x78\xbb\x28\xca\x69\xb5\x16\x95\xed\x7c"[..]);
        _test_create_load(&b"\x52\x20\xa4\xe9\xce\xea\xc6\x80\x5d\xf2\x36\x09"[..]);
    }

    #[test]
    fn test_bitwise_recovery() {
        assert!(bitwise_recovery(&[1], &[], &[]).is_err());
        assert!(bitwise_recovery(&[], &[1], &[]).is_err());
        assert!(bitwise_recovery(&[], &[], &[1]).is_err());
        assert!(bitwise_recovery(&[1], &[1], &[]).is_err());
        assert!(bitwise_recovery(&[1], &[], &[1]).is_err());
        assert!(bitwise_recovery(&[], &[1], &[1]).is_err());

        assert_eq!(bitwise_recovery(&[], &[], &[]).unwrap().as_slice(), &[]);
        assert_eq!(
            bitwise_recovery(
                &[0b10101010, 0b00001111, 0b00001111, 0b10101010, 0b11111111, 0b11110000],
                &[0b10101010, 0b11110000, 0b10101010, 0b00001111, 0b11110101, 0b11110101],
                &[0b10101010, 0b10101010, 0b11110000, 0b11110000, 0b11111111, 0b11110000],
            )
            .unwrap()
            .as_slice(),
            &[0b10101010, 0b10101010, 0b10101010, 0b10101010, 0b11111111, 0b11110000]
        );
    }
}
