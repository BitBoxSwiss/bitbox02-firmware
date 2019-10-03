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
#include <ui/components/button.h>
#include <ui/components/label.h>
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
 * Test buttons data.
 */
typedef struct {
    component_t* label;
} test_buttons_data_t;

/**
 * Collects all component functions.
 */
static const component_functions_t TEST_BUTTONS_FUNCTIONS = {
    .cleanup = ui_util_component_cleanup,
    .render = ui_util_component_render_subcomponents,
    .on_event = ui_util_on_event_noop};

/********************************** Create Instance **********************************/

static void test_buttons_b1_callback(component_t* component)
{
    test_buttons_data_t* data = (test_buttons_data_t*)component->parent->data;
    label_update(data->label, "bottom button 1");
}

static void test_buttons_b2_callback(component_t* component)
{
    test_buttons_data_t* data = (test_buttons_data_t*)component->parent->data;
    label_update(data->label, "bottom button 2");
}

static void test_buttons_b3_callback(component_t* component)
{
    test_buttons_data_t* data = (test_buttons_data_t*)component->parent->data;
    label_update(data->label, "bottom button 3");
}

static void test_buttons_b4_callback(component_t* component)
{
    test_buttons_data_t* data = (test_buttons_data_t*)component->parent->data;
    label_update(data->label, "bottom button 4");
}

static void test_buttons_b5_callback(component_t* component)
{
    test_buttons_data_t* data = (test_buttons_data_t*)component->parent->data;
    label_update(data->label, "bottom button 5");
}

static void test_buttons_b6_callback(component_t* component)
{
    test_buttons_data_t* data = (test_buttons_data_t*)component->parent->data;
    label_update(data->label, "top button a");
}

static void test_buttons_b7_callback(component_t* component)
{
    test_buttons_data_t* data = (test_buttons_data_t*)component->parent->data;
    label_update(data->label, "top button b");
}

static void test_buttons_b8_callback(component_t* component)
{
    test_buttons_data_t* data = (test_buttons_data_t*)component->parent->data;
    label_update(data->label, "top button c");
}

static void test_buttons_b9_callback(component_t* component)
{
    test_buttons_data_t* data = (test_buttons_data_t*)component->parent->data;
    label_update(data->label, "top button d");
}

static void test_buttons_b10_callback(component_t* component)
{
    test_buttons_data_t* data = (test_buttons_data_t*)component->parent->data;
    label_update(data->label, "top button e");
}

static component_t* test_buttons_create(void)
{
    test_buttons_data_t* data = malloc(sizeof(test_buttons_data_t));
    memset(data, 0, sizeof(test_buttons_data_t));

    component_t* test_buttons = malloc(sizeof(component_t));
    memset(test_buttons, 0, sizeof(component_t));

    test_buttons->f = &TEST_BUTTONS_FUNCTIONS;
    test_buttons->data = data;

    test_buttons->dimension.width = SCREEN_WIDTH;
    test_buttons->dimension.height = SCREEN_HEIGHT;

    component_t* label = label_create("Press a button", NULL, CENTER, test_buttons);
    data->label = label;
    ui_util_add_sub_component(test_buttons, label);

    const uint8_t offset = 6;
    ui_util_add_sub_component(
        test_buttons, button_create("1", bottom_slider, 0, test_buttons_b1_callback, test_buttons));
    ui_util_add_sub_component(
        test_buttons,
        button_create(
            "2", bottom_slider, SCREEN_WIDTH / 4 + offset, test_buttons_b2_callback, test_buttons));
    ui_util_add_sub_component(
        test_buttons,
        button_create(
            "3", bottom_slider, SCREEN_WIDTH / 2, test_buttons_b3_callback, test_buttons));
    ui_util_add_sub_component(
        test_buttons,
        button_create(
            "4",
            bottom_slider,
            SCREEN_WIDTH / 2 + SCREEN_WIDTH / 4 - offset,
            test_buttons_b4_callback,
            test_buttons));
    ui_util_add_sub_component(
        test_buttons,
        button_create("5", bottom_slider, SCREEN_WIDTH, test_buttons_b5_callback, test_buttons));

    ui_util_add_sub_component(
        test_buttons, button_create("a", top_slider, 0, test_buttons_b6_callback, test_buttons));
    ui_util_add_sub_component(
        test_buttons,
        button_create(
            "b", top_slider, SCREEN_WIDTH / 4 + offset, test_buttons_b7_callback, test_buttons));
    ui_util_add_sub_component(
        test_buttons,
        button_create("c", top_slider, SCREEN_WIDTH / 2, test_buttons_b8_callback, test_buttons));
    ui_util_add_sub_component(
        test_buttons,
        button_create(
            "d",
            top_slider,
            SCREEN_WIDTH / 2 + SCREEN_WIDTH / 4 - offset,
            test_buttons_b9_callback,
            test_buttons));
    ui_util_add_sub_component(
        test_buttons,
        button_create("e", top_slider, SCREEN_WIDTH, test_buttons_b10_callback, test_buttons));

    return test_buttons;
}

int main(void)
{
    system_init();
    screen_init();
    qtouch_init();

    component_t* test_buttons_screen = test_buttons_create();
    ui_screen_stack_push(test_buttons_screen);
    firmware_main_loop();
}

#pragma GCC diagnostic pop
