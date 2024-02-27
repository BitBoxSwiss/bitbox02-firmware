#include "iap2/iap2.h"
#include <stdint.h>
#include "util.h"
#include "usb/usb_protocol_iap2.h"

struct iap2_func_data {
    // Store the interface number. Used to check if the device support already is initialized
    uint8_t func_iface;

    // Store in endpoint addr
    uint8_t func_ep_in;

    // Store out endpoint addr
    uint8_t func_ep_out;

    // Enabled or disabled
    bool enabled;
};

/**
 * IAP2 Device function instance
 */
static struct usbdf_driver _iap2;

/**
 * IAP2 Device function data instance
 */
static struct iap2_func_data _iap2_func_data;

/**
 * Enables the endpoints for the interface described in the given descriptor.
 * @param desc The descriptor contains information about the interface configuration.
 * @param[out] drv The driver is filled with meta-data about the interface endpoints.
 */
static int32_t _enable(struct usbd_descriptors* desc, struct usbdf_driver* drv)
{
    uint8_t *ifc, *ep, i;
    usb_iface_desc_t ifc_desc;
    usb_ep_desc_t ep_desc;

    struct iap2_func_data* func_data = (struct iap2_func_data*)(drv->func_data);

    ifc = desc->sod;
    if (NULL == ifc) {
        return ERR_NOT_FOUND;
    }

    ifc_desc.bInterfaceNumber = ifc[2];
    ifc_desc.bInterfaceClass = ifc[5];
    ifc_desc.bInterfaceSubClass = ifc[6];

    // Check if it is the vendor interface class and iap2 sub class
    if (IAP2_CLASS == ifc_desc.bInterfaceClass && IAP2_SUB_CLASS == ifc_desc.bInterfaceSubClass) {
        if (func_data->func_iface == ifc_desc.bInterfaceNumber) { // Initialized
            return ERR_ALREADY_INITIALIZED;
        }
        if (func_data->func_iface != 0xFF) { // Occupied
            return ERR_NO_RESOURCE;
        }
        func_data->func_iface = ifc_desc.bInterfaceNumber;
    } else { // Not supported by this function driver
        return ERR_NOT_FOUND;
    }

    // Install endpoints
    for (i = 0; i < 2; i++) {
        ep = usb_find_ep_desc(usb_desc_next(desc->sod), desc->eod);
        desc->sod = ep;
        if (NULL != ep) {
            ep_desc.bEndpointAddress = ep[2];
            traceln("Found endpoint 0x%02x for interface",ep_desc.bEndpointAddress);
            ep_desc.bmAttributes = ep[3];
            ep_desc.wMaxPacketSize = usb_get_u16(ep + 4);
            if (usb_d_ep_init(
                    ep_desc.bEndpointAddress, ep_desc.bmAttributes, ep_desc.wMaxPacketSize)) {
                return ERR_NOT_INITIALIZED;
            }
            if (ep_desc.bEndpointAddress & USB_EP_DIR_IN) {
                func_data->func_ep_in = ep_desc.bEndpointAddress;
                usb_d_ep_enable(func_data->func_ep_in);
            } else {
                func_data->func_ep_out = ep_desc.bEndpointAddress;
                usb_d_ep_enable(func_data->func_ep_out);
            }
        } else {
            return ERR_NOT_FOUND;
        }
    }

    // Installed
    func_data->enabled = true;

    return ERR_NONE;
}

/**
 * Disables the endpoints for the interface described in the given descriptor.
 * @param[in] desc The descriptor contains information about the interface configuration.
 * @param[out] drv The driver is reset to a state that indicates that the interface is disabled.
 */
static int32_t _disable(const struct usbd_descriptors* desc, struct usbdf_driver* drv)
{
    struct iap2_func_data* func_data = (struct iap2_func_data*)(drv->func_data);

    usb_iface_desc_t ifc_desc;

    if (desc) {
        ifc_desc.bInterfaceClass = desc->sod[5];
        ifc_desc.bInterfaceSubClass = desc->sod[6];
        if (ifc_desc.bInterfaceClass != IAP2_CLASS || ifc_desc.bInterfaceSubClass != IAP2_SUB_CLASS) {
            return ERR_NOT_FOUND;
        }
    }

    if (func_data->func_iface != 0xFF) {
        func_data->func_iface = 0xFF;
    }

    if (func_data->func_ep_in != 0xFF) {
        usb_d_ep_deinit(func_data->func_ep_in);
        func_data->func_ep_in = 0xFF;
    }

    if (func_data->func_ep_out != 0xFF) {
        usb_d_ep_deinit(func_data->func_ep_out);
        func_data->func_ep_out = 0xFF;
    }

    func_data->enabled = false;
    return ERR_NONE;
}

/**
 * The control callback that is called to enable or disable the interface.
 * @param[out] drv The driver is reset to a state that indicates that the interface is disabled.
 * @param[in] ctrl The control flag which indicates which action to take.
 * @param[in] param Additional parameters passed to the callback.
 */
