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

#include "platform_init.h"
#include "memory/memory_shared.h"
#include "uart.h"
#include <driver_init.h>
#include <ui/oled/oled.h>
#if !defined(BOOTLOADER)
#include "sd_mmc/sd_mmc_start.h"
#endif
#include "util.h"

#if defined(BOOTLOADER)
#define PREFIX "boot"
#else
#define PREFIX "fw"
#endif

void platform_init(void)
{
    oled_init();
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        uart_init();
    }
// The factory setup image already has a c implementation of RTT.
#if FACTORYSETUP != 1
    // these two functions are noops if "rtt" feature isn't enabled in rust
    util_log_init();
    util_log(PREFIX ": platform_init");
#endif
#if !defined(BOOTLOADER)
    sd_mmc_start();
#endif
}
