// SPDX-License-Identifier: Apache-2.0

#ifndef _BITBOX02_SMARTEEPROM_H
#define _BITBOX02_SMARTEEPROM_H

#include "compiler_util.h"

#include <stdbool.h>
#include <stdint.h>

typedef enum {
    BITBOX02_SMARTEEPROM_LANGUAGE_EN = 0,
    BITBOX02_SMARTEEPROM_LANGUAGE_DE = 1,
} bitbox02_smarteeprom_language_t;

USE_RESULT bool bitbox02_smarteeprom_set_device_language(bitbox02_smarteeprom_language_t language);
bitbox02_smarteeprom_language_t bitbox02_smarteeprom_get_device_language(void);

USE_RESULT bool bitbox02_smarteeprom_set_mnemonic_passphrase_enabled(bool enabled);
bool bitbox02_smarteeprom_is_mnemonic_passphrase_enabled(void);

// Both buffers must contain MEMORY_DEVICE_MAX_LEN_WITH_NULL bytes, including any padding.
USE_RESULT bool bitbox02_smarteeprom_set_device_name(const uint8_t* name);
void bitbox02_smarteeprom_get_device_name(uint8_t* name_out);

// Clear the settings also cleared by memory_reset_hww(), once their migration has completed.
void bitbox02_smarteeprom_reset_device_settings(void);

/**
 * Reads and validates the last recorded number of unlock attempts.
 */
uint8_t bitbox02_smarteeprom_get_unlock_attempts(void);

/**
 * Increments the recorded number of unlock attempts.
 *
 * This will fail if the number of unlock attempts would
 * be increased past the allowed maximum.
 */
void bitbox02_smarteeprom_increment_unlock_attempts(void);

/**
 * Resets the recorded number of unlock attempts to 0.
 *
 * This will fail if the currently recorded number of unlock attempts
 * is invalid.
 */
void bitbox02_smarteeprom_reset_unlock_attempts(void);

/**
 * Makes sure that the contents of the SmartEEPROM in the BitBox02
 * are up-to-date with the latest struct definition.
 */
void bitbox02_smarteeprom_init(void);

#endif // _BITBOX02_SMARTEEPROM_H
