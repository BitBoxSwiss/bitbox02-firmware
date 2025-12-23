// SPDX-License-Identifier: Apache-2.0

#ifndef _SPI_MEM_H
#define _SPI_MEM_H

#include "compiler_util.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define SPI_MEM_PAGE_SIZE 0x100 // 256
#define SPI_MEM_SECTOR_SIZE 0x1000 // 4k
#define SPI_MEM_BLOCK_SIZE 0x10000 // 64k
#define SPI_MEM_MEMORY_SIZE 0x200000 // 2M
#define SPI_MEM_PROTECTED_BLOCKS 2 // First 2 blocks - 128KB

/**
 * @brief Erase the entire flash memory chip.
 *
 * This operation is blocking and will take 30-60 seconds.
 *
 * @return true on success, false on error.
 */
bool spi_mem_full_erase(void);

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

/**
 * @brief Enables write protection for the first SPI_MEM_PROTECTED_BLOCKS of the memory.
 *
 * Sets the protection bits in the status register to lock the configured protected
 * memory region, preventing accidental writes or erases.
 *
 */
void spi_mem_protected_area_lock(void);

/**
 * @brief Disables write protection for the protected memory region.
 *
 * Clears the protection bits in the status register, allowing writes and erases
 * to proceed in the previously locked memory area.
 *
 */
void spi_mem_protected_area_unlock(void);

/**
 * @brief Temporarily unlocks and writes to a protected flash memory region.
 *
 * This function reads the current protection configuration, disables protection,
 * writes the specified data, and restores the previous protection settings.
 *
 * @param[in] address  Start address to write to.
 * @param[in] input    Pointer to the data to write.
 * @param[in] size     Number of bytes to write.
 * @return true if the write operation succeeds, false otherwise.
 */
USE_RESULT bool spi_mem_protected_area_write(uint32_t address, const uint8_t* input, size_t size);

void spi_mem_test(void);

#endif // _SPI_MEM_H
