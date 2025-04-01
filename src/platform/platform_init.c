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
#include "rust/rust.h"

void platform_init(void)
{
    oled_init();
    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        uart_init();
    }
#if !defined(BOOTLOADER)
    // these two functions are noops if "rtt" feature isn't enabled in rust
    rust_rtt_init();
    util_log("platform_init");
    sd_mmc_start();
#endif
}
