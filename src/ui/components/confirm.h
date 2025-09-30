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

#ifndef _UI_CONFIRM_H_
#define _UI_CONFIRM_H_

#include <ui/component.h>
#include <ui/ugui/ugui.h>

#include <stdbool.h>

typedef struct {
    // The confirmation title of the screen.
    const char* title;
    // If true, will automatically break the title into two lines at a whitespace position if the
    // title is too long to fit on one line.
    bool title_autowrap;
    // The confirmation body of the screen.
    const char* body;
    const UG_FONT* font;
    // If true, the body is horizontally scrollable.
    bool scrollable;
    // If true, require the hold gesture to confirm instead of tap.
    bool longtouch;
    // If true, the user can only confirm, not reject.
    bool accept_only;
    // if true, the accept icon is a right arrow instead of a checkmark (indicating going to the
    // "next" screen).
    bool accept_is_nextarrow;
    // Print the value of this variable in the corner. Will not print when 0
    size_t display_size;
} confirm_params_t;

/**
 * Creates a confirm screen.
 * @param[in] params see confirm_params_t for details.
 * @param[in] callback The callback triggered when the user accepts or rejects. Will be called at
 * most once.
 * @param[in] user_data passed through to the callback.
 */
component_t* confirm_create(
    const confirm_params_t* params,
    void (*callback)(bool result, void* user_data),
    void* user_data);

#endif
