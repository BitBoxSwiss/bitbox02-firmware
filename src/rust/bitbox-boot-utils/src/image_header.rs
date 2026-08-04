// SPDX-License-Identifier: Apache-2.0

pub const IMAGE_HEADER_LEN: usize = 1024;
pub const IMAGE_HEADER_MAGIC_BOOT1: [u8; 4] = *b"BBB1";
pub const IMAGE_HEADER_MAGIC_FIRMWARE: [u8; 4] = *b"BBFW";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ImageHeader {
    pub magic: [u8; 4],
    pub header_len: u32,
    pub code_size: u32,
}

impl ImageHeader {
    pub const LEN: usize = 4 + 4 + 4;

    pub fn from_bytes(bytes: &[u8; Self::LEN]) -> Self {
        Self {
            magic: bytes[..4].try_into().unwrap(),
            header_len: u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
            code_size: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
        }
    }
}

const _: [(); ImageHeader::LEN] = [(); core::mem::size_of::<ImageHeader>()];
const _: [(); 0x000] = [(); core::mem::offset_of!(ImageHeader, magic)];
const _: [(); 0x004] = [(); core::mem::offset_of!(ImageHeader, header_len)];
const _: [(); 0x008] = [(); core::mem::offset_of!(ImageHeader, code_size)];
