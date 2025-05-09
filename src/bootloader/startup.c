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
#include <driver_init.h>
#include <hardfault.h>
#include <platform_config.h>
#include <platform_init.h>
#include <rust/rust.h>
#include <screen.h>
#include <string.h>
#include <usb/class/hid/hww/hid_hww.h>
#include <usb/usb_processing.h>

#if defined(BOOTLOADER_DEVDEVICE) || PLATFORM_BITBOX02PLUS == 1
#include <qtouch.h>
#endif

#if PLATFORM_BITBOX02PLUS == 1
#include <da14531/da14531.h>
#include <da14531/da14531_handler.h>
#include <da14531/da14531_protocol.h>
#include <uart.h>
#include <utils_ringbuffer.h>
#endif

extern void __attribute__((noreturn)) __stack_chk_fail(void);
void __attribute__((noreturn)) __stack_chk_fail(void)
{
    Abort("Stack smashing detected");
    while (1) {
    } // satisfy noreturn
}

uint32_t __stack_chk_guard = 0;

#if PLATFORM_BITBOX02PLUS == 1
extern volatile bool measurement_done_touch;
int bootloader_pairing_request = false;
uint8_t bootloader_pairing_code_bytes[16] = {0};
// Must be power of 2, must fit bond_db
#define UART_OUT_BUF_LEN 2048
struct ringbuffer uart_write_queue;
uint8_t uart_write_buf[UART_OUT_BUF_LEN];
#endif

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

    // If did not jump to firmware code, begin UART/USB processing
    const uint8_t* hww_data = NULL;
    uint8_t hww_frame[USB_REPORT_SIZE] = {0};

#if PLATFORM_BITBOX02PLUS == 1
    uint8_t uart_read_buf[USART_0_BUFFER_SIZE] = {0};
    uint16_t uart_read_buf_len = 0;

    ringbuffer_init(&uart_write_queue, &uart_write_buf, UART_OUT_BUF_LEN);
    bool usb_hww_request_seen = false;

    // Set product to bootloader string, this is necessary if we have rebooted from firmware. Must
    // be done after usb_processing is initalized to avoid getting request from the app to early.
#define DEVICE_MODE "{\"p\":\"bb02p-bl-multi\",\"v\":\"1.1.0\"}"
    da14531_handler_current_product = (const uint8_t*)DEVICE_MODE;
    da14531_handler_current_product_len = sizeof(DEVICE_MODE) - 1;
    da14531_set_product(
        da14531_handler_current_product, da14531_handler_current_product_len, &uart_write_queue);

    da14531_protocol_init();
#endif
    usb_processing_init();

    while (1) {
        // Do UART I/O
#if PLATFORM_BITBOX02PLUS == 1
        if (!usb_hww_request_seen) {
            if (uart_read_buf_len < sizeof(uart_read_buf) ||
                ringbuffer_num(&uart_write_queue) > 0) {
                // screen_sprintf_debug(1000, "uart poll");
                uart_poll(
                    &uart_read_buf[0],
                    sizeof(uart_read_buf),
                    &uart_read_buf_len,
                    &uart_write_queue);
            }
        }
#endif
        if (!hww_data) {
            hww_data = queue_pull(queue_hww_queue());
        }
        if (!hww_data && hid_hww_read(&hww_frame[0])) {
            usb_packet_process((const USB_FRAME*)hww_frame);
#if PLATFORM_BITBOX02PLUS == 1
            if (!usb_hww_request_seen) {
                // Enqueue a power down command to the da14531
                da14531_power_down(&uart_write_queue);
                // Flush out the power down command. This will be the last UART communication we do.
                while (ringbuffer_num(&uart_write_queue) > 0) {
                    uart_poll(NULL, 0, NULL, &uart_write_queue);
                }
                usb_hww_request_seen = true;
            }
#endif
        }
#if PLATFORM_BITBOX02PLUS == 1
        if (!usb_hww_request_seen) {
            struct da14531_protocol_frame* frame = da14531_protocol_poll(
                &uart_read_buf[0], &uart_read_buf_len, hww_data, &uart_write_queue);
            // da14531_protocol_poll has consumed the data, clear pointer
            hww_data = NULL;

            if (frame) {
                // screen_sprintf_debug(1000, "got frame");
                da14531_handler(frame, &uart_write_queue);
            }
        }
#endif
        if (hww_data) {
            if (hid_hww_write_poll(hww_data)) {
                hww_data = NULL;
            }
        }
        usb_processing_process(usb_processing_hww());

#if PLATFORM_BITBOX02PLUS == 1
        qtouch_process();
        if (bootloader_pairing_request) {
            if (!measurement_done_touch) {
                continue;
            }

            if (qtouch_is_scroller_active(top_slider)) {
                bool ok;
                UG_ClearBuffer();
                if (qtouch_get_scroller_position(top_slider) < 127) {
                    UG_PutString(0, 0, "deny", false);
                    ok = false;
                } else {
                    UG_PutString(0, 0, "confirm", false);
                    ok = true;
                }
                uint8_t payload[18] = {0};
                payload[0] = 11;
                memcpy(
                    &payload[1],
                    &bootloader_pairing_code_bytes[0],
                    sizeof(bootloader_pairing_code_bytes));
                payload[17] = ok ? 1 : 0; /* 1 yes, 0 no */

                uint8_t tmp[32];
                uint16_t len = da14531_protocol_format(
                    &tmp[0],
                    sizeof(tmp),
                    DA14531_PROTOCOL_PACKET_TYPE_CTRL_DATA,
                    payload,
                    sizeof(payload));
                ASSERT(len <= sizeof(tmp));
                ASSERT(ringbuffer_num(data->queue) + len <= data->queue->size);
                for (int i = 0; i < len; i++) {
                    ringbuffer_put(&uart_write_queue, tmp[i]);
                }
                bootloader_pairing_request = false;
                UG_SendBuffer();
            }
        }
#endif
    }
    return 0;
}
