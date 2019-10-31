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

#include "ui/screen_process.h"
#include "usart/usart.h"
#include "usb/usb_processing.h"

void firmware_main_loop(void)
{
    while (1) {
        screen_process();
        usb_processing_process(usb_processing_hww());
#if defined(APP_U2F)
        usb_processing_process(usb_processing_u2f());
#endif
    }
}
