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

#if !defined(PRODUCT_BITBOX_BASE)
#define PRODUCT_BITBOX_BASE 0
#elif PRODUCT_BITBOX_BASE != 1
#error "invalid product value"
#endif

#if !defined(PRODUCT_BITBOX02_FACTORYSETUP)
#define PRODUCT_BITBOX02_FACTORYSETUP 0
#elif PRODUCT_BITBOX02_FACTORYSETUP != 1
#error "invald product value"
#endif

#if !defined(PRODUCT_BITBOXBASE_FACTORYSETUP)
#define PRODUCT_BITBOXBASE_FACTORYSETUP 0
#elif PRODUCT_BITBOXBASE_FACTORYSETUP != 1
#error "invald product value"
#endif

// Derive other useful definitions from the product.

#if PRODUCT_BITBOX_MULTI == 1
#define PLATFORM_BITBOX02 1
#define PLATFORM_BITBOXBASE 0
#define FACTORYSETUP 0
#endif

#if PRODUCT_BITBOX_BTCONLY == 1
#define PLATFORM_BITBOX02 1
#define PLATFORM_BITBOXBASE 0
#define FACTORYSETUP 0
#endif

#if PRODUCT_BITBOX_BASE == 1
#define PLATFORM_BITBOX02 0
#define PLATFORM_BITBOXBASE 1
#define FACTORYSETUP 0
#endif

#if PRODUCT_BITBOX02_FACTORYSETUP == 1
#define PLATFORM_BITBOX02 1
#define PLATFORM_BITBOXBASE 0
#define FACTORYSETUP 1
#endif

#if PRODUCT_BITBOXBASE_FACTORYSETUP == 1
#define PLATFORM_BITBOX02 0
#define PLATFORM_BITBOXBASE 1
#define FACTORYSETUP 1
#endif

#endif
