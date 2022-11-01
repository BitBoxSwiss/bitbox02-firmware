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
#include "hww.h"
#include "touch/gestures.h"
#include "u2f.h"
#include "ui/screen_process.h"
#include "ui/screen_stack.h"
#include "usb/usb.h"
#include "usb/usb_processing.h"
#include <rust/rust.h>

void firmware_main_loop(void)
{
    while (1) {
        screen_process();
        /* And finally, run the high-level event processing. */

        rust_workflow_spin();

        if (usb_is_enabled()) {
            rust_async_usb_spin();

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
        }
    }
}
