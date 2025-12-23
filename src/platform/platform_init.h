// SPDX-License-Identifier: Apache-2.0

#ifndef _PLATFORM_INIT_H_
#define _PLATFORM_INIT_H_
#include <platform/platform_config.h>
#include <stddef.h>
void platform_init(void);

#if !(defined(BOOTLOADER) && PLATFORM_BITBOX02PLUS == 0)
// Returns a json string representing the firmware type and version
const char* platform_product(size_t* len);
#endif
#endif
