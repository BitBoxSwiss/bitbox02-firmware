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

#include "driver_init.h"
#include "hid_hww.h"
#include "hid_u2f.h"
#include "memory.h"
#include "qtouch.h"
#include "random.h"
#include "screen.h"
#include "ssd1306.h"
#include "touch.h"
#include "ugui.h"
#include "usb.h"
#include "usb_desc.h"
#include "utils.h"
#include <string.h>

// common test functions
#include "test_common.h"

uint32_t __stack_chk_guard = 0;

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    screen_print_debug("Stack smashing detected", 0);
    while (1) {
    }
}

static struct test_usb_metadata hww_metadata;

static struct test_usb_metadata u2f_metadata;

int main(void)
{
    system_init();
    __stack_chk_guard = random_uint32();

    UG_GUI guioled; // Global GUI structure (OLED)
    UG_Init(&guioled, (void (*)(UG_S16, UG_S16, UG_COLOR))OLED_PSET, SCREEN_WIDTH, SCREEN_HEIGHT);
    UG_SelectGUI(&guioled);

    hww_metadata.usb_cb_out = test_hww_out_print2screen;
    u2f_metadata.usb_cb_out = test_u2f_out_print2screen;

    test_usb_init(&hww_metadata, &u2f_metadata);

    while (1) {
    }
}
