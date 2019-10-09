// Copyright 2019 Shift Cryptosecurity AG
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

#include "hid_u2f.h"
#include "usb/usb_processing.h"
#include "usb_desc.h"
#include <queue.h>
#include <string.h>
#include <u2f/u2f_packet.h>

#define HID_U2F_VERSION 0x00000001u

/**
 * Holds descriptor and endpoint meta-data.
 */
static struct hid_func_data _func_data;

/**
 * The USB device function driver descriptor contains the control callback
 * for enabling and disabling the HID U2F endpoints and the descriptor and endpoint
 * meta-data.
 */
static struct usbdf_driver _func_driver;

/**
 * The report descriptors.
 */
static uint8_t _report_descriptor[] = {USB_DESC_U2F_REPORT};

/**
 * The USB device core request handler callback for the U2F interface.
 */
static int32_t _request(uint8_t ep, struct usb_req* req, enum usb_ctrl_stage stage)
{
    return hid_req(&_func_driver, ep, req, stage);
}

/**
 * Holds a pointer to the USB device core request handler callback.
 */
static struct usbdc_handler _request_handler = {NULL, (FUNC_PTR)_request};

// Stores the reports for the U2F interface.
static uint8_t _out_report[USB_HID_REPORT_OUT_SIZE];

/**
 * Sets the buffer address for the incoming endpoint to `hid_hww_out_report`.
 */
static int32_t _read(void)
{
    return hid_read(&_func_data, _out_report, USB_HID_REPORT_OUT_SIZE);
}

/** Set when the send channel is busy sending data. */
static bool _send_busy = false;

/**
 * Sends the next data, if the USB interface is ready.
 */
static void _send_next(void)
{
    if (_send_busy) {
        /*
         * We can't send yet. Whenever the current sender finished, it will
         * flush anything that's still queued.
         */
        return;
    }
    const uint8_t* data = queue_pull(queue_u2f_queue());
    if (data != NULL) {
        _send_busy = true;
        hid_write(&_func_data, data, USB_HID_REPORT_OUT_SIZE);
    }
}

/**
 * The callback function is called after usb data has been received (endpoint = OUT).
 * This is a result of calling _read().
 * The received data is stored in '_out_report'.
 */
static uint8_t _out(const uint8_t ep, const enum usb_xfer_code rc, const uint32_t count)
{
    (void)ep;
    (void)rc;
    (void)count;

    u2f_packet_process((const USB_FRAME*)_out_report);
    /* Incoming data has been processed completely. Start a new read. */
    _read();
    return ERR_NONE;
}

/**
 * Called when a usb frame has been replied to the host via the U2F interface
 * and the device is ready to send the next frame.
 */
static void _sent_done(void)
{
    _send_busy = false;
    /*
     * If there is more data queued, push it immediately to save some time.
     * Otherwise, sending will stop until somebody explicitely queues
     * a frame again.
     */
    _send_next();
}

/**
 * Initializes a U2F HID interface.
 * @param[in] callback The callback that is called upon status update (enabling/disabling or the
 * endpoints).
 */
int32_t hid_u2f_init(void (*callback)(void))
{
    _func_data.hid_status_callback = callback;
    _func_data.report_desc = _report_descriptor;
    _func_data.report_desc_len = USB_DESC_U2F_REPORT_LEN;
    _func_driver.func_data = &_func_data;

    return hid_init(&_func_driver, &_request_handler);
}

/**
 * Registers the HID U2F read and write callbacks and start listening for data.
 */
void hid_u2f_setup(void)
{
    hid_u2f_register_callback(HID_CB_READ, (FUNC_PTR)_out);
    // usb_report_sent is called when the outgoing usb frame is fully transmitted.
    hid_u2f_register_callback(HID_CB_WRITE, (FUNC_PTR)_sent_done);

    usb_processing_set_send(usb_processing_u2f(), _send_next);

    // Wait for data
    _read();
}

/**
 * Deinitializes the U2F HID interface.
 */
int32_t hid_u2f_deinit(void)
{
    return hid_deinit(&_func_driver, &_request_handler);
}

/**
 * Returns the endpoint for the given direction.
 * dir == 1: outgoing (host -> BitBox)
 * dir == 0: incoming (BitBox -> host)
 */
uint8_t hid_u2f_get_ep(uint8_t dir)
{
    return hid_get_ep(&_func_driver, dir);
}

/**
 * Deinitializes the U2F HID interface.
 */
bool hid_u2f_is_enabled(void)
{
    return hid_is_enabled(&_func_data);
}

/**
 * Registers a callback for a given transfer type.
 * @param[in] trans_type The transfer type for which the callback should be registered,
 *            which can be READ, WRITE or SET_REPORT.
 * @param[in] fund The function that is registered as a callback.
 */
int32_t hid_u2f_register_callback(enum hid_trans_type trans_type, FUNC_PTR func)
{
    return hid_register_callback(&_func_data, trans_type, func);
}

/**
 * Returns the version of the U2F interface.
 * @param[in] dir The direction of the endpoint:
 *            dir == 1: outgoing (host -> BitBox)
 *            dir == 0: incoming (BitBox -> host)
 */
uint32_t hid_u2f_get_version(void)
{
    return HID_U2F_VERSION;
}

// TODO: start timer when U2F message is received
// TODO: add function to stop timer when u2f msg was processed in time.
// TODO: if timer can finish without being cancelled, return a U2FHID_ERR_MSG_TIMEOUT.
// static void hid_u2f_timeout(void)
//{
//    if (!u2f_state_continue) {
//        return;
//    }
//
//    u2f_current_time_ms += 40;
//
//    if (u2f_current_time_ms > U2F_TIMEOUT) {
//        usb_packet_reset_state();
//        int32_t err =  usb_packet_send_err(U2FHID_ERR_MSG_TIMEOUT, cid);
//        if (err == ERR_NONE) {
//            _send_next();
//        }
//    }
//}
