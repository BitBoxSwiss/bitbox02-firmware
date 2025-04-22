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

#include "bootloader.h"
#include "mpu_regions.h"
#include "platform_config.h"
#include "platform_init.h"
#include "usb/class/hid/hww/hid_hww.h"

#include <driver_init.h>
#include <hardfault.h>
#ifdef BOOTLOADER_DEVDEVICE
#include <qtouch.h>
#endif
#include <screen.h>
#include <string.h>
#include <usb/usb_processing.h>

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    Abort("Stack smashing detected");
    while (1) {
    } // satisfy noreturn
}

uint32_t __stack_chk_guard = 0;

int main(void)
{
    // When in bootloader mode, the vector table should be 0. If not, halt.
    if (SCB->VTOR) {
        while (1) {
        };
    }

    // Order is important
    init_mcu();
    mpu_regions_bootloader_init();
    bootloader_init();
    platform_init();
    __stack_chk_guard = rand_sync_read32(&RAND_0);
    screen_init();
#ifdef BOOTLOADER_DEVDEVICE
    qtouch_init();
#endif
    bootloader_jump();

    const uint8_t* hww_data = NULL;
    uint8_t hww_frame[USB_REPORT_SIZE] = {0};

    // If did not jump to firmware code, begin USB processing
    while (1) {
        if (hid_hww_read(&hww_frame[0])) {
            usb_packet_process((const USB_FRAME*)hww_frame);
        }
        if (!hww_data) {
            hww_data = queue_pull(queue_hww_queue());
        }
        if (hww_data) {
            if (hid_hww_write_poll(hww_data)) {
                hww_data = NULL;
            }
        }
        usb_processing_process(usb_processing_hww());
    }
    return 0;
}
