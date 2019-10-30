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
#include <driver_init.h>
#include <ui/oled/oled.h>
#if !defined(BOOTLOADER)
#include "sd_mmc/sd_mmc_start.h"
#endif
#if PLATFORM_BITBOXBASE == 1
#include "leds.h"
#endif

extern void initialise_monitor_handles(void);

void platform_init(void)
{
#if defined(SEMIHOSTING)
    initialise_monitor_handles();
#endif
    oled_init();
#if !defined(BOOTLOADER) && PLATFORM_BITBOX02 == 1
    sd_mmc_start();
#endif
#if PLATFORM_BITBOXBASE == 1
    leds_init();
#endif
}
