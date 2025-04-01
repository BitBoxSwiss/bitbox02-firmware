/**
 * \file
 *
 * \brief SAM USB device HAL
 *
 * Copyright (c) 2015-2018 Microchip Technology Inc. and its subsidiaries.
 *
 * \asf_license_start
 *
 * \page License
 *
 * Subject to your compliance with these terms, you may use Microchip
 * software and any derivatives exclusively with Microchip products.
 * It is your responsibility to comply with third party license terms applicable
 * to your use of third party software (including open source software) that
 * may accompany Microchip software.
 *
 * THIS SOFTWARE IS SUPPLIED BY MICROCHIP "AS IS". NO WARRANTIES,
 * WHETHER EXPRESS, IMPLIED OR STATUTORY, APPLY TO THIS SOFTWARE,
 * INCLUDING ANY IMPLIED WARRANTIES OF NON-INFRINGEMENT, MERCHANTABILITY,
 * AND FITNESS FOR A PARTICULAR PURPOSE. IN NO EVENT WILL MICROCHIP BE
 * LIABLE FOR ANY INDIRECT, SPECIAL, PUNITIVE, INCIDENTAL OR CONSEQUENTIAL
 * LOSS, DAMAGE, COST OR EXPENSE OF ANY KIND WHATSOEVER RELATED TO THE
 * SOFTWARE, HOWEVER CAUSED, EVEN IF MICROCHIP HAS BEEN ADVISED OF THE
 * POSSIBILITY OR THE DAMAGES ARE FORESEEABLE.  TO THE FULLEST EXTENT
 * ALLOWED BY LAW, MICROCHIP'S TOTAL LIABILITY ON ALL CLAIMS IN ANY WAY
 * RELATED TO THIS SOFTWARE WILL NOT EXCEED THE AMOUNT OF FEES, IF ANY,
 * THAT YOU HAVE PAID DIRECTLY TO MICROCHIP FOR THIS SOFTWARE.
 *
 * \asf_license_stop
 *
 */

#ifndef _HAL_USB_DEVICE_H_INCLUDED
#define _HAL_USB_DEVICE_H_INCLUDED

