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

#include "common_main.h"
#include "driver_init.h"
#include "hid_hww.h"
#include "qtouch.h"
#include "random.h"
#include "screen.h"
#include "ui/oled/oled.h"
#include "ui/ugui/ugui.h"
#include "usb/usb_frame.h"
#include "usb_desc.h"
#include "utils.h"
#include <string.h>
#include <ui/fonts/arial_fonts.h>
#include <usb/usb.h>

// common test functions
#include "test_common.h"

uint32_t __stack_chk_guard = 0;

static struct test_usb_metadata hww_metadata;

int main(void)
{
    system_init();
    __stack_chk_guard = common_stack_chk_guard();

    UG_GUI guioled; // Global GUI structure (OLED)
    UG_Init(
        &guioled,
        (void (*)(UG_S16, UG_S16, UG_COLOR))oled_set_pixel,
        &font_font_a_9X9,
        SCREEN_WIDTH,
        SCREEN_HEIGHT);

    hww_metadata.usb_cb_out = test_hww_out_print2screen;

    test_usb_init(&hww_metadata, NULL);

    while (1) {
    }
}
