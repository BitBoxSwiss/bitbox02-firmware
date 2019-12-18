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

/**
 * Creates a confirm screen.
 * @param[in] title The confirmation title of the screen.
 * @param[in] body The confirmation body of the screen.
 * @param[in] font if not NULL will use the specified font for the body
 * @param[in] longtouch if true, require the hold gesture to confirm instead of tap.
 * @param[in] confirm_callback The callback triggered when the user pushes the confirm button.
 * @param[in] cancel_callback The callback triggered when the user pushes the cancel button.
 */
component_t* confirm_create(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool longtouch,
    void (*confirm_callback)(component_t*),
    void (*cancel_callback)(component_t*));

/**
 * Creates a confirm screen with a scrollable body.
 * @param[in] title The confirmation title of the screen.
 * @param[in] body The confirmation body of the screen.
 * @param[in] font if not NULL will use the specified font for the body
 * @param[in] longtouch if true, require the hold gesture to confirm instead of tap.
 * @param[in] confirm_callback The callback triggered when the user pushes the confirm button.
 * @param[in] cancel_callback The callback triggered when the user pushes the cancel button.
 */
component_t* confirm_create_scrollable(
    const char* title,
    const char* body,
    const UG_FONT* font,
    bool longtouch,
    void (*confirm_callback)(component_t*),
    void (*cancel_callback)(component_t*));

#endif
