// SPDX-License-Identifier: Apache-2.0

#ifndef _BB02_FLASH_H_
#define _BB02_FLASH_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// Configures the NVMCTRL flash controller. The NVMCTRL clock must already be enabled.
void flash_init(void);

// Waits until any in-flight flash command has finished.
void flash_deinit(void);

// Disables the boot protection fuse until the next reset.
bool flash_disable_bootprot(void);

// Locks the flash region containing `addr_in_region`.
bool flash_lock_region(uint32_t addr_in_region);

// Unlocks the flash region containing `addr_in_region`.
bool flash_unlock_region(uint32_t addr_in_region);

// Erases `num_pages` pages, preserving bytes in partially covered erase blocks.
bool flash_erase_pages(uint32_t page_addr, uint32_t num_pages);

// Writes bytes, preserving bytes outside the written range.
bool flash_write(uint32_t addr, const uint8_t* data, size_t len);

#endif
