// Copyright 2021 Shift Crypto AG
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

#ifndef _SSD1312_H_
#define _SSD1312_H_

#include <stdbool.h>
#include <stdint.h>

/*
 * The ssd1312 driver will store this pointer and later use it for "set_pixel" and "update".
 */
void ssd1312_configure(uint8_t* buf);

void ssd1312_set_pixel(uint16_t x, uint16_t y, uint8_t c);
void ssd1312_update(void);
void ssd1312_mirror(bool mirror);
void ssd1312_off(void);

#endif
