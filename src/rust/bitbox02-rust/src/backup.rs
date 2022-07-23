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

use hmac::{Hmac, Mac, NewMac};
use sha2::{Digest, Sha256};
use zeroize::{Zeroize, Zeroizing};

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
    content: &pb_backup::BackupContent,
    data: &pb_backup::BackupData,
) -> Result<Vec<u8>, ()> {
    let mut hasher = Sha256::new();
    let metadata = content.metadata.as_ref().unwrap();
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
    hasher.update(&padded_name);
    hasher.update(data.seed_length.to_le_bytes());
    if data.seed.len() != 32 {
        return Err(());
    }
    hasher.update(&padded_seed(&data.seed));
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
    hasher.update(&padded_generator);
    hasher.update(content.length.to_le_bytes());
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

            let checksum = compute_checksum(&content, &backup_data.0)?;
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

#[cfg(test)]
mod tests {
    use super::*;

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
