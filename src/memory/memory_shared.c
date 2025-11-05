// Copyright 2022 Shift Crypto AG
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

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <driver_init.h>
#include <flags.h>
#include <util.h>
#include <utils_assert.h>

#include "hardfault.h"
#include "memory.h"
#include "memory_shared.h"
#include "random.h"

#ifdef TESTING
    #include <fake_memory.h>
#endif

static void _clean_chunk(uint8_t** chunk_bytes)
{
    util_zero(*chunk_bytes, (size_t)CHUNK_SIZE);
}

#define CLEANUP_CHUNK(var)                                                                    \
    uint8_t* __attribute__((__cleanup__(_clean_chunk))) __attribute__((unused)) var##_bytes = \
        (var).bytes;

// chunk must have size CHUNK_SIZE. if chunk is NULL, the chunk is erased,
// i.e. filled with 0xFF.
//
// `offset` is a relative address (offset from where the memory starts at `base`) and starts at `0`.
static bool _write_to_address(uint32_t base, uint32_t offset, uint8_t* chunk)
{
#ifdef TESTING
    return memory_write_to_address_fake(base, offset, chunk);
#else
    uint32_t addr = base + offset;
    if (addr < base) {
        Abort("uint32_t overflow");
    }
    // Sanity check that the address is correctly aligned,
    // so the erase actually erases only one block.
    if (addr != (addr & ~(CHUNK_SIZE - 1))) {
        return false;
    }
    // Locking granularity is 64 pages, aligned at 16 pages, so we lock/unlock
    // more than just the chunk we want to write.
    const uint32_t lock_size = FLASH_REGION_PAGE_NUM;
    uint32_t lock_addr = addr & ~(lock_size - 1);
    const uint32_t lock_page = lock_addr / FLASH_PAGE_SIZE;
    const size_t n_pages = FLASH_SIZE / FLASH_PAGE_SIZE;
    /*
     * The last address we can unlock is (#pages) - FLASH_REGION_PAGE_NUM.
     * Adjust the address if we are above this point.
     */
    if (lock_page > n_pages - FLASH_REGION_PAGE_NUM && lock_page < n_pages) {
        lock_addr = (n_pages - FLASH_REGION_PAGE_NUM) * FLASH_PAGE_SIZE;
    }

    int res = flash_unlock(&FLASH_0, lock_addr, FLASH_REGION_PAGE_NUM);
    if (res != FLASH_REGION_PAGE_NUM) {
        return false;
    }
    if (chunk == NULL) {
        // Usually has a minimum granularity of 16 pages (one chunk), but the
        // flash_erase driver manually handles smaller/bigger erases.
        if (flash_erase(&FLASH_0, addr, FLASH_ERASE_PAGE_NUM) != ERR_NONE) {
            return false;
        }
    } else {
        // Usually flash_erase is needed before flash_write, the flash_write
        // driver already handles this.
        if (flash_write(&FLASH_0, addr, chunk, CHUNK_SIZE) != ERR_NONE) {
            return false;
        }
    }
    if (flash_lock(&FLASH_0, lock_addr, FLASH_REGION_PAGE_NUM) != FLASH_REGION_PAGE_NUM) {
        // pass, not a critical error.
    }
    return true;
#endif
}

void memory_read_shared_bootdata(chunk_shared_t* chunk_out)
{
#ifdef TESTING
    memory_read_shared_bootdata_fake(chunk_out->bytes);
#else
    memcpy(chunk_out->bytes, (uint8_t*)(FLASH_SHARED_DATA_START), FLASH_SHARED_DATA_LEN);
#endif
}

uint8_t memory_get_screen_type(void)
{
    chunk_shared_t chunk = {0};
    memory_read_shared_bootdata(&chunk);
    uint8_t screen_type = chunk.fields.screen_type;
    util_zero(&chunk, sizeof(chunk));
    switch (screen_type) {
    case MEMORY_SCREEN_TYPE_SSD1312:
        return screen_type;
    default:
        // Just in case the memory was not 0xFF for devices before we started using the
        // `screen_type` field, we default to the old screen type if it is not explicitly set to any
        // other screen type.
        return MEMORY_SCREEN_TYPE_SH1107;
    }
}

uint8_t memory_get_securechip_type(void)
{
    chunk_shared_t chunk = {0};
    memory_read_shared_bootdata(&chunk);
    uint8_t securechip_type = chunk.fields.securechip_type;
    util_zero(&chunk, sizeof(chunk));
    switch (securechip_type) {
    case MEMORY_SECURECHIP_TYPE_OPTIGA:
        return securechip_type;
    default:
        return MEMORY_SECURECHIP_TYPE_ATECC;
    }
}

uint8_t memory_get_platform(void)
{
    chunk_shared_t chunk = {0};
    memory_read_shared_bootdata(&chunk);
    uint8_t platform = chunk.fields.platform;
    util_zero(&chunk, sizeof(chunk));
    switch (platform) {
    case MEMORY_PLATFORM_BITBOX02_PLUS:
        return platform;
    default:
        return MEMORY_PLATFORM_BITBOX02;
    }
}

