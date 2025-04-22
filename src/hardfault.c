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
#include "system.h"
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
#define ABORT_COUNTDOWN 30
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
    // Break the program if we are debugging
    ASSERT(false);
    char buf[30] = {0};
    for (int i = 0; i < ABORT_COUNTDOWN; i++) {
        snprintf(buf, sizeof(buf), "Enter bootloader in %02d", ABORT_COUNTDOWN - i);
        UG_PutStringCentered(0, 56, 128, 8, buf, false);
        UG_SendBuffer();
        delay_ms(1000);
    }
    // Restart into bootloader
    auto_enter = sectrue_u32;
    _reset_mcu();
    while (1);
#endif
}
