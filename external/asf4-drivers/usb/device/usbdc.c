/**
 * \file
 *
 * \brief USB Device Stack Core Layer Implementation.
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

#include "usbdc.h"

#define USBDC_VERSION 0x00000001u

/**
 * \brief USB Device Core Sof Handler
 */
struct usbdc_sof_handler {
	struct usbdc_sof_handler *next;
	usbdc_sof_cb_t            cb;
};

/**
 * \brief USB Device Core Request Handler
 */
struct usbdc_req_handler {
	struct usbdc_req_handler *next;
	usbdc_req_cb_t            cb;
};

/**
 * \brief USB Device Core Change Handler
 */
struct usbdc_change_handler {
	struct usbdc_change_handler *next;
	usbdc_change_cb_t            cb;
};

/**
 * \brief USB Device Core Handler
 */
struct usbdc_handlers {
	struct list_descriptor sof_list;
	struct list_descriptor req_list;
	struct list_descriptor change_list;
};

/**
 * \brief USB Device Core Driver Structure
 */
struct usbdc_driver {
	/** Pointer to descriptions of descriptors. */
	struct usbdc_descriptors desces;
	/** Callback handlers. */
	struct usbdc_handlers handlers;
	/** list of function drivers. */
	struct list_descriptor func_list;
	/** Control buffer. */
	uint8_t *ctrl_buf;
	/** Device status. */
	uint16_t status;
	/** Device state. */
	uint8_t state;
	/** Configuration value. */
	uint8_t cfg_value;
	/** Control endpoint size. */
	uint8_t ctrl_size;
	/** Alternate interface used map */
	uint8_t ifc_alt_map;
};

/**
 * \brief USB Device Core Driver Instance
 */
static struct usbdc_driver usbdc;

/**
 * \brief Process the GetDeviceDescriptor request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_get_dev_desc(const uint8_t ep, struct usb_req *req)
{
	uint8_t *dev_desc = NULL;
	uint16_t length   = req->wLength;
	if (length > 0x12) {
		length = 0x12;
	}
#if CONF_USBD_HS_SP
	if (usb_d_get_speed() == USB_SPEED_HS && usbdc.desces.hs) {
		dev_desc = usb_find_desc(usbdc.desces.hs->sod, usbdc.desces.hs->eod, USB_DT_DEVICE);
	} else {
		/* Obtain descriptor from FS descriptors */
	}
#endif
	if (!dev_desc) {
		dev_desc = usb_find_desc(usbdc.desces.ls_fs->sod, usbdc.desces.ls_fs->eod, USB_DT_DEVICE);
	}
	if (!dev_desc) {
		return false;
	}
	if (ERR_NONE != usbdc_xfer(ep, dev_desc, length, false)) {
		return false;
	}
	return true;
}

/**
 * \brief Process the GetConfigurationDescriptor request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_get_cfg_desc(const uint8_t ep, struct usb_req *req)
{
	uint8_t *cfg_desc = NULL;
	uint16_t total_len;
	uint16_t length   = req->wLength;
	uint8_t  index    = req->wValue & 0x00FF;
	bool     need_zlp = !(length & (usbdc.ctrl_size - 1));

#if CONF_USBD_HS_SP
	if (usb_d_get_speed() == USB_SPEED_HS && usbdc.desces.hs) {
		cfg_desc = usb_find_cfg_desc(usbdc.desces.hs->sod, usbdc.desces.hs->eod, index + 1);
	} else {
		/* Obtain descriptor from FS descriptors */
	}
#endif
	if (!cfg_desc) {
		cfg_desc = usb_find_cfg_desc(usbdc.desces.ls_fs->sod, usbdc.desces.ls_fs->eod, index + 1);
	}
	if (NULL == cfg_desc) {
		return false;
	}
	total_len = usb_cfg_desc_total_len(cfg_desc);
	if (length <= total_len) {
		need_zlp = false;
	} else {
		length = total_len;
	}
	if (ERR_NONE != usbdc_xfer(ep, cfg_desc, length, need_zlp)) {
		return false;
	}
	return true;
}

