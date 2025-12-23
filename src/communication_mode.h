// SPDX-License-Identifier: Apache-2.0

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
