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
#include <stdbool.h>

// microseconds to wait after buffer has been sent out over SPI
// (works with 3, use 6 to have some margin)
#define WAIT_AFTER_TX_READY 6

static struct io_descriptor* _io;
volatile bool _tx_ready = true;

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
    io_write(_io, buf, buf_len);
}

// This function intentionally does not block until buffer is transferred
void oled_writer_write_data(const uint8_t* buf, size_t buf_len)
{
    _write(INTERFACE_DATA, buf, buf_len);
}

void oled_writer_write_data_blocking(const uint8_t* buf, size_t buf_len)
{
    _tx_ready = false;
    oled_writer_write_data(buf, buf_len);
    while (!_tx_ready);
    // Wait a moment so that CMD/CS doesn't change until device has processed all data
    delay_us(WAIT_AFTER_TX_READY);
    gpio_set_pin_level(PIN_OLED_CS, 1);
}

void oled_writer_write_cmd_blocking(uint8_t command)
{
    _tx_ready = false;
    const uint8_t buf[] __attribute((aligned(4))) = {command};
    _write(INTERFACE_COMMAND, buf, sizeof(buf));
    // Buffer is stack allocated, need to wait until done
    while (!_tx_ready);
    // Wait a moment so that CMD/CS doesn't change until device has processed all data
    delay_us(WAIT_AFTER_TX_READY);
    gpio_set_pin_level(PIN_OLED_CS, 1);
}

void oled_writer_write_cmd_with_param_blocking(uint8_t command, uint8_t value)
{
    _tx_ready = false;
    const uint8_t buf[] __attribute((aligned(4))) = {command, value};
    _write(INTERFACE_COMMAND, buf, sizeof(buf));
    // Buffer is stack allocated, need to wait until done
    while (!_tx_ready);
    // Wait a moment so that CMD/CS doesn't change until device has processed all data
    delay_us(WAIT_AFTER_TX_READY);
    gpio_set_pin_level(PIN_OLED_CS, 1);
}

static void _dma_tx_complete(struct _dma_resource* resource)
{
    (void)resource;
    _tx_ready = true;
}

void oled_writer_init(void)
{
    spi_m_dma_get_io_descriptor(&SPI_0, &_io);

    spi_m_dma_register_callback(&SPI_0, SPI_M_DMA_CB_TX_DONE, _dma_tx_complete);
    spi_m_dma_enable(&SPI_0);
}
