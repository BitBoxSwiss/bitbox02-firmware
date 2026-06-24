// SPDX-License-Identifier: Apache-2.0

use core::marker::PhantomData;

use bitbox_platform_stm32u5_sys as ffi;
use littlefs2::{
    consts::{U1, U256},
    driver::Storage,
    io::{Error as LfsError, Result as LfsResult},
};

const FLASH_BANK_SIZE: u32 = ffi::FLASH_SIZE_DEFAULT / 2;
const FLASH_PAGE_SIZE: usize = ffi::FLASH_PAGE_SIZE as usize;
const FLASH_PROGRAM_SIZE: usize = 16;
const LFS_BLOCK_CYCLES: isize = 500;

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

pub struct FlashStorage<const BASE: usize, const LEN: usize> {
    _private: PhantomData<()>,
}

impl<const BASE: usize, const LEN: usize> FlashStorage<BASE, LEN> {
    pub const fn new() -> Self {
        Self {
            _private: PhantomData,
        }
    }

    fn checked_address(off: usize, len: usize) -> LfsResult<u32> {
        let end = off.checked_add(len).ok_or(LfsError::IO)?;
        if end > LEN {
            return Err(LfsError::IO);
        }
        u32::try_from(BASE.checked_add(off).ok_or(LfsError::IO)?).map_err(|_| LfsError::IO)
    }
}

impl<const BASE: usize, const LEN: usize> Default for FlashStorage<BASE, LEN> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const BASE: usize, const LEN: usize> Storage for FlashStorage<BASE, LEN> {
    type CACHE_SIZE = U256;
    type LOOKAHEAD_SIZE = U1;

    const READ_SIZE: usize = FLASH_PROGRAM_SIZE;
    const WRITE_SIZE: usize = FLASH_PROGRAM_SIZE;
    const BLOCK_SIZE: usize = FLASH_PAGE_SIZE;
    const BLOCK_COUNT: usize = LEN / FLASH_PAGE_SIZE;
    const BLOCK_CYCLES: isize = LFS_BLOCK_CYCLES;

    fn read(&mut self, off: usize, buf: &mut [u8]) -> LfsResult<usize> {
        let address = Self::checked_address(off, buf.len())?;
        read(address, buf).map_err(|_| LfsError::IO)?;
        Ok(buf.len())
    }

    fn write(&mut self, off: usize, data: &[u8]) -> LfsResult<usize> {
        if data.len() % FLASH_PROGRAM_SIZE != 0 {
            return Err(LfsError::IO);
        }
        let address = Self::checked_address(off, data.len())?;
        if data.is_empty() {
            return Ok(0);
        }

        let mut flash = UnlockedFlash::unlock().map_err(|_| LfsError::IO)?;
        for (index, chunk) in data.chunks_exact(FLASH_PROGRAM_SIZE).enumerate() {
            let mut quadword = [0u8; FLASH_PROGRAM_SIZE];
            quadword.copy_from_slice(chunk);
            let chunk_address = address + (index * FLASH_PROGRAM_SIZE) as u32;
            flash
                .program_quadword(chunk_address, &quadword)
                .map_err(|_| LfsError::IO)?;
        }
        Ok(data.len())
    }

    fn erase(&mut self, off: usize, len: usize) -> LfsResult<usize> {
        if off % FLASH_PAGE_SIZE != 0 || len % FLASH_PAGE_SIZE != 0 {
            return Err(LfsError::IO);
        }
        let address = Self::checked_address(off, len)?;
        if len == 0 {
            return Ok(0);
        }

        let mut flash = UnlockedFlash::unlock().map_err(|_| LfsError::IO)?;
        for page in 0..(len / FLASH_PAGE_SIZE) {
            flash
                .erase_page(address + (page * FLASH_PAGE_SIZE) as u32)
                .map_err(|_| LfsError::IO)?;
        }
        Ok(len)
    }
}

pub fn storage_is_erased<S: Storage>(storage: &mut S) -> bool {
    let Some(len) = S::BLOCK_SIZE.checked_mul(S::BLOCK_COUNT) else {
        return false;
    };
    let mut buf = [0u8; 256];
    let mut off = 0;
    while off < len {
        let read_len = core::cmp::min(buf.len(), len - off);
        if storage.read(off, &mut buf[..read_len]).is_err() {
            return false;
        }
        if buf[..read_len].iter().any(|&byte| byte != 0xff) {
            return false;
        }
        off += read_len;
    }
    true
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
