// SPDX-License-Identifier: Apache-2.0

// Must stay in sync with HEADER_LEN in scripts/image_header.py.
pub const IMAGE_HEADER_LEN: usize = 1024;
pub const IMAGE_HEADER_MAGIC_BOOT1: [u8; 4] = *b"BBS1";
pub const IMAGE_HEADER_MAGIC_FIRMWARE: [u8; 4] = *b"BBFW";

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
// Must match bb02_stage1_header_t in src/bootloader_upgrade/bootloader_upgrade.h.
pub struct ImageHeader {
    pub magic: [u8; 4],
    pub flags: u32,
    pub header_version: u16,
    pub product_id: u16,
    // Total header length; the vector table starts at this offset.
    pub header_len: u32,
    // Total image length, including the header.
    pub image_len: u64,
    pub monotonic_version: u16,
    pub marketing_version_len: u8,
    pub marketing_version: [u8; 37],
    pub reserved: [u8; 768],
    // Signatures are the last 3 * 64 bytes of the header.
    pub signatures: [[u8; 64]; 3],
}

impl ImageHeader {
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        let bytes: &[u8; core::mem::size_of::<Self>()] = bytes.try_into().map_err(|_| ())?;
        Ok(Self {
            magic: bytes[..4].try_into().map_err(|_| ())?,
            flags: u32::from_le_bytes(bytes[4..8].try_into().map_err(|_| ())?),
            header_version: u16::from_le_bytes(bytes[8..10].try_into().map_err(|_| ())?),
            product_id: u16::from_le_bytes(bytes[10..12].try_into().map_err(|_| ())?),
            header_len: u32::from_le_bytes(bytes[12..16].try_into().map_err(|_| ())?),
            image_len: u64::from_le_bytes(bytes[16..24].try_into().map_err(|_| ())?),
            monotonic_version: u16::from_le_bytes(bytes[24..26].try_into().map_err(|_| ())?),
            marketing_version_len: bytes[26],
            marketing_version: bytes[27..64].try_into().map_err(|_| ())?,
            reserved: bytes[64..832].try_into().map_err(|_| ())?,
            signatures: [
                bytes[832..896].try_into().map_err(|_| ())?,
                bytes[896..960].try_into().map_err(|_| ())?,
                bytes[960..1024].try_into().map_err(|_| ())?,
            ],
        })
    }
}

const _: [(); 0x000] = [(); core::mem::offset_of!(ImageHeader, magic)];
const _: [(); 0x004] = [(); core::mem::offset_of!(ImageHeader, flags)];
const _: [(); 0x008] = [(); core::mem::offset_of!(ImageHeader, header_version)];
const _: [(); 0x00a] = [(); core::mem::offset_of!(ImageHeader, product_id)];
const _: [(); 0x00c] = [(); core::mem::offset_of!(ImageHeader, header_len)];
const _: [(); 0x010] = [(); core::mem::offset_of!(ImageHeader, image_len)];
const _: [(); 0x018] = [(); core::mem::offset_of!(ImageHeader, monotonic_version)];
const _: [(); 0x01a] = [(); core::mem::offset_of!(ImageHeader, marketing_version_len)];
const _: [(); 0x01b] = [(); core::mem::offset_of!(ImageHeader, marketing_version)];
const _: [(); 0x040] = [(); core::mem::offset_of!(ImageHeader, reserved)];
const _: [(); 0x340] = [(); core::mem::offset_of!(ImageHeader, signatures)];
const _: [(); IMAGE_HEADER_LEN] = [(); core::mem::size_of::<ImageHeader>()];
