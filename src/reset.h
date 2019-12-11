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

#ifndef _RESET_H_
#define _RESET_H_

#include <stdbool.h>

/**
 * Resets the device:
 * - Updates secure chip KDF keys.
 * - Resets the securechip eeprom (u2f counter).
 * - Resets MCU flash app memory.
 * - Resets smart eeprom memory.
 * - Shows a "Device reset" status message.
 * @param[in] status If the status message should indicate success or failure
 * (the reset was user invoked or forced).
 */
void reset_reset(bool status);
#endif