/**
 * \brief Process the GetStringDescriptor request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_get_str_desc(const uint8_t ep, struct usb_req *req)
{
	uint8_t *str_desc;
	uint16_t length   = req->wLength;
	uint8_t  index    = req->wValue & 0x00FF;
	bool     need_zlp = !(length & (usbdc.ctrl_size - 1));
	/* All string are in default descriptors block: FS/LS */
	str_desc = usb_find_str_desc(usbdc.desces.ls_fs->sod, usbdc.desces.ls_fs->eod, index);
	if (NULL == str_desc) {
		return false;
	}
	if (length <= str_desc[0]) {
		need_zlp = false;
	} else {
		length = str_desc[0];
	}
	if (ERR_NONE != usbdc_xfer(ep, str_desc, length, need_zlp)) {
		return false;
	}
	return true;
}

#if CONF_USBD_HS_SP
/**
 * \brief Process the GetDeviceQualifierDescriptor request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_get_devqual_desc(const uint8_t ep, struct usb_req *req)
{
	uint8_t *dev_desc = NULL;
	uint16_t length   = req->wLength;
	if (length > 0x12) {
		length = 0x12;
	}
	if (usb_d_get_speed() == USB_SPEED_HS && usbdc.desces.hs) {
		dev_desc = usb_find_desc(usbdc.desces.hs->sod, usbdc.desces.hs->eod, USB_DT_DEVICE_QUALIFIER);
	}
	if (!dev_desc) {
		dev_desc = usb_find_desc(usbdc.desces.ls_fs->sod, usbdc.desces.ls_fs->eod, USB_DT_DEVICE_QUALIFIER);
	}
	if (!dev_desc) {
		return false;
	}
	if (ERR_NONE != usbdc_xfer(ep, dev_desc, length, false)) {
		return false;
	}
	return true;
}

/**
 * \brief Process the GetOtherSpeedConfigurationDescriptor request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_get_othspdcfg_desc(const uint8_t ep, struct usb_req *req)
{
	uint8_t *cfg_desc = NULL;
	uint16_t total_len;
	uint16_t length   = req->wLength;
	uint8_t  index    = req->wValue & 0x00FF;
	bool     need_zlp = !(length & (usbdc.ctrl_size - 1));

	if (usb_d_get_speed() == USB_SPEED_HS && usbdc.desces.hs) {
		cfg_desc = usb_find_othspdcfg_desc(usbdc.desces.hs->sod, usbdc.desces.hs->eod, index + 1);
	} else {
		/* Obtain descriptor from FS descriptors */
	}
	if (!cfg_desc) {
		cfg_desc = usb_find_othspdcfg_desc(usbdc.desces.ls_fs->sod, usbdc.desces.ls_fs->eod, index + 1);
	}
	if (NULL == cfg_desc) {
		return false;
	}
	total_len = usb_cfg_desc_total_len(cfg_desc);
	if (length <= total_len) {
		need_zlp = false;
	} else {
		length = total_len;
	}
	if (ERR_NONE != usbdc_xfer(ep, cfg_desc, length, need_zlp)) {
		return false;
	}
	return true;
}
#endif

