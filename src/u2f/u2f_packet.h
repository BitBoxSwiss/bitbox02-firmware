// SPDX-License-Identifier: Apache-2.0

#ifndef _U2F_PACKET_H_
#define _U2F_PACKET_H_

#include "usb/usb_frame.h"
#include "usb/usb_packet.h"
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

/**
 * Processes an incoming USB packet.
 * @param[in] frame The frame that is to be processed.
 * @return true if we are waiting for more frames to complete a packet, false otherwise.
 */
bool u2f_packet_process(const USB_FRAME* frame);

/**
 * Checks if there has been a timeout
 */
bool u2f_packet_timeout_get(uint32_t* cid);

/**
 * Queue a timeout packet for cid
 */
void u2f_packet_timeout(uint32_t cid);

/**
 * Increase the timout timers with 1 step (steps in 100ms)
 */
void u2f_packet_timeout_tick(void);

/**
 * Enable timer for this cid
 */
void u2f_packet_timeout_enable(uint32_t cid);

/**
 * Called when a message has been received, but there is no
 * API registered to handle the requested U2F Command (endpoint) byte.
 */
void u2f_invalid_endpoint(struct queue* queue, uint32_t cid);

void u2f_packet_init(void);

#endif
