// SPDX-License-Identifier: Apache-2.0

#ifndef _TRINARY_CHOICE_H_
#define _TRINARY_CHOICE_H_

#include <ui/component.h>

typedef enum {
    TRINARY_CHOICE_LEFT,
    TRINARY_CHOICE_MIDDLE,
    TRINARY_CHOICE_RIGHT,
} trinary_choice_t;

/**
 * Let's the user choose between three options displayed at the bottom.
 " @param[in] message Display in the center. Can be NULL to skip.
 * @param[in] label_left Text shown for first choice.
 * @param[in] label_middle Text shown for second choice.
 * @param[in] label_right Text shown for third choice.
 * @param[in] chosen_cb will be called with the user choice.
 * @param[in] parent parent component.
 */
component_t* trinary_choice_create(
    const char* message,
    const char* label_left,
    const char* label_middle,
    const char* label_right,
    void (*chosen_cb)(trinary_choice_t, void*),
    void* chosen_cb_param,
    component_t* parent);

#endif
