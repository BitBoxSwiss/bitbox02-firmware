// SPDX-License-Identifier: Apache-2.0

#include <fake_uart.h>
#include <uart.h>

#include <assert.h>
#include <stddef.h>

static const uint8_t* _tx_buffer = NULL;
static uint16_t _tx_buffer_len = 0;
static uint16_t _tx_buffer_pos = 0;
static bool _write_done = true;

void fake_uart_reset(void)
{
    _tx_buffer = NULL;
    _tx_buffer_len = 0;
    _tx_buffer_pos = 0;
    _write_done = true;
}

bool uart_0_write(const uint8_t* buf, uint16_t buf_len)
{
    if (!_write_done) {
        return false;
    }
    assert(buf != NULL);
    assert(buf_len > 0);
    _tx_buffer = buf;
    _tx_buffer_len = buf_len;
    _tx_buffer_pos = 0;
    _write_done = false;
    return true;
}

bool uart_0_write_done(void)
{
    return _write_done;
}

const uint8_t* fake_uart_tx_buffer(void)
{
    return _tx_buffer;
}

uint16_t fake_uart_tx_buffer_len(void)
{
    return _tx_buffer_len;
}

bool fake_uart_transmit_next(uint8_t* byte_out)
{
    if (_write_done || _tx_buffer_pos >= _tx_buffer_len) {
        return false;
    }
    *byte_out = _tx_buffer[_tx_buffer_pos++];
    return true;
}

void fake_uart_tx_complete(void)
{
    assert(!_write_done);
    assert(_tx_buffer_pos == _tx_buffer_len);
    _write_done = true;
}
