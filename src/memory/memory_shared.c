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

#include <flags.h>
#include <util.h>
#include <utils_assert.h>

#include "memory_shared.h"

#ifdef TESTING
#include <mock_memory.h>
#endif

void memory_read_shared_bootdata(chunk_shared_t* chunk_out)
{
#ifdef TESTING
    memory_read_shared_bootdata_mock(chunk_out->bytes);
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

int16_t memory_get_ble_bond_db(uint8_t* data)
{
    chunk_shared_t chunk = {0};
    memory_read_shared_bootdata(&chunk);
    int16_t len = chunk.fields.ble_bond_db_len;
    if (len != -1) {
        memcpy(data, &chunk.fields.ble_bond_db[0], len);
    }

    util_zero(&chunk, sizeof(chunk));
    return len;
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
