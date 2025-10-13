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

#ifndef _WAITING_H_
#define _WAITING_H_

#include <ui/component.h>

/**
 * Creates a waiting screen. It starts out with the logo. Use `waiting_switch_to_lockscreen()` and
 * `waiting_switch_to_logo()` to change the display between logo and lockscreen.
 */
component_t* waiting_create(void);

/**
 * Switch the waiting screen to show the BitBox logo instead.
 */
void waiting_switch_to_logo(component_t* component);

/**
 * Switch the waiting screen to show the lockscreen instead (see lockscreen.c).
 */
void waiting_switch_to_lockscreen(component_t* component);

#endif
