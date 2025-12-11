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

#include "trinary_input_char.h"

#include "button.h"

#include <hardfault.h>
#include <touch/gestures.h>
#include <ui/fonts/password_11X12.h>
#include <ui/fonts/password_9X9.h>
#include <ui/ui_util.h>
#include <util.h>

#include <string.h>

typedef struct {
    char character;
    // Render position.
    UG_S16 x;
    UG_S16 y;
    // Slide to target.
    UG_S16 target_x;
    UG_S16 target_y;
    // Whether the character is new and does not need animation.
    bool newly_born;
} _element_t;

// excluding null terminator
#define MAX_CHARS 33

// Maximum depth for navigation stack
#define MAX_NAVIGATION_DEPTH 4

// Each of the three groups can occupy roughly a third of the width.
static const UG_S16 _group_width = SCREEN_WIDTH / 3;

// Stack entry for navigation history
typedef struct {
    char alphabet[MAX_CHARS + 1];
    UG_S16 horiz_space;
} navigation_stack_entry_t;

typedef struct {
    void (*character_chosen_cb)(component_t*, char);

    // Hold character sets to carry to the next step.
    char left_alphabet[MAX_CHARS + 1];
    char middle_alphabet[MAX_CHARS + 1];
    char right_alphabet[MAX_CHARS + 1];

    // Character render elements.
    _element_t elements[MAX_CHARS];
    // See trinary_input_char_in_progress()`.
    bool in_progress;
    // true if there are no available characters
    bool alphabet_is_empty;
    // Horizontal space between characters in a group.
    UG_S16 horiz_space;

    // Navigation stack to support going back to previous alphabet
    navigation_stack_entry_t navigation_stack[MAX_NAVIGATION_DEPTH];
    size_t navigation_stack_size;

    const UG_FONT* font;
} data_t;

/**
 * Push current alphabet onto navigation stack before navigating to a sub-alphabet.
 */
static void _navigation_stack_push(data_t* data, const char* current_alphabet)
{
    if (data->navigation_stack_size >= MAX_NAVIGATION_DEPTH) {
        // Stack is full, shift all entries down and add new one at the top
        for (size_t i = 0; i < MAX_NAVIGATION_DEPTH - 1; i++) {
            data->navigation_stack[i] = data->navigation_stack[i + 1];
        }
        data->navigation_stack_size = MAX_NAVIGATION_DEPTH - 1;
    }

    // Add current alphabet to stack
    navigation_stack_entry_t* entry = &data->navigation_stack[data->navigation_stack_size];
    snprintf(entry->alphabet, sizeof(entry->alphabet), "%s", current_alphabet);
    entry->horiz_space = data->horiz_space;
    data->navigation_stack_size++;
}

/**
 * Pop previous alphabet from navigation stack and return it.
 * Returns false if stack is empty.
 */
static bool _navigation_stack_pop(data_t* data, char* out_alphabet, UG_S16* out_horiz_space)
{
    if (data->navigation_stack_size == 0) {
        return false;
    }

    data->navigation_stack_size--;
    navigation_stack_entry_t* entry = &data->navigation_stack[data->navigation_stack_size];
    snprintf(out_alphabet, MAX_CHARS + 1, "%s", entry->alphabet);
    *out_horiz_space = entry->horiz_space;
    return true;
}

/**
 * Clear the navigation stack.
 */
static void _navigation_stack_clear(data_t* data)
{
    data->navigation_stack_size = 0;
    memset(data->navigation_stack, 0, sizeof(data->navigation_stack));
}

/**
 * Rebuild the full alphabet from left, middle, and right parts.
 */
static void _rebuild_full_alphabet(const data_t* data, char* out_buffer, size_t buffer_size)
{
    const size_t l = strlens(data->left_alphabet);
    const size_t m = strlens(data->middle_alphabet);
    const size_t r = strlens(data->right_alphabet);
    size_t total = l + m + r;

    if (total > buffer_size - 1) {
        total = buffer_size - 1;
    }

    size_t off = 0;
    size_t to_copy = (l > total - off) ? (total - off) : l;
    memcpy(out_buffer + off, data->left_alphabet, to_copy);
    off += to_copy;

    to_copy = (m > total - off) ? (total - off) : m;
    memcpy(out_buffer + off, data->middle_alphabet, to_copy);
    off += to_copy;

    to_copy = (r > total - off) ? (total - off) : r;
    memcpy(out_buffer + off, data->right_alphabet, to_copy);
    off += to_copy;

    out_buffer[off] = '\0';
}

/**
 * Internal helper to set the alphabet without clearing the navigation stack.
 * Used during navigation to preserve the stack.
 */
static void _set_alphabet_internal(
    component_t* component,
    const char* alphabet_input,
    UG_S16 horiz_space);

/**
 * Called when a selection on one of the alphabet options has been made.
 *
 * If the selected alphabet is a single letter, the input is completed.
 * Otherwise, the selected alphabet replaces the current one and the
 * input continues.
 */
