// SPDX-License-Identifier: Apache-2.0

#include "keyboard_switch.h"
#include "../event.h"
#include "../event_handler.h"

#include <hardfault.h>
#include <screen.h>
#include <touch/gestures.h>
#include <ui/fonts/arial_fonts.h>
#include <ui/ui_util.h>

#include <stdbool.h>
#include <string.h>

/**
 * Data that is required for the keyboard switch.
 */
typedef struct {
    keyboard_mode_t mode;
    bool active; // Marker is 'active', i.e., touched
    // if true, the special chars keyboard mode is available.
    bool special_chars;
    void (*on_keyboard_switch_cb)(keyboard_mode_t mode, void* user_data);
    void* user_data;
} keyboard_switch_data_t;

/**
 * Renders the keyboard switch button.
 * @param[IN] component The keyboard switch component.
 */
static void _render(component_t* component)
{
    keyboard_switch_data_t* ks_data = (keyboard_switch_data_t*)component->data;
    UG_FontSelect(&font_font_a_11X10);
    UG_S16 w = 0, h = 0;
    switch (ks_data->mode) {
    case LOWER_CASE:
        UG_MeasureString(&w, &h, "ABC");
        UG_PutString((SCREEN_WIDTH - w) / 2 + 1, 1, "ABC", false);
        break;
    case UPPER_CASE:
        UG_MeasureString(&w, &h, "123");
        UG_PutString((SCREEN_WIDTH - w) / 2 + 1, 1, "123", false);
        break;
    case DIGITS:
        if (ks_data->special_chars) {
            UG_MeasureString(&w, &h, "&?+");
            UG_PutString((SCREEN_WIDTH - w) / 2 + 1, 1, "&?+", false);
        } else {
            UG_MeasureString(&w, &h, "abc");
            UG_PutString((SCREEN_WIDTH - w) / 2 + 2, 1, "abc", false);
        }
        break;
    case SPECIAL_CHARS:
        UG_MeasureString(&w, &h, "abc");
        UG_PutString((SCREEN_WIDTH - w) / 2 + 2, 1, "abc", false);
        break;
    default:
        Abort("Keyboard mode unrecognized");
        break;
    }
    if (ks_data->active) {
        UG_DrawLine(
            (SCREEN_WIDTH - w) / 2 + 1,
            h + 2,
            (SCREEN_WIDTH + w) / 2 - 1,
            h + 2,
            screen_front_color);
    }
}

/**
 * Switches the keyboard mode from digits to lower case to upper case.
 * @param[IN] component The keyboard switch component.
 */
static void _on_event(const event_t* event, component_t* component)
{
    keyboard_switch_data_t* ks_data = (keyboard_switch_data_t*)component->data;
    if (event->data.source != top_slider) {
        return;
    }
    switch (event->id) {
    case EVENT_CONTINUOUS_TAP:
        if (event->data.position > SLIDER_POSITION_ONE_THIRD &&
            event->data.position <= SLIDER_POSITION_TWO_THIRD) {
            ks_data->active = true;
            break;
        }
        /* FALLTHROUGH */
    case EVENT_SHORT_TAP:
        if (event->data.position > SLIDER_POSITION_ONE_THIRD &&
            event->data.position <= SLIDER_POSITION_TWO_THIRD) {
            ks_data->active = false;
            switch (ks_data->mode) {
            case LOWER_CASE:
                ks_data->mode = UPPER_CASE;
                break;
            case UPPER_CASE:
                ks_data->mode = DIGITS;
                break;
            case DIGITS:
                ks_data->mode = ks_data->special_chars ? SPECIAL_CHARS : LOWER_CASE;
                break;
            case SPECIAL_CHARS:
                ks_data->mode = LOWER_CASE;
                break;
            default:
                Abort("Keyboard mode unrecognized");
                break;
            }
            ks_data->on_keyboard_switch_cb(ks_data->mode, ks_data->user_data);
            break;
        }
        /* FALLTHROUGH */
    default:
        ks_data->active = false;
        break;
    }
}

/********************************** Variable-length Input Functions *************************/

/**
 * Collects all component functions.
 */
static component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = _on_event,
};

/********************************** Create Instance **********************************/

component_t* keyboard_switch_create(
    bool special_chars,
    bool default_to_digits,
    component_t* parent,
    void (*on_keyboard_switch_cb)(keyboard_mode_t mode, void* user_data),
    void* user_data)
{
    component_t* keyboard_switch = malloc(sizeof(component_t));
    if (!keyboard_switch) {
        Abort("Error: malloc keyboard_switch");
    }
    memset(keyboard_switch, 0, sizeof(component_t));

    keyboard_switch_data_t* ks_data = malloc(sizeof(keyboard_switch_data_t));
    if (!ks_data) {
        Abort("Error: malloc keyboard_switch data");
    }
    memset(ks_data, 0, sizeof(keyboard_switch_data_t));

    ks_data->mode = default_to_digits ? DIGITS : LOWER_CASE;
    ks_data->active = false;
    ks_data->special_chars = special_chars;
    ks_data->on_keyboard_switch_cb = on_keyboard_switch_cb;
    ks_data->user_data = user_data;

    keyboard_switch->data = ks_data;
    keyboard_switch->f = &_component_functions;
    keyboard_switch->parent = parent;

    return keyboard_switch;
}

keyboard_mode_t keyboard_current_mode(const component_t* component)
{
    keyboard_switch_data_t* ks_data = (keyboard_switch_data_t*)component->data;
    return ks_data->mode;
}
