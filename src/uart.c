// Copyright 2025 Shift Crypto AG
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

#include "uart.h"
#include "driver_init.h"
#include "util.h"
#include "utils_assert.h"

#define EVENT_READ 0x01 // Available to read
#define EVENT_WRITE 0x02 // Available to write

static volatile int _usart_0_readyness = 0;

static struct io_descriptor* _io;

static void _rx_cb(const struct usart_async_descriptor* const descr)
{
    if (descr == &USART_0) {
        _usart_0_readyness |= EVENT_READ;
    }
}

static void _tx_cb(const struct usart_async_descriptor* const descr)
{
    if (descr == &USART_0) {
        _usart_0_readyness |= EVENT_WRITE;
    }
}

static int32_t _read(uint8_t* buf, uint16_t buf_len)
{
    int32_t read = 0;
    CRITICAL_SECTION_ENTER()
    _usart_0_readyness &= ~EVENT_READ;
    read = io_read(_io, buf, buf_len);
    CRITICAL_SECTION_LEAVE()

    // There was supposed to be data...
    ASSERT(read != 0);

    return read;
}

static int32_t _write(const uint8_t* buf, uint16_t buf_len)
{
    ASSERT(buf && buf_len);
    int16_t wrote = 0;
    CRITICAL_SECTION_ENTER()
    wrote = io_write(_io, buf, buf_len);
    _usart_0_readyness &= ~EVENT_WRITE;
    CRITICAL_SECTION_LEAVE()

    ASSERT(wrote == buf_len); // TODO: handle partial writes

    return wrote;
}

void uart_init(void)
{
    // util_log("uart_init");
    usart_async_get_io_descriptor(&USART_0, &_io);
    usart_async_register_callback(&USART_0, USART_ASYNC_RXC_CB, _rx_cb);
    usart_async_register_callback(&USART_0, USART_ASYNC_TXC_CB, _tx_cb);
    usart_async_enable(&USART_0);

    _usart_0_readyness |= EVENT_WRITE;
}

int32_t uart_0_read(uint8_t* buf, uint16_t buf_len)
{
    if (_usart_0_readyness & EVENT_READ) {
        return _read(buf, buf_len);
    }
    return 0;
}

bool uart_0_write(const uint8_t* buf, uint16_t buf_len)
{
    if (!(_usart_0_readyness & EVENT_WRITE)) {
        return false;
    }
    int32_t wrote = _write(buf, buf_len);
    ASSERT(wrote == buf_len);
    return wrote == buf_len;
}

bool uart_0_write_from_queue(struct ringbuffer* queue)
{
    // Must be static becuase UART driver will read from it until all bytes has been written out
    // over uart. Must not touch buffer unless EVENT_WRITE is set (indicating driver is done with
    // the buffer).
    static uint8_t _out_buf[1024];

    if (!(_usart_0_readyness & EVENT_WRITE)) {
        return false;
    }
    int32_t len;
    int32_t res;
    len = MIN(ringbuffer_num(queue), sizeof(_out_buf));
    for (int32_t i = 0; i < len; i++) {
        res = ringbuffer_get(queue, &_out_buf[i]);
        ASSERT(res == ERR_NONE);
        if (res != ERR_NONE) {
            break;
        }
    }
    // util_log("will write %d char, left %d", (int)len, (int)ringbuffer_num(queue));
    int32_t wrote = _write(_out_buf, len);
    ASSERT(wrote == len);
    return wrote == len;
}

void uart_poll(
    uint8_t* read_buf,
    uint16_t read_buf_cap,
    uint16_t* read_buf_len,
    struct ringbuffer* out_queue)
{
    *read_buf_len += uart_0_read(&read_buf[*read_buf_len], read_buf_cap - *read_buf_len);

    // If the whole buffer was used, some characters were probably missed.
    ASSERT(*read_buf_len < USART_0_BUFFER_SIZE);

    if (ringbuffer_num(out_queue) > 0) {
        uart_0_write_from_queue(out_queue);
    }
}
