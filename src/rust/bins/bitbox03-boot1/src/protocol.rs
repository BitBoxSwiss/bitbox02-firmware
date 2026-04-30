// SPDX-License-Identifier: Apache-2.0

use bitbox_boot_utils::FIRMWARE_MAX_LEN;
use bitbox_u2fhid::COMMAND_VENDOR_FIRST;

pub const BOOTLOADER_CMD: u8 = COMMAND_VENDOR_FIRST + 0x03;
pub const BOOT_OP_LEN: usize = 2;
pub const FIRMWARE_CHUNK_LEN: usize = 4 * 1024;
pub const MAX_FIRMWARE_NUM_CHUNKS: u16 = (FIRMWARE_MAX_LEN / FIRMWARE_CHUNK_LEN) as u16;

pub const OP_ERASE: u8 = b'e';
pub const OP_REBOOT: u8 = b'r';
pub const OP_WRITE_FIRMWARE_CHUNK: u8 = b'w';
pub const OP_WRITE_SIG_DATA: u8 = b's';
pub const OP_VERSIONS: u8 = b'v';
pub const OP_HASHES: u8 = b'h';
pub const OP_SCREEN_ROTATE: u8 = b'f';
pub const OP_SET_SHOW_FIRMWARE_HASH: u8 = b'H';
pub const OP_HARDWARE: u8 = b'W';

pub const OP_STATUS_OK: u8 = 0;
pub const OP_STATUS_ERR: u8 = b'Z';
pub const OP_STATUS_ERR_VERSION: u8 = b'V';
pub const OP_STATUS_ERR_LEN: u8 = b'N';
pub const OP_STATUS_ERR_MACRO: u8 = b'M';
pub const OP_STATUS_ERR_WRITE: u8 = b'W';
pub const OP_STATUS_ERR_CHECK: u8 = b'C';
pub const OP_STATUS_ERR_ABORT: u8 = b'A';
pub const OP_STATUS_ERR_ERASE: u8 = b'E';
pub const OP_STATUS_ERR_LOAD_FLAG: u8 = b'L';
pub const OP_STATUS_ERR_INVALID_CMD: u8 = b'I';
pub const OP_STATUS_ERR_UNLOCK: u8 = b'U';
pub const OP_STATUS_ERR_LOCK: u8 = b'K';

pub fn parse_num_chunks(input: &[u8]) -> Option<u16> {
    match input.len() {
        1 => Some(input[0] as u16),
        2 => Some(u16::from_le_bytes(input.try_into().unwrap())),
        _ => None,
    }
}

pub fn parse_chunk_index_and_data(input: &[u8]) -> Option<(u16, &[u8])> {
    match input.len() {
        len if len == 1 + FIRMWARE_CHUNK_LEN => Some((input[0] as u16, &input[1..])),
        len if len == 2 + FIRMWARE_CHUNK_LEN => Some((
            u16::from_le_bytes(input[..2].try_into().unwrap()),
            &input[2..],
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_num_chunks() {
        assert_eq!(parse_num_chunks(&[5]), Some(5));
        assert_eq!(parse_num_chunks(&[0x34, 0x12]), Some(0x1234));
        assert_eq!(parse_num_chunks(&[]), None);
        assert_eq!(parse_num_chunks(&[1, 2, 3]), None);
    }

    #[test]
    fn test_parse_chunk_index_and_data() {
        let mut short = [0u8; 1 + FIRMWARE_CHUNK_LEN];
        short[0] = 7;
        assert_eq!(
            parse_chunk_index_and_data(&short).map(|(chunk_num, data)| (chunk_num, data.len())),
            Some((7, FIRMWARE_CHUNK_LEN))
        );

        let mut long = [0u8; 2 + FIRMWARE_CHUNK_LEN];
        long[0] = 0x34;
        long[1] = 0x12;
        assert_eq!(
            parse_chunk_index_and_data(&long).map(|(chunk_num, data)| (chunk_num, data.len())),
            Some((0x1234, FIRMWARE_CHUNK_LEN))
        );

        assert_eq!(parse_chunk_index_and_data(&[]), None);
    }
}