// Default is BLE ENABLED
#define MEMORY_BLE_ENABLED 0xFF
#define MEMORY_BLE_DISABLED 0x01

bool memory_ble_enabled(void)
{
    chunk_shared_t chunk = {0};
    memory_read_shared_bootdata(&chunk);
    uint8_t ble_enabled = chunk.fields.ble_enabled;
    util_log("ble enabled %x", ble_enabled);
    util_zero(&chunk, sizeof(chunk));
    return ble_enabled != MEMORY_BLE_DISABLED;
}

int16_t memory_get_ble_bond_db(uint8_t* data)
{
#if FACTORYSETUP == 1
    // Always return "empty bond db" in factory setup to ensure idempotency. This will force the BLE
    // chip to always set the bond db when it has booted.
    return -1;
#endif
    chunk_shared_t chunk = {0};
    memory_read_shared_bootdata(&chunk);
    int16_t len = chunk.fields.ble_bond_db_len;
    if (len != -1) {
        memcpy(data, &chunk.fields.ble_bond_db[0], len);
    }

    util_zero(&chunk, sizeof(chunk));
    return len;
}

bool memory_set_ble_bond_db(uint8_t* data, int16_t data_len)
{
    ASSERT(data_len <= MEMORY_BLE_BOND_DB_LEN);
    if (data_len > MEMORY_BLE_BOND_DB_LEN) {
        return false;
    }
    chunk_shared_t chunk = {0};
    CLEANUP_CHUNK(chunk);
    memory_read_shared_bootdata(&chunk);
    chunk.fields.ble_bond_db_len = data_len;
    memcpy(&chunk.fields.ble_bond_db[0], data, data_len);
    if (memcmp(
            (uint8_t*)(FLASH_SHARED_DATA_START),
            chunk.bytes,
            (unsigned int)FLASH_SHARED_DATA_LEN) != 0) {
        util_log("Updated bond db");
        return _write_to_address(FLASH_SHARED_DATA_START, 0, chunk.bytes);
    }
    return true;
}

void memory_get_ble_irk(uint8_t* data)
{
    chunk_shared_t chunk = {0};
    memory_read_shared_bootdata(&chunk);

    memcpy(
        data,
        &chunk.fields.ble_identity_resolving_key,
        sizeof(chunk.fields.ble_identity_resolving_key));

    util_zero(&chunk, sizeof(chunk));
}

void memory_get_ble_identity_address(uint8_t* data)
{
    chunk_shared_t chunk = {0};
    memory_read_shared_bootdata(&chunk);
#if defined(DEBUG)
    uint8_t ones[MEMORY_BLE_ADDR_LEN] = {-1, -1, -1, -1, -1, -1};
    uint8_t zeros[MEMORY_BLE_ADDR_LEN] = {0};
#endif
    // In case address isn't valid, factory setup / hww reset needs to be run
    ASSERT(
        memcmp(&ones[0], &chunk.fields.ble_identity_address[0], sizeof(ones)) != 0 &&
        memcmp(&zeros[0], &chunk.fields.ble_identity_address[0], sizeof(zeros)) != 0);

    memcpy(data, &chunk.fields.ble_identity_address, sizeof(chunk.fields.ble_identity_address));

    util_zero(&chunk, sizeof(chunk));
}

void memory_get_ble_metadata(memory_ble_metadata_t* metadata_out)
{
    chunk_shared_t chunk = {0};
    memory_read_shared_bootdata(&chunk);
    metadata_out->active_index = chunk.fields.ble_active_index;
    memcpy(metadata_out->allowed_firmware_hash, chunk.fields.ble_allowed_firmware_hash, 32);
    metadata_out->firmware_sizes[0] = chunk.fields.ble_firmware_sizes[0];
    metadata_out->firmware_sizes[1] = chunk.fields.ble_firmware_sizes[1];
    metadata_out->firmware_checksums[0] = chunk.fields.ble_firmware_checksums[0];
    metadata_out->firmware_checksums[1] = chunk.fields.ble_firmware_checksums[1];
    util_zero(&chunk, sizeof(chunk));
}

void memory_random_name(char* name_out)
{
    static char cached_name[MEMORY_DEVICE_NAME_MAX_LEN] = {0};

    if (cached_name[0] == 0x00) {
        // Generate 4 random uppercase letters
        uint8_t random[32] = {0};
        random_32_bytes_mcu(random);
        uint8_t letters[4];
        for (size_t i = 0; i < sizeof(letters); i++) {
            letters[i] = 'A' + (random[i] % 26);
        }

        // Format into cached name
        snprintf(
            cached_name,
            MEMORY_DEVICE_NAME_MAX_LEN,
            "BitBox %c%c%c%c",
            letters[0],
            letters[1],
            letters[2],
            letters[3]);
    }

    // Copy cached result to output
    snprintf(name_out, MEMORY_DEVICE_NAME_MAX_LEN, "%s", cached_name);
}
