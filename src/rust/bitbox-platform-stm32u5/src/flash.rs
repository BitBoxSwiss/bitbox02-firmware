// SPDX-License-Identifier: Apache-2.0

use crate::ffi;

pub const FLASH_BASE_NS: usize = 0x0800_0000;
pub const TOTAL_SIZE: usize = 4 * 1024 * 1024;
pub const PAGE_SIZE: usize = 8 * 1024;

const BANK_SIZE: usize = TOTAL_SIZE / 2;
const QUADWORD_SIZE: usize = 16;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Error {
    InvalidAddress,
    Unaligned,
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

fn check_range(addr: usize, len: usize) -> Result<(), Error> {
    let Some(end) = addr.checked_add(len) else {
        return Err(Error::InvalidAddress);
    };
    if addr < FLASH_BASE_NS || end > FLASH_BASE_NS + TOTAL_SIZE {
        return Err(Error::InvalidAddress);
    }
    Ok(())
}

fn bank_for_address(addr: usize) -> u32 {
    if addr < FLASH_BASE_NS + BANK_SIZE {
        ffi::FLASH_BANK_1
    } else {
        ffi::FLASH_BANK_2
    }
}

fn page_for_address(addr: usize) -> u32 {
    let bank_start = if bank_for_address(addr) == ffi::FLASH_BANK_1 {
        FLASH_BASE_NS
    } else {
        FLASH_BASE_NS + BANK_SIZE
    };
    ((addr - bank_start) / PAGE_SIZE) as u32
}

pub fn read(addr: usize, out: &mut [u8]) {
    check_range(addr, out.len()).expect("flash read address out of range");
    unsafe {
        core::ptr::copy_nonoverlapping(addr as *const u8, out.as_mut_ptr(), out.len());
    }
}

pub fn write_page(addr: usize, page: &[u8; PAGE_SIZE]) -> Result<(), Error> {
    if addr % PAGE_SIZE != 0 {
        return Err(Error::Unaligned);
    }
    check_range(addr, PAGE_SIZE)?;

    unsafe {
        Error::from_hal_status(ffi::HAL_FLASH_Unlock())?;

        let mut page_error = 0u32;
        let mut erase = ffi::FLASH_EraseInitTypeDef {
            TypeErase: ffi::FLASH_TYPEERASE_PAGES,
            Banks: bank_for_address(addr),
            Page: page_for_address(addr),
            NbPages: 1,
        };
        let erase_status = ffi::HAL_FLASHEx_Erase(&mut erase, &mut page_error);
        if let Err(err) = Error::from_hal_status(erase_status) {
            let _ = ffi::HAL_FLASH_Lock();
            return Err(err);
        }

        for (index, quadword) in page.chunks_exact(QUADWORD_SIZE).enumerate() {
            let program_status = ffi::HAL_FLASH_Program(
                ffi::FLASH_TYPEPROGRAM_QUADWORD,
                (addr + index * QUADWORD_SIZE) as u32,
                quadword.as_ptr() as usize as u32,
            );
            if let Err(err) = Error::from_hal_status(program_status) {
                let _ = ffi::HAL_FLASH_Lock();
                return Err(err);
            }
        }

        Error::from_hal_status(ffi::HAL_FLASH_Lock())?;
    }

    let verify = unsafe { core::slice::from_raw_parts(addr as *const u8, PAGE_SIZE) };
    if verify != page {
        return Err(Error::Verify);
    }
    Ok(())
}