#include <hpl_usb_device.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup doc_driver_hal_usb_device
 *
 * @{
 */

/** USB device endpoint status structure. */
struct usb_d_ep_status {
	/** Endpoint address, including direction. */
	uint8_t ep;
	/** Endpoint transfer status code that triggers the callback.
	 *  \ref usb_xfer_code. */
	uint8_t code;
	/** Endpoint error, if \c code is \ref USB_TRANS_ERROR. */
	uint8_t error;
	/** Transfer state, \ref usb_ep_state. */
	uint8_t state;
	/** Transfer count. */
	uint32_t count;
	/** Transfer size. */
	uint32_t size;
};

/** Prototype function for callback that is invoked on USB device SOF. */
typedef void (*usb_d_sof_cb_t)(void);

/** Prototype function for callback that is invoked on USB device events. */
typedef void (*usb_d_event_cb_t)(const enum usb_event event, const uint32_t param);

/** USB device callbacks. */
struct usb_d_callbacks {
	/** Callback that is invoked on SOF. */
	usb_d_sof_cb_t sof;
	/** Callback that is invoked on USB RESET/WAKEUP/RESUME/SUSPEND. */
	usb_d_event_cb_t event;
};

/** Callback that is invoked when setup packet is received.
 *  Return \c true if request has been handled, or control endpoint will
 *  stall IN/OUT transactions.
 */
typedef bool (*usb_d_ep_cb_setup_t)(const uint8_t ep, const uint8_t *req);

/** Callback that is invoked when buffer is done without error, but last packet
 *  is full size packet without ZLP.
 *  Return \c true if more data has been requested.
 */
typedef bool (*usb_d_ep_cb_more_t)(const uint8_t ep, const uint32_t count);

/** Callback that is invoked when all data is finished, including background
 *  transfer, or error happens.
 *  In control transfer data stage, return value is checked,
 *  return \c false if no error happens.
 */
typedef bool (*usb_d_ep_cb_xfer_t)(const uint8_t ep, const enum usb_xfer_code code, void *param);

/**
 *  \brief Initialize the USB device driver
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t usb_d_init(void);

/**
 *  \brief Deinitialize the USB device driver
 */
void usb_d_deinit(void);

/**
 *  \brief Register the USB device callback
 *  \param[in] type The callback type to register.
 *  \param[in] func The callback function, NULL to disable callback.
 */
void usb_d_register_callback(const enum usb_d_cb_type type, const FUNC_PTR func);

/**
 *  \brief Enable the USB device driver
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t usb_d_enable(void);

/**
 *  \brief Disable the USB device driver
 */
void usb_d_disable(void);

/**
 *  \brief Attach the USB device
 */
void usb_d_attach(void);

/**
 *  \brief Detach the USB device
 */
void usb_d_detach(void);

/**
 *  \brief Retrieve current USB working speed.
 *  \return USB Speed. See \ref usb_speed.
 */
enum usb_speed usb_d_get_speed(void);

/**
 *  \brief Retrieve current USB frame number.
 *  \return Frame number.
 */
uint16_t usb_d_get_frame_num(void);

/**
 *  \brief Retrieve current USB micro frame number.
 *  \return Micro frame number inside a frame (0~7).
 *          0 if not available (not HS).
 */
uint8_t usb_d_get_uframe_num(void);

/**
 *  \brief Set the USB address that is used.
 *  \param[in] addr The address to set.
 */
void usb_d_set_address(const uint8_t addr);

/**
 *  \brief Send remote wakeup to host
 *  \return Operation status.
 */
void usb_d_send_remotewakeup(void);

/**
 *  \brief Initialize the endpoint 0.
 *
 *  Note that endpoint 0 must be initialized as control endpoint.
 *
 *  \param[in] max_pkt_size Max. packet size of EP0.
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t usb_d_ep0_init(const uint8_t max_pkt_size);

/**
 *  \brief Initialize the endpoint.
 *
 *  \param[in] ep The endpoint address.
 *  \param[in] attr The endpoint attributes.
 *  \param[in] max_pkt_size Max. packet size of EP0.
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t usb_d_ep_init(const uint8_t ep, const uint8_t attr, const uint16_t max_pkt_size);

/**
 *  \brief Disable and deinitialize the endpoint.
 *  \param[in] ep The endpoint address to deinitialize.
 */
void usb_d_ep_deinit(const uint8_t ep);

/**
 *  \brief Register the USB device endpoint callback on initialized endpoint.
 *
 *  \param[in] ep The endpoint address.
 *  \param[in] type The callback type to register.
 *  \param[in] func The callback function, NULL to disable callback.
 */
void usb_d_ep_register_callback(const uint8_t ep, const enum usb_d_ep_cb_type type, const FUNC_PTR func);

/**
 *  \brief Enabled the initialized endpoint.
 *
 *  Setup request will be monitored after enabling a control endpoint.
 *
 *  \param[in] ep The endpoint address.
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t usb_d_ep_enable(const uint8_t ep);

/**
 *  \brief Disable the initialized endpoint.
 *  \param[in] ep The endpoint address.
 */
void usb_d_ep_disable(const uint8_t ep);

/**
 *  \brief Get request data pointer to access received setup request packet
 *  \param[in] ep The endpoint address.
 *  \return Pointer to the request data.
 *  \retval NULL The endpoint is not a control endpoint.
 */
uint8_t *usb_d_ep_get_req(const uint8_t ep);

/**
 *  \brief Endpoint transfer.
 *
 *  For control endpoints, start the transfer according to the direction in the bmRequest
 *  type, and finish with STATUS stage.
 *  For non-control endpoints, the transfer will be unique direction. Defined by
 *  bit 8 of the endpoint address.
 *
 *  \param[in] xfer Pointer to the transfer description.
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t usb_d_ep_transfer(const struct usb_d_transfer *xfer);

/**
 *  \brief Abort an on-going transfer on a specific endpoint.
 *
 *  \param[in] ep The endpoint address.
 */
void usb_d_ep_abort(const uint8_t ep);

/**
 *  \brief Retrieve the endpoint status.
 *
 *  \param[in] ep The endpoint address.
 *  \param[out] stat Pointer to the buffer to fill the status description.
 *
 *  \return Endpoint status.
 *  \retval 1 Busy.
 *  \retval 0 Idle.
 *  \retval <0 Error code.
 */
int32_t usb_d_ep_get_status(const uint8_t ep, struct usb_d_ep_status *stat);

/**
 *  \brief Endpoint halt control.
 *
 *  \param[in] ep The endpoint address.
 *  \param[in] ctrl Control code (SET/CLEAR/GET).
 *
 *  \return Operation status or HALT state (if \c ctrl is \ref USB_EP_HALT_GET).
 */
int32_t usb_d_ep_halt(const uint8_t ep, const enum usb_ep_halt_ctrl ctrl);

/** \brief Retrieve the current driver version
 *
 *  \return Current driver version.
 */
uint32_t usb_d_get_version(void);

/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* _HAL_USB_DEVICE_H_INCLUDED */
