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

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <cmocka.h>

#include <touch/gestures.h>
#include <ui/components/confirm.h>
#include <ui/components/image.h>
#include <ui/components/info_centered.h>
#include <ui/components/keyboard_switch.h>
#include <ui/components/label.h>
#include <ui/components/left_arrow.h>
#include <ui/components/right_arrow.h>
#include <ui/components/status.h>
#include <ui/fonts/monogram_5X9.h>
#include <ui/ui_util.h>

#include "mock_component.h"
#include "mock_qtouch.h"

static void assert_ui_component_functions(component_t* component)
{
    assert_non_null(component->f->render);
    assert_non_null(component->f->cleanup);
}

static void test_ui_components_label(void** state)
{
    component_t* mock_component = mock_component_create();

    component_t* label = label_create("Test", NULL, CENTER, mock_component);
    assert_non_null(label);
    assert_ui_component_functions(label);
    label->f->cleanup(label);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_components_right_arrow(void** state)
{
    component_t* mock_component = mock_component_create();

    component_t* right_arrow = right_arrow_create(top_slider, mock_component);
    assert_non_null(right_arrow);
    assert_ui_component_functions(right_arrow);
    right_arrow->f->cleanup(right_arrow);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_components_left_arrow(void** state)
{
    component_t* mock_component = mock_component_create();

    component_t* left_arrow = left_arrow_create(top_slider, mock_component);
    assert_non_null(left_arrow);
    assert_ui_component_functions(left_arrow);
    left_arrow->f->cleanup(left_arrow);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_components_image(void** state)
{
    const unsigned char logo_bytes[] = {
        0x00, 0xc0, 0x3f, 0xff, 0x80, 0x00, 0x60, 0x3f, 0xff, 0xc0, 0x00, 0x78, 0x00, 0x00, 0x60,
        0x00, 0xff, 0x00, 0x00, 0x30, 0x00, 0x7f, 0x80, 0x00, 0x18, 0x00, 0xff, 0xf0, 0x00, 0x0c,
        0x00, 0x7f, 0xf8, 0x00, 0x06, 0x00, 0x7f, 0xfe, 0x00, 0x03, 0x00, 0xff, 0xff, 0xc0, 0x01,
        0x80, 0x7f, 0xff, 0xe0, 0x00, 0xc0, 0x00, 0x30, 0x00, 0x00, 0x60, 0x00, 0x18, 0x00, 0x00,
        0x30, 0x00, 0x0c, 0x00, 0x00, 0x18, 0x00, 0x06, 0x00, 0x00, 0x0c, 0x00, 0x03, 0x00, 0x00,
        0x0e, 0x00, 0x01, 0x80, 0x07, 0xff, 0xff, 0x00, 0xc0, 0x03, 0xff, 0xff, 0x80, 0x60, 0x00,
        0x7f, 0xff, 0x00, 0x30, 0x00, 0x1f, 0xff, 0x00, 0x18, 0x00, 0x07, 0xff, 0x80, 0x0c, 0x00,
        0x01, 0xff, 0x00, 0x06, 0x00, 0x00, 0xff, 0x80, 0x03, 0x00, 0x00, 0x1f, 0x00, 0x01, 0xff,
        0xfc, 0x03, 0x00, 0x00, 0xff, 0xfe, 0x01, 0x80, 0x00};

    component_t* mock_component = mock_component_create();

    component_t* image =
        image_create(logo_bytes, sizeof(logo_bytes), 41, 25, CENTER, mock_component);
    assert_non_null(image);
    assert_ui_component_functions(image);
    image->f->cleanup(image);

    mock_component->f->cleanup(mock_component);
}

static void confirm_callback(void* param)
{
    (void)param;
}

static void cancel_callback(void* param)
{
    (void)param;
}

static void test_ui_components_confirm(void** state)
{
    const confirm_params_t params = {
        .title = "Is the Code correct?",
        .body = "CODE",
        .font = &font_monogram_5X9,
    };
    component_t* confirm = confirm_create(&params, confirm_callback, NULL, cancel_callback, NULL);
    assert_non_null(confirm);
    assert_ui_component_functions(confirm);
    confirm->f->cleanup(confirm);
}

static void test_ui_components_info_centered(void** state)
{
    component_t* info_centered = info_centered_create("Some info", NULL);
    assert_non_null(info_centered);
    assert_ui_component_functions(info_centered);
    info_centered->f->cleanup(info_centered);
}

static void test_ui_components_keyboard_switch(void** state)
{
    component_t* mock_component = mock_component_create();

    component_t* keyboard_switch = keyboard_switch_create(top_slider, true, mock_component);
    assert_non_null(keyboard_switch);
    assert_ui_component_functions(keyboard_switch);
    keyboard_switch->f->cleanup(keyboard_switch);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_components_status(void** state)
{
    component_t* status = status_create("Password created", true, 10, NULL, NULL);
    assert_non_null(status);
    assert_ui_component_functions(status);
    status->f->cleanup(status);
}

int main(void)
{
    const struct CMUnitTest tests[] = {cmocka_unit_test(test_ui_components_label),
                                       cmocka_unit_test(test_ui_components_right_arrow),
                                       cmocka_unit_test(test_ui_components_left_arrow),
                                       cmocka_unit_test(test_ui_components_image),
                                       cmocka_unit_test(test_ui_components_info_centered),
                                       cmocka_unit_test(test_ui_components_keyboard_switch),
                                       cmocka_unit_test(test_ui_components_status),
                                       cmocka_unit_test(test_ui_components_confirm)};

    return cmocka_run_group_tests(tests, NULL, NULL);
}
