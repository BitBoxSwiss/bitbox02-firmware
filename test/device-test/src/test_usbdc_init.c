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
#include "memory.h"
#include "qtouch.h"
#include "random.h"
#include "screen.h"
#include "ssd1306.h"
#include "touch.h"
#include "ugui.h"
#include "usb.h"
#include "usb_desc.h"
#include "usbdc.h"
#include "utils.h"
#include <string.h>

uint32_t __stack_chk_guard = 0;

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    screen_print_debug("Stack smashing detected", 0);
    while (1) {
    }
}

// USB_REPORT_SIZE = 0x40
static uint8_t usb_ctrl_endpoint_buffer[USB_REPORT_SIZE];

int main(void)
{
    system_init();
    __stack_chk_guard = random_uint32();

    UG_GUI guioled; // Global GUI structure (OLED)
    UG_Init(&guioled, (void (*)(UG_S16, UG_S16, UG_COLOR))OLED_PSET, SCREEN_WIDTH, SCREEN_HEIGHT);
    UG_SelectGUI(&guioled);
    screen_print_debug("before test", 1000);

    int32_t init_error = usbdc_init(usb_ctrl_endpoint_buffer);

    if (init_error != ERR_NONE) {
        screen_print_debug("error", 1000);
        while (1) {
        }
    }
    screen_print_debug("after test", 1000);

    while (1) {
    }
}
