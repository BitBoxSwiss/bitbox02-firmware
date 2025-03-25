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

#include "oled_writer.h"
#include "driver_init.h"

enum _interface_t {
    INTERFACE_COMMAND,
    INTERFACE_DATA,
};
/**
 * Write to serial interface
 * @param [in] interface which interface to talk to.
 * @param [in] buf the bytes to write (must be at least buf_len long)
 * @param [in] buf_len the number of bytes to write
 */
static inline void _write(enum _interface_t interface, const uint8_t* buf, size_t buf_len)
{
    uint8_t cmd = interface == INTERFACE_COMMAND ? 0 : 1;
    gpio_set_pin_level(PIN_OLED_CMD, cmd);
    gpio_set_pin_level(PIN_OLED_CS, 0);
    // It is safe to cast from const here because "write_block" only reads from buf
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wcast-qual"
    SPI_OLED_write_block((void*)buf, buf_len);
#pragma GCC diagnostic pop
    gpio_set_pin_level(PIN_OLED_CS, 1);
}

void oled_writer_write_data(const uint8_t* buf, size_t buf_len)
{
    _write(INTERFACE_DATA, buf, buf_len);
}

void oled_writer_write_cmd(uint8_t command)
{
    const uint8_t buf[] = {command};
    _write(INTERFACE_COMMAND, buf, sizeof(buf));
}

void oled_writer_write_cmd_with_param(uint8_t command, uint8_t value)
{
    const uint8_t buf[] = {command, value};
    _write(INTERFACE_COMMAND, buf, sizeof(buf));
}
