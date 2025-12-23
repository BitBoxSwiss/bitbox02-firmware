// SPDX-License-Identifier: Apache-2.0

#ifndef _RANDOM_H_
#define _RANDOM_H_

#include <stdint.h>
#include <stdlib.h>

#define RANDOM_NUM_SIZE ((uint8_t)32)

// random_32_bytes_mcu generates 32 random bytes using the mcu trng and xors it into buf.
void random_32_bytes_mcu(uint8_t* buf);
// random_32_bytes generates 32 random bytes (a combination of mcu trng and secure chip trng).
void random_32_bytes(uint8_t* buf);

/**
 * Return single random byte.
 */
uint8_t random_byte_mcu(void);

#ifdef TESTING
// In testing, `rand()` is used for mocking. This function resets the seed using `srand(0)`, which
// allows individual unit tests to be independent of others.
void random_fake_reset(void);
#endif

#endif
