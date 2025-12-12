// SPDX-License-Identifier: Apache-2.0

#ifndef _KEYBOARD_SWITCH_H_
#define _KEYBOARD_SWITCH_H_

#include <screen.h>
#include <ui/component.h>

typedef enum { LOWER_CASE, UPPER_CASE, DIGITS, SPECIAL_CHARS } keyboard_mode_t;

/********************************** Create Instance **********************************/

/**
 * Creates a keyboard switch component.
 * @param[in] location The slider location.
 * @param[in] special_chars make special chars keyboard mode available.
 * @param[in] default_to_digits start with the digits keyboard instead of the lowercase keyboard.
 * @param[in] parent The parent component.
 */
component_t* keyboard_switch_create(
    bool special_chars,
    bool default_to_digits,
    component_t* parent,
    void (*on_keyboard_switch_cb)(keyboard_mode_t mode, void* user_data),
    void* user_data);

/**
 * @return the currently selected keyboard
 */
keyboard_mode_t keyboard_current_mode(const component_t* component);

#endif
