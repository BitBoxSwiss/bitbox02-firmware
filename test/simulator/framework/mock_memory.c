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

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>

#include <flags.h>
#include <memory/memory.h>
#include <mock_memory.h>
#include <stdio.h>
#include <string.h>

static uint8_t _memory_shared_data[FLASH_SHARED_DATA_LEN] = {0};
static uint8_t _memory_app_data[FLASH_APPDATA_LEN] = {0};
static uint8_t _memory_smarteeprom[SMARTEEPROM_RESERVED_FLASH_PAGES * FLASH_PAGE_SIZE] = {0};

void mock_memory_factoryreset(void)
{
    memset(_memory_shared_data, 0xff, sizeof(_memory_shared_data));
    memset(_memory_app_data, 0xff, sizeof(_memory_app_data));
    memset(_memory_smarteeprom, 0xff, sizeof(_memory_smarteeprom));
}

static uint8_t* _get_memory(uint32_t base)
{
    switch (base) {
    case FLASH_SHARED_DATA_START:
        return _memory_shared_data;
    case FLASH_APPDATA_START:
        return _memory_app_data;
    case FLASH_SMARTEEPROM_START:
        return _memory_smarteeprom;
    default:
        return NULL;
    }
}

bool memory_write_to_address_mock(uint32_t base, uint32_t addr, const uint8_t* chunk)
{
    if (chunk == NULL) {
        memset(_get_memory(base) + addr, 0xff, CHUNK_SIZE);
    } else {
        memcpy(_get_memory(base) + addr, chunk, CHUNK_SIZE);
    }
    return true;
}

bool memory_write_chunk_mock(uint32_t chunk_num, const uint8_t* chunk)
{
    return memory_write_to_address_mock(FLASH_APPDATA_START, chunk_num * CHUNK_SIZE, chunk);
}

void memory_read_chunk_mock(uint32_t chunk_num, uint8_t* chunk_out)
{
    memcpy(chunk_out, _memory_app_data + chunk_num * CHUNK_SIZE, CHUNK_SIZE);
}

void memory_read_shared_bootdata_mock(uint8_t* chunk_out)
{
    memcpy(chunk_out, _memory_shared_data, FLASH_SHARED_DATA_LEN);
}