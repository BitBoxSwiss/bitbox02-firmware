// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;

use bitbox_platform_stm32u5_sys as ffi;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    Hal(HalError),
    InvalidBootAddress,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HalError {
    Error,
    Busy,
    Timeout,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BootAddressConfig {
    NonSecure0,
    NonSecure1,
}

impl BootAddressConfig {
    fn as_hal(self) -> u32 {
        match self {
            Self::NonSecure0 => ffi::OB_BOOTADDR_NS0,
            Self::NonSecure1 => ffi::OB_BOOTADDR_NS1,
        }
    }
}

#[must_use]
#[derive(Debug)]
pub struct UnlockedFlash {
    _private: (),
}

impl UnlockedFlash {
    pub fn unlock() -> Result<Self> {
        ensure_hal_ok(unsafe { ffi::HAL_FLASH_Unlock() })?;
        Ok(Self { _private: () })
    }

    pub fn unlock_option_bytes(&mut self) -> Result<UnlockedOptionBytes<'_>> {
        ensure_hal_ok(unsafe { ffi::HAL_FLASH_OB_Unlock() })?;
        Ok(UnlockedOptionBytes {
            _flash: PhantomData,
        })
    }
}

impl Drop for UnlockedFlash {
    fn drop(&mut self) {
        unsafe {
            let _ = ffi::HAL_FLASH_Lock();
        }
    }
}

#[must_use]
#[derive(Debug)]
pub struct UnlockedOptionBytes<'a> {
    _flash: PhantomData<&'a mut UnlockedFlash>,
}

impl UnlockedOptionBytes<'_> {
    pub fn program_boot_address(&mut self, config: BootAddressConfig, address: u32) -> Result<()> {
        if !is_valid_boot_address(address) {
            return Err(Error::InvalidBootAddress);
        }

        let mut option_bytes = ffi::FLASH_OBProgramInitTypeDef {
            OptionType: ffi::OPTIONBYTE_BOOTADDR,
            BootAddrConfig: config.as_hal(),
            BootAddr: address,
            ..Default::default()
        };
        ensure_hal_ok(unsafe { ffi::HAL_FLASHEx_OBProgram(&mut option_bytes) })
    }

    pub fn launch(self) -> Result<()> {
        ensure_hal_ok(unsafe { ffi::HAL_FLASH_OB_Launch() })
    }
}

impl Drop for UnlockedOptionBytes<'_> {
    fn drop(&mut self) {
        unsafe {
            let _ = ffi::HAL_FLASH_OB_Lock();
        }
    }
}

pub fn boot_address(config: BootAddressConfig) -> u32 {
    let mut option_bytes = ffi::FLASH_OBProgramInitTypeDef {
        BootAddrConfig: config.as_hal(),
        ..Default::default()
    };
    unsafe {
        ffi::HAL_FLASHEx_OBGetConfig(&mut option_bytes);
    }
    option_bytes.BootAddr
}

fn is_valid_boot_address(address: u32) -> bool {
    address & !ffi::FLASH_NSBOOTADD0R_NSBOOTADD0 == 0
}

fn ensure_hal_ok(status: ffi::HAL_StatusTypeDef) -> Result<()> {
    match status {
        ffi::HAL_StatusTypeDef::HAL_OK => Ok(()),
        ffi::HAL_StatusTypeDef::HAL_ERROR => Err(Error::Hal(HalError::Error)),
        ffi::HAL_StatusTypeDef::HAL_BUSY => Err(Error::Hal(HalError::Busy)),
        ffi::HAL_StatusTypeDef::HAL_TIMEOUT => Err(Error::Hal(HalError::Timeout)),
    }
}
