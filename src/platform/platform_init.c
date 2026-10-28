// SPDX-License-Identifier: Apache-2.0

#include "platform_init.h"
#include "memory/memory_shared.h"
#include "memory/spi_mem.h"
#include <driver_init.h>
#include <ui/oled/oled.h>
#if !defined(BOOTLOADER)
    #include "sd_mmc/sd_mmc_start.h"
#endif
#include "util.h"
#include <platform/platform_config.h>

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
