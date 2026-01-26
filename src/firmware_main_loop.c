// SPDX-License-Identifier: Apache-2.0

#include "firmware_main_loop.h"

#include "da14531/da14531.h"
#include "da14531/da14531_handler.h"
#include "da14531/da14531_protocol.h"
#include "driver_init.h"
#include "hardfault.h"
#include "hid_hww.h"
#include "hww.h"
#include "memory/memory.h"
#include "memory/memory_shared.h"
#include "touch/gestures.h"
#include "uart.h"
#include "ui/screen_process.h"
#include "ui/screen_stack.h"
#include "usb/class/hid/hww/hid_hww.h"
#include "usb/usb.h"
#include "usb/usb_frame.h"
#include "usb/usb_processing.h"
#include <platform/platform_init.h>
#include <rust/rust.h>
#include <ui/fonts/monogram_5X9.h>
#include <utils_ringbuffer.h>
#if APP_U2F == 1
    #include "u2f.h"
    #include "u2f/u2f_packet.h"
    #include "usb/class/hid/u2f/hid_u2f.h"
#endif

// Must be power of 2
#define UART_OUT_BUF_LEN 2048

static void _orientation_screen_poll(struct ringbuffer* uart_write_queue)
{
    static bool orientation_set = false;
    bool _orientation;
    if (!orientation_set && rust_workflow_orientation_screen_poll(&_orientation)) {
        orientation_set = true;
        // hww handler in usb_process must be setup before we can allow ble connections
        if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
            size_t len;
            da14531_handler_current_product = (const uint8_t*)platform_product(&len);
            da14531_handler_current_product_len = len;
            util_log("%s %d", da14531_handler_current_product, da14531_handler_current_product_len);
            da14531_set_product(
                da14531_handler_current_product,
                da14531_handler_current_product_len,
                uart_write_queue);
        }
        usb_start();
    }
}

void firmware_main_loop(void)
{
    // Set the size of uart_read_buf to the size of the ringbuffer in the UART driver so we can read
    // out all bytes
    uint8_t uart_read_buf[USART_0_BUFFER_SIZE] = {0};
    uint16_t uart_read_buf_len = 0;

    struct ringbuffer uart_write_queue;
    uint8_t uart_write_buf[UART_OUT_BUF_LEN];
    ringbuffer_init(&uart_write_queue, &uart_write_buf, UART_OUT_BUF_LEN);

    /// If the bootloader has booted the BLE chip, the BLE chip isn't aware of the name according to
    /// the fw. Send it over.
    char buf[MEMORY_DEVICE_NAME_MAX_LEN] = {0};
    memory_get_device_name(buf);
    da14531_set_name(buf, strlen(buf), &uart_write_queue);

    // This starts the async orientation screen workflow, which is processed by the loop below.
    rust_workflow_spawn_orientation_screen();

    const uint8_t* hww_data = NULL;
    USB_FRAME hww_frame = {0};

#if APP_U2F == 1
    u2f_packet_init();
    const uint8_t* u2f_data = NULL;
    USB_FRAME u2f_frame = {0};
#endif

    if (!memory_ble_enabled()) {
        rust_communication_mode_ble_disable();
    }

    while (1) {
        // Do UART I/O
        if (rust_communication_mode_ble_enabled()) {
            if (uart_read_buf_len < sizeof(uart_read_buf) ||
                ringbuffer_num(&uart_write_queue) > 0) {
                uart_poll(
                    &uart_read_buf[0],
                    sizeof(uart_read_buf),
                    &uart_read_buf_len,
                    &uart_write_queue);
            }
        }

        // Check if there is outgoing data
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
            // If USB stack was locked and there is no more messages to send out, time to
            // unlock it.
            if (!u2f_data && usb_processing_locked(usb_processing_u2f())) {
                usb_processing_unlock();
            }
        }
#endif
        // Do USB Input
        if (!hww_data && hid_hww_read((uint8_t*)&hww_frame)) {
            if (usb_packet_process(&hww_frame)) {
                if (rust_communication_mode_ble_enabled()) {
                    // Enqueue a power down command to the da14531
                    da14531_power_down(&uart_write_queue);
                    // Flush out the power down command. This will be the last UART communication we
                    // do.
                    while (ringbuffer_num(&uart_write_queue) > 0) {
                        uart_poll(NULL, 0, NULL, &uart_write_queue);
                    }
                    rust_communication_mode_ble_disable();
                }
            } else {
                util_log("usb_packet_process: invalid");
            }
        }
#if APP_U2F == 1
        if (!u2f_data && hid_u2f_read((uint8_t*)&u2f_frame)) {
            util_log("u2f data %s", util_dbg_hex((void*)&u2f_frame, 16));
            u2f_packet_process(&u2f_frame);
        }
#endif

        // Do UART Output
        if (rust_communication_mode_ble_enabled()) {
            struct da14531_protocol_frame* frame = da14531_protocol_poll(
                &uart_read_buf[0], &uart_read_buf_len, &hww_data, &uart_write_queue);

            if (frame) {
                da14531_handler(frame, &uart_write_queue);
            }
        }

        // Do USB Output
        if (!rust_communication_mode_ble_enabled() && hww_data) {
            if (hid_hww_write_poll(hww_data)) {
                hww_data = NULL;
            }
        }
#if APP_U2F == 1
        if (u2f_data) {
            if (hid_u2f_write_poll(u2f_data)) {
                util_log("u2f wrote %s", util_dbg_hex(u2f_data, 16));
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

        _orientation_screen_poll(&uart_write_queue);
    }
}
