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

#ifndef _INFO_CENTERED_H_
#define _INFO_CENTERED_H_

#include <ui/component.h>

/**
 * Creates an info centered screen with a given text and an optional skip callback.
 * @param[IN] text The text that is displayed in the middle of the screen.
 * @param[IN] skip_callback The optional callback. If specified, a "Skip"
 * button is shown on the bottom and the callback is called when the button is
 * pressed.
 */
component_t* info_centered_create(const char* text, void (*skip_callback)(component_t*));

#endif
