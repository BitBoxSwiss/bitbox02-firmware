/**
 * \file
 *
 * \brief USB Human Interface Device (HID) protocol definitions.
 *
 * Copyright (c) 2015 Atmel Corporation. All rights reserved.
 *
 * \asf_license_start
 *
 * \page License
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. The name of Atmel may not be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * 4. This software may only be redistributed and used in connection with an
 *    Atmel microcontroller product.
 *
 * THIS SOFTWARE IS PROVIDED BY ATMEL "AS IS" AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT ARE
 * EXPRESSLY AND SPECIFICALLY DISCLAIMED. IN NO EVENT SHALL ATMEL BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 * ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * \asf_license_stop
 *
 */
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

#ifndef _USB_PROTOCOL_HID_H_
#define _USB_PROTOCOL_HID_H_

#include "usb_includes.h"

#define HID_CLASS 0x03
#define HID_PROTOCOL_GENERIC 0x00
// Hid USB requests (bRequest)
#define USB_REQ_HID_GET_REPORT 0x01
#define USB_REQ_HID_GET_IDLE 0x02
#define USB_REQ_HID_GET_PROTOCOL 0x03
#define USB_REQ_HID_SET_REPORT 0x09
#define USB_REQ_HID_SET_IDLE 0x0A
#define USB_REQ_HID_SET_PROTOCOL 0x0B
// HID USB descriptor types
#define USB_DT_HID 0x21
#define USB_DT_HID_REPORT 0x22
#define USB_DT_HID_PHYSICAL 0x23
// HID Type for report descriptor
#define USB_HID_ITEM_REPORT_TYPE_MAIN 0
#define USB_HID_ITEM_REPORT_TYPE_GLOBAL 1
#define USB_HID_ITEM_REPORT_TYPE_LOCAL 2
#define USB_HID_ITEM_REPORT_TYPE_LONG 3
// HID report type
#define USB_HID_REPORT_TYPE_INPUT 1
#define USB_HID_REPORT_TYPE_OUTPUT 2
#define USB_HID_REPORT_TYPE_FEATURE 3
// HID protocol
#define USB_HID_PROCOTOL_BOOT 0
#define USB_HID_PROCOTOL_REPORT 1
// HID Report type
// Used by SETUP_HID_GET_REPORT & SETUP_HID_SET_REPORT
#define REPORT_TYPE_INPUT 0x01
#define REPORT_TYPE_OUTPUT 0x02
#define REPORT_TYPE_FEATURE 0x03
// Constants of field DESCRIPTOR_HID
// Numeric expression identifying the HID Class
// Specification release (here V1.11)
#define USB_HID_BDC_V1_11 0x0111
// Numeric expression specifying the number of class descriptors
// Note: Always at least one i.e. Report descriptor.
#define USB_HID_NUM_DESC 0x01
// Country code
#define USB_HID_NO_COUNTRY_CODE 0// Not Supported

#define USB_HID_DESC_BYTES(bLength, bCountryCode, bNumDescriptors, bDescriptorType, bDescriptorLength)                 \
    bLength, 0x21, 0x10, 0x01, bCountryCode, bNumDescriptors, bDescriptorType, LE_BYTE0(bDescriptorLength),            \
        LE_BYTE1(bDescriptorLength)

// HID Descriptor
COMPILER_PACK_SET(1)
typedef struct usb_hid_descriptor {
    uint8_t bLength;           // Size of this descriptor in bytes
    uint8_t bDescriptorType;   // HID descriptor type
    le16_t  bcdHID;            // Binary Coded Decimal Spec. release
    uint8_t bCountryCode;      // Hardware target country
    uint8_t bNumDescriptors;   // Number of HID class descriptors to follow
    uint8_t bRDescriptorType;  // Report descriptor type
    le16_t  wDescriptorLength; // Total length of Report descriptor
} usb_hid_descriptor_t;
COMPILER_PACK_RESET()

/**
 * Fill a GetHIDDescriptor request
 * @param[out] req   Pointer to the request to fill
 * @param[in]  type  Descriptor type
 * @param[in]  index Descriptor index
 * @param[in]  iface Interface Number
 * @param[in]  len   Descriptor Length
 */
