// SPDX-License-Identifier: Apache-2.0

#include "usb_packet.h"

void usb_invalid_endpoint(RustUsbReportQueue* queue, uint32_t cid)
{
    // TODO: if U2F is disabled, we used to return a 'channel busy' command.
    // now we return an invalid cmd, because there is not going to be a matching
    // cmd in '_registered_cmds' if the U2F bit it not set (== U2F disabled).
    // TODO: figure out the consequences.
    usb_frame_prepare_err(FRAME_ERR_INVALID_CMD, cid, queue);
}

bool usb_packet_process(const USB_FRAME* frame)
{
    return rust_usb_packet_process(frame);
}
