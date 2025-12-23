// SPDX-License-Identifier: Apache-2.0

#ifndef _USB_DESC_H_
#define _USB_DESC_H_

#include "usb_desc_common.h"
#include "usb_size.h"
#include <bootloader/bootloader_version.h>
#include <version.h>
#ifndef TESTING
    #include "usb_protocol.h"
    #include "usb_protocol_hid.h"
    #include <usb_u2f_desc.h>
#endif

#if defined(BOOTLOADER)
    #if PRODUCT_BITBOX_BTCONLY == 1
        #define USB_DESC_IPRODUCT_STR_DESC                                                      \
            38, /* bLength */                                                                   \
                0x03, /* bDescriptorType */                                                     \
                'b', 0, 'b', 0, '0', 0, '2', 0, 'b', 0, 't', 0, 'c', 0, '-', 0, 'b', 0, 'o', 0, \
                'o', 0, 't', 0, 'l', 0, 'o', 0, 'a', 0, 'd', 0, 'e', 0, 'r', 0,
    #else
        #define USB_DESC_IPRODUCT_STR_DESC                                                      \
            32, /* bLength */                                                                   \
                0x03, /* bDescriptorType */                                                     \
                'b', 0, 'b', 0, '0', 0, '2', 0, '-', 0, 'b', 0, 'o', 0, 'o', 0, 't', 0, 'l', 0, \
                'o', 0, 'a', 0, 'd', 0, 'e', 0, 'r', 0,
    #endif
#elif FACTORYSETUP == 1
    #define USB_DESC_IPRODUCT_STR_DESC                                                           \
        26, /* bLength */                                                                        \
            0x03, /* bDescriptorType */                                                          \
            'b', 0, 'b', 0, '0', 0, '2', 0, '-', 0, 'f', 0, 'a', 0, 'c', 0, 't', 0, 'o', 0, 'r', \
            0, 'y', 0,
#elif PRODUCT_BITBOX_BTCONLY == 1
    #define USB_DESC_IPRODUCT_STR_DESC                                                           \
        24, /* bLength */                                                                        \
            0x03, /* bDescriptorType */                                                          \
            'B', 0, 'i', 0, 't', 0, 'B', 0, 'o', 0, 'x', 0, '0', 0, '2', 0, 'B', 0, 'T', 0, 'C', \
            0,
#else
    #define USB_DESC_IPRODUCT_STR_DESC  \
        18, /* bLength */               \
            0x03, /* bDescriptorType */ \
            'B', 0, 'i', 0, 't', 0, 'B', 0, 'o', 0, 'x', 0, '0', 0, '2', 0,
#endif

#define USB_STR_DESC            \
    USB_DESC_LANGID_DESC        \
    USB_DESC_IMANUFACT_STR_DESC \
    USB_DESC_IPRODUCT_STR_DESC  \
    USB_DESC_ISERIALNUM_STR_DESC

#define USB_DESC_HWW_REPORT_LEN 34
#define USB_DESC_HWW_REPORT                                        \
    0x06, 0xff, 0xff, /* USAGE_PAGE (Vendor Defined) */            \
        0x09, 0x01, /* USAGE (HID Generic Device) */               \
        0xa1, 0x01, /* COLLECTION (Application) */ /* In Report */ \
        0x09, 0x20, /* USAGE (Input Report Data) */                \
        0x15, 0x00, /* LOGICAL_MINIMUM (0) */                      \
        0x26, 0xff, 0x00, /* LOGICAL_MAXIMUM (255) */              \
        0x75, 0x08, /* REPORT_SIZE (8) */                          \
        0x95, 0x40, /* REPORT_COUNT (64)  */                       \
        0x81, 0x02, /* INPUT (Data,Var,Abs) */ /* Out Report */    \
        0x09, 0x21, /* USAGE (Output Report Data) */               \
        0x15, 0x00, /* LOGICAL_MINIMUM (0) */                      \
        0x26, 0xff, 0x00, /* LOGICAL_MAXIMUM (255) */              \
        0x75, 0x08, /* REPORT_SIZE (8) */                          \
        0x95, 0x40, /* REPORT_COUNT (64) */                        \
        0x91, 0x02, /* OUTPUT (Data,Var,Abs) */                    \
        0xc0 /* END_COLLECTION */

