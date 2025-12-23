// SPDX-License-Identifier: Apache-2.0

#ifndef _COMMON_MAIN_H_
#define _COMMON_MAIN_H_

#include <stdint.h>

uint32_t common_stack_chk_guard(void);

/**
 * This performs common setup at boot of the firmware/factorysetup.
 */
void common_main(void);

#endif
