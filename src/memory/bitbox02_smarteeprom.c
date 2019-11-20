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

#include "bitbox02_smarteeprom.h"

#include "hardfault.h"
#include "memory/mpu.h"
#include "memory/smarteeprom.h"

#include <stddef.h>
#include <string.h>

/**
 * Current version of the data stored in SmartEEPROM.
 */
#define BITBOX02_SMARTEEPROM_DATA_VERSION (1)

#define BITBOX02_SMARTEEPROM_UNINITIALIZED_VERSION (0xFF)

/**
 * Contents of the SmartEEPROM, as stored in the BitBox02.
 * Version 1.
 *
 * Each field is stored in a separate SmartEEPROM page (32B) to help
 * maximizing the lifetime of the device.
 * The version is fixed at virtual address zero for every possible
 * future structure (see _get_data_version).
 *
 * See smarteeprom_setup() for a description of how the SmartEEPROM
 * is configured.
 */
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wpacked"
typedef struct __attribute__((__packed__)) {
    /** == 1 */
    uint8_t version;
    uint8_t padding[SMARTEEPROM_PAGE_SIZE - sizeof(uint8_t)];
    /** Number of unlock attempts since last successful unlock. */
    uint8_t unlock_attempts;
    uint8_t padding_2[SMARTEEPROM_PAGE_SIZE - sizeof(uint8_t)];
} bitbox02_smarteeprom_image_v1_t;
#pragma GCC diagnostic pop

/**
 * Reads the version field at address zero.
 *
 * The offset (in memory) of the version needs to stay out
 * of the data structure. Otherwise, we need to know the version
 * of the struct we're going to use in order to know the location
 * of the version field... Which loses the point of the field itself.
 */
static uint8_t _get_data_version(void)
{
    uint8_t result;
    smarteeprom_read(0, sizeof(result), &result);
    return result;
}

/**
 * Reads the version field at address zero.
 */
static void _set_data_version(uint8_t new_version)
{
    smarteeprom_write(0, sizeof(new_version), &new_version);
}

static void _init_v1(void)
{
    bitbox02_smarteeprom_image_v1_t new_image;
    smarteeprom_read(0, sizeof(new_image), (uint8_t*)&new_image);
    /*
     * Note that this forcefully resets the unlock counter
     * the first and only time the device is upgraded.
     * Not too bad...
     */
    new_image.unlock_attempts = 0;
    smarteeprom_write(0, sizeof(new_image), (uint8_t*)&new_image);
    _set_data_version(0x01);
}

void bitbox02_smarteeprom_init(void)
{
    uint8_t current_version = _get_data_version();
    if (current_version == BITBOX02_SMARTEEPROM_DATA_VERSION) {
        return;
    }
    /*
     * Migrate from old versions.
     * FUTURE: if the data structures are changed, add here the code
     * to migrate from each version.
     */
    if (current_version == BITBOX02_SMARTEEPROM_UNINITIALIZED_VERSION) {
        _init_v1();
    } else {
        /*
         * Incorrect version!
         * Something has gone terribly wrong.
         * Maybe reset the whole device?
         */
        Abort("Unrecognized SmartEEPROM version.");
    }
}

uint8_t bitbox02_smarteeprom_get_unlock_attempts(void)
{
    uint8_t result;
    smarteeprom_read(
        offsetof(bitbox02_smarteeprom_image_v1_t, unlock_attempts),
        sizeof(((bitbox02_smarteeprom_image_v1_t*)0)->unlock_attempts),
        &result);
    /*
     * Sanity-check the value.
     *
     * At no point this number should be allowed to go above MAX_UNLOCK_ATTEMPTS.
     */
    if (result > MAX_UNLOCK_ATTEMPTS) {
        Abort("#Unlock attempts increased past the maximum allowed value.");
    }
    return result;
}

void bitbox02_smarteeprom_increment_unlock_attempts(void)
{
    uint8_t unlock_attempts = bitbox02_smarteeprom_get_unlock_attempts();
    /*
     * The number of attempts can never grow past the maximum.
     * Catch an attempt to increment it past the maximum before we
     * do the actual increment.
     */
    if (unlock_attempts == MAX_UNLOCK_ATTEMPTS) {
        Abort("Tried to increment the number of unlocks past the maximum allowed value.");
    }
    unlock_attempts++;
    smarteeprom_write(
        offsetof(bitbox02_smarteeprom_image_v1_t, unlock_attempts),
        sizeof(unlock_attempts),
        &unlock_attempts);
}

void bitbox02_smarteeprom_reset_unlock_attempts(void)
{
    /* Sanity-check the current value */
    (void)bitbox02_smarteeprom_get_unlock_attempts();
    uint8_t w_unlock_attempts = 0;
    smarteeprom_write(
        offsetof(bitbox02_smarteeprom_image_v1_t, unlock_attempts),
        sizeof(w_unlock_attempts),
        &w_unlock_attempts);
}
