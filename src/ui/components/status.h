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

#ifndef _STATUS_H_
#define _STATUS_H_

#include <stdbool.h>
#include <ui/component.h>

/********************************** Create Instance **********************************/

/**
 * Creates a status component with a given text. Calls the callback after delay.
 * @param[IN] text The text of the status screen.
 * @param[IN] status_success If true, indicates a success. Otherwise, false.
 * @param[IN] callback The callback that is called after <delay> time. Will be called at most once.
 */
component_t* status_create(
    const char* text,
    bool status_success,
    void (*callback)(void* user_data),
    void* user_data);

#endif
