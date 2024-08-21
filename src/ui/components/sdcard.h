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

#ifndef _SD_CARD_H_
#define _SD_CARD_H_

#include <ui/component.h>

/**
 * Creates an insert/remove SD card screen.
 * @param[in] insert if true, the user is asked to insert the sdcard. Otherwise the user is asked to
 *            remove it.
 */
component_t* sdcard_create(void (*callback)(bool, void*), void* callback_param);

#endif
