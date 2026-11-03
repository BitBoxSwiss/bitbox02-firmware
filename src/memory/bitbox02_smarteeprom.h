// SPDX-License-Identifier: Apache-2.0

#ifndef _BITBOX02_SMARTEEPROM_H
#define _BITBOX02_SMARTEEPROM_H

#include "compiler_util.h"

#include <stdbool.h>
#include <stdint.h>

typedef enum {
    BITBOX02_SMARTEEPROM_LANGUAGE_EN = 0,
    BITBOX02_SMARTEEPROM_LANGUAGE_DE = 1,
    BITBOX02_SMARTEEPROM_LANGUAGE_IT = 2,
    BITBOX02_SMARTEEPROM_LANGUAGE_ES = 3,
} bitbox02_smarteeprom_language_t;

USE_RESULT bool bitbox02_smarteeprom_set_device_language(bitbox02_smarteeprom_language_t language);
bitbox02_smarteeprom_language_t bitbox02_smarteeprom_get_device_language(void);

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
