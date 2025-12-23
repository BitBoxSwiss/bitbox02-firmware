// SPDX-License-Identifier: Apache-2.0

#include <stdio.h>
#include <string.h>
#ifndef TESTING
    #include "driver_init.h"
    #include "flags.h"
    #include "securechip/securechip.h"
    #include <hal_rand_sync.h>
#endif
#include "hardfault.h"
#include "random.h"
#include "util.h"
#include <rust/rust.h>

void random_32_bytes_mcu(uint8_t* buf)
{
    if (buf == NULL) {
        Abort("Abort: random_32_bytes_mcu");
    }
    uint8_t random[RANDOM_NUM_SIZE] = {0};
    UTIL_CLEANUP_32(random);
#ifdef TESTING
    // Use standard libary for off chip RNG
    for (size_t i = 0; i < sizeof(random); i++) {
        random[i] = rand();
    }
#else
    rand_sync_read_buf8(&RAND_0, random, sizeof(random));
#endif
    for (size_t i = 0; i < sizeof(random); i++) {
        buf[i] ^= random[i];
    }
}

uint8_t random_byte_mcu(void)
{
#ifdef TESTING
    // Use standard libary for off chip RNG
    return rand();
#else
    uint8_t random;
    rand_sync_read_buf8(&RAND_0, &random, sizeof(random));
    return random;
#endif
}

// random_32_bytes_sec xors random bytes, generated using the secure chip trng,
// into buf.
static void random_32_bytes_sec(uint8_t* buf)
{
    if (buf == NULL) {
        Abort("Abort: random_32_bytes_sec");
    }
    uint8_t random[RANDOM_NUM_SIZE] = {0};
    UTIL_CLEANUP_32(random);
#ifdef TESTING
    // Use standard libary for off chip RNG
    for (size_t i = 0; i < sizeof(random); i++) {
        random[i] = rand();
    }
#else
    if (!securechip_random(random)) {
        Abort("Abort: securechip_random");
    }
#endif
    for (size_t i = 0; i < sizeof(random); i++) {
        buf[i] ^= random[i];
    }
}

void random_32_bytes(uint8_t* buf)
{
    if (buf == NULL) {
        Abort("Abort: random_32_bytes");
    }
    uint8_t random[RANDOM_NUM_SIZE] = {0};
    UTIL_CLEANUP_32(random);
    random_32_bytes_mcu(random);
    random_32_bytes_sec(random);

#ifndef TESTING
    { // mix in factory randomness
        const uint8_t* factory_randomness = (uint8_t*)(FLASH_BOOT_START + FLASH_BOOT_LEN - 32);
        for (uint32_t i = 0; i < RANDOM_NUM_SIZE; i++) {
            random[i] ^= factory_randomness[i];
        }
    }
#endif

    rust_sha256(random, sizeof(random), buf);
}

#ifdef TESTING
void random_fake_reset(void)
{
    srand(0);
}
#endif
