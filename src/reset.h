// SPDX-License-Identifier: Apache-2.0

#ifndef _RESET_H_
#define _RESET_H_

#include <stdbool.h>

/**
 * Restarts the Bluetooth chip. This also means it will re-load the Bluetooth firmware from SPI
 * memory.
 */
void reset_ble(void);
#endif
