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

#include "idle_workflow.h"

#include <stdlib.h>
#include <string.h>

#include <hww.h>
#include <platform_config.h>
#include <ui/components/info_centered.h>
#include <ui/components/waiting.h>
#include <ui/screen_stack.h>
#include <ui/ugui/ugui.h>

#if PLATFORM_BITBOXBASE == 1
#include <usart/usart.h>
#elif PLATFORM_BITBOX02 == 1
#include <usb/usb.h>
#endif
#include <util.h>

#ifndef TESTING
#include <hal_delay.h>
#endif

static void _init_communication(void)
{
#if PLATFORM_BITBOXBASE == 1
    usart_start();
    hww_setup();
#elif PLATFORM_BITBOX02 == 1
    usb_start(hww_setup);
#endif
    ui_screen_stack_push(info_centered_create("See the BitBoxApp", NULL));
}

void idle_workflow_blocking(void)
{
    component_t* waiting_screen = waiting_create();
    UG_ClearBuffer();
    waiting_screen->f->render(waiting_screen);
    UG_SendBuffer();
    waiting_screen->f->cleanup(waiting_screen);
#ifndef TESTING
    // Added deliberately as a UX/visual improvement, to show the BB02 logo first.
    delay_ms(1300);
#endif
    _init_communication();
}
