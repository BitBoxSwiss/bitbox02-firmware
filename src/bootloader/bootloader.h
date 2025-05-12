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

#ifndef _BOOTLOADER_H_
#define _BOOTLOADER_H_

#include <stdbool.h>

void bootloader_jump(void);

/**
 * Renders the default bootloader screen
 */
void bootloader_render_default_screen(void);

/**
 * Renders a BLE pairing confirmations screen. Use the `confirmed` argument to display the
 * "user has confirmed on bitbox" version.
 *
 * confirmed - If false renders Yes/No icons at top, otherwise renders "See app".
 */
void bootloader_render_ble_confirm_screen(bool confirmed);

#endif
