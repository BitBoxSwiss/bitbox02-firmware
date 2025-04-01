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

#ifndef UART_H
#define UART_H
#include <stdbool.h>
#include <stdint.h>
#include <utils_ringbuffer.h>

void uart_init(void);
int32_t uart_0_read(uint8_t* buf, uint16_t buf_len);
bool uart_0_write(const uint8_t* buf, uint16_t buf_len);
bool uart_0_write_from_queue(struct ringbuffer* queue);

// Check if there are new bytes and try to send out if there are bytes to send
void uart_poll(
    uint8_t* read_buf,
    uint16_t read_buf_cap,
    uint16_t* read_buf_len,
    struct ringbuffer* out_queue);
#endif
