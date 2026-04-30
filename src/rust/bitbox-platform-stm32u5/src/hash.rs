// SPDX-License-Identifier: Apache-2.0

use crate::ffi;

pub const SHA256_LEN: usize = 32;

const HAL_MAX_DELAY: u32 = u32::MAX;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    InvalidLen,
    InvalidChunkLen,
    Peripheral,
    Busy,
    Timeout,
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

fn init() -> Result<(), Error> {
    unsafe {
        ffi::hhash.Init.DataType = ffi::HASH_DATATYPE_8B;
        Error::from_hal_status(ffi::HAL_HASH_Init(&raw mut ffi::hhash))
    }
}

pub fn sha256(data: &[u8]) -> Result<[u8; SHA256_LEN], Error> {
    let len = u32::try_from(data.len()).map_err(|_| Error::InvalidLen)?;
    let mut out = [0u8; SHA256_LEN];

    init()?;
    unsafe {
        Error::from_hal_status(ffi::HAL_HASHEx_SHA256_Start(
            &raw mut ffi::hhash,
            data.as_ptr(),
            len,
            out.as_mut_ptr(),
            HAL_MAX_DELAY,
        ))?;
    }
    Ok(out)
}

pub fn sha256_two_parts(first: &[u8], second: &[u8]) -> Result<[u8; SHA256_LEN], Error> {
    if first.is_empty() {
        return sha256(second);
    }
    if !first.len().is_multiple_of(4) {
        return Err(Error::InvalidChunkLen);
    }

    let first_len = u32::try_from(first.len()).map_err(|_| Error::InvalidLen)?;
    let second_len = u32::try_from(second.len()).map_err(|_| Error::InvalidLen)?;
    let mut out = [0u8; SHA256_LEN];

    init()?;
    unsafe {
        Error::from_hal_status(ffi::HAL_HASHEx_SHA256_Accmlt(
            &raw mut ffi::hhash,
            first.as_ptr(),
            first_len,
        ))?;
        Error::from_hal_status(ffi::HAL_HASHEx_SHA256_Accmlt_End(
            &raw mut ffi::hhash,
            second.as_ptr(),
            second_len,
            out.as_mut_ptr(),
            HAL_MAX_DELAY,
        ))?;
    }
    Ok(out)
}

pub fn double_sha256_two_parts(first: &[u8], second: &[u8]) -> Result<[u8; SHA256_LEN], Error> {
    sha256(&sha256_two_parts(first, second)?)
}
