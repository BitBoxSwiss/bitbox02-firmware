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

#ifndef _BITBOX02_SMARTEEPROM_H
#define _BITBOX02_SMARTEEPROM_H

#include <stdint.h>

/**
 * After this many failed unlock attempts, the keystore becomes locked until a
 * device reset.
 */
#define MAX_UNLOCK_ATTEMPTS (10)

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
