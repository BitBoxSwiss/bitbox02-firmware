// SPDX-License-Identifier: Apache-2.0

#ifndef _USB_PACKET_H_
#define _USB_PACKET_H_

#include "usb_frame.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/**
 * The USB packet that contains a pointer to the packet data, the length, the
 * command identifier and the channel id.
 */
typedef struct {
    uint8_t data_addr[USB_DATA_MAX_LEN];
    size_t len;
    uint8_t cmd;
    uint32_t cid;
} Packet;

/**
 * Contains command callbacks
 */
typedef struct {
    uint8_t cmd;
    void (*process_cmd)(const Packet*, Packet*, const size_t);
} CMD_Callback;

/**
 * Processes an incoming USB packet.
 * @param[in] frame The frame that is to be processed.
 * @return true if packet was successfully parsed, false otherwise
 */
bool usb_packet_process(const USB_FRAME* frame);

void usb_invalid_endpoint(struct queue* queue, uint32_t cid);

#endif