/**
 * \brief Process the GetDescriptor request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_get_desc_req(const uint8_t ep, struct usb_req *req)
{
	uint8_t type = (uint8_t)(req->wValue >> 8);
	switch (type) {
	case USB_DT_DEVICE:
		return usbdc_get_dev_desc(ep, req);
	case USB_DT_CONFIG:
		return usbdc_get_cfg_desc(ep, req);
#if CONF_USBD_HS_SP
	case USB_DT_DEVICE_QUALIFIER:
		return usbdc_get_devqual_desc(ep, req);
	case USB_DT_OTHER_SPEED_CONFIG:
		return usbdc_get_othspdcfg_desc(ep, req);
#endif
	case USB_DT_STRING:
		return usbdc_get_str_desc(ep, req);
	default:
		break;
	}
	return false;
}

/**
 * \brief Process the GetStatus request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_get_status_req(const uint8_t ep, const struct usb_req *req)
{
	int32_t st;
	(void)ep;

	switch (req->bmRequestType & USB_REQT_RECIP_MASK) {
	case USB_REQT_RECIP_DEVICE:
	case USB_REQT_RECIP_INTERFACE:
		st = 0;
		break;
	case USB_REQT_RECIP_ENDPOINT:
		st = usb_d_ep_halt(req->wIndex & 0xFF, USB_EP_HALT_GET);
		if (st < 0) {
			return false;
		}
		st = st & 0x1;
		break;
	default:
		return false;
	}
	memcpy(usbdc.ctrl_buf, &st, 2);
	usbdc_xfer(ep, usbdc.ctrl_buf, 2, false);
	return true;
}

/**
 * \brief Process the standard Get Interface
 * \param[in] req Point to usb request struct.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_get_interface(struct usb_req *req)
{
	struct usbdf_driver *func = (struct usbdf_driver *)usbdc.func_list.head;
	int32_t              rc;

	if (!(usbdc.ifc_alt_map & (1 << req->wIndex))) {
		/* Return 0 if alternate is not used */
		usbdc.ctrl_buf[0] = 0;
		usbdc_xfer(0, usbdc.ctrl_buf, 1, false);
		return true;
	}
	/* Check function drivers only if alternate is used */
	while (NULL != func) {
		if (0 > (rc = func->ctrl(func, USBDF_GET_IFACE, req))) {
			func = func->next;
		} else {
			usbdc.ctrl_buf[0] = (uint8_t)rc;
			usbdc_xfer(0, usbdc.ctrl_buf, 1, false);
			return true;
		}
	}
	return false;
}

/**
 * \brief Process the standard Get request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_get_req(const uint8_t ep, struct usb_req *req)
{
	switch (req->bRequest) {
	case USB_REQ_GET_DESC:
		return usbdc_get_desc_req(ep, req);
	case USB_REQ_GET_CONFIG:
		*(uint8_t *)usbdc.ctrl_buf = usbdc.cfg_value;
		usbdc_xfer(ep, usbdc.ctrl_buf, 1, false);
		return true;
	case USB_REQ_GET_STATUS:
		return usbdc_get_status_req(ep, req);
	case USB_REQ_GET_INTERFACE:
		return usbdc_get_interface(req);
	default:
		return false;
	}
}

/**
 * \brief Process the standard ClearFeature request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_clear_ftr_req(const uint8_t ep, const struct usb_req *req)
{
	(void)ep;
	switch (req->bmRequestType & USB_REQT_RECIP_MASK) {
	case USB_REQT_RECIP_ENDPOINT:
		if (req->wLength != 0) {
			return false;
		}
		usb_d_ep_halt(req->wIndex & 0xFF, USB_EP_HALT_CLR);
		usbdc_xfer(ep, NULL, 0, true);
		return true;
	default:
		return false;
	}
}

/**
 * \brief Process the standard SetFeature request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_set_ftr_req(const uint8_t ep, const struct usb_req *req)
{
	(void)ep;
	switch (req->bmRequestType & USB_REQT_RECIP_MASK) {
	case USB_REQT_RECIP_ENDPOINT:
		if (req->wLength != 0) {
			return false;
		}
		usb_d_ep_halt(req->wIndex & 0xFF, USB_EP_HALT_SET);
		usbdc_xfer(ep, NULL, 0, true);
		return true;
	default:
		return false;
	}
}

/**
 * \brief Unconfig, close all interfaces
 */
static void usbdc_unconfig(void)
{
	struct usbdf_driver *func = (struct usbdf_driver *)usbdc.func_list.head;
	while (NULL != func) {
		func->ctrl(func, USBDF_DISABLE, NULL);
		func = func->next;
	}
}

/**
 * \brief Apply Set Configuration Value
 * \param[in] cfg_value Configuration Value
 * \retval true Set configuration OK.
 * \retval false Request error.
 */
