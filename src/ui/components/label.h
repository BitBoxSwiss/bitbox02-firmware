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

#ifndef _LABEL_H_
#define _LABEL_H_

#include <ui/component.h>
#include <ui/ugui/ugui.h>
#include <ui/ui_util.h>

/**
 * Creates a label with the given font and positions it in the center.
 * @param[in] component The component to update.
 * @param[in] text The new text of the label.
 */
void label_update(component_t* component, const char* text);

/**
 * Creates a label with the given font.
 * @param[in] text The text of the label.
 * @param[in] font The font of the label.
 * @param[in] position The position of the label.
 * @param[in] parent The parent component.
 */
component_t* label_create(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    component_t* parent);

/**
 * Creates a label with the given font.
 * @param[in] text The text of the label.
 * @param[in] font The font of the label.
 * @param[in] position The position of the label.
 * @param[in] offset The horizontal offset if position == CUSTOM_OFFSET.
 * @param[in] parent The parent component.
 */
component_t* label_create_offset(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    uint8_t offset,
    component_t* parent);

/**
 * Creates a label with the given font that can be scrolled.
 * @param[in] text The text of the label.
 * @param[in] font The font of the label.
 * @param[in] position The position of the label.
 * @param[in] scrollable The horizontal offset if position == CUSTOM_OFFSET.
 * @param[in] parent The parent component.
 */
component_t* label_create_scrollable(
    const char* text,
    const UG_FONT* font,
    enum screen_position_t position,
    component_t* parent);

#endif
