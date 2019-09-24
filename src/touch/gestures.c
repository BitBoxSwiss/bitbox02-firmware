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
#include <stdio.h>
#include <stdlib.h>

#ifndef TESTING
#include <driver_init.h>
#else
#include "mock_qtouch.h"
#endif

#include "gestures.h"
#include "ui/event.h"
#include "ui/event_handler.h"
#include "util.h"
#include <ui/component.h>

#define MAX_REGISTRATIONS 7
#define MAX_HISTORY 30

// The minimum amount of sliding difference required, so that the gesture is detected as slide, not
// touch
static const uint8_t SLIDE_DETECTION_DIFF = MAX_SLIDER_POS * 0.04; // Percent of slider range

extern volatile uint8_t measurement_done_touch;

/********************************** STATE **********************************/

static const uint16_t LONG_TOUCH = 500;

enum slider_status_t { INACTIVE, ACTIVE, RELEASED };

enum gesture_type_t { NONE, SLIDE, TAP };

// Stores whether or not sliders were released since a new screen reset the state.
bool _released_since_new_screen = false;

/**
 * The state of the gesture detection while the slider is not released.
 */
typedef struct {
    // Incremented by 1 for every update.
    uint32_t duration;
    // The start of the position.
    uint16_t position_start;
    // The last measured position.
    uint16_t position_current;
    int32_t velocity_sum;
    uint16_t velocity_index;
    // Stores the velocities.
    int16_t velocity_values[MAX_HISTORY];
    // The gesture type (tap, slide).
    enum gesture_type_t gesture_type;
    // The status of the slider.
    enum slider_status_t slider_status;
} gestures_detection_state_t;

/**
 * The current state. A gesture is detected over multiple function calls to
 * gestures_measure_and_emit(). Thus, we need to store the information continuously in a file-local
 * state.
 */
static gestures_detection_state_t _state[TOUCH_NUM_SLIDERS] = {0};

/********************************** STATE UPDATE **********************************/

/**
 * Updates the state of a slider.
 */
static void _slider_state_update(gestures_detection_state_t* state, uint16_t position)
{
    if (state->duration == 0) {
        state->position_start = position;
        state->position_current = position;
        state->gesture_type = TAP;
    }
    int16_t velocity_current = position - state->position_current;
    state->position_current = position;
    int16_t velocity_removed = state->velocity_values[state->velocity_index];
    state->velocity_sum = state->velocity_sum - velocity_removed + velocity_current;
    state->velocity_values[state->velocity_index] = velocity_current;
    state->velocity_index = (state->velocity_index + 1) % MAX_HISTORY;

    state->slider_status = ACTIVE;
    if (abs(state->position_current - state->position_start) > SLIDE_DETECTION_DIFF) {
        state->gesture_type = SLIDE;
    }
    state->duration++;
}

/**
 * Reads the status of the touch sliders and updates the state.
 */
static void _slider_state_read_and_update(const uint8_t location)
{
    if (qtouch_is_scroller_active(location)) {
        uint16_t current_pos = qtouch_get_scroller_position(location);
        if (location == bottom_slider) {
            current_pos = MAX_SLIDER_POS - current_pos;
        }
        _slider_state_update(&_state[location], current_pos);
    } else if (_state[location].duration > 0 && _state[location].slider_status != RELEASED) {
        _state[location].slider_status = RELEASED;
    } else {
        _state[location].slider_status = INACTIVE;
        _state[location].duration = 0;
    }
}

/**
 * Resets the state.
 */
static void _reset_state(void)
{
    memset(_state, 0, sizeof(_state));
}

/**
 * Prepares the gestures data to be sent with an emitted event
 */
static void _collect_gestures_data(
    gestures_detection_state_t* state,
    gestures_slider_data_t* slider_data)
{
    slider_data->position = state->position_current;
    slider_data->diff = state->position_current - state->position_start;
    slider_data->velocity = state->velocity_sum;
}

/********************************** GESTURE DETECTION **********************************/

static bool _is_continuous_tap(uint8_t location)
{
    return _state[location].gesture_type == TAP && _state[location].slider_status == ACTIVE;
}

