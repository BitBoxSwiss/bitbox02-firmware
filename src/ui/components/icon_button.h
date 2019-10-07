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

#ifndef _ICON_BUTTON_H_
#define _ICON_BUTTON_H_

#include <screen.h>
#include <ui/component.h>

typedef enum {
    ICON_BUTTON_CHECK,
    ICON_BUTTON_CROSS,
} icon_button_type_t;

/**
 * Creates an icon_button component on the right side of the screen.
 * @param[in] type which icon to display
 * @param[in] location whether to render the component on top or bottom (UPPER/LOWER slider)
 */
component_t* icon_button_create(
    slider_location_t location,
    icon_button_type_t type,
    void (*callback)(component_t* component));

#endif
