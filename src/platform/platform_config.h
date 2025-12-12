// SPDX-License-Identifier: Apache-2.0

#ifndef _PLATFORM_CONFIG_H
#define _PLATFORM_CONFIG_H

// Force the PRODUCT_ defines to be 0 or 1, so they can be used safely without risk of typos.

#if !defined(PRODUCT_BITBOX_MULTI)
    #define PRODUCT_BITBOX_MULTI 0
#elif PRODUCT_BITBOX_MULTI != 1
    #error "invalid product value"
#endif

#if !defined(PRODUCT_BITBOX_BTCONLY)
    #define PRODUCT_BITBOX_BTCONLY 0
#elif PRODUCT_BITBOX_BTCONLY != 1
    #error "invalid product value"
#endif

#if !defined(PRODUCT_BITBOX_PLUS_MULTI)
    #define PRODUCT_BITBOX_PLUS_MULTI 0
#elif PRODUCT_BITBOX_PLUS_MULTI != 1
    #error "invalid product value"
#endif

#if !defined(PRODUCT_BITBOX_PLUS_BTCONLY)
    #define PRODUCT_BITBOX_PLUS_BTCONLY 0
#elif PRODUCT_BITBOX_PLUS_BTCONLY != 1
    #error "invalid product value"
#endif

#if !defined(PRODUCT_BITBOX02_FACTORYSETUP)
    #define PRODUCT_BITBOX02_FACTORYSETUP 0
#elif PRODUCT_BITBOX02_FACTORYSETUP != 1
    #error "invald product value"
#endif

// Derive other useful definitions from the product.

#if PRODUCT_BITBOX_MULTI == 1
    #define PLATFORM_BITBOX02PLUS 0
    #define PLATFORM_BITBOX02 1
    #define FACTORYSETUP 0
#endif

#if PRODUCT_BITBOX_BTCONLY == 1
    #define PLATFORM_BITBOX02PLUS 0
    #define PLATFORM_BITBOX02 1
    #define FACTORYSETUP 0
#endif

#if PRODUCT_BITBOX_PLUS_MULTI == 1
    #define PLATFORM_BITBOX02PLUS 1
    #define PLATFORM_BITBOX02 0
    #define FACTORYSETUP 0
#endif

#if PRODUCT_BITBOX_PLUS_BTCONLY == 1
    #define PLATFORM_BITBOX02PLUS 1
    #define PLATFORM_BITBOX02 0
    #define FACTORYSETUP 0
#endif

#if PRODUCT_BITBOX02_FACTORYSETUP == 1
    #define PLATFORM_BITBOX02PLUS 0
    #define PLATFORM_BITBOX02 1
    #define FACTORYSETUP 1
#endif

#endif
