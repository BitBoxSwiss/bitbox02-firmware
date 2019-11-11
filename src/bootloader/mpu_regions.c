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

#include "mpu_regions.h"

#ifndef TESTING

#include <flags.h>
#include <mpu.h>

#include <core_cm4.h>
#include <samd51j20a.h>

#include <stdint.h>

static void _set_mpu_regions(void)
{
    uint32_t rbar;
    uint32_t rasr;

    // Whole flash background region
    // read-write
    // 0 to 1 MB
    rbar = FLASH_ADDR | MPU_REGION_VALID | MPU_REGION_NUMBER_FLASH;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | mpu_region_size(FLASH_SIZE) |
           MPU_REGION_READ_WRITE;
    mpu_set_region(rbar, rasr);

    // SRAM region
    // read-write non-excutable (overlaps flash region but higher priority)
    rbar = HSRAM_ADDR | MPU_REGION_VALID | MPU_REGION_NUMBER_SRAM;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           mpu_region_size(HSRAM_SIZE) | MPU_REGION_READ_WRITE;
    mpu_set_region(rbar, rasr);

    // Bootloader region
    // read-only (overlaps flash region but has higher priority)
    // 0 to 64 kB
    rbar = FLASH_BOOT_START | MPU_REGION_VALID | MPU_REGION_NUMBER_BOOTLOADER;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL |
           mpu_region_size(FLASH_APP_START - FLASH_BOOT_START) | MPU_REGION_READ_ONLY;
    mpu_set_region(rbar, rasr);

    // Shared data region after bootloader and before firmware app
    // read-write non-excutable (overlaps bootloader region but higher priority)
    // 56 to 64 kB
    rbar = FLASH_SHARED_DATA_START | MPU_REGION_VALID | MPU_REGION_NUMBER_SHARED_DATA;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           mpu_region_size(FLASH_SHARED_DATA_LEN) | MPU_REGION_READ_WRITE;
    mpu_set_region(rbar, rasr);

    // Appdata region 0 after firmware app
    // Due to MPU alignment rules, the 64kB area must be split into 2 32kB regions.
    // read-write non-excutable (overlaps flash region but higher priority)
    // End of FLASH_APP to FLASH_END - 8kB
    rbar = FLASH_APPDATA_START | MPU_REGION_VALID | MPU_REGION_NUMBER_APPDATA_0;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           mpu_region_size(FLASH_APPDATA_LEN / 2) | MPU_REGION_READ_WRITE;
    mpu_set_region(rbar, rasr);
    // Appdata region 1 after firmware app
    rbar = (FLASH_APPDATA_START + (FLASH_APPDATA_LEN / 2)) | MPU_REGION_VALID |
           MPU_REGION_NUMBER_APPDATA_1;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           mpu_region_size(FLASH_APPDATA_LEN / 2) | MPU_REGION_READ_WRITE;
    mpu_set_region(rbar, rasr);

    // Bootdata region
    // read-write non-excutable
    // FLASH_END - 8kB to FLASH_END (overlaps flash region but higher priority)
    rbar = FLASH_BOOTDATA_START | MPU_REGION_VALID | MPU_REGION_NUMBER_BOOTDATA;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           mpu_region_size(FLASH_BOOTDATA_LEN) | MPU_REGION_READ_WRITE;
    mpu_set_region(rbar, rasr);
}

static void _update_mpu_regions(void)
{
    uint32_t rbar;
    uint32_t rasr;

    // Whole flash region background
    // read-only
    rbar = FLASH_ADDR | MPU_REGION_VALID | MPU_REGION_NUMBER_FLASH;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | mpu_region_size(FLASH_SIZE) |
           MPU_REGION_READ_ONLY;
    mpu_disable_region(MPU_REGION_NUMBER_FLASH);
    mpu_set_region(rbar, rasr);

    // Bootdata region
    // no-access non-excutable (overlaps flash region but higher priority)
    rbar = FLASH_BOOTDATA_START | MPU_REGION_VALID | MPU_REGION_NUMBER_BOOTDATA;
    rasr = MPU_REGION_ENABLE | MPU_REGION_TYPE_NORMAL | MPU_REGION_EXECUTE_NEVER |
           mpu_region_size(FLASH_BOOTDATA_LEN) | MPU_REGION_NO_ACCESS;
    mpu_disable_region(MPU_REGION_NUMBER_BOOTDATA);
    mpu_set_region(rbar, rasr);
}

void mpu_regions_bootloader_init(void)
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

void mpu_regions_firmware_init(void)
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
