// SPDX-License-Identifier: Apache-2.0

#ifndef _MEMORY_SPI_H_
#define _MEMORY_SPI_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include <compiler_util.h>

/**
 * Retrieve the BLE firmware and associated size and checksum from the SPI memory chip. It takes
 * into account the currently active firmware area, and verifies that it matches the
 * `ble_allowed_firmware_hash` stored in the MCU shared memory.
 * @param[out] firmware_out, if not NULL, will be set to a pointer to the heap of size `size_out`.
 *            The caller must take care to `free()` it. Can be NULL.
 * @param[out] size_out size of the loaded firmware.
 * @param[out] checksum_out the checksum of the loaded firmware.
 * @returns true on success, false on failure.
 */
USE_RESULT bool memory_spi_get_active_ble_firmware(
    uint8_t** firmware_out,
    size_t* size_out,
    uint8_t* checksum_out);

// This struct is always placed at 0x110 in the firmware
struct da14531_firmware_version {
    uint8_t metadata_version; // The version of the format of this struct.
    uint16_t version; // The version of the firmware
    uint8_t hash[20];
} __attribute__((packed));

USE_RESULT bool memory_spi_get_active_ble_firmware_version(
    struct da14531_firmware_version* version);

#endif
