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

#include "memory_spi.h"
#include "memory.h"
#include "memory_shared.h"
#include "spi_mem.h"

#include <hardfault.h>
#include <rust/rust.h>
#include <util.h>

bool memory_spi_get_active_ble_firmware(
    uint8_t** firmware_out,
    size_t* size_out,
    uint8_t* checksum_out)
{
    memory_ble_metadata_t metadata = {0};
    memory_get_ble_metadata(&metadata);
    util_log("ble active index: %d", metadata.active_index);
    if (metadata.active_index != 0 && metadata.active_index != 1) {
        return false;
    }
    size_t size = metadata.firmware_sizes[metadata.active_index];
    if (size == 0 || size > MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE) {
        return false;
    }
    if (firmware_out != NULL) {
        uint32_t ble_addr = metadata.active_index == 0 ? MEMORY_SPI_BLE_FIRMWARE_1_ADDR
                                                       : MEMORY_SPI_BLE_FIRMWARE_2_ADDR;
        *firmware_out = spi_mem_read(ble_addr, size);
        if (!*firmware_out) {
            return false;
        }
        uint8_t fw_hash[32] = {0};
        rust_sha256(*firmware_out, size, fw_hash);
        if (!MEMEQ(fw_hash, metadata.allowed_firmware_hash, 32)) {
            free(*firmware_out);
            *firmware_out = NULL;
            return false;
        }
    }
    *size_out = size;
    *checksum_out = metadata.firmware_checksums[metadata.active_index];
    return true;
}
