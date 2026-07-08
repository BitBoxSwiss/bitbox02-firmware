// SPDX-License-Identifier: Apache-2.0

#ifndef _SYSTEM_H_
#define _SYSTEM_H_

#include <stdbool.h>

/**
 * Reboots the device to stage1 and waits there.
 */
void boot_bootloader_wait(bool upside_down);

/**
 * Reboots the device.
 */
void reboot(void);

#endif
