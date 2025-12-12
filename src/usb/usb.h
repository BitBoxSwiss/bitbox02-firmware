// SPDX-License-Identifier: Apache-2.0

#ifndef _USB_H_
#define _USB_H_

#include <stdbool.h>
#include <stdint.h>

/**
 * Start the USB HID interfaces.
 */
int32_t usb_start(void);

/**
 * Stop the USB interfaces.
 */
void usb_stop(void);

bool usb_is_enabled(void);

#endif
