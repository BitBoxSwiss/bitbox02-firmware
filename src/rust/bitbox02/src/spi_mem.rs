// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

pub use bitbox02_sys::MEMORY_SPI_BLE_FIRMWARE_1_ADDR as BLE_FIRMWARE_1_ADDR;
pub use bitbox02_sys::MEMORY_SPI_BLE_FIRMWARE_2_ADDR as BLE_FIRMWARE_2_ADDR;
pub use bitbox02_sys::MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE as BLE_FIRMWARE_MAX_SIZE;

use alloc::string::String;

pub fn write_protected(address: u32, data: &[u8]) -> Result<(), ()> {
    match unsafe { bitbox02_sys::spi_mem_protected_area_write(address, data.as_ptr(), data.len()) }
    {
        true => Ok(()),
        false => Err(()),
    }
}

pub fn get_active_ble_firmware_version() -> Result<String, ()> {
    let mut ble_fw_version = core::mem::MaybeUninit::uninit();
    unsafe {
        match bitbox02_sys::memory_spi_get_active_ble_firmware_version(ble_fw_version.as_mut_ptr())
        {
            true => {
                let ble_fw_version = ble_fw_version.assume_init();
                // Copy to avoid taking references to unaligned struct fields.
                let version = ble_fw_version.version;
                Ok(format!("{}", version))
            }
            false => Err(()),
        }
    }
}
