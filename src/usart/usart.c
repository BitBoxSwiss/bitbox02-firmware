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

#include "usart.h"

#include "hardfault.h"
#include "usart_frame.h"
#include "usart_hww.h"
#include "usb/usb_processing.h"

#include <driver_init.h>
#include <hal_gpio.h>

#include <stdint.h>

#include <inttypes.h>
#include <stdio.h>

/**
 * Handle for the USART port.
 */
static struct usart_async_descriptor _usart_descriptor;

#define USART_RX_BUFFER_SIZE 256

/**
 * RX buffer for the UART.
 * The driver will use this as a circular buffer in the
 * background (interrupt context); we consume data from
 * here in bulk at every iteration of the main loop.
 */
uint8_t _usart_rx_buf[USART_RX_BUFFER_SIZE];

/**
 * \brief USART Clock initialization function
 *
 * Enables register interface and peripheral clock
 */
static void _usart_clock_init(void)
{
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SERCOM2_GCLK_ID_CORE, CONF_GCLK_SERCOM2_CORE_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SERCOM2_GCLK_ID_SLOW, CONF_GCLK_SERCOM2_SLOW_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
    hri_mclk_set_APBBMASK_SERCOM2_bit(MCLK);
}

/**
 * \brief USART pinmux initialization function
 *
 * Set each required pin to USART functionality
 */
static void _usart_port_init(void)
{
    gpio_set_pin_function(PIN_AUX_TX, PINMUX_PA12C_SERCOM2_PAD0);
    gpio_set_pin_function(PIN_AUX_RX, PINMUX_PA13C_SERCOM2_PAD1);
}

/**
 * \brief USART pinmux deinitialization function.
 *
 * Set the TX pin to drive zero, the RX pin as an unused input.
 */
static void _usart_port_deinit(void)
{
    gpio_set_pin_function(PIN_AUX_TX, GPIO_PIN_FUNCTION_OFF);
    gpio_set_pin_direction(PIN_AUX_TX, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_AUX_TX, false);
    gpio_set_pin_function(PIN_AUX_RX, GPIO_PIN_FUNCTION_OFF);
    gpio_set_pin_direction(PIN_AUX_RX, GPIO_DIRECTION_IN);
}

static bool _usart_hw_initialized = false;

/**
 * This function polls the USART rx status. If there is
 * data available, it will unframe it and buffer any
 * resulting packet for later processing.
 */
bool usart_receive(void)
{
    struct io_descriptor* io;
    bool read_smth = false;
    usart_async_get_io_descriptor(&_usart_descriptor, &io);
    while (usart_async_is_rx_not_empty(&_usart_descriptor)) {
        uint8_t buf[USART_RX_BUFFER_SIZE] = {0};
        int32_t n_read = io_read(io, buf, sizeof(buf));
        if (n_read < 0) {
            Abort("USART: negative n_read.");
        }
        if (n_read > USART_RX_BUFFER_SIZE - 10) {
            /*
             * We are overflowing the RX buffer...
             *
             * FUTURE: handle error states. Also, the callback for USART
             * error states can probably be used to handle this state.
             * Investigate this better.
             */
        }
        read_smth = true;
        usart_frame_process_rx(buf, (size_t)n_read);
    };
    return read_smth;
}

void usart_start(void)
{
    if (_usart_hw_initialized) {
        Abort("Tried to run\nusart_start twice!");
    }
    _usart_clock_init();
    usart_async_init(&_usart_descriptor, SERCOM2, _usart_rx_buf, USART_RX_BUFFER_SIZE, (void*)NULL);
    /*
     * 115200, 8N1
     * FUTURE: this is not working? Investigate why. Anyway,
     * the default configuration works just fine...
     *
     * usart_async_set_baud_rate(&_usart_descriptor, 115200);
     * usart_async_set_parity(&_usart_descriptor, USART_PARITY_NONE);
     * usart_async_set_stopbits(&_usart_descriptor, USART_STOP_BITS_ONE);
     */
    usart_hww_init(&_usart_descriptor);
    usart_frame_init();
    _usart_port_init();
    usart_async_enable(&_usart_descriptor);
    _usart_hw_initialized = true;
    usb_processing_init();
}

void usart_stop(void)
{
    usart_async_deinit(&_usart_descriptor);
    _usart_port_deinit();
    _usart_hw_initialized = false;
}
