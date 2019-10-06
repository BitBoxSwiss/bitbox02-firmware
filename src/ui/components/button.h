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

#ifndef _BUTTON_H_
#define _BUTTON_H_

#include <screen.h>
#include <ui/component.h>

#include <stdint.h>
/**
 * Creates a button with the given text.
 * @param[in] text The text of the button.
 * @param[in] location The location of the button (top or bottom).
 * @param[in] screen_position The location of the button.
 * @param[in] callback The callback that is called when the button is pushed.
 * @param[in] parent The parent component.
 */
component_t* button_create(
    const char* text,
    slider_location_t location,
    uint8_t screen_position,
    void (*callback)(component_t*),
    component_t* parent);

/**
 * Creates a button with the given text that spans the whole slider.
 * @param[in] text The text of the button.
 * @param[in] location The location of the button (top or bottom).
 * @param[in] callback The callback that is called when the button is pushed.
 * @param[in] parent The parent component.
 */
component_t* button_create_wide(
    const char* text,
    slider_location_t location,
    void (*callback)(component_t*),
    component_t* parent);

/**
 * Creates an upside-down button with the given text.
 * @param[in] text The text of the button.
 * @param[in] location The location of the button (top or bottom).
 * @param[in] screen_position The location of the button.
 * @param[in] callback The callback that is called when the button is pushed.
 * @param[in] parent The parent component.
 */
component_t* button_create_upside_down(
    const char* text,
    slider_location_t location,
    uint8_t screen_position,
    void (*callback)(component_t*),
    component_t* parent);

/**
 * Creates an upside-down button with the given text that spans over the whole slider.
 * @param[in] text The text of the button.
 * @param[in] location The location of the button (top or bottom).
 * @param[in] callback The callback that is called when the button is pushed.
 * @param[in] parent The parent component.
 */
component_t* button_create_wide_upside_down(
    const char* text,
    slider_location_t location,
    void (*callback)(component_t*),
    component_t* parent);

/**
 * Updates a button with the given text and callback.
 * @param[in] text The text of the button.
 * @param[in] callback The callback that is called when the button is pushed.
 */
void button_update(component_t* button, const char* text, void (*callback)(component_t*));

#endif
