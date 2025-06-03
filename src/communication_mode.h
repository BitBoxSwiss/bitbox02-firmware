// Copyright 2025 Shift Crypto AG
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

#ifndef _FIRMWARE_COMMUNICATION_MODE_H_
#define _FIRMWARE_COMMUNICATION_MODE_H_

#include <stdbool.h>

/**
 * Call this when the first USB request is seen. After this, `communication_mode_ble_enabled()` will
 * be false even on Bluetooth enabled devices (USB takes priority).
 */
void communication_mode_ble_disable(void);

/**
 * Returns true if this device is Bluetooth-enabled and we have not seen a USB request yet, which
 * means we are communicating via Bluetooth.
 */
bool communication_mode_ble_enabled(void);
#endif
