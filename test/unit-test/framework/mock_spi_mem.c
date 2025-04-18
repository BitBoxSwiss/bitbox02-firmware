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

#include <memory/spi_mem.h>
#include <stdlib.h>
#include <string.h>

__extension__ static uint8_t _memory[] = {[0 ... SPI_MEM_MEMORY_SIZE] = 0xFF};

void spi_mem_full_erase(void)
{
    memset(_memory, 0xFF, sizeof(_memory));
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
