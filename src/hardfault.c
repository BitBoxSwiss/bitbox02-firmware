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

#include "hardfault.h"
#include "util.h"
#include <platform_config.h>
#include <screen.h>
#include <usb/usb.h>
#ifndef TESTING
#include <driver_init.h>

void HardFault_Handler(void)
{
    Abort("Hard Fault");
}

void MemManage_Handler(void)
{
    Abort("Memory Fault");
}
#endif

void Abort(const char* msg)
{
    screen_print_debug(msg, 0);
    traceln("Aborted: %s", msg);
#if PLATFORM_BITBOX02 == 1
    usb_stop();
#endif
#if !defined(TESTING)
#if defined(BOOTLOADER)
    bootloader_close_interfaces();
#else
    system_close_interfaces();
#endif
#endif
    while (1) {
    }
}
