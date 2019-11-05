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

#if defined(PRODUCT_BITBOX_MULTI)
#define PLATFORM_BITBOX02 1
#define PLATFORM_BITBOXBASE 0
#endif

#if defined(PRODUCT_BITBOX_BTCONLY)
#define PLATFORM_BITBOX02 1
#define PLATFORM_BITBOXBASE 0
#endif

#if defined(PRODUCT_BITBOX_BASE)
#define PLATFORM_BITBOX02 0
#define PLATFORM_BITBOXBASE 1
#endif

// TODO(nc): should be two factory setups, one for bb02 and one for bbb
#if defined(FACTORYSETUP)
#define PLATFORM_BITBOX02 1
#define PLATFORM_BITBOXBASE 0
#endif

#endif
