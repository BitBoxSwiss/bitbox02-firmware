// SPDX-License-Identifier: Apache-2.0

#include "hid_hww.h"
#include "usb/usb_processing.h"
#include "usb_desc.h"
#include <queue.h>
#include <string.h>
#include <usb/usb_packet.h>

#define HID_HWW_VERSION 0x00000001u

/**
 * Holds descriptor and endpoint meta-data.
 */
static struct hid_func_data _func_data;

/**
 * The USB device function driver descriptor contains the control callback
 * for enabling and disabling the HID HWW endpoints and the descriptor and endpoint
 * meta-data.
 */
static struct usbdf_driver _func_driver;

/**
 * The report descriptors.
 */
static uint8_t _report_descriptor[] = {USB_DESC_HWW_REPORT};

/**
 * The USB device core request handler callback for the HWW interface.
 */
static int32_t _request(uint8_t ep, struct usb_req* req, enum usb_ctrl_stage stage)
{
    return hid_req(&_func_driver, ep, req, stage);
}

/**
 * Holds a pointer to the USB device core request handler callback.
 */
static struct usbdc_handler _request_handler = {NULL, (FUNC_PTR)_request};

/*
 * Flags for communication between main loop and ISR
 */
static volatile bool _send_busy = false;
static volatile bool _has_data = false;
static volatile bool _request_in_flight = false;

// First time this function is called it initiates a transfer. Call it multiple times to poll for
// completion. Once it returns true, there is data in the buffer.
bool hid_hww_read(uint8_t* data)
{
    if (_request_in_flight && _has_data) {
        _request_in_flight = false;
        return true;
    }
    if (_request_in_flight) {
        return false;
    }
    if (hid_read(&_func_data, data, USB_HID_REPORT_OUT_SIZE) == ERR_NONE) {
        _has_data = false;
        _request_in_flight = true;
    }
    return false;
}

/**
 * Sends the next frame, if the USB interface is ready.
 */
bool hid_hww_write_poll(const uint8_t* data)
{
    ASSERT(data);
    if (_send_busy) {
        return false;
    }
    if (hid_write(&_func_data, data, USB_HID_REPORT_OUT_SIZE) == ERR_NONE) {
        _send_busy = true;
        return true;
    }
    return false;
}

/**
 * The callback function is called after usb data has been received (endpoint = OUT).
 */
static uint8_t _rx_cb(const uint8_t ep, const enum usb_xfer_code rc, const uint32_t count)
{
    (void)ep;
    (void)rc;
    (void)count;
    _has_data = true;
    return ERR_NONE;
}

/**
 * Called when a usb frame has been replied to the host via the HWW interface
 * and the device is ready to send the next frame.
 */
static void _tx_cb(void)
{
    _send_busy = false;
}

/**
 * Initializes a HWW HID interface.
 * @param[in] callback The callback that is called upon status update (enabling/disabling or the
 * endpoints).
 */
int32_t hid_hww_init(void (*callback)(void))
{
    _func_data.hid_status_callback = callback;
    _func_data.report_desc = _report_descriptor;
    _func_data.report_desc_len = USB_DESC_HWW_REPORT_LEN;
    _func_driver.func_data = &_func_data;

    return hid_init(&_func_driver, &_request_handler);
}

/**
 * Registers the HID HWW read and write callbacks and start listening for data.
 */
void hid_hww_setup(void)
{
    // RX callback is called when there is data available to read
    hid_hww_register_callback(HID_CB_READ, (FUNC_PTR)_rx_cb);
    // TX callback is called when data has been sent
    hid_hww_register_callback(HID_CB_WRITE, (FUNC_PTR)_tx_cb);
}

/**
 * Deinitializes the HWW HID interface.
 */
int32_t hid_hww_deinit(void)
{
    return hid_deinit(&_func_driver, &_request_handler);
}

/**
 * Returns the endpoint for the given direction.
 * dir == 1: outgoing (host -> BitBox)
 * dir == 0: incoming (BitBox -> host)
 */
uint8_t hid_hww_get_ep(uint8_t dir)
{
    return hid_get_ep(&_func_driver, dir);
}

/**
 * Deinitializes the HWW HID interface.
 */
bool hid_hww_is_enabled(void)
{
    return hid_is_enabled(&_func_data);
}

/**
 * Registers a callback for a given transfer type.
 * @param[in] trans_type The transfer type for which the callback should be registered,
 *            which can be READ, WRITE or SET_REPORT.
 * @param[in] fund The function that is registered as a callback.
 */
int32_t hid_hww_register_callback(enum hid_trans_type trans_type, FUNC_PTR func)
{
    return hid_register_callback(&_func_data, trans_type, func);
}

/**
 * Returns the version of the HWW interface.
 * @param[in] dir The direction of the endpoint:
 *            dir == 1: outgoing (host -> BitBox)
 *            dir == 0: incoming (BitBox -> host)
 */
uint32_t hid_hww_get_version(void)
{
    return HID_HWW_VERSION;
}
