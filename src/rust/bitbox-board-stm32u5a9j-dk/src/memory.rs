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
