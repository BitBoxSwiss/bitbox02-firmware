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

#ifndef _ORIENTATION_ARROWS_H_
#define _ORIENTATION_ARROWS_H_

#include "ui/component.h"

#include <stdbool.h>

/**
 * Creates an orientation screen and registers a done callback.
 * @param[in] done_callback The callback that is called when the orientation has been selected.
 * @param[in] cb_param The user-defined parameter that will be passed into the callback when it's
 * invoked.
 */
component_t* orientation_arrows_create(void (*done_callback)(bool, void*), void* cb_param);

#endif
