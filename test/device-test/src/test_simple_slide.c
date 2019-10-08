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

#include "qtouch.h"
#include "random.h"
#include "util.h"
#include <driver_init.h>
#include <firmware_main_loop.h>
#include <screen.h>
#include <string.h>
#include <ui/components/label.h>
#include <ui/event.h>
#include <ui/screen_stack.h>
#include <ui/ui_util.h>
#include <usb/usb.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-function"

uint32_t __stack_chk_guard = 0;

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    screen_print_debug("Stack smashing detected", 0);
    while (1) {
    }
}

/********************************** Component Functions **********************************/

/**
 * Test slide data.
 */
typedef struct {
    component_t* label;
} test_slide_data_t;

static void test_slide_on_event(const event_t* event, component_t* component)
{
    // test_slide_data_t* data = (test_slide_data_t*)component->data;
    // gestures_slider_data_t* event_data = (gestures_slider_data_t*)event->data;
    // if (event_data->direction == FORWARD) {
    //    label_update(data->label, "slide forward");
    //} else {
    //    label_update(data->label, "slide backward");
    //}
}

/**
 * Collects all component functions.
 */
static const component_functions_t TEST_SLIDE_FUNCTIONS = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = test_slide_on_event};

/********************************** Create Instance **********************************/

static component_t* test_slide_create(void)
{
    test_slide_data_t* data = malloc(sizeof(test_slide_data_t));
    memset(data, 0, sizeof(test_slide_data_t));

    component_t* test_slide = malloc(sizeof(component_t));
    memset(test_slide, 0, sizeof(component_t));

    test_slide->f = &TEST_SLIDE_FUNCTIONS;
    test_slide->data = data;

    test_slide->dimension.width = SCREEN_WIDTH;
    test_slide->dimension.height = SCREEN_HEIGHT;

    component_t* label = label_create("Slide top or bottom", NULL, CENTER, test_slide);
    data->label = label;
    ui_util_add_sub_component(test_slide, label);

    // ui_util_add_sub_component(test_slide, slider_bar_create(top_slider, test_slide));
    // ui_util_add_sub_component(test_slide, slider_bar_create(bottom_slider, test_slide));

    return test_slide;
}

int main(void)
{
    system_init();
    screen_init();
    qtouch_init();

    component_t* test_slide_screen = test_slide_create();
    ui_screen_stack_push(test_slide_screen);
    firmware_main_loop();
}

#pragma GCC diagnostic pop
