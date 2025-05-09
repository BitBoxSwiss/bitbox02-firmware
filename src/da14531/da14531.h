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

#ifndef DA14531_H
#define DA14531_H

#include "utils_ringbuffer.h"

// Control commands
#define CTRL_CMD_DEVICE_NAME 1
#define CTRL_CMD_BOND_DB_GET 2
#define CTRL_CMD_BOND_DB_SET 3
#define CTRL_CMD_PAIRING_CODE 4
#define CTRL_CMD_BLE_STATUS 5
#define CTRL_CMD_IRK 6
#define CTRL_CMD_PRODUCT_STRING 7
#define CTRL_CMD_BLE_CHIP_RESET 8
#define CTRL_CMD_IDENTITY_ADDRESS 9
#define CTRL_CMD_PAIRING_SUCCESSFUL 10
#define CTRL_CMD_TK_CONFIRM 11
#define CTRL_CMD_BLE_POWER_DOWN 12
#define CTRL_CMD_DEBUG 254

void da14531_power_down(struct ringbuffer* uart_out);

void da14531_reset(struct ringbuffer* uart_out);

// product is an array of characters to be set as product characteristic (not null terminated)
// procuct_len is the number of characters in the product array
// uart_out is the queue where to put the outgoing serially encoded bytes
void da14531_set_product(
    volatile const uint8_t* product,
    volatile uint16_t product_len,
    struct ringbuffer* uart_out);

#endif
