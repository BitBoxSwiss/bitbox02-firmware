// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;

use bitbox_platform_stm32u5_sys as ffi;

const FLASH_BANK_SIZE: u32 = ffi::FLASH_SIZE_DEFAULT / 2;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    Hal(HalError),
    InvalidBootAddress,
    InvalidAddress,
    InvalidLength,
    PageErase(u32),
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

    pub fn erase_page(&mut self, address: u32) -> Result<()> {
        if address % ffi::FLASH_PAGE_SIZE != 0 {
            return Err(Error::InvalidAddress);
        }
        let (bank, page) = bank_page(address)?;
        let mut page_error = u32::MAX;
        let mut erase = ffi::FLASH_EraseInitTypeDef {
            TypeErase: ffi::FLASH_TYPEERASE_PAGES,
            Banks: bank,
            Page: page,
            NbPages: 1,
        };
        critical_section::with(|_| {
            ensure_hal_ok(unsafe { ffi::HAL_FLASHEx_Erase(&mut erase, &mut page_error) })
        })
        .map_err(|err| match err {
            Error::Hal(_) if page_error != u32::MAX => Error::PageErase(page_error),
            err => err,
        })
    }

    pub fn program_quadword(&mut self, address: u32, data: &[u8; 16]) -> Result<()> {
        if address % 16 != 0 {
            return Err(Error::InvalidAddress);
        }
        if !is_valid_flash_range(address, data.len() as u32) {
            return Err(Error::InvalidAddress);
        }

        let mut words = [0u32; 4];
        for (word, bytes) in words.iter_mut().zip(data.chunks_exact(4)) {
            *word = u32::from_le_bytes(bytes.try_into().unwrap());
        }
        critical_section::with(|_| {
            ensure_hal_ok(unsafe {
                ffi::HAL_FLASH_Program(
                    ffi::FLASH_TYPEPROGRAM_QUADWORD,
                    address,
                    words.as_ptr() as u32,
                )
            })
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

pub fn read(address: u32, out: &mut [u8]) -> Result<()> {
    if !is_valid_flash_range(address, out.len() as u32) {
        return Err(Error::InvalidAddress);
    }
    unsafe {
        core::ptr::copy_nonoverlapping(address as *const u8, out.as_mut_ptr(), out.len());
    }
    Ok(())
}

fn is_valid_boot_address(address: u32) -> bool {
    address & !ffi::FLASH_NSBOOTADD0R_NSBOOTADD0 == 0
}

fn is_valid_flash_range(address: u32, len: u32) -> bool {
    let Some(end) = address.checked_add(len) else {
        return false;
    };
    address >= ffi::FLASH_BASE_NS && end <= ffi::FLASH_BASE_NS + ffi::FLASH_SIZE_DEFAULT
}

fn bank_page(address: u32) -> Result<(u32, u32)> {
    if !is_valid_flash_range(address, ffi::FLASH_PAGE_SIZE) {
        return Err(Error::InvalidAddress);
    }
    let offset = address - ffi::FLASH_BASE_NS;
    let (bank, bank_offset) = if offset < FLASH_BANK_SIZE {
        (ffi::FLASH_BANK_1, offset)
    } else {
        (ffi::FLASH_BANK_2, offset - FLASH_BANK_SIZE)
    };
    Ok((bank, bank_offset / ffi::FLASH_PAGE_SIZE))
}

fn ensure_hal_ok(status: ffi::HAL_StatusTypeDef) -> Result<()> {
    match status {
        ffi::HAL_StatusTypeDef::HAL_OK => Ok(()),
        ffi::HAL_StatusTypeDef::HAL_ERROR => Err(Error::Hal(HalError::Error)),
        ffi::HAL_StatusTypeDef::HAL_BUSY => Err(Error::Hal(HalError::Busy)),
        ffi::HAL_StatusTypeDef::HAL_TIMEOUT => Err(Error::Hal(HalError::Timeout)),
    }
}
