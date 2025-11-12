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

#ifndef _PLATFORM_INIT_H_
#define _PLATFORM_INIT_H_
#include <platform/platform_config.h>
#include <stddef.h>
void platform_init(void);

#if !(defined(BOOTLOADER) && PLATFORM_BITBOX02PLUS == 0)
// Returns a json string representing the firmware type and version
const char* platform_product(size_t* len);
#endif
#endif
