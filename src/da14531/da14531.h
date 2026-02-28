// SPDX-License-Identifier: Apache-2.0

#ifndef DA14531_H
#define DA14531_H

#include <stdint.h>

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

enum da14531_connected_state {
    DA14531_CONNECTED_ADVERTISING = 0,
    DA14531_CONNECTED_CONNECTED = 1,
    DA14531_CONNECTED_CONNECTED_SECURED = 2,
};

extern enum da14531_connected_state da14531_connected_state;
#endif
