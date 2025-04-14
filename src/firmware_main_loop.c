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

#include "firmware_main_loop.h"

#include "hardfault.h"
#include "hid_hww.h"
#include "hww.h"
#include "touch/gestures.h"
#include "ui/screen_process.h"
#include "ui/screen_stack.h"
#include "usb/class/hid/hww/hid_hww.h"
#include "usb/usb.h"
#include "usb/usb_frame.h"
#include "usb/usb_processing.h"
#include "workflow/orientation_screen.h"
#include <rust/rust.h>
#if APP_U2F == 1
#include "u2f.h"
#include "u2f/u2f_packet.h"
#include "usb/class/hid/u2f/hid_u2f.h"
#endif

void firmware_main_loop(void)
{
    // This starts the async orientation screen workflow, which is processed by the loop below.
    orientation_screen();

    const uint8_t* hww_data = NULL;
    uint8_t hww_frame[USB_REPORT_SIZE] = {0};

#if APP_U2F == 1
    u2f_packet_init();
    const uint8_t* u2f_data = NULL;
    uint8_t u2f_frame[USB_REPORT_SIZE] = {0};
#endif

    while (1) {
        // Do USB I/O
        if (!hww_data) {
            hww_data = queue_pull(queue_hww_queue());
        }
#if APP_U2F == 1
        // Generate timeout packets
        uint32_t timeout_cid;
        while (u2f_packet_timeout_get(&timeout_cid)) {
            u2f_packet_timeout(timeout_cid);
        }
        if (!u2f_data) {
            u2f_data = queue_pull(queue_u2f_queue());
        }
#endif
        // Only read new messages if we have nothing to send
        if (!hww_data && hid_hww_read(&hww_frame[0])) {
            usb_packet_process((const USB_FRAME*)hww_frame);
        }
#if APP_U2F == 1
        if (!u2f_data && hid_u2f_read(&u2f_frame[0])) {
            u2f_packet_process((const USB_FRAME*)u2f_frame);
        }
#endif

        if (hww_data) {
            if (hid_hww_write_poll(hww_data)) {
                hww_data = NULL;
            }
        }
#if APP_U2F == 1
        if (u2f_data) {
            if (hid_u2f_write_poll(u2f_data)) {
                u2f_data = NULL;
            }
        }
#endif

        /* First, process all the incoming USB traffic. */
        usb_processing_process(usb_processing_hww());
#if APP_U2F == 1
        usb_processing_process(usb_processing_u2f());
#endif
        /*
         * If USB has generated events at the application level,
         * process them now.
         */
        hww_process();
#if APP_U2F == 1
        u2f_process();
#endif

        screen_process();
        /* And finally, run the high-level event processing. */

        rust_workflow_spin();

        rust_async_usb_spin();
    }
}