static void _alphabet_selected(component_t* component, const char* alphabet)
{
    data_t* data = (data_t*)component->data;
    size_t len = strlens(alphabet);
    if (len == 0) {
        /* No letters available in this section of the screen. */
        return;
    }
    if (len == 1) {
        /*
         * Reset elements so that the previous letters don't slide back
         * on a new alphabet.
         */
        memset(data->elements, 0, sizeof(data->elements));
        data->character_chosen_cb(component, alphabet[0]);
        data->in_progress = false;
        /* Clear navigation stack when a character is chosen */
        _navigation_stack_clear(data);
    } else {
        /* Select a sub-alphabet. */

        /* Rebuild current full alphabet and push it before going deeper */
        char prev_alphabet[MAX_CHARS + 1];
        _rebuild_full_alphabet(data, prev_alphabet, sizeof(prev_alphabet));
        _navigation_stack_push(data, prev_alphabet);

        /* Use internal function to preserve the navigation stack */
        _set_alphabet_internal(component, alphabet, data->horiz_space);
        data->in_progress = true;
    }
}

static void _on_event(const event_t* event, component_t* component)
{
    if (!(event->id == EVENT_SHORT_TAP && event->data.source == bottom_slider)) {
        return;
    }
    data_t* data = (data_t*)component->data;
    const char* selected_alphabet;
    if (event->data.position <= SLIDER_POSITION_ONE_THIRD) {
        selected_alphabet = data->left_alphabet;
    } else if (event->data.position <= SLIDER_POSITION_TWO_THIRD) {
        selected_alphabet = data->middle_alphabet;
    } else {
        selected_alphabet = data->right_alphabet;
    }
    _alphabet_selected(component, selected_alphabet);
}

static void _render(component_t* component)
{
    data_t* data = (data_t*)component->data;

    // Update positions: slide to target.
    static int frame_counter = 0;
    frame_counter++;
    if (frame_counter >= 3) {
        frame_counter = 0;

        for (size_t idx = 0; idx < MAX_CHARS; idx++) {
            _element_t* element = &data->elements[idx];
            if (element->character == '\0') {
                continue;
            }
            int fx = element->target_x > element->x ? 1 : -1;
            int fy = element->target_y > element->y ? 1 : -1;

            if (element->target_x != element->x) {
                int offset = (element->target_x - element->x) / 5;
                if (offset == 0) offset = fx;
                element->x += offset;
            }
            if (element->target_y != element->y) {
                int offset = (element->target_y - element->y) / 5;
                if (offset == 0) offset = fy;
                element->y += offset;
            }
        }
    }

    // Render
    UG_FontSelect(data->font);
    for (size_t idx = 0; idx < MAX_CHARS; idx++) {
        _element_t* element = &data->elements[idx];
        if (element->character == '\0') {
            continue;
        }
        UG_PutChar(
            element->character,
            element->x,
            element->y,
            screen_front_color,
            screen_back_color,
            false);
    }
    ui_util_component_render_subcomponents(component);
}

/**
 * Collects all component functions.
 */
static const component_functions_t _component_functions = {
    .cleanup = ui_util_component_cleanup,
    .render = _render,
    .on_event = _on_event,
};

static void _put_string(
    const UG_FONT* font,
    UG_S16 x_offset,
    UG_S16 y_offset,
    UG_S16 horiz_space,
    _element_t** elements,
    size_t elements_size)
{
    UG_S16 total_width = 0;
    for (size_t idx = 0; idx < elements_size; idx++) {
        char c = elements[idx]->character;
        total_width += font->widths[c - font->start_char];
        total_width += horiz_space;
    }

    // Split into two rows of roughly equal size by number of elements if too big.
    if (elements_size > 6) {
        // split in two halfs; size/2 rounded up
        const size_t half = (elements_size + 1) / 2;
        _put_string(font, x_offset, y_offset - font->char_height - 1, horiz_space, elements, half);
        _put_string(font, x_offset, y_offset, horiz_space, elements + half, elements_size - half);
        return;
    }

    UG_S16 x = x_offset + _group_width / 2 - total_width / 2;
    for (size_t idx = 0; idx < elements_size; idx++) {
        _element_t* element = elements[idx];
        char c = elements[idx]->character;
        bool update_position = !element->newly_born;
        element->target_x = x;
        x += c == ' ' ? UI_UTIL_VISIBLE_SPACE_WIDTH
                      : font->widths[element->character - font->start_char];
        x += horiz_space;
        element->target_y = y_offset;
        if (!update_position) {
            element->x = element->target_x;
            element->y = element->target_y;
        }
    }
}