static inline void usb_fill_GetHIDDesc_req(struct usb_req *req, uint8_t type, uint8_t index, uint8_t iface,
                                           uint16_t len)
{
    req->bmRequestType = 0x81;
    req->bRequest      = USB_REQ_GET_DESC;
    req->V.wValue      = (type << 8) | index;
    req->I.wIndex      = iface;
    req->L.wLength     = len;
}

/**
 * Fill a SetHIDDescriptor request
 * @param[out] req   Pointer to the request to fill
 * @param[in]  type  Descriptor type
 * @param[in]  index Descriptor index
 * @param[in]  iface Interface Number
 * @param[in]  len   Descriptor Length
 */
static inline void usb_fill_SetHIDDesc_req(struct usb_req *req, uint8_t type, uint8_t index, uint8_t iface,
                                           uint16_t len)
{
    req->bmRequestType = 0x01;
    req->bRequest      = USB_REQ_SET_DESC;
    req->V.wValue      = (type << 8) | index;
    req->I.wIndex      = iface;
    req->L.wLength     = len;
}

/**
 * Fill a GetReport request
 * @param[out] req   Pointer to the request to fill
 * @param[in]  type  Report type
 * @param[in]  id    Report ID
 * @param[in]  iface Interface Number
 * @param[in]  len   Report Length
 */
static inline void usb_fill_GetReport_req(struct usb_req *req, uint8_t type, uint8_t id, uint8_t iface, uint16_t len)
{
    req->bmRequestType = 0xA1;
    req->bRequest      = USB_REQ_HID_GET_REPORT;
    req->V.wValue      = (type << 8) | id;
    req->I.wIndex      = iface;
    req->L.wLength     = len;
}

/**
 * Fill a SetReport request
 * @param[out] req   Pointer to the request to fill
 * @param[in]  type  Report type
 * @param[in]  id    Report ID
 * @param[in]  iface Interface Number
 * @param[in]  len   Report Length
 */
static inline void usb_fill_SetReport_req(struct usb_req *req, uint8_t type, uint8_t id, uint8_t iface, uint16_t len)
{
    req->bmRequestType = 0x21;
    req->bRequest      = USB_REQ_HID_GET_REPORT;
    req->V.wValue      = (type << 8) | id;
    req->I.wIndex      = iface;
    req->L.wLength     = len;
}

/**
 * Fill a GetIdle request
 * @param[out] req   Pointer to the request to fill
 * @param[in]  id    Report ID
 * @param[in]  iface Interface Number
 */
static inline void usb_fill_GetIdle_req(struct usb_req *req, uint8_t id, uint8_t iface)
{
    req->bmRequestType = 0xA1;
    req->bRequest      = USB_REQ_HID_GET_IDLE;
    req->V.wValue      = id;
    req->I.wIndex      = iface;
    req->L.wLength     = 1;
}

/**
 * Fill a SetIdle request
 * @param[out] req      Pointer to the request to fill
 * @param[in]  duration Duration value
 * @param[in]  id       Report ID
 * @param[in]  iface    Interface Number
 */
static inline void usb_fill_SetIdle_req(struct usb_req *req, uint8_t duration, uint8_t id, uint8_t iface)
{
    req->bmRequestType = 0x21;
    req->bRequest      = USB_REQ_HID_SET_IDLE;
    req->V.wValue      = (duration << 8) | id;
    req->I.wIndex      = iface;
    req->L.wLength     = 0;
}

/**
 * Fill a GetProtocol request
 * @param[out] req   Pointer to the request to fill
 * @param[in]  iface Interface Number
 */
static inline void usb_fill_GetProtocol_req(struct usb_req *req, uint8_t iface)
{
    req->bmRequestType = 0xA1;
    req->bRequest      = USB_REQ_HID_GET_PROTOCOL;
    req->V.wValue      = 0;
    req->I.wIndex      = iface;
    req->L.wLength     = 1;
}

/**
 * Fill a SetProtocol request
 * @param[out] req   Pointer to the request to fill
 * @param[in]  iface Interface Number
 */
static inline void usb_fill_SetProtocol_req(struct usb_req *req, uint8_t protocol, uint8_t iface)
{
    req->bmRequestType = 0x21;
    req->bRequest      = USB_REQ_HID_SET_PROTOCOL;
    req->V.wValue      = protocol;
    req->I.wIndex      = iface;
    req->L.wLength     = 0;
}

#endif
