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
 * Runs the UI once.
 *
 * This function will update the screen (if needed)
 * and process gesture-related events.
 */
void screen_process(void);

/**
 * Period of screen updates.
 * The screen is refreshed every SCREEN_FRAME_RATE event loops cycles.
 */
#define SCREEN_FRAME_RATE 30

#endif
