// SPDX-License-Identifier: Apache-2.0

#ifndef _SH1107_H_
#define _SH1107_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/*
 * The sh1107 driver will store this pointer and later use it for "set_pixel" and "update".
 */
void sh1107_configure(uint8_t* buf);

void sh1107_set_pixel(int16_t x, int16_t y, uint8_t c);
void sh1107_update(void);
void sh1107_mirror(bool mirror);
void sh1107_off(void);

#endif
