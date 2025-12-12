// SPDX-License-Identifier: Apache-2.0

#include <memory/spi_mem.h>
#include <stdlib.h>
#include <string.h>

__extension__ static uint8_t _memory[] = {[0 ... SPI_MEM_MEMORY_SIZE] = 0xFF};

bool spi_mem_full_erase(void)
{
    memset(_memory, 0xFF, sizeof(_memory));
    return true;
}

bool spi_mem_write(uint32_t address, const uint8_t* input, size_t size)
{
    memcpy(&_memory[address], input, size);
    return true;
}

uint8_t* spi_mem_read(uint32_t address, size_t size)
{
    uint8_t* result = (uint8_t*)malloc(size);
    if (!result) {
        return NULL;
    }
    memcpy(result, &_memory[address], size);
    return result;
}

void spi_mem_protected_area_lock(void) {}

void spi_mem_protected_area_unlock(void) {}

bool spi_mem_protected_area_write(uint32_t address, const uint8_t* input, size_t size)
{
    return spi_mem_write(address, input, size);
}
