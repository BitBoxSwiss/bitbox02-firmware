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

#include "screen_process.h"
#include "screen_stack.h"
#include <hardfault.h>
#include <touch/gestures.h>
#include <ui/components/waiting.h>
#include <ui/screen_process.h>
#include <ui/ugui/ugui.h>
#include <usb/usb_processing.h>

#define SCREEN_FRAME_RATE 30

static uint8_t screen_frame_cnt = 0;

void ui_screen_render_component(component_t* component)
{
    UG_ClearBuffer();
    component->position.left = 0;
    component->position.top = 0;
    component->f->render(component);
    UG_SendBuffer();
}

void ui_screen_process(bool (*is_done)(void))
{
    static component_t* waiting_screen = NULL;
    if (waiting_screen == NULL) {
        waiting_screen = waiting_create();
        if (waiting_screen == NULL) {
            Abort("could not create\nwaiting screen");
        }
    }

    bool screen_new = false;
    component_t* component = NULL;
    while (is_done == NULL || !is_done()) {
        component_t* next_component = ui_screen_stack_top();
        if (next_component == NULL) {
            next_component = waiting_screen;
        }
        screen_new = false;
        if (next_component != component) {
            screen_new = true;
            component = next_component;
        }
        gestures_detect(screen_new, component->emit_without_release);
        if ((screen_frame_cnt % SCREEN_FRAME_RATE) == 0) {
            screen_frame_cnt = 0;
            ui_screen_render_component(component);
        }
        screen_frame_cnt++;
        ui_screen_stack_cleanup();
        if (is_done == NULL) {
            usb_processing_process();
        }
    }
}
