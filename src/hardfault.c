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

#include "hardfault.h"
#include "util.h"
#include "utils_assert.h"
#include <driver_init.h>
#include <memory/memory.h>
#include <platform_config.h>
#include <screen.h>
#include <usb/usb.h>

#if defined(TESTING)
#include <stdio.h>
#include <stdlib.h>
#endif

#ifndef TESTING
void HardFault_Handler(void)
{
    Abort("Unexpected error.\nPlease contact support.");
}

void MemManage_Handler(void)
{
    Abort("Memory Fault");
}
#endif

void Abort(const char* msg)
{
#if defined(TESTING)
    fprintf(stderr, "%s\n", msg);
    exit(1);
#else
    util_log("%s", msg);
    screen_print_debug(msg, 0);
    usb_stop();
#if defined(BOOTLOADER)
    bootloader_close_interfaces();
#else
    system_close_interfaces();
#endif
    // Break the program if we are debugging
    ASSERT(false);
    while (1) {
    }
#endif
}

void AbortAutoenter(const char* msg)
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
    Abort(msg);
}
