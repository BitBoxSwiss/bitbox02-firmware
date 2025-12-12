// SPDX-License-Identifier: Apache-2.0

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef TESTING
    #include "qtouch.h"
    #include <driver_init.h>
#else
    #include "fake_qtouch.h"
#endif

#include "gestures.h"
#include "ui/event.h"
#include "ui/event_handler.h"
#include "util.h"
#include <platform_config.h>
#include <ui/component.h>

#define MAX_REGISTRATIONS 7
#define MAX_HISTORY 30

/** The minimum amount of sliding difference required, so that the gesture is detected as slide. */
static const uint8_t SLIDE_DETECTION_DIFF = MAX_SLIDER_POS * 0.04; // Percent of slider range
/**
 * The maximum amount of sliding that the user's finger is allowed to move
 * for its gesture to be still considered a tap.
 */
static const uint8_t TAP_SLIDE_TOLERANCE = MAX_SLIDER_POS * 0.1; // Percent of slider range

extern volatile bool measurement_done_touch;

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
    /*
     * The maximum distance the finger has travelled
     * since starting the gesture.
     */
    uint16_t max_slide_travel;
    int32_t velocity_sum;
    uint16_t velocity_index;
    // Stores the velocities.
    int16_t velocity_values[MAX_HISTORY];
    // The gesture type (tap, slide). FUTURE: remove this?
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
        state->max_slide_travel = 0;
        state->gesture_type = TAP;
    }
    int16_t velocity_current = position - state->position_current;
    state->position_current = position;
    int16_t velocity_removed = state->velocity_values[state->velocity_index];
    state->velocity_sum = state->velocity_sum - velocity_removed + velocity_current;
    state->velocity_values[state->velocity_index] = velocity_current;
    state->velocity_index = (state->velocity_index + 1) % MAX_HISTORY;
    uint16_t distance_from_start = abs((int)position - (int)state->position_start);
    state->max_slide_travel = MAX(distance_from_start, state->max_slide_travel);

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

/********************************** GESTURE DETECTION **********************************/

typedef bool (*gesture_detect_fn)(uint8_t location);

static bool _is_continuous_tap(uint8_t location)
{
    return _state[location].max_slide_travel < TAP_SLIDE_TOLERANCE &&
           _state[location].slider_status == ACTIVE;
}

static bool _is_tap_release(uint8_t location)
{
    return _state[location].max_slide_travel < TAP_SLIDE_TOLERANCE &&
           _state[location].slider_status == RELEASED;
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
    event_slider_data_t slider_data = {
        .source = location,
        .position = _state[location].position_current,
        .diff = _state[location].position_current - _state[location].position_start,
        .velocity = _state[location].velocity_sum,
    };
    event_t event = {.id = id, .data = slider_data};
    emit_event(&event);
}

static gesture_detect_fn _emit_event_detect_fns[EVENT_ENUM_MAX] = {
    [EVENT_SLIDE] = _is_continuous_slide,
    [EVENT_SLIDE_RELEASED] = _is_slide_released,
    [EVENT_LONG_TAP] = _is_long_tap_release,
    [EVENT_SHORT_TAP] = _is_tap_release,
    [EVENT_CONTINUOUS_TAP] = _is_continuous_tap,
};

/********************************** MEASURE, DETECT and CALLBACK **********************************/

/**
 * Measures the slider usage and calls registered callbacks to inform a client
 * about a detected gesture.
 */
static void _measure_and_emit(void)
{
    qtouch_process(); // Non blocking
    if (!measurement_done_touch) {
        return;
    }

    bool gesture_detected = false;

    for (int location = 0; location < TOUCH_NUM_SLIDERS; location++) {
        _slider_state_read_and_update(location);
        gesture_detected = gesture_detected || _state[location].gesture_type != NONE;
    }

    if (gesture_detected) {
        for (int event_idx = 0; event_idx < EVENT_ENUM_MAX; event_idx++) {
            if (_emit_event_detect_fns[event_idx](top_slider)) {
                _gesture_emit_event(event_idx, top_slider);
            }
            if (_emit_event_detect_fns[event_idx](bottom_slider)) {
                _gesture_emit_event(event_idx, bottom_slider);
            }
        }
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
