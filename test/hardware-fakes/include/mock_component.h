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

#ifndef _MOCK_COMPONENT_H_
#define _MOCK_COMPONENT_H_

#include <ui/component.h>

/********************************** Create Instance **********************************/

/**
 * Creates a label with the given font either upside down or normal.
 * @param[in] text The text of the label.
 * @param[in] upside_down Whether the text should be rotated 180 degree or not.
 * @param[in] font The font of the label.
 */
component_t* mock_component_create(void);

#endif
