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
#include <memory/memory_shared.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#define FAKE_MEMORY_ENV_VAR "FAKE_MEMORY_FILEPATH"
#define SHARED_SUFFIX "shared"
#define APP_SUFFIX "app"
#define EEPROM_SUFFIX "eeprom"

static uint8_t _memory_shared_data[FLASH_SHARED_DATA_LEN] = {0};
static uint8_t _memory_app_data[FLASH_APPDATA_LEN] = {0};
static uint8_t _memory_smarteeprom[SMARTEEPROM_RESERVED_FLASH_PAGES * FLASH_PAGE_SIZE] = {0};

static void _init_file_if_needed(
    const char* base_path,
    const char* suffix,
    const uint8_t* data,
    size_t data_size)
{
    char path[512];
    int written = snprintf(path, sizeof(path), "%s_%s", base_path, suffix);
    if (written < 0 || (size_t)written >= sizeof(path)) {
        fprintf(stderr, "file path %s_%s too long\n", base_path, suffix);
        exit(EXIT_FAILURE);
    }

    struct stat st;
    if (stat(path, &st) == 0) {
        return; // The file already exists, do nothing.
    }

    FILE* f = fopen(path, "wb");
    if (f) {
        fwrite(data, 1, data_size, f);
        fclose(f);
    } else {
        fprintf(stderr, "error: could not create fake memory file %s\n", path);
        exit(EXIT_FAILURE);
    }
}

void fake_memory_factoryreset(void)
{
    memset(_memory_shared_data, 0xff, sizeof(_memory_shared_data));
    memset(_memory_app_data, 0xff, sizeof(_memory_app_data));
    memset(_memory_smarteeprom, 0xff, sizeof(_memory_smarteeprom));

    const char* base_path = getenv(FAKE_MEMORY_ENV_VAR);
    if (base_path) {
        _init_file_if_needed(
            base_path, SHARED_SUFFIX, _memory_shared_data, sizeof(_memory_shared_data));
        _init_file_if_needed(base_path, APP_SUFFIX, _memory_app_data, sizeof(_memory_app_data));
        _init_file_if_needed(
            base_path, EEPROM_SUFFIX, _memory_smarteeprom, sizeof(_memory_smarteeprom));
    }
}

#define ALLOWED_HASH                                                                               \
    "\x1e\x4a\xa8\x36\x4e\x93\x5c\x07\x85\xe4\xf8\x91\x20\x83\x07\xd8\x32\xf7\x88\x17\x2e\x4b\xf6" \
    "\x16\x21\xde\x6d\xf9\xec\x3c\x21\x5f"

bool fake_memory_nova(void)
{
    chunk_shared_t* shared_ptr = (chunk_shared_t*)&_memory_shared_data[0];
    shared_ptr->fields.platform = MEMORY_PLATFORM_BITBOX02_PLUS;

    memory_ble_metadata_t ble_metadata = {0};
    memcpy(ble_metadata.allowed_firmware_hash, ALLOWED_HASH, sizeof(ALLOWED_HASH) - 1);
    if (!memory_set_ble_metadata(&ble_metadata)) {
        return false;
    }
    if (!memory_ble_enable(false)) {
        return false;
    }
    return true;
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

static void _write_file_chunk(const char* suffix, uint32_t offset, const uint8_t* chunk, size_t len)
{
    char path[512];
    const char* base_path = getenv(FAKE_MEMORY_ENV_VAR);
    if (!base_path) {
        return;
    }
    int written = snprintf(path, sizeof(path), "%s_%s", base_path, suffix);
    if (written < 0 || (size_t)written >= sizeof(path)) {
        fprintf(stderr, "file path %s_%s too long\n", base_path, suffix);
        exit(EXIT_FAILURE);
    }
    FILE* f = fopen(path, "r+b");
    if (!f) {
        fprintf(stderr, "error: fake memory file %s not found\n", path);
        exit(EXIT_FAILURE);
    }
    fseek(f, offset, SEEK_SET);
    fwrite(chunk, 1, len, f);
    fclose(f);
}

static void _read_file_chunk(const char* suffix, uint32_t offset, uint8_t* chunk, size_t len)
{
    char path[512];
    const char* base_path = getenv(FAKE_MEMORY_ENV_VAR);
    if (!base_path) {
        return;
    }
    int written = snprintf(path, sizeof(path), "%s_%s", base_path, suffix);
    if (written < 0 || (size_t)written >= sizeof(path)) {
        fprintf(stderr, "file path %s_%s too long\n", base_path, suffix);
        exit(EXIT_FAILURE);
    }
    FILE* f = fopen(path, "rb");
    if (!f) {
        fprintf(stderr, "error: fake memory file %s not found\n", path);
        exit(EXIT_FAILURE);
    }
    fseek(f, offset, SEEK_SET);
    size_t nread = fread(chunk, 1, len, f);
    if (nread != len) {
        fprintf(stderr, "expected %zu bytes, got %zu\n", len, nread);
        exit(EXIT_FAILURE);
    }
    fclose(f);
}

bool memory_write_to_address_fake(uint32_t base, uint32_t addr, const uint8_t* chunk)
{
    const char* base_path = getenv(FAKE_MEMORY_ENV_VAR);

    if (base_path) {
        size_t len = CHUNK_SIZE;
        const char* suffix = NULL;
        switch (base) {
        case FLASH_SHARED_DATA_START:
            suffix = SHARED_SUFFIX;
            break;
        case FLASH_APPDATA_START:
            suffix = APP_SUFFIX;
            break;
        case FLASH_SMARTEEPROM_START:
            suffix = EEPROM_SUFFIX;
            break;
        default:
            return false;
        }
        if (chunk == NULL) {
            uint8_t empty[CHUNK_SIZE];
            memset(empty, 0xff, CHUNK_SIZE);
            _write_file_chunk(suffix, addr, empty, len);
        } else {
            _write_file_chunk(suffix, addr, chunk, len);
        }
        return true;
    }

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
    const char* base_path = getenv(FAKE_MEMORY_ENV_VAR);
    if (base_path) {
        _read_file_chunk(APP_SUFFIX, chunk_num * (size_t)CHUNK_SIZE, chunk_out, (size_t)CHUNK_SIZE);
        return;
    }

    memcpy(chunk_out, _memory_app_data + chunk_num * (size_t)CHUNK_SIZE, (size_t)CHUNK_SIZE);
}

void memory_read_shared_bootdata_fake(uint8_t* chunk_out)
{
    const char* base_path = getenv(FAKE_MEMORY_ENV_VAR);
    if (base_path) {
        _read_file_chunk(SHARED_SUFFIX, 0, chunk_out, (size_t)FLASH_SHARED_DATA_LEN);
        return;
    }

    memcpy(chunk_out, _memory_shared_data, (size_t)FLASH_SHARED_DATA_LEN);
}

static uint8_t _salt_root[32] = {
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
    0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33,
};

const uint8_t* fake_memory_get_salt_root(void)
{
    return _salt_root;
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
