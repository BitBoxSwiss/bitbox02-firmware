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

#ifndef _USB_DESC_COMMON_H_
#define _USB_DESC_COMMON_H_

#include <version.h>

#define USB_DESC_LE16(a) ((uint8_t)(a)), ((uint8_t)((a) >> 8))

#define USB_DESC_IDVENDER 0x03eb
#define USB_DESC_IDPRODUCT 0x2403
#define USB_DESC_HWW_EP_IN (1 | USB_EP_DIR_IN)
#define USB_DESC_HWW_EP_OUT (2 | USB_EP_DIR_OUT)
#if APP_U2F == 1
#define USB_DESC_IFACE_NUM_U2F 1
#define USB_DESC_NUM_IFACES 2
#else
#define USB_DESC_NUM_IFACES 1
#endif
#define USB_DESC_IFACE_NUM_HWW 0
#define USB_DESC_IFACE_LEN 32
#define USB_DESC_CONFIG_LEN 9
#define USB_DESC_WTOTALLEN (USB_DESC_CONFIG_LEN + USB_DESC_IFACE_LEN * USB_DESC_NUM_IFACES)
#define USB_DESC_BMAXPKSZ0 0x40
#define USB_DESC_BCDUSB 0x200 // 0x0200 => USB 2.0 version; 0x0210 => USB 2.1 version
#define USB_DESC_BCDDEVICE 0x100
#define USB_DESC_BNUMCONFIG 0x1
#define USB_DESC_BCONFIGVAL 0x1
#define USB_DESC_BMATTRI 0x80 // Bus power supply, no support for remote wakeup
#define USB_DESC_BMAXPOWER 0x32
#define USB_DESC_HID_EP_SIZE USB_REPORT_SIZE

#define USB_DESC_LANGID 0x0409 // English - United States
#define USB_DESC_LANGID_DESC        \
    4, /* bLength */                \
        0x03, /* bDescriptorType */ \
        USB_DESC_LE16(USB_DESC_LANGID), /* wLANGID[0] */

#define USB_DESC_IMANUFACT 1
#define USB_DESC_IMANUFACT_STR_DESC                                                             \
    26, /* bLength */                                                                           \
        0x03, /* bDescriptorType */                                                             \
        'b', 0, 'i', 0, 't', 0, 'b', 0, 'o', 0, 'x', 0, '.', 0, 's', 0, 'w', 0, 'i', 0, 's', 0, \
        's', 0,

#define USB_DESC_ISERIALNUM 3

#ifdef BOOTLOADER
#define USB_DESC_ISERIALNUM_STR_DESC                                                    \
    (2 + BOOTLOADER_VERSION_LEN * 2 + BOOTLOADER_VERSION_APPEND_LEN * 2), /* bLength */ \
        0x03, /* bDescriptorType */                                                     \
        BOOTLOADER_VERSION_W16 BOOTLOADER_VERSION_APPEND_W16
#else
#define USB_DESC_ISERIALNUM_STR_DESC                    \
    (2 + DIGITAL_BITBOX_VERSION_LEN * 2), /* bLength */ \
        0x03, /* bDescriptorType */                     \
        DIGITAL_BITBOX_VERSION_W16
#endif

#if defined(BOOTLOADER_DEVDEVICE) && defined(BOOTLOADER_VERSION_HAS_METADATA)
#define BOOTLOADER_VERSION_APPEND_LEN 4
#define BOOTLOADER_VERSION_APPEND_W16 '.', 0, 'd', 0, 'e', 0, 'v', 0,
#elif defined(BOOTLOADER_DEVDEVICE)
#define BOOTLOADER_VERSION_APPEND_LEN 4
#define BOOTLOADER_VERSION_APPEND_W16 '+', 0, 'd', 0, 'e', 0, 'v', 0,
#else
#define BOOTLOADER_VERSION_APPEND_LEN 0
#define BOOTLOADER_VERSION_APPEND_W16
#endif

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

#define USB_DESC_IPRODUCT 2
#define USB_DEV_DESC                                       \
    18, /* bLength */                                      \
        0x01, /* bDescriptorType: DEVICE */                \
        USB_DESC_LE16(USB_DESC_BCDUSB), /* bcdUSB */       \
        USB_CLASS_NO, /* bDeviceClass */                   \
        USB_SUBCLASS_NO, /* bDeviceSubClass */             \
        USB_PROTOCOL_NO, /* bDeviceProtocol */             \
        USB_DESC_BMAXPKSZ0, /* bMaxPacketSize0 */          \
        USB_DESC_LE16(USB_DESC_IDVENDER), /* idVendor */   \
        USB_DESC_LE16(USB_DESC_IDPRODUCT), /* idProduct */ \
        USB_DESC_LE16(USB_DESC_BCDDEVICE), /* bcdDevice */ \
        USB_DESC_IMANUFACT, /* iManufacturer */            \
        USB_DESC_IPRODUCT, /* iProduct */                  \
        USB_DESC_ISERIALNUM, /* iSerialNumber */           \
        USB_DESC_BNUMCONFIG /* bNumConfigurations */

#endif
