// Copyright 2025 Shift Crypto AG
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

#ifndef _SPI_MEM_H
#define _SPI_MEM_H

#include "compiler_util.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define SPI_MEM_PAGE_SIZE 0x100 // 256
#define SPI_MEM_SECTOR_SIZE 0x1000 // 4k
#define SPI_MEM_BLOCK_SIZE 0x8000 // 32k
#define SPI_MEM_MEMORY_SIZE 0x200000 // 2M

/**
 * @brief Erase the entire flash memory chip.
 *
 * This operation is blocking and will take 30-60 seconds.
 */
void spi_mem_full_erase(void);

/**
 * @brief Erase a single SECTOR_SIZE sector from flash memory.
 *
 * @param[in] sector_addr Sector-aligned address (must be divisible by SECTOR_SIZE).
 * @return true on success, false on error.
 */
USE_RESULT bool spi_mem_sector_erase(uint32_t sector_addr);

/**
 * @brief Read a full page (PAGE_SIZE bytes) from flash memory.
 *
 * @param[in] page_addr Page-aligned address (must be divisible by PAGE_SIZE).
 * @param[out] data_out Pointer to buffer to store the read data (must be at least PAGE_SIZE bytes).
 * @return true on success, false on error.
 */
USE_RESULT bool spi_mem_page_read(uint32_t page_addr, uint8_t* data_out);

/**
 * @brief Read arbitrary-sized data from flash memory.
 *
 * This function allocates a buffer of `size` bytes on the heap using malloc
 * and returns a pointer to it. The caller is responsible for freeing the
 * memory using free() to avoid memory leaks.
 *
 * @param[in] address Start address to read from.
 * @param[in] size Number of bytes to read.
 * @return Pointer to allocated buffer containing the read data on success,
 *         or NULL on error (e.g., out-of-bounds or allocation failure).
 */
USE_RESULT uint8_t* spi_mem_read(uint32_t address, size_t size);

/**
 * @brief Write arbitrary-sized data to flash memory (across pages/sectors).
 *
 * This function handles reading, modifying, and rewriting affected sectors
 * as needed. Sectors are erased automatically before writing.
 *
 * @param[in] address Start address to write to.
 * @param[in] input Pointer to the data to be written.
 * @param[in] size Number of bytes to write.
 * @return true on success, false on error.
 */
USE_RESULT bool spi_mem_write(uint32_t address, const uint8_t* input, size_t size);

/**
 * @brief Erases only non-erased sectors in flash memory.
 *
 * This function scans all sectors of the flash memory and performs an erase
 * only on sectors that contain at least one byte different from 0xFF.
 * It helps avoiding unnecessary erasures, and is faster than full erase if
 * the memory is mostly empty.
 *
 * @return The number of sectors erased on success, -1 on failure.
 */
USE_RESULT int32_t spi_mem_smart_erase(void);

#endif // _SPI_MEM_H
