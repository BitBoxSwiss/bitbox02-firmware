// Copyright 2019 Shift Cryptosecurity AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include "mpu.h"

#include <flags.h>
#include <stdint.h>
#ifndef TESTING
#include <core_cm4.h>
#include <samd51j20a.h>

/**
 * MPU - Memory Protection Unit
 */
#define MPU_ENABLE (0x01)
#define MPU_DISABLE (0x00)

/**
 * RBAR - Region Base Address Register (CMSIS: SCB->RBAR)
 *
 * Bit 0..3   - Region number. `0` is lowest priority. `7` is highest priority
 * Bit 4      - Region valid
 * Bit 5..31  - Base address of the region. Bottom 5 bits ignored. Base address must be a multiple
 * of its region size. Regions can overlap.
 */
#define MPU_REGION_NUMBER_FLASH (0u)
#define MPU_REGION_NUMBER_SRAM (1u)
#define MPU_REGION_NUMBER_BOOTLOADER (2u)
#define MPU_REGION_NUMBER_SHARED_DATA (3u)
#define MPU_REGION_NUMBER_APPDATA_0 (4u)
#define MPU_REGION_NUMBER_APPDATA_1 (5u)
#define MPU_REGION_NUMBER_BOOTDATA (6u)
#define MPU_REGION_VALID (0x01 << MPU_RBAR_VALID_Pos)

/**
 * RASR - MPU Region Attribute and Size Register (CMSIS: SCB->RASR)
 *
 * Bit 0      – Enable region
 * Bit 1..5   – Region size; 2^(flash_size + 1)
 * Bit 16..21 – Memory type; TEX | Shareable | Cacheable | Bufferable
 * Bit 24..26 – Access Privilege
 * Bit 28     – Non-excutable region (XN)
 */
#define MPU_REGION_DISABLE (0x00 << MPU_RASR_ENABLE_Pos)
#define MPU_REGION_ENABLE (0x01 << MPU_RASR_ENABLE_Pos)
#define MPU_REGION_TYPE_NORMAL \
    (0x08 << MPU_RASR_ATTRS_Pos) // TEX:b001 S:b0 C:b0 B:b0 - non-shareable, non-cacheable
#define MPU_REGION_TYPE_STRONGLY_ORDERED (0x04 << MPU_RASR_ATTRS_Pos) // TEX:b000 S:b1 C:b0 B:b0
#define MPU_REGION_TYPE_DEVICE (0x05 << MPU_RASR_ATTRS_Pos) // TEX:b000 S:b1 C:b0 B:b1
#define MPU_REGION_NO_ACCESS (0x00 << MPU_RASR_AP_Pos)
#define MPU_REGION_PRIVILEGED_READ_WRITE (0x01 << MPU_RASR_AP_Pos) // No user-mode access
#define MPU_REGION_PRIVILEGED_RW_USER_RO (0x02 << MPU_RASR_AP_Pos) // No user-mode access
#define MPU_REGION_READ_WRITE (0x03 << MPU_RASR_AP_Pos)
#define MPU_REGION_PRIVILEGED_READ_ONLY (0x05 << MPU_RASR_AP_Pos) // No user-mode access
#define MPU_REGION_READ_ONLY (0x06 << MPU_RASR_AP_Pos)
#define MPU_REGION_EXECUTE_NEVER (0x01 << MPU_RASR_XN_Pos)

static uint32_t _region_size(uint32_t size)
{
    uint32_t regionSize = 32;
    uint32_t ret = 4;

    while (ret < 31) {
        if (size <= regionSize) {
            break;
        }
        ret++;
        regionSize <<= 1;
    }
    return (ret << MPU_RASR_SIZE_Pos);
}

// Set region properties and enable region
static void _set_region(uint32_t rbar, uint32_t rasr)
{
    MPU->RBAR = rbar;
    MPU->RASR = rasr;
}

// Disable a region
static void _disable_region(uint32_t region_number)
{
    MPU->RNR = region_number;
    MPU->RASR &= 0xfffffffe;
}

