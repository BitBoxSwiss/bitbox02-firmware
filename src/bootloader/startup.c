// SPDX-License-Identifier: Apache-2.0

#include "bootloader.h"
#include "mpu_regions.h"
#include <bootloader/bootloader_version.h>
#include <driver_init.h>
#include <hardfault.h>
#include <platform_config.h>
#include <platform_init.h>
#include <rust/rust.h>
#include <screen.h>
#include <string.h>
#include <ui/oled/oled.h>
#include <usb/class/hid/hww/hid_hww.h>
#include <usb/usb_processing.h>

#if defined(BOOTLOADER_DEVDEVICE) || PLATFORM_BITBOX02PLUS == 1
    #include <qtouch.h>
#endif

#if PLATFORM_BITBOX02PLUS == 1
    #include <da14531/da14531.h>
    #include <da14531/da14531_handler.h>
    #include <da14531/da14531_protocol.h>
    #include <memory/memory.h>
    #include <memory/memory_shared.h>
    #include <uart.h>
    #include <utils_ringbuffer.h>

    #if PRODUCT_BITBOX_PLUS_MULTI == 1
        #define DEVICE_MODE "{\"p\":\"bb02p-bl-multi\",\"v\":\"" BOOTLOADER_VERSION "\"}"
    #elif PRODUCT_BITBOX_PLUS_BTCONLY == 1
        #define DEVICE_MODE "{\"p\":\"bb02p-bl-btconly\",\"v\":\"" BOOTLOADER_VERSION "\"}"
    #else
        #error "unknown product"
    #endif
#endif // PLATFORM_BITBOX02PLUS == 1

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
uint8_t bootloader_pairing_code_bytes[4] = {0};
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
    screen_init(oled_set_pixel, oled_mirror, oled_clear_buffer);
#if defined(BOOTLOADER_DEVDEVICE) || PLATFORM_BITBOX02PLUS == 1
    qtouch_init();
#endif
    bootloader_jump();

    // If did not jump to firmware code, begin UART/USB processing
    const uint8_t* hww_data = NULL;
    USB_FRAME hww_frame = {0};

#if PLATFORM_BITBOX02PLUS == 1
    uint8_t uart_read_buf[USART_0_BUFFER_SIZE] = {0};
    uint16_t uart_read_buf_len = 0;

    ringbuffer_init(&uart_write_queue, &uart_write_buf, UART_OUT_BUF_LEN);
    if (!memory_ble_enabled()) {
        rust_communication_mode_ble_disable();
    }

    // Set product to bootloader string, this is necessary if we have rebooted from firmware. Must
    // be done after usb_processing is initalized to avoid getting request from the app to early.
    size_t product_len;
    da14531_handler_current_product = (const uint8_t*)platform_product(&product_len);
    da14531_handler_current_product_len = product_len;
    da14531_set_product(
        da14531_handler_current_product, da14531_handler_current_product_len, &uart_write_queue);

    // Set device name, the MCU and BLE chip will probably not have the same name after a reset of
    // only the MCU.
    char buf[MEMORY_DEVICE_NAME_MAX_LEN] = {0};
    memory_random_name(buf);
    da14531_set_name(buf, strlen(buf), &uart_write_queue);

    // Ask for the current conection state
    da14531_get_connection_state(&uart_write_queue);

    da14531_protocol_init();
#endif
    usb_processing_init();

    while (1) {
        // Do UART I/O
#if PLATFORM_BITBOX02PLUS == 1
        if (rust_communication_mode_ble_enabled()) {
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
        if (!hww_data && hid_hww_read((uint8_t*)&hww_frame)) {
            usb_packet_process(&hww_frame);
#if PLATFORM_BITBOX02PLUS == 1
            if (rust_communication_mode_ble_enabled()) {
                // Enqueue a power down command to the da14531
                da14531_power_down(&uart_write_queue);
                // Flush out the power down command. This will be the last UART communication we do.
                while (ringbuffer_num(&uart_write_queue) > 0) {
                    uart_poll(NULL, 0, NULL, &uart_write_queue);
                }
                rust_communication_mode_ble_disable();
                bootloader_render_default_screen();
            }
#endif
        }
#if PLATFORM_BITBOX02PLUS == 1
        if (rust_communication_mode_ble_enabled()) {
            struct da14531_protocol_frame* frame = da14531_protocol_poll(
                &uart_read_buf[0], &uart_read_buf_len, &hww_data, &uart_write_queue);

            if (frame) {
                // screen_sprintf_debug(1000, "got frame");
                da14531_handler(frame, &uart_write_queue);
            }
        }
#endif

#if PLATFORM_BITBOX02PLUS == 1
        if (!rust_communication_mode_ble_enabled()) {
#endif
            if (hww_data) {
                if (hid_hww_write_poll(hww_data)) {
                    hww_data = NULL;
                }
            }
#if PLATFORM_BITBOX02PLUS == 1
        }
#endif
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
                    bootloader_render_default_screen();
                    ok = false;
                } else {
                    bootloader_render_ble_confirm_screen(true);
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
                ASSERT(ringbuffer_num(&uart_write_queue) + len <= uart_write_queue.size);
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