#define USB_DESC_IFACE_HWW                                                  \
    9, /* iface.bLength */                                                  \
        0x04, /* iface.bDescriptorType: INTERFACE */                        \
        USB_DESC_IFACE_NUM_HWW, /* iface.bInterfaceNumber */                \
        0x00, /* iface.bAlternateSetting */                                 \
        0x02, /* iface.bNumEndpoints */                                     \
        HID_CLASS, /* iface.bInterfaceClass */                              \
        USB_SUBCLASS_NO, /* iface.bInterfaceSubClass */                     \
        USB_PROTOCOL_NO, /* iface.bInterfaceProtocol */                     \
        0x00, /* iface.iInterface */                                        \
        9, /* hid.bLength */                                                \
        USB_DT_HID, /* hid.bDescriptorType: HID */                          \
        USB_DESC_LE16(USB_HID_BDC_V1_11), /* hid.bcdHID */                  \
        0x00, /* hid.bCountryCode */                                        \
        0x01, /* hid.bNumDescriptors */                                     \
        0x22, /* hid.bRDescriptorType */                                    \
        USB_DESC_LE16(USB_DESC_HWW_REPORT_LEN), /* hid.wDescriptorLength */ \
        7, /* ep_in.bLength */                                              \
        0x05, /* ep_in.bDescriptorType: ENDPOINT */                         \
        USB_DESC_HWW_EP_IN, /* ep_in.bEndpointAddress */                    \
        0x03, /* ep_in.bmAttributes */                                      \
        USB_DESC_LE16(USB_DESC_HID_EP_SIZE), /* ep_in.wMaxPacketSize */     \
        4, /* ep_in.bInterval */                                            \
        7, /* ep_out.bLength */                                             \
        0x05, /* ep_out.bDescriptorType: ENDPOINT */                        \
        USB_DESC_HWW_EP_OUT, /* ep_out.bEndpointAddress */                  \
        0x03, /* ep_out.bmAttributes */                                     \
        USB_DESC_LE16(USB_DESC_HID_EP_SIZE), /* ep_out.wMaxPacketSize */    \
        4 /* ep_out.bInterval */

#define USB_DESC_CONFIG                                       \
    USB_DESC_CONFIG_LEN, /* bLength */                        \
        0x02, /* bDescriptorType: CONFIGURATION */            \
        USB_DESC_LE16(USB_DESC_WTOTALLEN), /* wTotalLength */ \
        USB_DESC_NUM_IFACES, /* bNumInterfaces */             \
        USB_DESC_BCONFIGVAL, /* bConfigurationValue */        \
        0x00, /* iConfiguration */                            \
        USB_DESC_BMATTRI, /* bmAttributes */                  \
        USB_DESC_BMAXPOWER /* bMaxPower */

//  ** If add an interface, adjust USB_DESC_WTOTALLEN **
// TODO: USB_DESC_D_MAX_EP_N doesn't exist, but there is CONF_USB_D_NUM_EP_SP
// (= supported endpoints) - is that the one that needs to change?
//  ** If add more endpoints, adjust USB_DESC_D_MAX_EP_N  **
#if APP_U2F == 0
    #define USB_DESC_FS USB_DEV_DESC, USB_DESC_CONFIG, USB_DESC_IFACE_HWW, USB_STR_DESC
#else
    #define USB_DESC_FS \
        USB_DEV_DESC, USB_DESC_CONFIG, USB_DESC_IFACE_HWW, USB_DESC_IFACE_U2F, USB_STR_DESC
#endif

#endif