static bool usbdc_set_config(uint8_t cfg_value)
{
	struct usbd_descriptors desc;
	struct usbdf_driver *   func;
	uint8_t *               cfg_desc = NULL;
	uint16_t                total_len;
	uint8_t                 last_iface = 0xFF;

	if (cfg_value == 0) {
		usbdc_unconfig();
		return true;
	}

#if CONF_USBD_HS_SP
	if (usb_d_get_speed() == USB_SPEED_HS && usbdc.desces.hs) {
		cfg_desc = usb_find_cfg_desc(usbdc.desces.hs->sod, usbdc.desces.hs->eod, cfg_value);
	} else {
		/* Obtain descriptor from FS descriptors */
	}
#endif
	if (!cfg_desc) {
		cfg_desc = usb_find_cfg_desc(usbdc.desces.ls_fs->sod, usbdc.desces.ls_fs->eod, cfg_value);
	}
	if (NULL == cfg_desc) {
		return false;
	}

	total_len = usb_cfg_desc_total_len(cfg_desc);
	desc.eod  = cfg_desc + total_len;
	desc.sod  = usb_find_desc(cfg_desc, desc.eod, USB_DT_INTERFACE);

	while (NULL != desc.sod) {
		/* Apply very first alternate setting (must be 0) of the interface */
		if (last_iface != desc.sod[2] /* bInterfaceNumber */) {
			last_iface = desc.sod[2];
			func       = (struct usbdf_driver *)usbdc.func_list.head;
			while (NULL != func) {
				if (func->ctrl(func, USBDF_ENABLE, &desc)) {
					func = func->next;
				} else {
					break;
				}
			}
		}
		desc.sod = usb_desc_next(desc.sod);
		desc.sod = usb_find_desc(desc.sod, desc.eod, USB_DT_INTERFACE);
	}
	return true;
}

/**
 * \brief Apply the USB device address
 * \param[in] addr address to be set.
 */
static void usbdc_set_address(uint8_t addr)
{
	usb_d_set_address(addr);
}

/**
 * \brief Process the standard Set Interface
 * \param[in] alt_set Alternate Setting.
 * \param[in] ifc_id Interface Index.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_set_interface(uint16_t alt_set, uint16_t ifc_id)
{
	struct usbd_descriptors desc;
	struct usbdf_driver *   func;
	uint8_t *               ifc = NULL;

#if CONF_USBD_HS_SP
	if (usb_d_get_speed() == USB_SPEED_HS && usbdc.desces.hs) {
		ifc = usb_find_cfg_desc(usbdc.desces.hs->sod, usbdc.desces.hs->eod, usbdc.cfg_value);
	} else {
		/* Obtain descriptor from FS descriptors */
	}
#endif
	if (!ifc) {
		ifc = usb_find_cfg_desc(usbdc.desces.ls_fs->sod, usbdc.desces.ls_fs->eod, usbdc.cfg_value);
	}
	if (NULL == ifc) {
		return false;
	}
	desc.sod = ifc;
	desc.eod = ifc + usb_cfg_desc_total_len(ifc);

	if (NULL == (ifc = usb_find_desc(desc.sod, desc.eod, USB_DT_INTERFACE))) {
		return false;
	}

	while (ifc[2] != ifc_id || ifc[3] != alt_set) {
		desc.sod = usb_desc_next(desc.sod);
		ifc      = usb_find_desc(desc.sod, desc.eod, USB_DT_INTERFACE);
		if (NULL == ifc) {
			return false;
		}
	}

	desc.sod = ifc;
	func     = (struct usbdf_driver *)usbdc.func_list.head;

	while (NULL != func) {
		if (func->ctrl(func, USBDF_DISABLE, &desc)) {
			func = func->next;
		} else if (ERR_NONE == func->ctrl(func, USBDF_ENABLE, &desc)) {
			if (alt_set) {
				/* Alternate settings are used from now on */
				usbdc.ifc_alt_map |= 1 << ifc_id;
			}
			usbdc_xfer(0, NULL, 0, 0);
			return true;
		} else {
			return false;
		}
	}

	return false;
}

