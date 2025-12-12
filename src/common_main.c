// SPDX-License-Identifier: Apache-2.0

#include "common_main.h"
#include "driver_init.h"
#include "flags.h"
#include "hardfault.h"
#include "memory/memory.h"
#include "memory/mpu.h"
#include "memory/smarteeprom.h"
#include "random.h"
#include "screen.h"
#include "securechip/securechip.h"
#include "util.h"

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    Abort("Stack smashing detected");
    while (1) {
    } // satisfy noreturn
}

uint32_t common_stack_chk_guard(void)
{
    return rand_sync_read32(&RAND_0);
}

static const memory_interface_functions_t _memory_interface_functions = {
    // Use random_32_bytes_mcu over random_32_bytes as the latter mixes in
    // randomness from the securechip, which is initialized only later.
    .random_32_bytes = random_32_bytes_mcu,
};

static const securechip_interface_functions_t _securechip_interface_functions = {
    .get_auth_key = memory_get_authorization_key,
    .get_io_protection_key = memory_get_io_protection_key,
    .get_encryption_key = memory_get_encryption_key,
    .random_32_bytes = random_32_bytes,
};

void common_main(void)
{
    mpu_bitbox02_init();
    if (!memory_setup(&_memory_interface_functions)) {
        // If memory setup failed, this also might fail, but can't hurt to try.
        AbortAutoenter("memory_setup failed");
    }

    /* Enable/configure SmartEEPROM. */
    smarteeprom_bb02_config();

    if (!securechip_init()) {
        AbortAutoenter("Failed to detect securechip");
    }
    // securechip_setup must come after memory_setup, so the io/auth keys to be
    // used are already initialized.
    int securechip_result = securechip_setup(&_securechip_interface_functions);
    if (securechip_result) {
        char errmsg[100] = {0};
        snprintf(
            errmsg,
            sizeof(errmsg),
            "Securechip setup failed.\nError code: %i\nPlease contact support.",
            securechip_result);
        AbortAutoenter(errmsg);
    }
}