static void _set_mpu_regions(void)
{
    uint32_t rbar;
    uint32_t rasr;

    // Whole flash background region
    // read-write
    // 0 to 1 MB
    rbar = FLASH_ADDR | MPU_REGION_VALID | MPU_REGION_NUMBER_FLASH;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | _region_size(FLASH_SIZE) |
           MPU_REGION_READ_WRITE;
    _set_region(rbar, rasr);

    // SRAM region
    // read-write non-excutable (overlaps flash region but higher priority)
    rbar = HSRAM_ADDR | MPU_REGION_VALID | MPU_REGION_NUMBER_SRAM;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           _region_size(HSRAM_SIZE) | MPU_REGION_READ_WRITE;
    _set_region(rbar, rasr);

    // Bootloader region
    // read-only (overlaps flash region but has higher priority)
    // 0 to 64 kB
    rbar = FLASH_BOOT_START | MPU_REGION_VALID | MPU_REGION_NUMBER_BOOTLOADER;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL |
           _region_size(FLASH_APP_START - FLASH_BOOT_START) | MPU_REGION_READ_ONLY;
    _set_region(rbar, rasr);

    // Shared data region after bootloader and before firmware app
    // read-write non-excutable (overlaps bootloader region but higher priority)
    // 56 to 64 kB
    rbar = FLASH_SHARED_DATA_START | MPU_REGION_VALID | MPU_REGION_NUMBER_SHARED_DATA;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           _region_size(FLASH_SHARED_DATA_LEN) | MPU_REGION_READ_WRITE;
    _set_region(rbar, rasr);

    // Appdata region 0 after firmware app
    // Due to MPU alignment rules, the 64kB area must be split into 2 32kB regions.
    // read-write non-excutable (overlaps flash region but higher priority)
    // End of FLASH_APP to FLASH_END - 8kB
    rbar = FLASH_APPDATA_START | MPU_REGION_VALID | MPU_REGION_NUMBER_APPDATA_0;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           _region_size(FLASH_APPDATA_LEN / 2) | MPU_REGION_READ_WRITE;
    _set_region(rbar, rasr);
    // Appdata region 1 after firmware app
    rbar = (FLASH_APPDATA_START + (FLASH_APPDATA_LEN / 2)) | MPU_REGION_VALID |
           MPU_REGION_NUMBER_APPDATA_1;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           _region_size(FLASH_APPDATA_LEN / 2) | MPU_REGION_READ_WRITE;
    _set_region(rbar, rasr);

    // Bootdata region
    // read-write non-excutable
    // FLASH_END - 8kB to FLASH_END (overlaps flash region but higher priority)
    rbar = FLASH_BOOTDATA_START | MPU_REGION_VALID | MPU_REGION_NUMBER_BOOTDATA;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           _region_size(FLASH_BOOTDATA_LEN) | MPU_REGION_READ_WRITE;
    _set_region(rbar, rasr);
}

static void _update_mpu_regions(void)
{
    uint32_t rbar;
    uint32_t rasr;

    // Whole flash region background
    // read-only
    rbar = FLASH_ADDR | MPU_REGION_VALID | MPU_REGION_NUMBER_FLASH;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | _region_size(FLASH_SIZE) |
           MPU_REGION_READ_ONLY;
    _disable_region(MPU_REGION_NUMBER_FLASH);
    _set_region(rbar, rasr);

    // Bootdata region
    // no-access non-excutable (overlaps flash region but higher priority)
    rbar = FLASH_BOOTDATA_START | MPU_REGION_VALID | MPU_REGION_NUMBER_BOOTDATA;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           _region_size(FLASH_BOOTDATA_LEN) | MPU_REGION_NO_ACCESS;
    _disable_region(MPU_REGION_NUMBER_BOOTDATA);
    _set_region(rbar, rasr);
}

void mpu_bootloader_init(void)
{
    // Disable interrupts
    __disable_irq();

    // Wait for all memory accesses and instructions to complete
    __DSB();
    __ISB();

    MPU->CTRL = MPU_DISABLE;

    _set_mpu_regions();

    // ENAble MEMory management FAULT exception, i.e., use of MemManage_Handler()
    SCB->SHCSR |= SCB_SHCSR_MEMFAULTENA_Msk;

    // ENABLE the MPU region
    // ENAable DEFault memory map for memory areas not specified by the MPU in PRIVledged mode;
    // otherwise a fault occurs. ENAble MPU operation during HardFault and NMI handlers.
    MPU->CTRL = MPU_ENABLE | MPU_CTRL_PRIVDEFENA_Msk | MPU_CTRL_HFNMIENA_Msk;

    // Wait for all memory accesses and instructions to complete
    __DSB();
    __ISB();

    // Enable interrupts
    __enable_irq();
}

void mpu_firmware_init(void)
{
    // Disable interrupts
    __disable_irq();

    // Wait for all memory accesses and instructions to complete
    __DSB();
    __ISB();

    _update_mpu_regions();

    // Wait for all memory accesses and instructions to complete
    __DSB();
    __ISB();

    // Enable interrupts
    __enable_irq();
}
#endif
