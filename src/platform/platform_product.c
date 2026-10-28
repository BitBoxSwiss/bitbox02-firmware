// SPDX-License-Identifier: Apache-2.0

#include "platform_product.h"

#if defined(BOOTLOADER)
    #include <bootloader_version.h>
#else
    #include <version.h>
#endif

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
