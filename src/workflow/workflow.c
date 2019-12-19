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

#include <string.h>

#include "attestation.h"
#include "password.h"
#include "platform_config.h"
#include "unlock.h"
#include "workflow.h"

#include <hardfault.h>
#include <hww.h>
#include <platform_config.h>
#include <screen.h>
#include <sd.h>
#include <ui/components/confirm.h>
#include <ui/components/info_centered.h>
#include <ui/components/orientation_arrows.h>
#include <ui/components/show_logo.h>
#include <ui/components/waiting.h>
#include <ui/screen_stack.h>
#if PLATFORM_BITBOXBASE == 1
#include <usart/usart.h>
#elif PLATFORM_BITBOX02 == 1
#include <usb/usb.h>
#endif
#include <util.h>

static void _confirm_dismiss(component_t* component)
{
    (void)component;
    ui_screen_stack_switch(waiting_create());
}

void workflow_confirm_dismiss(const char* title, const char* body)
{
    ui_screen_stack_switch(confirm_create(title, body, NULL, false, _confirm_dismiss, NULL));
}

void workflow_start(void)
{
#if PLATFORM_BITBOXBASE == 1
    usart_start();
    hww_setup();
#elif PLATFORM_BITBOX02 == 1
    usb_start(hww_setup);
#endif
    ui_screen_stack_pop_all();
    ui_screen_stack_push(info_centered_create("See the BitBoxApp", NULL));
}

#if PLATFORM_BITBOX02 == 1
/**
 * Called when the "select orientation" screen is over.
 * Switch to the main view.
 */
static void _select_orientation_done(bool upside_down)
{
    if (upside_down) {
        screen_rotate();
    }
    component_t* show_logo = show_logo_create(workflow_start, 200);
    ui_screen_stack_switch(show_logo);
}
#endif

void workflow_start_orientation_screen(void)
{
#if PLATFORM_BITBOXBASE == 1
    component_t* show_logo = show_logo_create(workflow_start, 200);
    ui_screen_stack_switch(show_logo);
#elif PLATFORM_BITBOX02 == 1
    component_t* select_orientation = orientation_arrows_create(_select_orientation_done);
    ui_screen_stack_switch(select_orientation);
#endif
}
