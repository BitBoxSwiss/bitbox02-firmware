// SPDX-License-Identifier: Apache-2.0

#include "u2f_packet.h"

void u2f_invalid_endpoint(RustUsbReportQueue* queue, uint32_t cid)
{
    // TODO: if U2F is disabled, we used to return a 'channel busy' command.
    // now we return an invalid cmd, because there is not going to be a matching
    // cmd in '_registered_cmds' if the U2F bit it not set (== U2F disabled).
    // TODO: figure out the consequences.
    usb_frame_prepare_err(FRAME_ERR_INVALID_CMD, cid, queue);
}

void u2f_packet_timeout_enable(uint32_t cid)
{
    rust_u2f_packet_timeout_enable(cid);
}

bool u2f_packet_timeout_get(uint32_t* cid)
{
    return rust_u2f_packet_timeout_get(cid);
}

void u2f_packet_timeout_tick(void)
{
    rust_u2f_packet_timeout_tick();
}

void u2f_packet_timeout(uint32_t cid)
{
    rust_u2f_packet_timeout(cid);
}

bool u2f_packet_process(const USB_FRAME* frame)
{
    return rust_u2f_packet_process(frame);
}

void u2f_packet_init(void)
{
    rust_u2f_packet_init();
}
