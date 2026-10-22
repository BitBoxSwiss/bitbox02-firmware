// SPDX-License-Identifier: Apache-2.0

#include <setjmp.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <string.h>
#include <cmocka.h>

#include <touch/gestures.h>
#include <ui/components/confirm_gesture.h>
#include <ui/components/left_arrow.h>
#include <ui/components/right_arrow.h>
#include <ui/fonts/regular_fonts.h>
#include <ui/screen_stack.h>
#include <ui/ugui/ugui.h>
#include <ui/ui_util.h>

#include "fake_component.h"
#include "mock_gestures.h"
#include "mock_qtouch.h"

static void _cb(void* user_data)
{
    bool* flag = (bool*)user_data;
    *flag = true;
}

static void test_ui_right_arrow_tap(void** state)
{
    const component_functions_t modified_functions = {
        .cleanup = ui_util_component_cleanup,
        .render = ui_util_component_render_subcomponents,
        .on_event = NULL};

    component_t* mock_component = fake_component_create();
    mock_component->f = &modified_functions;
    ui_screen_stack_push(mock_component);

    bool flag = false;
    component_t* right_arrow = right_arrow_create(top_slider, mock_component, _cb, &flag);
    assert_non_null(right_arrow);
    ui_util_add_sub_component(mock_component, right_arrow);

    mock_gestures_touch_init();
    for (int i = 0; i < 11; i++) {
        mock_gestures_touch(top_slider, right_arrow->position.left);
    }
    mock_gestures_touch_release();

    assert_true(flag);

    mock_component->f->cleanup(mock_component);
}

static void test_ui_left_arrow_tap(void** state)
{
    const component_functions_t modified_functions = {
        .cleanup = ui_util_component_cleanup,
        .render = ui_util_component_render_subcomponents,
        .on_event = NULL};

    component_t* mock_component = fake_component_create();
    mock_component->f = &modified_functions;
    ui_screen_stack_push(mock_component);

    bool flag = false;
    component_t* left_arrow = left_arrow_create(top_slider, mock_component, _cb, &flag);
    assert_non_null(left_arrow);
    ui_util_add_sub_component(mock_component, left_arrow);

    mock_gestures_touch_init();
    for (int i = 0; i < 11; i++) {
        mock_gestures_touch(top_slider, 0);
    }
    mock_gestures_touch_release();

    assert_true(flag);

    mock_component->f->cleanup(mock_component);
}

static bool pixels_clipped;

static void _set_pixel(UG_S16 x, UG_S16 y, UG_COLOR color)
{
    if (color != C_BLACK && (x < 0 || x >= 128 || y < 0 || y >= 64)) {
        pixels_clipped = true;
    }
}

static void test_ui_confirm_gesture_position(void** state)
{
    (void)state;
    static UG_GUI gui;
    UG_Init(&gui, _set_pixel, &font_regular_11, 128, 64);
    const int16_t offsets[] = {0, 3};
    for (size_t i = 0; i < sizeof(offsets) / sizeof(*offsets); i++) {
        bool confirmed = false;
        component_t* gesture = confirm_gesture_create(_cb, &confirmed);
        gesture->position.top = offsets[i];
        event_t touch = {
            .id = EVENT_CONTINUOUS_TAP, .data = {.source = top_slider, .position = MAX_SLIDER_POS}};
        gesture->f->on_event(&touch, gesture);
        for (size_t frame = 0; frame < 40; frame++) {
            pixels_clipped = false;
            gesture->f->render(gesture);
            assert_false(confirmed);
        }
        // The lower arrow must be fully visible after sliding in, before the second touch.
        assert_false(pixels_clipped);
        touch.data.source = bottom_slider;
        gesture->f->on_event(&touch, gesture);
        // Both layouts must require the full hold, confirming on the same animation frame.
        for (size_t frame = 0; frame < 150; frame++) {
            gesture->f->render(gesture);
            assert_false(confirmed);
        }
        gesture->f->render(gesture);
        assert_true(confirmed);
        gesture->f->cleanup(gesture);
    }
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_ui_right_arrow_tap),
        cmocka_unit_test(test_ui_left_arrow_tap),
        cmocka_unit_test(test_ui_confirm_gesture_position),
    };

    return cmocka_run_group_tests(tests, NULL, NULL);
}
