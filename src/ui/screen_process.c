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

/**
 * Detects if the screen component being displayed has changed
 * since the last time this function was called.
 * This stores the last observed component into a global.
 *
 * @param[in] current_component Current on-screen component.
 */
static bool _screen_has_changed(const component_t* current_component)
{
    static const component_t* last_observed_comp = NULL;
    if (last_observed_comp != current_component) {
        last_observed_comp = current_component;
        return true;
    }
    return false;
}

static component_t* _get_waiting_screen(void)
{
    static component_t* waiting_screen = NULL;
    if (waiting_screen == NULL) {
        waiting_screen = waiting_create();
        if (waiting_screen == NULL) {
            Abort("Could not create\nwaiting screen");
        }
    }
    return waiting_screen;
}

static void _screen_process(
    bool (*is_done)(void*),
    void* is_done_param,
    void (*on_timeout)(void),
    const uint32_t timeout)
{
    component_t* waiting_screen = _get_waiting_screen();
    uint32_t timeout_cnt = 0;

    component_t* component = NULL;

    if (is_done == NULL) {
        Abort("is_done function\nis NULL.");
    }

    while (!is_done(is_done_param)) {
        /*
         * Pick which activity we should draw next
         * (or fallback to the idle screen).
         * If we have changed activity, the gestures
         * detection must start over.
         */
        component = ui_screen_stack_top();
        if (component == NULL) {
            component = waiting_screen;
        }
        bool screen_new = _screen_has_changed(component);
        gestures_detect(screen_new, component->emit_without_release);
        if (screen_frame_cnt == SCREEN_FRAME_RATE) {
            if (on_timeout != NULL && timeout_cnt > timeout) {
                on_timeout();
            }
            screen_frame_cnt = 0;
            timeout_cnt += 1;
            ui_screen_render_component(component);
        }
        screen_frame_cnt++;
        ui_screen_stack_cleanup();
    }
}

void ui_screen_process(bool (*is_done)(void*), void* is_done_param)
{
    _screen_process(is_done, is_done_param, NULL, 0);
}

void ui_screen_process_with_timeout(
    bool (*is_done)(void*),
    void* is_done_param,
    void (*on_timeout)(void),
    uint32_t timeout)
{
    _screen_process(is_done, is_done_param, on_timeout, timeout);
}
