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

#ifndef _OLED_WRITER_H
#define _OLED_WRITER_H

#include <stdint.h>
#include <stddef.h>

/*
 * Write display data to graphics RAM
 */
void oled_writer_write_data(const uint8_t* buf, size_t buf_len);

/*
 * Write single byte command
 */
void oled_writer_write_cmd(uint8_t command);

/*
 * Write double byte command
 */
void oled_writer_write_cmd_with_param(uint8_t command, uint8_t value);

#endif