/**
 * \brief Process the standard Set request
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_set_req(const uint8_t ep, struct usb_req *req)
{
	switch (req->bRequest) {
	case USB_REQ_SET_ADDRESS:
		return (ERR_NONE == usbdc_xfer(ep, NULL, 0, true));
	case USB_REQ_SET_CONFIG:
		if (!usbdc_set_config(req->wValue)) {
			return false;
		}
		return (ERR_NONE == usbdc_xfer(ep, NULL, 0, true));
	case USB_REQ_CLEAR_FTR:
		return usbdc_clear_ftr_req(ep, req);
	case USB_REQ_SET_FTR:
		return usbdc_set_ftr_req(ep, req);
	case USB_REQ_SET_INTERFACE:
		return usbdc_set_interface(req->wValue, req->wIndex);
	default:
		return false;
	}
}

/** Invoke all registered SOF callbacks. */
static void usbdc_sof_notify(void)
{
	struct usbdc_sof_handler *sof = (struct usbdc_sof_handler *)usbdc.handlers.sof_list.head;

	while (sof != NULL) {
		if (NULL != sof->cb) {
			sof->cb();
		}
		sof = sof->next;
	}
}

/** Invoke all registered Change notification callbacks. */
static void usbdc_change_notify(enum usbdc_change_type change, uint32_t value)
{
	struct usbdc_change_handler *cg = (struct usbdc_change_handler *)usbdc.handlers.change_list.head;

	while (cg != NULL) {
		if (NULL != cg->cb) {
			cg->cb(change, value);
		}
		cg = cg->next;
	}
}

/** Invoke all registered request callbacks until request handled. */
static int32_t usbdc_request_handler(uint8_t ep, struct usb_req *req, enum usb_ctrl_stage stage)
{
	struct usbdc_req_handler *h = (struct usbdc_req_handler *)usbdc.handlers.req_list.head;
	int32_t                   rc;

	while (h != NULL) {
		if (NULL != h->cb) {
			rc = h->cb(ep, req, stage);
			if (0 == rc) {
				return true;
			} else if (ERR_NOT_FOUND != rc) {
				return -1;
			}
		}
		h = h->next;
	}
	return false;
}

/**
 * \brief Callback invoked on USB device SOF
 */
static void usbd_sof_cb(void)
{
	usbdc_sof_notify();
}

/**
 * \brief Callback invoked when control request is received
 * \param[in] ep Endpoint address.
 * \param[in] req Pointer to the request.
 * \return Operation status.
 * \retval true Request is handled OK.
 * \retval false Request not supported.
 */
static bool usbdc_cb_ctl_req(const uint8_t ep, struct usb_req *req)
{
	switch (usbdc_request_handler(ep, req, USB_SETUP_STAGE)) {
	case true:
		return true;
	case -1:
		return false;
	default:
		break;
	}

	// STD request handling
	switch (req->bmRequestType & (USB_REQT_TYPE_MASK | USB_REQT_DIR_IN)) {
	case USB_REQT_TYPE_STANDARD:
		return usbdc_set_req(ep, req);
	case (USB_REQT_TYPE_STANDARD | USB_REQT_DIR_IN):
		return usbdc_get_req(ep, req);
	default:
		return false;
	}
}

/**
 * \brief When control status stage is end
 * \param[in] req Pointer to the request.
 */
static void usbdc_ctrl_status_end(const struct usb_req *req)
{
	if (req->bmRequestType != USB_REQT_TYPE_STANDARD) {
		return;
	}
	switch (req->bRequest) {
	case USB_REQ_SET_CONFIG:
		usbdc.cfg_value = req->wValue;
		usbdc.state     = req->wValue ? USBD_S_CONFIG : USBD_S_ADDRESS;
		usbdc_change_notify(USBDC_C_STATE, usbdc.state);
		break;
	case USB_REQ_SET_ADDRESS:
		usbdc_set_address(req->wValue);
		usbdc.state = req->wValue ? USBD_S_ADDRESS : USBD_S_DEFAULT;
		usbdc_change_notify(USBDC_C_STATE, usbdc.state);
		break;
	default:
		break;
	}
}