static bool _is_tap_release(uint8_t location)
{
    return _state[location].gesture_type == TAP && _state[location].slider_status == RELEASED;
}

static bool _is_long_tap_release(uint8_t location)
{
    return _is_tap_release(location) && _state[location].duration > LONG_TOUCH;
}

static bool _is_continuous_slide(uint8_t location)
{
    return _state[location].gesture_type == SLIDE && _state[location].slider_status == ACTIVE;
}

static bool _is_slide_released(uint8_t location)
{
    return _state[location].gesture_type == SLIDE && _state[location].slider_status == RELEASED;
}

/********************************** EVENT HANDLER **********************************/

static void _gesture_emit_event(uint8_t id, slider_location_t location)
{
    // If a slider is being touched on a new screen,
    // wait until released before emitting events.
    if (!_released_since_new_screen) {
        return;
    }
    gestures_slider_data_t slider_data;
    _collect_gestures_data(&_state[location], &slider_data);
    event_t event = {.data = &slider_data, .id = id};
    emit_event(&event);
}

static void _emit_continuous_slide_event(void)
{
    if (_is_continuous_slide(top_slider)) {
        _gesture_emit_event(EVENT_TOP_SLIDE, top_slider);
    }
    if (_is_continuous_slide(bottom_slider)) {
        _gesture_emit_event(EVENT_BOTTOM_SLIDE, bottom_slider);
    }
}

static void _emit_slide_release_event(void)
{
    if (_is_slide_released(top_slider)) {
        _gesture_emit_event(EVENT_TOP_SLIDE_RELEASED, top_slider);
    }
    if (_is_slide_released(bottom_slider)) {
        _gesture_emit_event(EVENT_BOTTOM_SLIDE_RELEASED, bottom_slider);
    }
}

static void _emit_long_tap_event(void)
{
    if (_is_long_tap_release(top_slider)) {
        _gesture_emit_event(EVENT_TOP_LONG_TAP, top_slider);
    }
    if (_is_long_tap_release(bottom_slider)) {
        _gesture_emit_event(EVENT_BOTTOM_LONG_TAP, bottom_slider);
    }
}

static void _emit_short_tap_event(void)
{
    if (_is_tap_release(top_slider)) {
        _gesture_emit_event(EVENT_TOP_SHORT_TAP, top_slider);
    }
    if (_is_tap_release(bottom_slider)) {
        _gesture_emit_event(EVENT_BOTTOM_SHORT_TAP, bottom_slider);
    }
}

static void _emit_continuous_tap_event(void)
{
    if (_is_continuous_tap(top_slider)) {
        _gesture_emit_event(EVENT_TOP_CONTINUOUS_TAP, top_slider);
    }
    if (_is_continuous_tap(bottom_slider)) {
        _gesture_emit_event(EVENT_BOTTOM_CONTINUOUS_TAP, bottom_slider);
    }
}

/********************************** MEASURE, DETECT and CALLBACK **********************************/

/**
 * Measures the slider usage and calls registered callbacks to inform a client
 * about a detected gesture.
 */
static void _measure_and_emit(void)
{
    qtouch_process(); // Non blocking
    if (measurement_done_touch != 1) {
        return;
    }

    bool gesture_detected = false;

    for (int location = 0; location < TOUCH_NUM_SLIDERS; location++) {
        _slider_state_read_and_update(location);
        gesture_detected = gesture_detected || _state[location].gesture_type != NONE;
    }

    if (gesture_detected) {
        _emit_continuous_slide_event();
        _emit_slide_release_event();
        _emit_long_tap_event();
        _emit_short_tap_event();
        _emit_continuous_tap_event();
    }

    bool both_sliders_released_or_inactive = true;
    for (int location = 0; location < TOUCH_NUM_SLIDERS; location++) {
        if (_state[location].slider_status != RELEASED &&
            _state[location].slider_status != INACTIVE) {
            both_sliders_released_or_inactive = false;
        }
    }
    if (both_sliders_released_or_inactive) {
        _reset_state();
        _released_since_new_screen = true;
    }
}

void gestures_detect(bool reset, bool emit_without_release)
{
    if (reset) {
        _reset_state();
        _released_since_new_screen = emit_without_release;
    }
    _measure_and_emit();
}
