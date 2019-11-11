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

#ifndef _MPU_H_
#define _MPU_H_
#ifndef TESTING

#include <stdint.h>

/**
 * MPU - Memory Protection Unit
 */
#define MPU_ENABLE (0x01)
#define MPU_DISABLE (0x00)

/**
 * RBAR - Region Base Address Register (CMSIS: SCB->RBAR)
 *
 * Bit 0..3   - Region number. `0` is lowest priority. `7` is highest priority (see below)
 * Bit 4      - Region valid
 * Bit 5..31  - Base address of the region. Bottom 5 bits ignored. Base address must be a multiple
 * of its region size. Regions can overlap.
 */

/**
 * Region numbers - as set in the MPU_RNR register (0-7).
 * MPU rules will be evaluated using the order of the region number.
 * The last match for a region will be the one that is enforced; so
 * regions with the higher number have higher priority.
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

/**
 * Computes the "Region size" (bits 1..5) field of the RASR (see above),
 * given the desired value.
 * @param[in] size Value of the field
 * @return Value of the register.
 */
uint32_t mpu_region_size(uint32_t size);

/** Set region properties and enable region */
void mpu_set_region(uint32_t rbar, uint32_t rasr);

/** Disable the region with the given number */
void mpu_disable_region(uint32_t region_number);

/**
 * Sets the correct MPU configuration for BitBox firmwares.
 *
 * Extends the APPDATA_1 MPU region up to the end of the
 * flash memory, and sets it to R/W.
 */
void mpu_bitbox02_init(void);

#endif
#endif
