#ifndef DA14531_SERIAL_LINK_H
#define DA14531_SERIAL_LINK_H
#include <stddef.h>
#include <stdint.h>

enum serial_link_type {
    SERIAL_LINK_TYPE_ACK = 0x2d, /* 0b00101101*/
    SERIAL_LINK_TYPE_NAK = 0x5a, /*0b01011010*/
    SERIAL_LINK_TYPE_BLE_DATA = 0x3C, /*0b00111100*/
    SERIAL_LINK_TYPE_CTRL_DATA = 0xb4, /*0b10110100*/
    SERIAL_LINK_TYPE_PING = 0x4b, /*0b01001011*/
};

// Control commands
#define SL_CTRL_CMD_DEVICE_NAME 1
#define SL_CTRL_CMD_BOND_DB_GET 2
#define SL_CTRL_CMD_BOND_DB_SET 3
#define SL_CTRL_CMD_PAIRING_CODE 4
#define SL_CTRL_CMD_BLE_STATUS 5
#define SL_CTRL_CMD_IRK 6
#define SL_CTRL_CMD_PRODUCT_STRING 7
#define SL_CTRL_CMD_BLE_CHIP_RESET 8
#define SL_CTRL_CMD_IDENTITY_ADDRESS 9
#define SL_CTRL_CMD_PAIRING_SUCCESSFUL 10
#define SL_CTRL_CMD_TK_CONFIRM 11
#define SL_CTRL_CMD_DEBUG 254

struct serial_link_frame {
    enum serial_link_type type;
    uint16_t payload_length;
    uint8_t payload[];
} __attribute((packed));

enum serial_link_in_state {
    SERIAL_LINK_STATE_READING,
    SERIAL_LINK_STATE_CHECK,
};

enum escape_state {
    ESCAPE_STATE_WAIT,
    ESCAPE_STATE_ACCEPT,
    ESCAPE_STATE_ESCAPE,
};

struct SerialLinkIn {
    enum serial_link_in_state state;
    enum escape_state escape_state;
    uint8_t buf_in[64];
    size_t buf_in_len;
    uint8_t frame[700];
    size_t frame_len;
    uint8_t buf_out[64];
    size_t buf_out_len;
    // uint16_t counter;
};

void serial_link_in_init(struct SerialLinkIn* self);

struct serial_link_frame* serial_link_in_poll(
    struct SerialLinkIn* self,
    const uint8_t* buf_in,
    uint16_t* buf_in_len);

uint16_t serial_link_out_format(
    uint8_t* buf,
    uint16_t buf_len,
    uint8_t type,
    const uint8_t* payload,
    uint16_t payload_len);
#endif
