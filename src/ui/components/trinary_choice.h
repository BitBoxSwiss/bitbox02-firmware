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
