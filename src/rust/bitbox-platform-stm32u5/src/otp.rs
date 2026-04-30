// SPDX-License-Identifier: Apache-2.0

use crate::ffi;

pub const BASE: usize = 0x0BFA_0000;
pub const TOTAL_SIZE: usize = 512;
pub const BLOCK_PROGRAM_SIZE: usize = 16;
pub const OTP_BLOCK_RANDOMNESS_1: usize = 0;
pub const OTP_BLOCK_RANDOMNESS_2: usize = 1;
pub const OTP_BLOCK_HARDWARE_VERSION: usize = 2;
pub const HARDWARE_VERSION_PLATFORM_BITBOX03: u16 = 3;

pub const RANDOMNESS_LEN: usize = 32;
pub const HARDWARE_VERSION_LEN: usize = 4;

pub const RANDOMNESS_OFFSET: usize = OTP_BLOCK_RANDOMNESS_1 * BLOCK_PROGRAM_SIZE;
pub const HARDWARE_VERSION_OFFSET: usize = OTP_BLOCK_HARDWARE_VERSION * BLOCK_PROGRAM_SIZE;
// STM32U5 OTP is programmed with the same quadword granularity as regular flash.
// Once any part of a quadword has been programmed, the whole quadword must be
// treated as consumed and cannot be extended later with another normal write.
//
// Therefore, if block 2 is ever expanded to store more than the 4-byte hardware
// version, the full 16-byte layout of that block must be decided and written in
// the very first programming operation.
const HARDWARE_VERSION_STORAGE_LEN: usize = BLOCK_PROGRAM_SIZE;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    InvalidAddress,
    Unaligned,
    AlreadyProgrammed,
    Peripheral,
    Busy,
    Timeout,
    Verify,
    Unknown(u32),
}

impl Error {
    fn from_hal_status(status: ffi::HAL_StatusTypeDef) -> Result<(), Self> {
        match status as u32 {
            0 => Ok(()),
            1 => Err(Self::Peripheral),
            2 => Err(Self::Busy),
            3 => Err(Self::Timeout),
            code => Err(Self::Unknown(code)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HardwareVersion(u32);

impl HardwareVersion {
    /// Stores the platform in the upper 16 bits and the product in the lower
    /// 16 bits.
    pub const fn new(platform: u16, product: u16) -> Self {
        Self(((platform as u32) << 16) | product as u32)
    }

    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u32 {
        self.0
    }

    pub const fn platform(self) -> u16 {
        (self.0 >> 16) as u16
    }

    pub const fn product(self) -> u16 {
        self.0 as u16
    }
}

fn check_range(offset: usize, len: usize) -> Result<(), Error> {
    let Some(end) = offset.checked_add(len) else {
        return Err(Error::InvalidAddress);
    };
    if end > TOTAL_SIZE {
        return Err(Error::InvalidAddress);
    }
    Ok(())
}

fn read<const N: usize>(offset: usize) -> &'static [u8; N] {
    check_range(offset, N).expect("OTP read address out of range");
    unsafe { &*((BASE + offset) as *const [u8; N]) }
}

fn read_slice(offset: usize, len: usize) -> &'static [u8] {
    check_range(offset, len).expect("OTP read address out of range");
    unsafe { core::slice::from_raw_parts((BASE + offset) as *const u8, len) }
}

fn program(offset: usize, data: &[u8]) -> Result<(), Error> {
    if offset % BLOCK_PROGRAM_SIZE != 0 || data.len() % BLOCK_PROGRAM_SIZE != 0 {
        return Err(Error::Unaligned);
    }
    check_range(offset, data.len())?;

    // STM32U5 flash/OTP programming is single-shot per 16-byte quadword. If any
    // byte in the target quadword is no longer erased, the block must be
    // treated as already programmed and cannot be written again.
    if read_slice(offset, data.len())
        .iter()
        .any(|byte| *byte != 0xff)
    {
        return Err(Error::AlreadyProgrammed);
    }

    unsafe {
        Error::from_hal_status(ffi::HAL_FLASH_Unlock())?;

        for (index, quadword) in data.chunks_exact(BLOCK_PROGRAM_SIZE).enumerate() {
            let status = ffi::HAL_FLASH_Program(
                ffi::FLASH_TYPEPROGRAM_QUADWORD,
                (BASE + offset + index * BLOCK_PROGRAM_SIZE) as u32,
                quadword.as_ptr() as usize as u32,
            );
            if let Err(err) = Error::from_hal_status(status) {
                let _ = ffi::HAL_FLASH_Lock();
                return Err(err);
            }
        }

        Error::from_hal_status(ffi::HAL_FLASH_Lock())?;
    }

    if read_slice(offset, data.len()) != data {
        return Err(Error::Verify);
    }
    Ok(())
}

pub fn randomness() -> &'static [u8; RANDOMNESS_LEN] {
    read(RANDOMNESS_OFFSET)
}

pub fn program_randomness(randomness: &[u8; RANDOMNESS_LEN]) -> Result<(), Error> {
    program(RANDOMNESS_OFFSET, randomness)
}

pub fn hardware_version() -> Option<HardwareVersion> {
    let storage = read::<HARDWARE_VERSION_STORAGE_LEN>(HARDWARE_VERSION_OFFSET);
    let raw_bytes: [u8; HARDWARE_VERSION_LEN] = storage[..HARDWARE_VERSION_LEN].try_into().unwrap();
    if raw_bytes == [0xff; HARDWARE_VERSION_LEN] {
        None
    } else {
        Some(HardwareVersion::from_raw(u32::from_le_bytes(raw_bytes)))
    }
}

pub fn program_hardware_version(version: HardwareVersion) -> Result<(), Error> {
    let mut storage = [0xff; HARDWARE_VERSION_STORAGE_LEN];
    storage[..HARDWARE_VERSION_LEN].copy_from_slice(&version.raw().to_le_bytes());
    program(HARDWARE_VERSION_OFFSET, &storage)
}

#[cfg(test)]
mod tests {
    use super::HardwareVersion;

    #[test]
    fn test_hardware_version_roundtrip() {
        let version = HardwareVersion::new(0x1234, 0xabcd);
        assert_eq!(version.raw(), 0x1234_abcd);
        assert_eq!(version.platform(), 0x1234);
        assert_eq!(version.product(), 0xabcd);
    }

    #[test]
    fn test_hardware_version_from_raw() {
        let version = HardwareVersion::from_raw(0xbeef_cafe);
        assert_eq!(version.platform(), 0xbeef);
        assert_eq!(version.product(), 0xcafe);
    }
}
