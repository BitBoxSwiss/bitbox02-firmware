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

#include <fake_memory.h>
#include <flags.h>
#include <memory/memory.h>
#include <stdio.h>
#include <string.h>

static uint8_t _memory_shared_data[FLASH_SHARED_DATA_LEN] = {0};
static uint8_t _memory_app_data[FLASH_APPDATA_LEN] = {0};
static uint8_t _memory_smarteeprom[SMARTEEPROM_RESERVED_FLASH_PAGES * FLASH_PAGE_SIZE] = {0};

void fake_memory_factoryreset(void)
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

bool memory_write_to_address_fake(uint32_t base, uint32_t addr, const uint8_t* chunk)
{
    if (chunk == NULL) {
        memset(_get_memory(base) + addr, 0xff, (size_t)CHUNK_SIZE);
    } else {
        memcpy(_get_memory(base) + addr, chunk, (size_t)CHUNK_SIZE);
    }
    return true;
}

bool memory_write_chunk_fake(uint32_t chunk_num, const uint8_t* chunk)
{
    return memory_write_to_address_fake(FLASH_APPDATA_START, chunk_num * (size_t)CHUNK_SIZE, chunk);
}

void memory_read_chunk_fake(uint32_t chunk_num, uint8_t* chunk_out)
{
    memcpy(chunk_out, _memory_app_data + chunk_num * (size_t)CHUNK_SIZE, (size_t)CHUNK_SIZE);
}

void memory_read_shared_bootdata_fake(uint8_t* chunk_out)
{
    memcpy(chunk_out, _memory_shared_data, (size_t)FLASH_SHARED_DATA_LEN);
}

static uint8_t _salt_root[32] = {
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
    0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
};
void fake_memory_set_salt_root(const uint8_t* salt_root)
{
    memcpy(_salt_root, salt_root, 32);
}
bool __wrap_memory_get_salt_root(uint8_t* salt_root_out)
{
    memcpy(salt_root_out, _salt_root, 32);
    return true;
}

// Arbitrary value.
static uint8_t _bootloader_hash[32] =
    "\x71\x3d\xf0\xd5\x8c\x71\x7d\x40\x31\x78\x7c\xdc\x8f\xa3\x5b\x90\x25\x82\xbe\x6a\xb6\xa2\x2e"
    "\x09\xde\x44\x77\xd3\x0e\x22\x30\xfc";

void memory_bootloader_hash_fake(uint8_t* hash_out)
{
    memcpy(hash_out, _bootloader_hash, 32);
}

void memory_set_bootloader_hash_fake(const uint8_t* fake_hash)
{
    // NOLINTNEXTLINE(bugprone-not-null-terminated-result)
    memcpy(_bootloader_hash, fake_hash, sizeof(_bootloader_hash));
}
