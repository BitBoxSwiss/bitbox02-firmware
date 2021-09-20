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

#include "common_main.h"
#include "driver_init.h"
#include "flags.h"
#include "hardfault.h"
#include "keystore.h"
#include "memory/memory.h"
#include "memory/mpu.h"
#include "memory/smarteeprom.h"
#include "random.h"
#include "screen.h"
#include "securechip/securechip.h"
#include "util.h"
#include <wally_core.h>

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

static void _wally_patched_bzero(void* ptr, size_t len)
{
    util_zero(ptr, len);
}

static bool _setup_wally(void)
{
    static struct wally_operations _ops = {0};
    if (wally_get_operations(&_ops) != WALLY_OK) {
        return false;
    }
    _ops.bzero_fn = _wally_patched_bzero;
    return wally_set_operations(&_ops) == WALLY_OK;
}

// Go into bootloader if there was an error during startup, so a firmware update can be
// applied. Otherwise, if there is an Abort() during startup, there would no way to reboot into the
// bootloader and the device would be bricked.
static void _bootloader_autoenter(void)
{
    auto_enter_t auto_enter = {
        .value = sectrue_u8,
    };
    upside_down_t upside_down = {
        .value = screen_is_upside_down(),
    };
    if (!memory_bootloader_set_flags(auto_enter, upside_down)) {
        // If this failed, we might not be able to reboot into the bootloader.
    }
}

void common_main(void)
{
    mpu_bitbox02_init();
    if (!memory_setup(&_memory_interface_functions)) {
        // If memory setup failed, this also might fail, but can't hurt to try.
        _bootloader_autoenter();
        Abort("memory_setup failed");
    }

    if (!_setup_wally()) {
        _bootloader_autoenter();
        Abort("_setup_wally failed");
    }

    /* Enable/configure SmartEEPROM. */
    smarteeprom_bb02_config();

    // securechip_setup must come after memory_setup, so the io/auth keys to be
    // used are already initialized.
    int securechip_result = securechip_setup(&_securechip_interface_functions);
    if (securechip_result) {
        _bootloader_autoenter();
        char errmsg[100] = {0};
        snprintf(
            errmsg,
            sizeof(errmsg),
            "Securechip setup failed.\nError code: %i\nPlease contact support.",
            securechip_result);
        Abort(errmsg);
    }
}
