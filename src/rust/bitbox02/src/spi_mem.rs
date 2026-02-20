// SPDX-License-Identifier: Apache-2.0

extern crate alloc;

/// Start address of BLE firmware slot 1 in SPI memory. Cannot change this as it defines the memory
/// layout.
pub const BLE_FIRMWARE_1_ADDR: u32 = 0x00;

/// Start address of BLE firmware slot 2 in SPI memory. Cannot change this as it defines the memory
/// layout.
pub const BLE_FIRMWARE_2_ADDR: u32 = bitbox_hal::memory::BLE_FIRMWARE_MAX_SIZE as u32;
const _: [(); 32 * 1024] = [(); bitbox_hal::memory::BLE_FIRMWARE_MAX_SIZE];

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
