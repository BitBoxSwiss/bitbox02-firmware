// Copyright 2025 Shift Crypto AG
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

#ifndef _USB_DESC_BITBOX02_PLUS_H_
#define _USB_DESC_BITBOX02_PLUS_H_

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
#define USB_DESC_BB02PLUS_IPRODUCT_STR_DESC                                                     \
    34, /* bLength */                                                                           \
        0x03, /* bDescriptorType */                                                             \
        'b', 0, 'b', 0, '0', 0, '2', 0, 'p', 0, '-', 0, 'b', 0, 'l', 0, '-', 0, 'b', 0, 't', 0, \
        'c', 0, 'o', 0, 'n', 0, 'l', 0, 'y', 0,
#else
#define USB_DESC_BB02PLUS_IPRODUCT_STR_DESC                                                     \
    30, /* bLength */                                                                           \
        0x03, /* bDescriptorType */                                                             \
        'b', 0, 'b', 0, '0', 0, '2', 0, 'p', 0, '-', 0, 'b', 0, 'l', 0, '-', 0, 'm', 0, 'u', 0, \
        'l', 0, 't', 0, 'i', 0,
#endif
#elif FACTORYSETUP == 1
#define USB_DESC_BB02PLUS_IPRODUCT_STR_DESC                                                     \
    28, /* bLength */                                                                           \
        0x03, /* bDescriptorType */                                                             \
        'b', 0, 'b', 0, '0', 0, '2', 0, 'p', 0, '-', 0, 'f', 0, 'a', 0, 'c', 0, 't', 0, 'o', 0, \
        'r', 0, 'y', 0,
#elif PRODUCT_BITBOX_BTCONLY == 1
#define USB_DESC_BB02PLUS_IPRODUCT_STR_DESC                                                     \
    28, /* bLength */                                                                           \
        0x03, /* bDescriptorType */                                                             \
        'b', 0, 'b', 0, '0', 0, '2', 0, 'p', 0, '-', 0, 'b', 0, 't', 0, 'c', 0, 'o', 0, 'n', 0, \
        'l', 0, 'y', 0,
#else
#define USB_DESC_BB02PLUS_IPRODUCT_STR_DESC \
    24, /* bLength */                       \
        0x03, /* bDescriptorType */         \
        'b', 0, 'b', 0, '0', 0, '2', 0, 'p', 0, '-', 0, 'm', 0, 'u', 0, 'l', 0, 't', 0, 'i', 0,
#endif

#define USB_STR_DESC_BB02PLUS           \
    USB_DESC_LANGID_DESC                \
    USB_DESC_IMANUFACT_STR_DESC         \
    USB_DESC_BB02PLUS_IPRODUCT_STR_DESC \
    USB_DESC_ISERIALNUM_STR_DESC

#define USB_DESC_BB02PLUS_HWW_REPORT_LEN 34
#define USB_DESC_BB02PLUS_HWW_REPORT                               \
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

//  ** If add an interface, adjust USB_DESC_BB02PLUS_WTOTALLEN **
// TODO: USB_DESC_BB02PLUS_D_MAX_EP_N doesn't exist, but there is CONF_USB_D_NUM_EP_SP
// (= supported endpoints) - is that the one that needs to change?
//  ** If add more endpoints, adjust USB_DESC_BB02PLUS_D_MAX_EP_N  **
#if APP_U2F == 0
#define USB_DESC_BB02PLUS_FS \
    USB_DEV_DESC, USB_DESC_CONFIG, USB_DESC_IFACE_HWW, USB_STR_DESC_BB02PLUS
#else
#define USB_DESC_BB02PLUS_FS \
    USB_DEV_DESC, USB_DESC_CONFIG, USB_DESC_IFACE_HWW, USB_DESC_IFACE_U2F, USB_STR_DESC_BB02PLUS
#endif

#endif
