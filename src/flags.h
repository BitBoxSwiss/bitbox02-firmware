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

#ifndef _FLAGS_H_
#define _FLAGS_H_

#ifdef TESTING
#define FLASH_ADDR (0x00000000U)
#define FLASH_SIZE (0x00100000U)
#define FLASH_PAGE_SIZE (512U)
#else
#include <samd51j20a.h>
#endif

#include "memory/smarteeprom.h"

// The total size of the flash is 1MB (2048 pages).
#define FLASH_END (2048 * FLASH_PAGE_SIZE)

// Even though FLASH_SIZE is 1MB (1024*1024 bytes),
// the portion of flash that we use ends at page 2000.
#define FLASH_USER_END (2000 * FLASH_PAGE_SIZE)

// Erase granularity is 1 block (8kB; 16 pages)
// Can (un)lock exactly 64 pages (32kB) at a time,
// aligned at 16 pages (8kB); app start must be multiple of 8kB.
#define FLASH_REGION_NUM (32U)
#define FLASH_REGION_PAGE_NUM (FLASH_SIZE / (FLASH_REGION_NUM * FLASH_PAGE_SIZE)) // = 64
// Erase granularity: can only erase this many pages at a time in a single flash_erase() call.
// (erasing more does not work due to a bug (?) with _flash_erase_block() not working as expected.
#define FLASH_ERASE_PAGE_NUM (16U)
#define FLASH_ERASE_MIN_LEN \
    (FLASH_ERASE_PAGE_NUM * FLASH_PAGE_SIZE) // 8kB; minimum erase granularity
// Bootloader memory area - 64kB total, first 56kB read-only, last 8kB R/W data reserved for
// bootloader
#define FLASH_BOOT_START (FLASH_ADDR)
#define FLASH_BOOT_LEN (0x0000E000U) // 56kB
#define FLASH_SHARED_DATA_START \
    (FLASH_BOOT_START +         \
     FLASH_BOOT_LEN) // Do NOT change! The fixed bootloader needs access to data here
#define FLASH_SHARED_DATA_LEN (FLASH_ERASE_MIN_LEN) // 8kB
#define FLASH_BOOTDATA_LEN (FLASH_ERASE_MIN_LEN) // 8kB
#define FLASH_BOOTDATA_START \
    (FLASH_USER_END -        \
     FLASH_BOOTDATA_LEN) // Do NOT change! The fixed bootloader needs access to data here
#define FLASH_BOOTPROTECTION \
    (15 - FLASH_BOOT_LEN / 8192) // Register value for the boot protection size
                                 // The equation is from datasheet section 25.6.2
// Firmware start
#define FLASH_APP_START (FLASH_BOOT_START + FLASH_BOOT_LEN + FLASH_SHARED_DATA_LEN)
// Appdata is 64kB of the flash space after the app and is reserved for app data.
// Must be a multiple of FLASH_REGION_PAGE_NUM (0x8000 kB = 32kB), the minimum lock/unlock size.
#define FLASH_APPDATA_LEN (0x000010000U)
// Appdata start: If app length is changed, may need to subtract an offset in order to satisfy
// the MPU setup conditions tested below.
#define FLASH_APPDATA_START (FLASH_USER_END - FLASH_BOOTDATA_LEN - FLASH_APPDATA_LEN)
#define FLASH_APP_LEN (FLASH_APPDATA_START - FLASH_APP_START)
#define FLASH_APP_PAGE_NUM (FLASH_APP_LEN / FLASH_PAGE_SIZE)
#define FLASH_APP_VERSION_LEN (4) // 4 byte big endian unsigned int
#define FLASH_APP_VERSION_START (FLASH_APP_START + FLASH_APP_LEN - FLASH_APP_VERSION_LEN)

// SmartEEPROM reserved memory start
#define FLASH_SMARTEEPROM_START (FLASH_END - SMARTEEPROM_RESERVED_FLASH_PAGES * FLASH_PAGE_SIZE)

// Check MPU conditions
#if (FLASH_APP_START % FLASH_ERASE_MIN_LEN)
#error "Appdata start must be aligned to the minimum erase granularity"
#endif
#if (FLASH_BOOTDATA_START % FLASH_ERASE_MIN_LEN)
#error "Bootdata start must be aligned to the minimum erase granularity"
#endif
#if (FLASH_BOOTDATA_START % FLASH_BOOTDATA_LEN)
#error "Bootdata start must be a multiple of bootdata length for the MPU setup"
#endif
#if (FLASH_APPDATA_START % FLASH_ERASE_MIN_LEN)
#error "Appdata start must be aligned to the minimum erase granularity"
#endif
#if (FLASH_APPDATA_START % (FLASH_APPDATA_LEN / 2))
#error "Appdata region 0 start must be a multiple of app region 0 data length for the MPU setup"
#endif
#if ((FLASH_APPDATA_START + FLASH_APPDATA_LEN / 2) % (FLASH_APPDATA_LEN / 2))
#error "Appdata region 1 start must be a multiple of app region 1 data length for the MPU setup"
#endif

#endif
