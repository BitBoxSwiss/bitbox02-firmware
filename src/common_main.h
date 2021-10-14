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

#ifndef _COMMON_MAIN_H_
#define _COMMON_MAIN_H_

#include <stdint.h>

uint32_t common_stack_chk_guard(void);

/**
 * Go into bootloader on next reboot. This should be called before any Abort during boot, so a
 * firmware update can be applied. Otherwise, if there is an Abort() during startup, there would no
 * way to reboot into the bootloader and the device would be bricked.
 */
void common_main_bootloader_autoenter(void);

/**
 * This performs common setup at boot of the firmware/factorysetup.
 */
void common_main(void);

#endif
