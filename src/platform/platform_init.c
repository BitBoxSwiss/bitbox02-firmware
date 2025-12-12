// SPDX-License-Identifier: Apache-2.0

#include "platform_init.h"
#include "memory/memory_shared.h"
#include "memory/spi_mem.h"
#include <driver_init.h>
#include <ui/oled/oled.h>
#if defined(BOOTLOADER)
    #include <bootloader_version.h>
#else
    #include "sd_mmc/sd_mmc_start.h"
#endif
#include "util.h"
#include <platform/platform_config.h>
#include <version.h>

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

#if !(defined(BOOTLOADER) && PLATFORM_BITBOX02PLUS == 0)
    #if defined(BOOTLOADER)
        #if PRODUCT_BITBOX_PLUS_MULTI == 1
            #define DEVICE_MODE "{\"p\":\"bb02p-bl-multi\",\"v\":\"" BOOTLOADER_VERSION "\"}"
        #elif PRODUCT_BITBOX_PLUS_BTCONLY == 1
            #define DEVICE_MODE "{\"p\":\"bb02p-bl-btconly\",\"v\":\"" BOOTLOADER_VERSION "\"}"
        #else
            #error "unknown product"
        #endif
    #else
        // Currently we have one firmware for both BB02 and BB02_PLUS, and only the
        // PRODUCT_BITBOX_MULTI/BTCONLY definitions apply. The PRODUCT_BITBOX_PLUS_MULTI/BTCONLY
        // defs currently only apply in the bootloader, which we don't need here.
        #if PRODUCT_BITBOX_MULTI == 1
            #define PRODUCT_STRING_SUFFIX "multi"
        #elif PRODUCT_BITBOX_BTCONLY == 1
            #define PRODUCT_STRING_SUFFIX "btconly"
        #elif PRODUCT_BITBOX02_FACTORYSETUP == 1
            // Dummy, not actually needed, but this file is currently needlessly compiled for
            // factorysetup.
            #define PRODUCT_STRING_SUFFIX "factory"
        #else
            #error "unknown edition"
        #endif
        #define DEVICE_MODE \
            "{\"p\":\"bb02p-" PRODUCT_STRING_SUFFIX "\",\"v\":\"" DIGITAL_BITBOX_VERSION "\"}"
    #endif

const char* platform_product(size_t* len)
{
    *len = sizeof(DEVICE_MODE) - 1;
    return DEVICE_MODE;
}
#endif
