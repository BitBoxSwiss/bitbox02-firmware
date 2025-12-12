// SPDX-License-Identifier: Apache-2.0

#include "memory_spi.h"
#include "memory_shared.h"
#include "spi_mem.h"

#include <hardfault.h>
#include <rust/rust.h>
#include <util.h>
#include <utils_assert.h>

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
    if (size == 0 || size > (size_t)MEMORY_SPI_BLE_FIRMWARE_MAX_SIZE) {
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
            // TODO: Register this error with some component that can report the error to the end
            // user.
            return false;
        }
    }
    *size_out = size;
    *checksum_out = metadata.firmware_checksums[metadata.active_index];
    return true;
}

USE_RESULT bool memory_spi_get_active_ble_firmware_version(struct da14531_firmware_version* version)
{
    memset(version, 0, sizeof(struct da14531_firmware_version));

    memory_ble_metadata_t metadata = {0};
    memory_get_ble_metadata(&metadata);
    util_log("ble active index: %d", metadata.active_index);
    if (metadata.active_index != 0 && metadata.active_index != 1) {
        return false;
    }

    uint32_t ble_addr = metadata.active_index == 0 ? MEMORY_SPI_BLE_FIRMWARE_1_ADDR
                                                   : MEMORY_SPI_BLE_FIRMWARE_2_ADDR;
    uint8_t* firmware_bytes =
        spi_mem_read(ble_addr + 0x110, sizeof(struct da14531_firmware_version));
    ASSERT(firmware_bytes);
    if (!firmware_bytes) {
        return false;
    }

    struct da14531_firmware_version* firmware = (struct da14531_firmware_version*)firmware_bytes;
    if (firmware->metadata_version == 1) {
        memcpy((uint8_t*)version, firmware_bytes, sizeof(struct da14531_firmware_version));
    } else {
        util_log("Invalid metadata version! No firmware?");
    }

    free(firmware_bytes);

    return true;
}
