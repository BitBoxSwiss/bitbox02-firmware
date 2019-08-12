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

#ifndef _CONFIRM_GESTURE_
#define _CONFIRM_GESTURE_

#include <stdbool.h>
#include <ui/component.h>

/********************************** Create Instance **********************************/

/**
 * Creates a confirm_gesture component on the top slider.
 * @param[in] parent The parent component.
 */
component_t* confirm_gesture_create(component_t* parent);

bool confirm_gesture_is_active(component_t* component);

#endif
