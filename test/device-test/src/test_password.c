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
#include <screen.h>
#include <touch/gestures.h>
#include <ui/components/confirm_gesture.h>
#include <ui/components/keyboard_switch.h>
#include <ui/components/left_arrow.h>
#include <ui/event.h>
#include <ui/event_handler.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/screen_process.h>
#include <ui/screen_stack.h>
#include <ui/ui_util.h>

#include "hardfault.h"
#include "qtouch.h"
#include "screen.h"
#include "util.h"

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-function"

#define MAX_PASSWORD_LENGTH 10
#define EMPTY_CHAR '_'

static char ALPHABET[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static char alphabet[] = "abcdefghijklmnopqrstuvwxyz";
static char digits[] = "01234567890123456789"; // For more consistent scrolling, make similar string
                                               // length to alphabet strings

// Helper variables for init screen
static bool init_screen = true;
static bool first_tap = true;
static uint16_t position_top = 0;

/**
 * Set password data.
 */
typedef struct {
    bool bottom_slider_is_touched;
    bool top_slider_is_touched;
    keyboard_mode_t input_mode;
    int8_t password_index;
    uint16_t position_bottom;
    char password[MAX_PASSWORD_LENGTH];
    void (*on_done_callback)(const char* password);
} set_password_data_t;

static void _render(component_t* component)
{
    set_password_data_t* data = (set_password_data_t*)component->data;

    // TODO - move this to separate screen
    if (init_screen) {
        static uint16_t init_screen_count = 0;
        int x, y;
        UG_FontSelect(&font_font_a_9X9);
        UG_PutString(8, 26, "   Enter a password", false);

        // Night rider (cosine wave movement)
        float cos = (float)((512 / 4 + init_screen_count++) % (512 + 1));
        cos = cos / 512 * 3.14F * 2 - 3.14F;
        if (cos > 0) {
            cos = 1.27323954F * cos - 0.405284735F * cos * cos;
        } else {
            cos = 1.27323954F * cos + 0.405284735F * cos * cos;
        }
        x = (int)(cos * 32) + SCREEN_WIDTH / 2 - 2;
        y = SCREEN_HEIGHT - 1;
        UG_DrawLine(x - 4, y, x + 4, y, screen_front_color);

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
        return;
    }

    // Password
    uint8_t x, y;
    UG_FontSelect(&font_font_a_11X12);
    for (size_t i = 0, x = 10; i < strlens(data->password); i++) {
        char chr = data->password[i];
        uint8_t w = font_font_a_11X12.widths[chr - font_font_a_11X12.start_char];
        y = 30;
        if (i == strlens(data->password) - 1) {
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
    // x = x - 11 - 2;

    // Slider indicators
    y = SCREEN_HEIGHT - 1;
    x = data->position_bottom * (SCREEN_WIDTH - 1) / (DEF_SCROLLER_RESOLUTION - 1);
    if (data->bottom_slider_is_touched) {
        UG_DrawLine(MIN(SCREEN_WIDTH, x + 3), y, MAX(0, x - 3), y, screen_front_color);
    }

    // Render sub-components
    for (uint8_t i = 0; i < component->sub_components.amount; i++) {
        component->sub_components.sub_components[i]->f->render(
            component->sub_components.sub_components[i]);
    }
}

static void _on_event(const event_t* event, component_t* component)
{
    static uint16_t letter_last = 0;
    static uint16_t letter = 0;
    static uint16_t number = 0;
    static float position_diff = 0;
    static bool waiting_for_input = false;
    set_password_data_t* data = (set_password_data_t*)component->data;

    data->bottom_slider_is_touched = false;
    data->top_slider_is_touched = false;

    switch (event->id) {
    case EVENT_BOTTOM_SLIDE: {
        gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;

        // Variable scroll speed
        position_diff +=
            (float) (0.005 * slider_data->velocity * abs(slider_data->velocity) /
            (1 + 0.002 * slider_data->velocity * slider_data->velocity)); // sigmoid function

        // Divide by 10 in order to allow `position_diff` to change value and
        // accumulate, without changing the displayed character. This
        // allows detailed motion.
        letter =
            (strlens(ALPHABET) + letter_last + (int32_t)position_diff / 10) % strlens(ALPHABET);
        number = (strlens(digits) + letter_last + (int32_t)position_diff / 10) % strlens(digits);

        data->position_bottom = slider_data->position;
        data->bottom_slider_is_touched = true;
        waiting_for_input = false;
        init_screen = false;
        break;
    }

    case EVENT_BOTTOM_SLIDE_RELEASED:
        position_diff = 0;
        letter_last = letter;
        break;

    case EVENT_TOP_CONTINUOUS_TAP:
    case EVENT_TOP_SLIDE: {
        // FIXME - Only used for init_screen
        gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;
        position_top = slider_data->position;
    }
        data->top_slider_is_touched = true;
        break;

    case EVENT_TOGGLE_ALPHANUMERIC:
        data->input_mode = (data->input_mode + 1) % NUM_INPUT_TYPES;
        break;

    case EVENT_BOTTOM_CONTINUOUS_TAP: {
        gestures_slider_data_t* slider_data = (gestures_slider_data_t*)event->data;
        data->position_bottom = slider_data->position;
        data->bottom_slider_is_touched = true;
        init_screen = false;
        break;
    }

    case EVENT_BOTTOM_SHORT_TAP:
    case EVENT_FORWARD:
        // FIXME -  Remove `first_tap` and `init_screen` if init screen
        //          is put into its own screen.
        if (!init_screen && !first_tap && !waiting_for_input) {
            data->password_index++;
            letter_last = 0;
            waiting_for_input = true;
        }
        first_tap = false;
        break;

    case EVENT_BACKWARD:
        // Move cursor backward and display preceeding character
        data->password[data->password_index] = '\0';
        data->password_index = MAX(0, data->password_index - 1);
        letter_last = 0;
        letter = 0;
        number = 0;
        if (strlens(data->password)) {
            // Update the alphanumeric input mode
            if (strchr(alphabet, data->password[data->password_index])) {
                letter =
                    (uint8_t)(strchr(alphabet, data->password[data->password_index]) - alphabet);
                data->input_mode = LOWER_CASE;
            } else if (strchr(ALPHABET, data->password[data->password_index])) {
                letter =
                    (uint8_t)(strchr(ALPHABET, data->password[data->password_index]) - ALPHABET);
                data->input_mode = UPPER_CASE;
            } else if (strchr(digits, data->password[data->password_index])) {
                number = (uint8_t)(strchr(digits, data->password[data->password_index]) - digits);
                data->input_mode = DIGITS;
            }
            // Update keyboard_switch submodule mode in order to stay in sync
            event_t e = {
                .id = EVENT_UPDATE_ALPHANUMERIC,
                .data = &data->input_mode,
            };
            emit_event(&e);
            waiting_for_input = false;
        } else {
            waiting_for_input = true;
        }
        break;

    case EVENT_CONFIRM:
        if (waiting_for_input) {
            data->password[data->password_index] = '\0';
            data->password_index = MAX(0, data->password_index - 1);
        }
        data->on_done_callback(data->password);
        break;

    default:
        break;
    }

    if (waiting_for_input) {
        data->password[data->password_index] = EMPTY_CHAR;
    } else {
        switch (data->input_mode) {
        case DIGITS:
            data->password[data->password_index] = digits[number];
            break;
        case LOWER_CASE:
            data->password[data->password_index] = alphabet[letter];
            break;
        case UPPER_CASE:
            data->password[data->password_index] = ALPHABET[letter];
            break;
        default:
            break;
        }
    }

    if (data->password_index + 1 >= MAX_PASSWORD_LENGTH) {
        event_t e;
        e.id = EVENT_CONFIRM;
        emit_event(&e);
    }
}

static const component_functions_t component_functions = {.cleanup = ui_util_component_cleanup,
                                                          .render = _render,
                                                          .on_event = _on_event};

/********************************** Create Instance **********************************/

/**
 * Creates a set password screen.
 * @param[in] on_done_callback The callback that is called when the user entered the password.
 */
static component_t* test_pw_create(void (*on_done_callback)(const char* password))
{
    component_t* set_password = malloc(sizeof(component_t));
    if (!set_password) Abort("Error: malloc set_password");
    set_password_data_t* data = malloc(sizeof(set_password_data_t));
    if (!data) Abort("Error: malloc data");
    memset(set_password, 0, sizeof(component_t));
    memset(data, 0, sizeof(set_password_data_t));

    data->on_done_callback = on_done_callback;
    data->input_mode = LOWER_CASE;

    set_password->data = data;
    set_password->parent = NULL;
    set_password->f = &component_functions;
    set_password->dimension.width = SCREEN_WIDTH;
    set_password->dimension.height = SCREEN_HEIGHT;
    set_password->position.top = 0;
    set_password->position.left = 0;

    component_t* sc_left_arrow = left_arrow_create(top_slider, set_password);
    ui_util_add_sub_component(set_password, sc_left_arrow);

    component_t* sc_confirm_gesture = confirm_gesture_create(set_password);
    ui_util_add_sub_component(set_password, sc_confirm_gesture);

    component_t* sc_keyboard_switch = keyboard_switch_create(top_slider, set_password);
    ui_util_add_sub_component(set_password, sc_keyboard_switch);

    return set_password;
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////

static void done_callback(const char* password)
{
    UG_ClearBuffer();
    UG_FontSelect(&font_font_a_9X9);
    char msg[128];
    snprintf(msg, 128, "You entered:\n %s", password);
    UG_PutString(8, 26, msg, false);
    UG_SendBuffer();
    delay_ms(2000);

    UG_ClearBuffer();
    UG_FontSelect(&font_font_a_9X9);
    UG_PutString(8, 26, "   Restarting", false);
    UG_SendBuffer();
    delay_ms(500);

    _reset_mcu();
}

int main(void)
{
    system_init();
    screen_init();
    qtouch_init();
    ui_screen_stack_push(test_pw_create(done_callback));
    ui_screen_process(NULL);
}

#pragma GCC diagnostic pop
