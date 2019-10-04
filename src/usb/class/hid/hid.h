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

#ifndef _HID_H_
#define _HID_H_

#include <stdbool.h>
#include <stdint.h>

#if !defined(TESTING)
#include "usb_protocol.h"
#include "usb_protocol_hid.h"
#include "usbdc.h"
#else
struct usbdc_handler;
struct usbdf_handler;
struct usbdf_driver;
typedef void (*FUNC_PTR)(void);
#endif

/* endpoint direction */
#define DIR_OUT 1
#define DIR_IN 0

/**
 * The callback that is called when a set report USB request
 * is received. Can be set with hid_register_callback via hid_trans_type=HID_CB_SET_REPORT.
 */
typedef bool (*hid_set_report_t)(uint8_t*, uint16_t);

/**
 * Holds descriptor and endpoint meta-data.
 */
struct hid_func_data {
    uint8_t* hid_desc;
    uint8_t* report_desc;
    uint32_t report_desc_len;
    uint8_t func_iface;
    uint8_t func_ep_in;
    uint8_t func_ep_out;
    uint8_t protocol;
    bool enabled;
    void (*hid_status_callback)(void);
    hid_set_report_t hid_set_report;
};

/**
 * The transfer type that distinguises whether a callback should be
 * registered for read, write or set report transfers.
 */
enum hid_trans_type { HID_CB_READ, HID_CB_WRITE, HID_CB_SET_REPORT };

/**
 * Initializes a HID interface.
 * @param[in] func_driver The driver data that contains the function data.
 * @param[in] hid_req_h The USB device core request handler.
 */
int32_t hid_init(struct usbdf_driver* func_driver, struct usbdc_handler* hid_req_h);

/**
 * Deinitializes the function driver.
 * @param[in] func_driver The driver data that contains the function data.
 */
int32_t hid_deinit(struct usbdf_driver* func_driver, struct usbdc_handler* hid_req_h);

/**
 * Checks whether the interface is enabled.
 * @param[in] func_driver The driver data that contains the function data.
 */
bool hid_is_enabled(struct hid_func_data* func_data);

/**
 * Sets the buffer address for the incoming endpoint.
 * @param[IN] func_data The interface meta data.
 * @param[OUT] buf The address of the buffer to which we write.
 * @param[IN] size The size of the buffer.
 */
int32_t hid_read(struct hid_func_data* func_data, uint8_t* buf, uint32_t size);

/**
 * Sets the buffer address for the outgoing endpoint.
 * @param[IN] func_data The interface meta data.
 * @param[IN] buf The address of the buffer from which we read.
 * @param[IN] size The size of the buffer.
 */
int32_t hid_write(struct hid_func_data* func_data, const uint8_t* buf, uint32_t size);

/**
 * Registers a callback for a given transfer type.
 * @param[IN] func_data The interface meta data.
 * @param[in] trans_type The transfer type for which the callback should be registered,
 *            which can be READ, WRITE or SET_REPORT.
 * @param[in] func The function that is registered as a callback.
 */
int32_t hid_register_callback(
    struct hid_func_data* func_data,
    enum hid_trans_type trans_type,
    FUNC_PTR func);

/**
 * Returns the endpoint for the given direction.
 * @param[in] func_driver The driver data that contains the function data.
 * @param[in] dir The direction of the endpoint:
 *            dir == 1: outgoing (host -> BitBox)
 *            dir == 0: incoming (BitBox -> host)
 */
uint8_t hid_get_ep(struct usbdf_driver* func_driver, uint8_t dir);

/**
 * The USB device core request handler callback.
 * @param[out] drv The driver is reset to a state that indicates that the interface is disabled.
 * @param[in] ep The endpoint.
 * @param[in] req The usb request.
 * @param[in] stage The usb control stage.
 */
int32_t hid_req(
    struct usbdf_driver* drv,
    uint8_t ep,
    struct usb_req* req,
    enum usb_ctrl_stage stage);

/**
 * The control callback that is called to enable or disable the interface.
 * @param[out] drv The driver is reset to a state that indicates that the interface is disabled.
 * @param[in] ctrl The control flag which indicates which action to take.
 * @param[in] param Additional parameters passed to the callback.
 */
// int32_t hid_ctrl(struct usbdf_driver *drv, enum usbdf_control ctrl, void *param);

#endif
