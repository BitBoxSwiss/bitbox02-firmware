// SPDX-License-Identifier: Apache-2.0

#include "usb_frame.h"

UsbReportQueueError usb_frame_reply(
    uint8_t cmd,
    const uint8_t* data,
    uint32_t len,
    uint32_t cid,
    RustUsbReportQueue* queue)
{
    return rust_u2fhid_frame_reply(cmd, data, len, cid, queue);
}

UsbReportQueueError usb_frame_prepare_err(uint8_t err, uint32_t cid, RustUsbReportQueue* queue)
{
    return rust_u2fhid_frame_prepare_err(err, cid, queue);
}

int32_t usb_frame_process(const USB_FRAME* frame, State* state)
{
    return rust_u2fhid_frame_process(frame, state);
}