/**
 * \brief When control data stage is end
 * \param[in] req Pointer to the request.
 */
static bool usbdc_ctrl_data_end(struct usb_req *req)
{
	usbdc_request_handler(0, req, USB_DATA_STAGE);
	return false;
}

/**
 * \brief Callback invoked when control data done or status done
 * \param[in] ep Endpoint number with direction on bit 8.
 * \param[in] code Status code.
 * \param[in] req Pointer to the control setup request.
 * \return Data has error or not.
 * \retval true There is data error, protocol error.
 * \retval false There is no data error.
 */
static bool usbdc_cb_ctl_done(const uint8_t ep, const enum usb_xfer_code code, struct usb_req *req)
{
	(void)ep;

	switch (code) {
	case USB_XFER_DONE:
		usbdc_ctrl_status_end(req);
		break;
	case USB_XFER_DATA:
		return usbdc_ctrl_data_end(req);
	default:
		break;
	}
	return false;
}

/**
 * \brief USB Device Core Reset
 */
void usbdc_reset(void)
{
	usbdc_unconfig();

	usbdc.state       = USBD_S_DEFAULT;
	usbdc.cfg_value   = 0;
	usbdc.ifc_alt_map = 0;

	// Setup EP0
	usb_d_ep_deinit(0);
	usb_d_ep0_init(usbdc.ctrl_size);
	usb_d_ep_register_callback(0, USB_D_EP_CB_SETUP, (FUNC_PTR)usbdc_cb_ctl_req);
	usb_d_ep_register_callback(0, USB_D_EP_CB_XFER, (FUNC_PTR)usbdc_cb_ctl_done);
	usb_d_ep_enable(0);
}

/**
 * \brief Callback invoked on USB device events
 * \param[in] ev Event code.
 * \param[in] param Event parameter for event handling.
 */
static void usbd_event_cb(const enum usb_event ev, const uint32_t param)
{
	(void)param;

	switch (ev) {
	case USB_EV_VBUS:
		usbdc_change_notify(USBDC_C_CONN, param);
		break;

	case USB_EV_RESET:
		usbdc_reset();
		break;

	default:
		break;
	}
}

/**
 * \brief Issue USB device transfer
 */
int32_t usbdc_xfer(uint8_t ep, uint8_t *buf, uint32_t size, bool zlp)
{
	struct usb_d_transfer xfer = {(uint8_t *)buf, size, ep, zlp};
	return usb_d_ep_transfer(&xfer);
}

/**
 * \brief Register the handler
 */
void usbdc_register_handler(enum usbdc_handler_type type, const struct usbdc_handler *h)
{
	switch (type) {
	case USBDC_HDL_SOF:
		list_insert_at_end(&usbdc.handlers.sof_list, (void *)h);
		break;
	case USBDC_HDL_REQ:
		list_insert_at_end(&usbdc.handlers.req_list, (void *)h);
		break;
	case USBDC_HDL_CHANGE:
		list_insert_at_end(&usbdc.handlers.change_list, (void *)h);
		break;
	default:
		break;
	}
}

/**
 * \brief Unregister the handler
 */
void usbdc_unregister_handler(enum usbdc_handler_type type, const struct usbdc_handler *h)
{
	switch (type) {
	case USBDC_HDL_SOF:
		list_delete_element(&usbdc.handlers.sof_list, (void *)h);
		break;
	case USBDC_HDL_REQ:
		list_delete_element(&usbdc.handlers.req_list, (void *)h);
		break;
	case USBDC_HDL_CHANGE:
		list_delete_element(&usbdc.handlers.change_list, (void *)h);
		break;
	default:
		break;
	}
}

/**
 * \brief Initialize the USB device core driver
 */
