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

#include "confirm_gesture.h"
#include "keyboard_switch.h"
#include "left_arrow.h"
#include "set_password.h"

#include "hardfault.h"
#include "screen.h"
#include "util.h"

#include <touch/gestures.h>
#include <ui/event.h>
#include <ui/event_handler.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/ugui/ugui.h>
#include <ui/ui_util.h>

#ifndef TESTING
#include <driver_init.h>
#endif

#define EMPTY_CHAR '_'

// Frames until we automatically move to the next char.
#define AUTO_FORWARD_DELAY (350)

static char ALPHABET[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static char alphabet[] = "abcdefghijklmnopqrstuvwxyz";
static char digits[] = "01234567890123456789"; // For more consistent scrolling, make similar string
                                               // length to alphabet strings

/**
 * Set password data.
 */
typedef struct {
    bool bottom_slider_is_touched;
    bool top_slider_is_touched;
    bool waiting_for_next_input;
    int8_t password_index;
    keyboard_mode_t input_mode;
    uint16_t position_bottom;
    uint16_t letter_last;
    uint16_t letter;
    uint16_t number;
    bool auto_forward_active;
    uint16_t auto_forward_counter;
    char password[SET_PASSWORD_MAX_PASSWORD_LENGTH];
    float position_diff;
    void (*on_done_callback)(const char* password);
    component_t* confirm_gesture_component;
    component_t* left_arrow_component;
    component_t* keyboard_switch_component;
} set_password_data_t;

static void _cleanup(component_t* component)
{
    set_password_data_t* data = (set_password_data_t*)component->data;
    util_zero(data, sizeof(set_password_data_t));
    ui_util_component_cleanup(component);
}

static void _render(component_t* component)
{
    set_password_data_t* data = (set_password_data_t*)component->data;

    if (data->auto_forward_active) {
        data->auto_forward_counter++;
        if (data->auto_forward_counter == AUTO_FORWARD_DELAY) {
            event_t event;
            event.id = EVENT_FORWARD;
            emit_event(&event);
            data->auto_forward_counter = 0;
        }
    }

    bool confirm_gesture_active = confirm_gesture_is_active(data->confirm_gesture_component);
    // Password
    uint8_t x = 10;
    uint8_t y = 30;
    UG_FontSelect(&font_font_a_11X12);
    for (size_t i = 0; i < strlens(data->password); i++) {
        char chr = data->password[i];
        uint8_t w = font_font_a_11X12.widths[chr - font_font_a_11X12.start_char];
        if (i == strlens(data->password) - 1) {
            if (data->password[i] == EMPTY_CHAR && confirm_gesture_active) {
                // Don't show trailing char during confirm, to make it clear
                // that it is not part of the pw.
                continue;
            }
            UG_PutChar(chr, x - (w - 1) / 2, y, screen_front_color, screen_back_color, false);
            if (data->password[i] == EMPTY_CHAR && data->bottom_slider_is_touched) {
                UG_FillCircle(x, y + 4, 1, screen_front_color);
            }
        } else if (
            i == strlens(data->password) - 2 &&
            data->password[data->password_index] == EMPTY_CHAR) {
            // Show last entered character
            UG_PutChar(chr, x - (w - 1) / 2, y, screen_front_color, screen_back_color, false);
        } else {
            chr = '*';
            w = font_font_a_11X12.widths[chr - font_font_a_11X12.start_char];
            UG_PutChar(chr, x - (w - 1) / 2, y + 3, screen_front_color, screen_back_color, false);
        }
        x += 11;
    }

    // Slider indicators
    y = SCREEN_HEIGHT - 1;
    x = data->position_bottom * (SCREEN_WIDTH - 1) / MAX_SLIDER_POS;
    if (data->bottom_slider_is_touched) {
        UG_DrawLine(MIN(SCREEN_WIDTH, x + 3), y, MAX(0, x - 3), y, screen_front_color);
    }

    // Render sub-components
    data->confirm_gesture_component->f->render(data->confirm_gesture_component);
    if (!confirm_gesture_active) {
        data->left_arrow_component->f->render(data->left_arrow_component);
        data->keyboard_switch_component->f->render(data->keyboard_switch_component);
    }
}

static void _on_event(const event_t* event, component_t* component)
{
    set_password_data_t* data = (set_password_data_t*)component->data;

    data->bottom_slider_is_touched = false;
    data->top_slider_is_touched = false;

    if (event->id == EVENT_CONFIRM) {
        if (data->waiting_for_next_input) {
            data->password[data->password_index] = '\0';
            data->password_index = MAX(0, data->password_index - 1);
        }
        data->on_done_callback(data->password);
        return;
    }

    // Other gestures deactivated during confirming.
    if (confirm_gesture_is_active(data->confirm_gesture_component)) {
        return;
    }

    switch (event->id) {
    case EVENT_BOTTOM_SLIDE: {
        gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;

        // Variable scroll speed
        data->position_diff += SIGMOID(slider_data->velocity);
        // Divide by 10 in order to allow `position_diff` to change value and
        // accumulate, without changing the displayed character. This
        // allows detailed motion.
        data->letter = (strlens(ALPHABET) + data->letter_last + (int32_t)data->position_diff / 10) %
                       strlens(ALPHABET);
        data->number = (strlens(digits) + data->letter_last + (int32_t)data->position_diff / 10) %
                       strlens(digits);

        data->position_bottom = slider_data->position;
        data->bottom_slider_is_touched = true;
        data->waiting_for_next_input = false;

        data->auto_forward_active = true;
        data->auto_forward_counter = 0;
        break;
    }

    case EVENT_BOTTOM_SLIDE_RELEASED:
        data->position_diff = 0;
        data->letter_last = data->letter;
        break;

    case EVENT_TOP_CONTINUOUS_TAP:
    case EVENT_TOP_SLIDE:
        data->top_slider_is_touched = true;
        break;

    case EVENT_TOGGLE_ALPHANUMERIC:
        data->auto_forward_active = true;
        data->auto_forward_counter = 0;
        data->input_mode = (data->input_mode + 1) % NUM_INPUT_TYPES;
        break;

    case EVENT_BOTTOM_CONTINUOUS_TAP: {
        gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;
        data->position_bottom = slider_data->position;
        data->bottom_slider_is_touched = true;
        break;
    }

    case EVENT_BOTTOM_SHORT_TAP:
    case EVENT_FORWARD:
        if (!data->waiting_for_next_input) {
            data->password_index++;
            data->letter_last = 0;
            data->waiting_for_next_input = true;
        }
        break;

    case EVENT_BACKWARD:
        data->auto_forward_active = false;

        // Move cursor backward and display preceeding character
        data->password[data->password_index] = '\0';
        data->password_index = MAX(0, data->password_index - 1);
        data->letter_last = 0;
        data->letter = 0;
        data->number = 0;
        if (strlens(data->password)) {
            // Update the alphanumeric input mode
            if (strchr(alphabet, data->password[data->password_index])) {
                data->letter =
                    (uint8_t)(strchr(alphabet, data->password[data->password_index]) - alphabet);
                data->input_mode = LOWER_CASE;
            } else if (strchr(ALPHABET, data->password[data->password_index])) {
                data->letter =
                    (uint8_t)(strchr(ALPHABET, data->password[data->password_index]) - ALPHABET);
                data->input_mode = UPPER_CASE;
            } else if (strchr(digits, data->password[data->password_index])) {
                data->number =
                    (uint8_t)(strchr(digits, data->password[data->password_index]) - digits);
                data->input_mode = DIGITS;
            }
            // Update keyboard_switch submodule mode in order to stay in sync
            event_t e = {
                .id = EVENT_UPDATE_ALPHANUMERIC,
                .data = &data->input_mode,
            };
            emit_event(&e);
            data->waiting_for_next_input = false;
        } else {
            data->waiting_for_next_input = true;
        }
        break;
    default:
        break;
    }

    if (data->waiting_for_next_input) {
        data->password[data->password_index] = EMPTY_CHAR;
    } else {
        switch (data->input_mode) {
        case DIGITS:
            data->password[data->password_index] = digits[data->number];
            break;
        case LOWER_CASE:
            data->password[data->password_index] = alphabet[data->letter];
            break;
        case UPPER_CASE:
            data->password[data->password_index] = ALPHABET[data->letter];
            break;
        default:
            break;
        }
    }

    if (data->password_index + 1 >= SET_PASSWORD_MAX_PASSWORD_LENGTH) {
        event_t e;
        e.id = EVENT_CONFIRM;
        emit_event(&e);
    }
}

static const component_functions_t component_functions = {
    .cleanup = _cleanup,
    .render = _render,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/

/**
 * Creates a set password screen.
 * @param[in] on_done_callback The callback that is called when the user entered the password.
 */
component_t* set_password_create(void (*on_done_callback)(const char* password))
{
    component_t* set_password = malloc(sizeof(component_t));
    if (!set_password) {
        Abort("Error: malloc set_password");
    }
    set_password_data_t* data = malloc(sizeof(set_password_data_t));
    if (!data) {
        Abort("Error: malloc set_password data");
    }
    memset(set_password, 0, sizeof(component_t));
    memset(data, 0, sizeof(set_password_data_t));

    data->on_done_callback = on_done_callback;
    data->input_mode = LOWER_CASE;
    data->auto_forward_active = false;
    data->auto_forward_counter = 0;
    data->password[0] = 'a';
    data->letter_last = 0;
    data->letter = 0;
    data->number = 0;
    data->position_diff = 0;
    data->waiting_for_next_input = false;

    set_password->data = data;
    set_password->parent = NULL;
    set_password->f = &component_functions;
    set_password->dimension.width = SCREEN_WIDTH;
    set_password->dimension.height = SCREEN_HEIGHT;
    set_password->position.top = 0;
    set_password->position.left = 0;
    set_password->emit_without_release = true;

    data->left_arrow_component = left_arrow_create(top_slider, set_password);
    ui_util_add_sub_component(set_password, data->left_arrow_component);

    data->confirm_gesture_component = confirm_gesture_create(set_password);
    ui_util_add_sub_component(set_password, data->confirm_gesture_component);

    data->keyboard_switch_component = keyboard_switch_create(top_slider, set_password);
    ui_util_add_sub_component(set_password, data->keyboard_switch_component);

    return set_password;
}