static int32_t _ctrl(struct usbdf_driver* drv, enum usbdf_control ctrl, void* param)
{
    traceln("%s %u", "got here", ctrl);
    switch (ctrl) {
    case USBDF_ENABLE:
        return _enable((struct usbd_descriptors*)param, drv);

    case USBDF_DISABLE:
        return _disable((const struct usbd_descriptors*)param, drv);

    case USBDF_GET_IFACE:
        return ERR_UNSUPPORTED_OP;

    default:
        return ERR_INVALID_ARG;
    }
}

//static int _req_handler(
//    struct usbdf_driver* drv,
//    uint8_t ep,
//    struct usb_req* req,
//    enum usb_ctrl_stage stage)
//{
//    (void) drv;
//    (void) ep;
//    (void) req;
//    (void) stage;
//    //struct hid_func_data* func_data = (struct hid_func_data*)(drv->func_data);
//    //uint8_t* ctrl_buf = usbdc_get_ctrl_buffer();
//    //uint16_t len = req->L.wLength;
//
//    traceln("iap2 request type: 0x%02x request 0x%02x iface: 0x%04x 0x%04x", req->bmRequestType, req->bRequest, req->I.wIndex, _iap2_func_data.func_iface);
//    //for(int i=0; i<len; ++i) {
//    //    trace("%02x", req->V.wValue);
//    //}
//    //printf("\n");
//    return ERR_NOT_FOUND;
//
//    //if ((0x81 == req->bmRequestType) && (0x06 == req->bRequest) &&
//    //    (req->I.wIndex == _iap2_func_data->func_iface)) {
//    //    return _get_descriptor(drv, ep, req);
//    //}
//    //if (0x01 != ((req->bmRequestType >> 5) & 0x03)) { // class request
//    //    return ERR_NOT_FOUND;
//    //}
//    //if (req->I.wIndex == _iap2_func_data.func_iface) {
//    //    if (req->bmRequestType & USB_EP_DIR_IN) {
//    //        return ERR_INVALID_ARG;
//    //    }
//    //    switch (req->bRequest) {
//    //    case 0x03: /* Get Protocol */
//    //        return usbdc_xfer(ep, &_iap2_func_data->protocol, 1, 0);
//    //    case 0x0B: /* Set Protocol */
//    //        func_data->protocol = req->V.wValue;
//    //        return usbdc_xfer(ep, NULL, 0, 0);
//    //    case USB_REQ_HID_SET_REPORT:
//    //        if (USB_SETUP_STAGE == stage) {
//    //            return usbdc_xfer(ep, ctrl_buf, len, false);
//    //        } else {
//    //            if (NULL != func_data->hid_set_report) {
//    //                func_data->hid_set_report(ctrl_buf, len);
//    //            }
//    //            return ERR_NONE;
//    //        }
//    //    default:
//    //        return ERR_INVALID_ARG;
//    //    }
//    //} else {
//    //    return ERR_NOT_FOUND;
//    //}
//}

//static struct usbdc_handler _iap2_req_handler = {NULL, (FUNC_PTR) _req_handler};

int32_t iap2_init(void) {
    if(usbdc_get_state() > USBD_S_POWER) {
        return ERR_DENIED;
    }

    _iap2.ctrl = _ctrl;
    _iap2.func_data = &_iap2_func_data;

    usbdc_register_function(&_iap2);
    //usbdc_register_handler(USBDC_HDL_REQ, &_iap2_req_handler);

    return ERR_NONE;
}

int32_t iap2_deinit(void) {
    if (usbdc_get_state() > USBD_S_POWER) {
        return ERR_DENIED;
    }

    _iap2.ctrl = NULL;
    _iap2.func_data = NULL;

    usbdc_unregister_function(&_iap2);
    //usbdc_unregister_handler(USBDC_HDL_REQ, &_iap2_req_handler);

    return ERR_NONE;
}

bool iap2_is_enabled(void) {
    return _iap2_func_data.enabled;
}

int32_t iap2_read(uint8_t* buf, uint32_t size) {
    if (!iap2_is_enabled()) {
        return ERR_DENIED;
    }
    return usbdc_xfer(_iap2_func_data.func_ep_out, buf, size, false);
}

int32_t iap2_write(const uint8_t* buf, uint32_t size) {
    if (!iap2_is_enabled()) {
        return ERR_DENIED;
    }
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wcast-qual"
    return usbdc_xfer(_iap2_func_data.func_ep_in, (uint8_t*)buf, size, false);
#pragma GCC diagnostic pop
}

/**
 * \brief USB HID Generic Function Register Callback
 */
int32_t iap2_register_callback(enum iap2_cb_type cb_type, FUNC_PTR func)
{
	if (!iap2_is_enabled()) {
		return ERR_DENIED;
	}
	switch (cb_type) {
	case IAP2_CB_READ:
		usb_d_ep_register_callback(_iap2_func_data.func_ep_out, USB_D_EP_CB_XFER, func);
		break;
	case IAP2_CB_WRITE:
		usb_d_ep_register_callback(_iap2_func_data.func_ep_in, USB_D_EP_CB_XFER, func);
		break;
	default:
		return ERR_INVALID_ARG;
	}

	return ERR_NONE;
}
