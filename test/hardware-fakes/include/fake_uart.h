// SPDX-License-Identifier: Apache-2.0

#ifndef _FAKE_UART_H_
#define _FAKE_UART_H_

#include <stdbool.h>
#include <stdint.h>

void fake_uart_reset(void);
const uint8_t* fake_uart_tx_buffer(void);
uint16_t fake_uart_tx_buffer_len(void);
bool fake_uart_transmit_next(uint8_t* byte_out);
void fake_uart_tx_complete(void);

#endif
