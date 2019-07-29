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

#include "mock_component.h"
#include "mock_gestures.h"
#include "mock_qtouch.h"

// TODO: PR381 replaced callbacks with emitted events.
//       This test now has nothing to test. Need to check
//       instead that an event was emitted by `gestures.c`.

static void reset_state(void)
{
    // TODO: This was resetting callback 'called' boolean states,
    // but the callbacks were removed and replaced with events.
    // Therefore need to check that an event was emitted.
}

// TODO: SLIDE RELEASE
// TODO: HOLD RELEASE
// TODO: HOLD RELEASE + TAP RELEASE
// TODO: TAP RELEASE + HOLD + TAP RELEASE
// TODO: HOLD
// TODO: TAP
// TODO: SLIDE TAP

/**
 * Tests whether a slide / tap-release / slide gesture is successfully detected.
 */
static void test_gestures_slide_tap_slide_detected(void** state)
{
    (void)state;

    component_t* mock_component = mock_component_create();

    mock_gestures_touch_init();

    // SLIDE:
    reset_state();
    mock_gestures_touch(bottom_slider, 0);

    reset_state();
    mock_gestures_touch(bottom_slider, 2);

    reset_state();
    mock_gestures_touch(bottom_slider, 51);

    reset_state();
    mock_gestures_touch_release();

    // TAP RELEASE:
    reset_state();
    mock_gestures_touch(bottom_slider, SCREEN_WIDTH);

    reset_state();
    mock_gestures_touch(bottom_slider, SCREEN_WIDTH);

    reset_state();
    mock_gestures_touch_release();

    // SLIDE:
    reset_state();
    mock_gestures_touch(bottom_slider, 0);

    reset_state();
    mock_gestures_touch(bottom_slider, 2);

    reset_state();
    mock_gestures_touch(bottom_slider, 51);

    reset_state();
    mock_gestures_touch_release();

    free(mock_component);
}

/**
 * Tests whether the slide gesture and a successive tap gesture is detected.
 */
static void test_gestures_slide_and_tap_detected(void** state)
{
    (void)state;

    component_t* mock_component = mock_component_create();

    // SLIDE:
    mock_gestures_touch_init();
    reset_state();
    mock_gestures_touch(bottom_slider, 0);

    reset_state();
    mock_gestures_touch(bottom_slider, 2);

    reset_state();
    mock_gestures_touch(bottom_slider, 51);

    reset_state();
    mock_gestures_touch_release();

    // TAP RELEASE:
    reset_state();
    mock_gestures_touch(bottom_slider, 0);

    reset_state();
    mock_gestures_touch(bottom_slider, 0);

    reset_state();
    mock_gestures_touch_release();

    free(mock_component);
}

/**
 * Tests whether the slide gesture is detected.
 */
static void test_gestures_slide_left_to_right_detected(void** state)
{
    (void)state;

    component_t* mock_component = mock_component_create();

    mock_gestures_touch_init();
    reset_state();
    mock_gestures_touch(bottom_slider, 0);

    reset_state();
    mock_gestures_touch(bottom_slider, 51);

    reset_state();
    mock_gestures_touch(bottom_slider, 51);

    reset_state();
    mock_gestures_touch_release();

    reset_state();
    mock_gestures_touch(bottom_slider, 51);

    reset_state();
    mock_gestures_touch(bottom_slider, 121);

    reset_state();
    mock_gestures_touch_release();

    free(mock_component);
}

int main(void)
{
    const struct CMUnitTest tests[] = {
        cmocka_unit_test(test_gestures_slide_left_to_right_detected),
        cmocka_unit_test(test_gestures_slide_and_tap_detected),
        cmocka_unit_test(test_gestures_slide_tap_slide_detected),
    };
    return cmocka_run_group_tests(tests, NULL, NULL);
}
