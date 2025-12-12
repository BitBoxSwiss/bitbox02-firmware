// SPDX-License-Identifier: Apache-2.0

#ifndef _SSD1312_H_
#define _SSD1312_H_

#include <stdbool.h>
#include <stdint.h>

/*
 * The ssd1312 driver will store this pointer and later use it for "set_pixel" and "update".
 */
void ssd1312_configure(uint8_t* buf);

void ssd1312_set_pixel(int16_t x, int16_t y, uint8_t c);
void ssd1312_update(void);
void ssd1312_mirror(bool mirror);
void ssd1312_off(void);

#endif
