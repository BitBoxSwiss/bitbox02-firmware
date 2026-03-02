// SPDX-License-Identifier: Apache-2.0

#ifndef UART_H
#define UART_H
#include <stdbool.h>
#include <stdint.h>

struct RustByteQueue;

void uart_init(void);
int32_t uart_0_read(uint8_t* buf, uint16_t buf_len);
bool uart_0_write(const uint8_t* buf, uint16_t buf_len);
bool uart_0_write_from_queue(struct RustByteQueue* queue);

// Check if there are new bytes and try to send out if there are bytes to send
void uart_poll(
    uint8_t* read_buf,
    uint16_t read_buf_cap,
    uint16_t* read_buf_len,
    struct RustByteQueue* out_queue);
#endif
