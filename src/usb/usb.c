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

#include "usb.h"
#ifndef TESTING
#include "hid_hww.h"
#include "usb_desc.h"
#include "usbdc.h"
#ifndef BOOTLOADER
#include "u2f.h"
#include <drivers/usb/class/hid/u2f/hid_u2f.h>
#endif
#endif

#ifndef TESTING
static uint8_t _ctrl_endpoint_buffer[USB_REPORT_SIZE];
static uint8_t _descriptor_bytes[] = {
    USB_DESC_FS}; // Device descriptors and Configuration descriptors list.
static struct usbd_descriptors _descriptor[] = {
    {_descriptor_bytes, _descriptor_bytes + sizeof(_descriptor_bytes)}};
static void (*_on_hww_init)(void) = NULL;
static void _hww_endpoint_available(void);
#ifndef BOOTLOADER
static void _u2f_endpoint_available(void);
#endif

/* ==== HWW ==== */
static void _hww_endpoint_available(void)
{
    if (!hid_hww_is_enabled()) {
        return;
    }
    if (_on_hww_init != NULL) {
        _on_hww_init();
    }
    hid_hww_setup();
}

#ifndef BOOTLOADER
/* ==== U2F ==== */
static void _u2f_endpoint_available(void)
{
    if (!hid_u2f_is_enabled()) {
        return;
    };
    u2f_device_setup();
    hid_u2f_setup();
}
#endif
#endif

int32_t usb_start(void (*on_hww_init)(void))
{
#ifndef TESTING
    // required before hid init
    int32_t ret = 0;
    ret = usbdc_init(_ctrl_endpoint_buffer);
    if (ret != 0) {
        return ret;
    }
    _on_hww_init = on_hww_init;
    ret = hid_hww_init(_hww_endpoint_available);
    if (ret != 0) {
        return ret;
    }
#ifndef BOOTLOADER
    ret = hid_u2f_init(_u2f_endpoint_available);
    if (ret != 0) {
        return ret;
    }
#endif
    usbdc_start(_descriptor);
    usbdc_attach();
#else
    (void)on_hww_init;
#endif
    return 0;
}

void usb_stop(void)
{
#ifndef TESTING
    usbdc_detach();
    usbdc_stop();
    usbdc_deinit();
#endif
}