int32_t usbdc_init(uint8_t *ctrl_buf)
{
	ASSERT(ctrl_buf);

	int32_t rc;

	rc = usb_d_init();
	if (rc < 0) {
		return rc;
	}

	memset(&usbdc, 0, sizeof(usbdc));
	usbdc.ctrl_buf = ctrl_buf;
	usb_d_register_callback(USB_D_CB_SOF, (FUNC_PTR)usbd_sof_cb);
	usb_d_register_callback(USB_D_CB_EVENT, (FUNC_PTR)usbd_event_cb);

	return 0;
}

/**
 * \brief De-initialize the USB device core driver
 */
int32_t usbdc_deinit(void)
{
	usb_d_deinit();
	return 0;
}

/**
 * \brief Register/unregister function support of a USB device function
 *
 * Must be invoked when USB device is stopped.
 */
void usbdc_register_function(struct usbdf_driver *func)
{
	list_insert_at_end(&usbdc.func_list, func);
}

/**
 * \brief Unregister function support of a USB device function
 *
 * Must be invoked when USB device is stopped.
 */
void usbdc_unregister_function(struct usbdf_driver *func)
{
	list_delete_element(&usbdc.func_list, func);
}

/**
 * \brief Validate the descriptor
 */
int32_t usbdc_validate_desces(struct usbd_descriptors *desces)
{
	uint8_t *sod, *eod;
	if (desces == NULL) {
		return ERR_NOT_FOUND;
	}
	sod = usb_find_desc(desces->sod, desces->eod, USB_DT_DEVICE);
	if (sod == NULL) {
		return ERR_BAD_DATA;
	}
	sod = usb_find_desc(desces->sod, desces->eod, USB_DT_CONFIG);
	if (sod == NULL) {
		return ERR_BAD_DATA;
	}
	eod = sod + usb_cfg_desc_total_len(sod);
	if (eod > desces->eod) {
		return ERR_BAD_DATA;
	}
	return 0;
}

/**
 * \brief Validate the descriptor
 */
int32_t usbdc_check_desces(struct usbdc_descriptors *desces)
{
#if CONF_USBD_HS_SP
	int32_t rc;
	if (desces->hs == NULL && desces->ls_fs == NULL) {
		return ERR_NOT_FOUND;
	}
	if (desces->hs) {
		rc = usbdc_validate_desces(desces->hs);
		if (rc < 0) {
			return rc;
		}
	}
#endif
	return usbdc_validate_desces(desces->ls_fs);
}

/**
 * \brief Start the USB device driver with specific descriptors set
 */
int32_t usbdc_start(struct usbd_descriptors *desces)
{
	if (usbdc.state >= USBD_S_POWER) {
		return ERR_BUSY;
	}

	if (desces) {
		usbdc.desces.ls_fs = desces;
#if CONF_USBD_HS_SP
		usbdc.desces.hs = &desces[1];
#endif
	} else {
		return ERR_BAD_DATA;
	}

	usbdc.ctrl_size = desces->sod[7];
	usbdc.state     = USBD_S_POWER;
	usb_d_enable();
	return ERR_NONE;
}

/**
 * \brief Stop the USB device driver
 */
int32_t usbdc_stop(void)
{
	usb_d_disable();
	usbdc.state = USBD_S_OFF;
	return ERR_NONE;
}

/**
 * \brief Attach the USB device to host
 */
void usbdc_attach(void)
{
	usb_d_attach();
}

/**
 * \brief Detach the USB device from host
 */
void usbdc_detach(void)
{
	usb_d_detach();
}

/**
 * \brief Send remote wakeup to host
 */
void usbdc_remotewakeup(void)
{
	usb_d_send_remotewakeup();
	usbdc.state = USBD_S_POWER;
}

/**
 * \brief Return USB Device endpoint0 buffer
 */
uint8_t *usbdc_get_ctrl_buffer(void)
{
	return usbdc.ctrl_buf;
}

/**
 * \brief Return current USB state
 */
uint8_t usbdc_get_state(void)
{
	if (usbdc.state & USBD_S_SUSPEND) {
		return USBD_S_SUSPEND;
	}
	return usbdc.state;
}

/**
 * \brief Return version
 */
uint32_t usbdc_get_version(void)
{
	return USBDC_VERSION;
}
