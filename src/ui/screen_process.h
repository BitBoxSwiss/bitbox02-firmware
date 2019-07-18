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

#ifndef _UI_SCREEN_PROCESS_H_
#define _UI_SCREEN_PROCESS_H_

#include "component.h"
#include <stdbool.h>

void ui_screen_render_component(component_t* component);

/**
 * Process screen, gestures, in a loop. If is_done is NULL, usb packets are also
 * processed.
 * @param[in] is_done if NULL, runs forever. Otherwise runs until is_done().
 * This mode should only be called during the processing of a usb packet.
 */
void ui_screen_process(bool (*is_done)(void));

/**
 * Process screen, gestures, in a loop with timeout.
 * @param[in] is_done
 * @param[in] on_timeout called when timout occurs
 * @param[in] timeout number of screen refreshes until timeout
 */
void ui_screen_process_with_timeout(
    bool (*is_done)(void),
    void (*on_timeout)(void),
    uint32_t timeout);

#endif
