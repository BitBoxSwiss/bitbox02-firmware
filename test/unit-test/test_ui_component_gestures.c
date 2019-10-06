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
#include <string.h>
#include <cmocka.h>

#include <touch/gestures.h>
#include <ui/components/left_arrow.h>
#include <ui/components/right_arrow.h>
#include <ui/screen_stack.h>
#include <ui/ui_util.h>

#include "mock_component.h"
#include "mock_gestures.h"
#include "mock_qtouch.h"

static uint8_t _correct_event;
static bool _correct_event_seen = false;

static void test_on_event(const event_t* _event, component_t* _component)
{
    if (_event->id == _correct_event) {
        _correct_event_seen = true;
    }
}

static void test_ui_right_arrow_tap(void** state)
{
    const component_functions_t modified_functions = {
        .cleanup = ui_util_component_cleanup,
        .render = ui_util_component_render_subcomponents,
        .on_event = test_on_event};

    component_t* mock_component = mock_component_create();
    mock_component->f = &modified_functions;
    ui_screen_stack_push(mock_component);

    component_t* right_arrow = right_arrow_create(top_slider, mock_component);
    assert_non_null(right_arrow);
    ui_util_add_sub_component(mock_component, right_arrow);

    _correct_event_seen = false;
    _correct_event = EVENT_FORWARD;

    mock_gestures_touch_init();
    for (int i = 0; i < 11; i++) {
        mock_gestures_touch(top_slider, right_arrow->position.left);
    }
    mock_gestures_touch_release();

    assert_true(_correct_event_seen);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_left_arrow_tap(void** state)
{
    const component_functions_t modified_functions = {
        .cleanup = ui_util_component_cleanup,
        .render = ui_util_component_render_subcomponents,
        .on_event = test_on_event};

    component_t* mock_component = mock_component_create();
    mock_component->f = &modified_functions;
    ui_screen_stack_push(mock_component);

    component_t* left_arrow = left_arrow_create(top_slider, mock_component);
    assert_non_null(left_arrow);
    ui_util_add_sub_component(mock_component, left_arrow);

    _correct_event_seen = false;
    _correct_event = EVENT_BACKWARD;

    mock_gestures_touch_init();
    for (int i = 0; i < 11; i++) {
        mock_gestures_touch(top_slider, 0);
    }
    mock_gestures_touch_release();

    assert_true(_correct_event_seen);

    mock_component->f->cleanup(mock_component);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_ui_right_arrow_tap),
        cmocka_unit_test(test_ui_left_arrow_tap),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
