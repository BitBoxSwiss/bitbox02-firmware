// SPDX-License-Identifier: Apache-2.0

#ifndef _OLED_WRITER_H
#define _OLED_WRITER_H

#include <stddef.h>
#include <stdint.h>

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
