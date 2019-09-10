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

#include "random.h"
#include <driver_init.h>
#include <qtouch/qtouch.h>
#include <screen.h>
#include <string.h>
#include <usb/usb.h>

uintptr_t __stack_chk_guard = 0;

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    screen_print_debug("Stack smash detected", 0);
    usb_stop();
    while (1) {
    }
}

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wformat-overflow"
static void smash_stack(const char* input)
{
    char buf[10];
    sprintf(buf, "%s\n", input);
    screen_print_debug(input, 0);
}
#pragma GCC diagnostic pop

int main(void)
{
    system_init();
    uint8_t random[RANDOM_NUM_SIZE];
    random_32_bytes_mcu(random);
    __stack_chk_guard = ((uint32_t*)random)[0];

    screen_init();
    qtouch_init();

    smash_stack("This input is way too long: SSP failed");

    while (1) {
    }

    return 0;
}
