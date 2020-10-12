// Copyright 2019 Shift Crypto AG
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

#ifndef _SCREENSAVER_H_
#define _SCREENSAVER_H_

#include <ui/component.h>

component_t* screensaver_create(void);

/**
 * Resets the animation so that the logo starts to scroll in from left, out of screen.
 * The vertical position is unchanged, it starts where it left off.
 */
void screensaver_reset(component_t* component);

#endif
