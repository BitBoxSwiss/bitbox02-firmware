// SPDX-License-Identifier: Apache-2.0

pub const FLASH_BASE_NS: usize = 0x0800_0000;
pub const FLASH_TOTAL_SIZE: usize = 4 * 1024 * 1024;
pub const FLASH_PAGE_SIZE: usize = 8 * 1024;

pub const BOOT_ARGS_ADDR: usize = 0x2000_0000;
pub const BOOT_ARGS_LEN: usize = 512;
pub const RAM_ADDR: usize = BOOT_ARGS_ADDR + BOOT_ARGS_LEN;
pub const RAM_LEN: usize = 2496 * 1024 - BOOT_ARGS_LEN;
pub const SRAM4_ADDR: usize = 0x2800_0000;
pub const SRAM4_LEN: usize = 16 * 1024;
pub const GRAM_ADDR: usize = 0xa000_0000;
pub const GRAM_LEN: usize = 64 * 1024 * 1024;

pub const BOOT0_ADDR: usize = 0x0800_2000;
pub const BOOT0_MAX_LEN: usize = 56 * 1024;
pub const BOOT1_ADDR: usize = 0x0801_0000;
pub const BOOT1_MAX_LEN: usize = 256 * 1024;
pub const FIRMWARE_ADDR: usize = 0x0805_2000;
pub const VENDOR_DATA_LEN: usize = 128 * 1024;
pub const USER_DATA_LEN: usize = 512 * 1024;
pub const DFU_BOOT1_MAX_LEN: usize = BOOT1_MAX_LEN;
pub const DFU_BOOT1_ADDR: usize = FLASH_BASE_NS + FLASH_TOTAL_SIZE - DFU_BOOT1_MAX_LEN;
pub const USER_DATA_ADDR: usize = DFU_BOOT1_ADDR - USER_DATA_LEN;
pub const VENDOR_DATA_ADDR: usize = USER_DATA_ADDR - VENDOR_DATA_LEN;
pub const FIRMWARE_MAX_LEN: usize = VENDOR_DATA_ADDR - FIRMWARE_ADDR;
pub const IMMUTABLE_PAGE_ADDR: usize = FLASH_BASE_NS;

// These values are part of the BitBox03 memory layout and must not change without an explicit
// migration.
const _: () = {
    assert!(FLASH_BASE_NS == 0x0800_0000);
    assert!(FLASH_TOTAL_SIZE == 4 * 1024 * 1024);
    assert!(FLASH_PAGE_SIZE == 8 * 1024);

    assert!(BOOT_ARGS_ADDR == 0x2000_0000);
    assert!(BOOT_ARGS_LEN == 512);
    assert!(RAM_ADDR == 0x2000_0200);
    assert!(RAM_LEN == 0x0026_FE00);
    assert!(SRAM4_ADDR == 0x2800_0000);
    assert!(SRAM4_LEN == 16 * 1024);
    assert!(GRAM_ADDR == 0xa000_0000);
    assert!(GRAM_LEN == 64 * 1024 * 1024);

    assert!(IMMUTABLE_PAGE_ADDR == 0x0800_0000);
    assert!(BOOT0_ADDR == 0x0800_2000);
    assert!(BOOT0_MAX_LEN == 56 * 1024);
    assert!(BOOT1_ADDR == 0x0801_0000);
    assert!(BOOT1_MAX_LEN == 256 * 1024);
    assert!(FIRMWARE_ADDR == 0x0805_2000);
    assert!(FIRMWARE_MAX_LEN == 0x002c_e000);
    assert!(VENDOR_DATA_ADDR == 0x0832_0000);
    assert!(VENDOR_DATA_LEN == 128 * 1024);
    assert!(USER_DATA_ADDR == 0x0834_0000);
    assert!(USER_DATA_LEN == 512 * 1024);
    assert!(DFU_BOOT1_ADDR == 0x083c_0000);
    assert!(DFU_BOOT1_MAX_LEN == 256 * 1024);

    assert!(IMMUTABLE_PAGE_ADDR + FLASH_PAGE_SIZE == BOOT0_ADDR);
    assert!(BOOT0_ADDR + BOOT0_MAX_LEN == BOOT1_ADDR);
    assert!(BOOT1_ADDR + BOOT1_MAX_LEN + FLASH_PAGE_SIZE == FIRMWARE_ADDR);
    assert!(FIRMWARE_ADDR + FIRMWARE_MAX_LEN == VENDOR_DATA_ADDR);
    assert!(VENDOR_DATA_ADDR + VENDOR_DATA_LEN == USER_DATA_ADDR);
    assert!(USER_DATA_ADDR + USER_DATA_LEN == DFU_BOOT1_ADDR);
    assert!(DFU_BOOT1_ADDR + DFU_BOOT1_MAX_LEN == FLASH_BASE_NS + FLASH_TOTAL_SIZE);

    assert!(BOOT_ARGS_ADDR + BOOT_ARGS_LEN == RAM_ADDR);
    assert!(RAM_ADDR + RAM_LEN == 0x2027_0000);

    assert!(IMMUTABLE_PAGE_ADDR % FLASH_PAGE_SIZE == 0);
    assert!(BOOT0_ADDR % FLASH_PAGE_SIZE == 0);
    assert!(BOOT0_MAX_LEN % FLASH_PAGE_SIZE == 0);
    assert!(BOOT1_ADDR % FLASH_PAGE_SIZE == 0);
    assert!(BOOT1_MAX_LEN % FLASH_PAGE_SIZE == 0);
    assert!(FIRMWARE_ADDR % FLASH_PAGE_SIZE == 0);
    assert!(FIRMWARE_MAX_LEN % FLASH_PAGE_SIZE == 0);
    assert!(VENDOR_DATA_ADDR % FLASH_PAGE_SIZE == 0);
    assert!(VENDOR_DATA_LEN % FLASH_PAGE_SIZE == 0);
    assert!(USER_DATA_ADDR % FLASH_PAGE_SIZE == 0);
    assert!(USER_DATA_LEN % FLASH_PAGE_SIZE == 0);
    assert!(DFU_BOOT1_ADDR % FLASH_PAGE_SIZE == 0);
    assert!(DFU_BOOT1_MAX_LEN % FLASH_PAGE_SIZE == 0);
};
