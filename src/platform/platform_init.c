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
#include "memory/spi_mem.h"
#include <driver_init.h>
#include <ui/oled/oled.h>
#if !defined(BOOTLOADER)
    #include "sd_mmc/sd_mmc_start.h"
#endif
#include "util.h"

#if !(defined(BOOTLOADER) && PLATFORM_BITBOX02 == 1)
    #include "uart.h"
#endif

#if defined(BOOTLOADER)
    #define PREFIX "boot"
#else
    #define PREFIX "fw"
#endif

void platform_init(void)
{
    oled_init();
#if !(defined(BOOTLOADER) && PLATFORM_BITBOX02 == 1)
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        uart_init();
    }
#endif
    // these two functions are noops if "rtt" feature isn't enabled in rust
    util_log_init();
    util_log(PREFIX ": platform_init");
#if !defined(BOOTLOADER)
    sd_mmc_start();
#endif
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        spi_mem_protected_area_lock();
    }
}
