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
#include <platform/driver_init.h>
#include <touch/gestures.h>
#include <ui/canvas.h>
#include <ui/components/waiting.h>
#include <ui/oled/oled.h>
#include <ui/screen_process.h>
#include <ui/screen_saver.h>
#include <ui/ugui/ugui.h>

volatile bool update_ui = true;

#if !defined(TESTING)
static struct timer_task task_update_ui;

static void update_ui_cb(const struct timer_task* const timer_task)
{
    (void)timer_task;
    update_ui = true;
}
#endif

void screen_process_init(void)
{
#if !defined(TESTING)
    // Limit UI updateing to 100Hz
    task_update_ui.interval = 10;
    task_update_ui.cb = update_ui_cb;
    task_update_ui.mode = TIMER_TASK_REPEAT;

    timer_add_task(&TIMER_0, &task_update_ui);
#endif
}

void ui_screen_render_component(component_t* component)
{
    component->position.left = 0;
    component->position.top = 0;
    component->f->render(component);
    canvas_commit();
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

void screen_process_waiting_switch_to_logo(void)
{
    waiting_switch_to_logo(_get_waiting_screen());
}

void screen_process_waiting_switch_to_lockscreen(void)
{
    waiting_switch_to_lockscreen(_get_waiting_screen());
}

component_t* screen_process_get_top_component(void)
{
    component_t* saver = screen_saver_get();
    if (saver != NULL) {
        return saver;
    }

    component_t* result = ui_screen_stack_top();
    if (!result) {
        return _get_waiting_screen();
    }
    return result;
}

#ifndef TESTING
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
#endif

void screen_process(void)
{
    component_t* component = screen_process_get_top_component();

    if (update_ui) {
        update_ui = false;
        screen_saver_process();

        ui_screen_render_component(component);
    }

#ifndef TESTING
    /*
     * If we have changed activity, the gestures
     * detection must start over.
     */
    bool screen_new = _screen_has_changed(component);
    gestures_detect(screen_new, component->emit_without_release);
#endif

    ui_screen_stack_cleanup();
}
