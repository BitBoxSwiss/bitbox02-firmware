// Copyright 2020 Shift Crypto AG
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

#ifndef _UI_SCREEN_SAVER_H_
#define _UI_SCREEN_SAVER_H_

#include "component.h"

/**
 * If the screensaver is active, returns a static screen saver component.
 * If the screensaver is not active, returns NULL.
 */
component_t* screen_saver_get(void);

/**
 * Should be called from the mainloop. Handles the timing of the screensaver.
 * After a certain time, the screen saver becomes active.
 * Call `screen_saver_reset()` to reset the screensaver and the timer.
 */
void screen_saver_process(void);

/**
 * Call this to remove an active screensaver, or to reset the timer.
 */
void screen_saver_reset(void);

/**
 * disables the screensaver until enabled with `screen_saver_enable`.
 */
void screen_saver_disable(void);

/**
 * re-enables the screensaver.
 */
void screen_saver_enable(void);

#endif
