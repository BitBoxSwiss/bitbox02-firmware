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

#ifndef __SMARTEEPROM_H
#define __SMARTEEPROM_H

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Initializes the SmartEEPROM functionality of the chip
 * with suitable values (page size: 32B; allocated blocks: 2).
 *
 * We need very little EEPROM space. The minimum allocatable size
 * is 2 blocks (16kB). The smaller the page size, the higher the wear
 * levelling.  At the same time though, the maximum number of pages
 * that can be allocated in the SmartEEPROM is 128.
 * So any page size < 32B gives less available EEPROM size, with no real
 * benefit.
 *
 * Ref. SAMD5x/E5x Family Data Sheet, Section 25.6.8 "SmartEEPROM".
 *
 * Note that a reboot is required for this change to take effect.
 */
void smarteeprom_setup(void);

/**
 * Reads N contiguous bytes from the SmartEEPROM, starting at
 * the specified address.
 *
 * @param[in] address Start address.
 * @param[in] size Number of bytes to read.
 * @param[out] out_buffer Buffer in which to read. Must be non-null.
 *                        Must be at least size bytes wide.
 */
void smarteeprom_read(size_t address, size_t bytes, uint8_t* out_buffer);

/**
 * Writes N contiguous bytes to the SmartEEPROM, starting at
 * the specified address.
 *
 * Note that this will not affect the write endurance of the
 * underlying memory, unless the content currently stored at
 * the specified address has some bits that must be flipped from
 * "0" to "1". Use this wisely!
 *
 * @param[in] address Start address.
 * @param[in] size Number of bytes to write.
 * @param[in] buffer Buffer from which to read the data. Must be non-null.
 *                   Must be at least size bytes wide.
 */
void smarteeprom_write(size_t address, size_t bytes, const uint8_t* buffer);

/**
 * Selected page size (in bytes) of each virtual page (SEEP) of the
 * SmartEEPROM. The wear of the underlying flash memory is affected
 * whenever a page flips a bit from 0 to 1. That page (and only that page)
 * will be relocated to a new physical location in flash memory.
 */
#define SMARTEEPROM_PAGE_SIZE (32)

bool smarteeprom_is_enabled(void);

/**
 * Disables the SmartEEPROM.
 *
 * Note that a reboot is required for this change to take effect.
 */
void smarteeprom_disable(void);

/** Value to set SEESTAT.PSZ to, so that SMARTEEPROM_PAGE_SIZE is correct. */
#define SMARTEEPROM_PSZ_VALUE (3)

/**
 * Number of 8KB blocks reserved to the EEPROM. 2 is the minimum that
 * can be allocated.
 */
#define SMARTEEPROM_ALLOCATED_BLOCKS (2)

/**
 * Value of the SBLK register to match SMARTEEPROM_ALLOCATED_BLOCKS.
 */
#define SMARTEEPROM_SBLK_VALUE (SMARTEEPROM_ALLOCATED_BLOCKS / 2)

/**
 * Number of flash pages allocated to the SmartEEPROM.
 */
#define SMARTEEPROM_RESERVED_FLASH_PAGES (SMARTEEPROM_ALLOCATED_BLOCKS * 8192 / FLASH_PAGE_SIZE)

/**
 * Ensures that the system is running the correct configuration
 * for the BitBox02 SmartEEPROM. If the configuration is incorrect,
 * it is adjusted and the devices is rebooted.
 */
void smarteeprom_bb02_config(void);

#endif // __SMARTEEPROM_H
