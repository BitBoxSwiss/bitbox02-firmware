// SPDX-License-Identifier: Apache-2.0

#ifndef _ATECC_COMMAND_H_
#define _ATECC_COMMAND_H_

#include <stdint.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wint-conversion"
#pragma GCC diagnostic ignored "-Wpedantic"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#include <cryptoauthlib.h>
#pragma GCC diagnostic pop

typedef struct {
    ATCADevice device;
    ATCAPacket* packet;
    uint16_t polls_remaining;
    bool pending;
} atecc_command_ctx_t;

ATCA_STATUS atecc_command_start(
    atecc_command_ctx_t* ctx,
    ATCAPacket* packet,
    ATCADevice device,
    uint16_t* wait_ms_out);
ATCA_STATUS atecc_command_poll(atecc_command_ctx_t* ctx, uint16_t* wait_ms_out);
void atecc_command_abort(atecc_command_ctx_t* ctx);
ATCA_STATUS atecc_command_execute(ATCAPacket* packet, ATCADevice device);

#endif