static void _set_alphabet_internal(
    component_t* component,
    const char* alphabet_input,
    UG_S16 horiz_space)
{
    data_t* data = (data_t*)component->data;
    data->horiz_space = horiz_space;
    // copy so that alphabet_input can overlap with left_alphabet, middle_alphabet, right_alphabet
    // overwritten below.
    char alphabet[MAX_CHARS + 1];
    snprintf(alphabet, sizeof(alphabet), "%s", alphabet_input);
    size_t len = strlens(alphabet);

    // Switch to larger font for fewer characters
    if (len < 12) {
        data->font = &font_password_11X12;
    } else {
        data->font = &font_password_9X9;
    }

    size_t a = 0;
    size_t b = len / 3;
    size_t c = 2 * len / 3;
    size_t d = len;
    size_t left_size = b - a;
    size_t middle_size = c - b;
    size_t right_size = d - c;
    // force the smaller group to the middle (e.g. `ab - cde - fgh` becomes `abc - de - fgh`.
    if (left_size == 2 && middle_size == 3) {
        left_size = 3;
        middle_size = 2;
    }

    // Wipe all elements that are not present anymore.
    for (size_t element_idx = 0; element_idx < MAX_CHARS; element_idx++) {
        bool found = false;
        for (size_t char_idx = 0; char_idx < len; char_idx++) {
            if (data->elements[element_idx].character == alphabet[char_idx]) {
                found = true;
                break;
            }
        }
        if (!found) {
            data->elements[element_idx].character = '\0';
        }
    }
    _element_t* elements_lookup[MAX_CHARS] = {0};

    for (size_t char_idx = 0; char_idx < len; char_idx++) {
        // Find element of same character to update position, or get an unused one.
        _element_t* element = NULL;
        for (size_t element_idx = 0; element_idx < MAX_CHARS; element_idx++) {
            if (data->elements[element_idx].character == alphabet[char_idx]) {
                element = &data->elements[element_idx];
                break;
            }
            if (element == NULL && data->elements[element_idx].character == '\0') {
                element = &data->elements[element_idx];
            }
        }
        if (element == NULL) {
            Abort("trinary: could not find element");
        }
        elements_lookup[char_idx] = element;
        element->newly_born = element->character == '\0';
        element->character = alphabet[char_idx];
    }

    UG_S16 y_offset = SCREEN_HEIGHT - data->font->char_height;
    { // left
        _put_string(data->font, 0, y_offset, data->horiz_space, elements_lookup, left_size);
    }
    { // middle
        _put_string(
            data->font,
            _group_width,
            y_offset,
            data->horiz_space,
            elements_lookup + left_size,
            middle_size);
    }
    { // right
        _put_string(
            data->font,
            2 * _group_width,
            y_offset,
            data->horiz_space,
            elements_lookup + left_size + middle_size,
            right_size);
    }

    snprintf(data->left_alphabet, sizeof(data->left_alphabet), "%.*s", (int)left_size, alphabet);
    snprintf(
        data->middle_alphabet,
        sizeof(data->middle_alphabet),
        "%.*s",
        (int)middle_size,
        alphabet + left_size);
    snprintf(
        data->right_alphabet,
        sizeof(data->right_alphabet),
        "%.*s",
        (int)right_size,
        alphabet + left_size + middle_size);

    data->alphabet_is_empty = len == 0;
}

void trinary_input_char_set_alphabet(
    component_t* component,
    const char* alphabet_input,
    UG_S16 horiz_space)
{
    data_t* data = (data_t*)component->data;
    /* Clear navigation stack when alphabet is set externally */
    _navigation_stack_clear(data);
    data->in_progress = false;

    _set_alphabet_internal(component, alphabet_input, horiz_space);
}

/********************************** Create Instance **********************************/

component_t* trinary_input_char_create(
    void (*character_chosen_cb)(component_t*, char),
    component_t* parent)
{
    data_t* data = malloc(sizeof(data_t));
    if (!data) {
        Abort("Error: malloc trinary char data");
    }
    memset(data, 0, sizeof(data_t));

    component_t* component = malloc(sizeof(component_t));
    if (!component) {
        Abort("Error: malloc trinary char");
    }
    memset(component, 0, sizeof(component_t));

    data->font = &font_password_9X9;
    component->data = data;
    component->parent = parent;
    component->f = &_component_functions;
    component->dimension.width = SCREEN_WIDTH;
    component->dimension.height = SCREEN_HEIGHT;

    data->character_chosen_cb = character_chosen_cb;

    return component;
}

bool trinary_input_char_go_back(component_t* component)
{
    data_t* data = (data_t*)component->data;

    char previous_alphabet[MAX_CHARS + 1];
    UG_S16 previous_horiz_space;

    if (_navigation_stack_pop(data, previous_alphabet, &previous_horiz_space)) {
        // Successfully popped from stack, restore previous alphabet
        // Use internal function to preserve the remaining stack
        _set_alphabet_internal(component, previous_alphabet, previous_horiz_space);
        data->in_progress = data->navigation_stack_size > 0; // Still in progress if stack not empty
        return true;
    }

    // Stack was empty, can't go back further
    data->in_progress = false;
    return false;
}

bool trinary_input_char_in_progress(component_t* component)
{
    const data_t* data = (data_t*)component->data;
    return data->in_progress;
}

bool trinary_input_char_alphabet_is_empty(component_t* component)
{
    const data_t* data = (data_t*)component->data;
    return data->alphabet_is_empty;
}
