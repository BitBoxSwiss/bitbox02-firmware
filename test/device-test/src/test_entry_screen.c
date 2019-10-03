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

#include <stdbool.h>
#include <stdint.h>
#include <string.h>

#include <driver_init.h>
#include <firmware_main_loop.h>
#include <screen.h>
#include <touch/gestures.h>
#include <ui/components/entry_screen.h>
#include <ui/event.h>
#include <ui/event_handler.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/screen_stack.h>
#include <ui/ui_util.h>

#include "hardfault.h"
#include "qtouch.h"
#include "screen.h"
#include "util.h"

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-function"

typedef struct {
    // bool top_slider_is_touched;
    void (*done_callback)(void);
} entry_screen_data;

static void _render(component_t* component)
{
    /*
    if (data->top_slider_is_touched) {
        // Arrow
        x = SCREEN_WIDTH / 2 - 2;
        y = SCREEN_HEIGHT - 12;
        UG_DrawLine(x, y, x, y - 8, screen_front_color);
        UG_DrawLine(x, y, x - 2, y - 2, screen_front_color);
        UG_DrawLine(x, y, x + 2, y - 2, screen_front_color);
        // Slide marker
        y = 0;
        x = position_top * (SCREEN_WIDTH - 1) / MAX_SLIDER_POS;
        UG_DrawLine(MIN(SCREEN_WIDTH, x + 4), y, MAX(0, x - 4), y, screen_front_color);
    }
    */

    // Render sub-components
    uint8_t i;
    for (i = 0; i < component->sub_components.amount; i++) {
        component->sub_components.sub_components[i]->f->render(
            component->sub_components.sub_components[i]);
    }
}

static void _on_event(const event_t* event, component_t* component)
{
    entry_screen_data* data = (entry_screen_data*)component->data;
    switch (event->id) {
    case EVENT_BOTTOM_CONTINUOUS_TAP:
    case EVENT_BOTTOM_SLIDE:
        data->done_callback();
        break;
    default:
        break;
    }
}

static const component_functions_t component_functions = {.cleanup = ui_util_component_cleanup,
                                                          .render = _render,
                                                          .on_event = _on_event};

/********************************** Create Instance **********************************/

/**
 * Creates an entry screen.
 * @param[in] done_callback The callback that is called when the user touches to enter.
 */
// static component_t* entry_screen_create(const char* text, void (*done_callback)(void))
//{
//    component_t* entry_screen = malloc(sizeof(component_t));
//    if (!entry_screen) Abort("Error: malloc entry_screen");
//    entry_screen_data* data = malloc(sizeof(entry_screen_data));
//    if (!data) Abort("Error: malloc entry_screen data");
//    memset(entry_screen, 0, sizeof(component_t));
//    memset(data, 0, sizeof(entry_screen_data));
//
//    data->done_callback = done_callback;
//
//    entry_screen->data = data;
//    entry_screen->parent = NULL;
//    entry_screen->f = &component_functions;
//    entry_screen->dimension.width = SCREEN_WIDTH;
//    entry_screen->dimension.height = SCREEN_HEIGHT;
//    entry_screen->position.top = 0;
//    entry_screen->position.left = 0;
//
//    ui_util_add_sub_component(entry_screen, knight_rider_create(entry_screen));
//    ui_util_add_sub_component(entry_screen, label_create_big(text, CENTER, 0, entry_screen));
//
//    return entry_screen;
//}

//////////////////////////////////////////////////////////////////////////////////////////////////////////

static void done_callback(void)
{
    UG_ClearBuffer();
    UG_FontSelect(&font_font_a_9X9);
    UG_PutString(8, 26, "done_callback\nrestarting", false);
    UG_SendBuffer();
    delay_ms(500);
    _reset_mcu();
}

int main(void)
{
    system_init();
    screen_init();
    qtouch_init();
    ui_screen_stack_push(entry_screen_create("Enter password", done_callback));
    firmware_main_loop();
}

#pragma GCC diagnostic pop
