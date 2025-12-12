// SPDX-License-Identifier: Apache-2.0

// THIS IS A GENERATED FILE, MODIFY AS LITTLE AS POSSIBLE

#ifndef _DRIVER_INIT_H_
#define _DRIVER_INIT_H_

#ifndef TESTING
    #include "CryptoLib_Headers_pb.h"
    #include <bitbox02_pins.h>
    #include <hal_atomic.h>
    #include <hal_delay.h>
    #include <hal_flash.h>
    #include <hal_i2c_m_sync.h>
    #include <hal_init.h>
    #include <hal_io.h>
    #include <hal_mci_sync.h>
    #include <hal_pac.h>
    #include <hal_rand_sync.h>
    #include <hal_sha_sync.h>
    #include <hal_sleep.h>
    #include <hal_timer.h>
    #include <hal_usart_async.h>
    #include <hal_usb_device.h>
    #include <hpl_rtc_base.h>
    #include <sd_mmc.h>
    #include <spi_lite.h>
#endif
#include <utils.h>
#include <utils_assert.h>

#include "platform_config.h"

#define SHA256_DIGEST_LENGTH 32
// 64 is our typical packet size and it gets a little bit longer over UART due to framing. Set the
// buffer size so we can handle at least one whole frame. Must be a power of 2.
#define USART_0_BUFFER_SIZE 512

#ifndef TESTING
extern struct timer_descriptor TIMER_0;
extern struct i2c_m_sync_desc I2C_0;
extern struct mci_sync_desc MCI_0;
extern struct aes_sync_descriptor CRYPTOGRAPHY_0;
extern struct sha_sync_descriptor HASH_ALGORITHM_0;
extern struct flash_descriptor FLASH_0;
extern struct rand_sync_desc RAND_0;
extern PPUKCL_PARAM pvPUKCLParam;
extern PUKCL_PARAM PUKCLParam;
extern struct usart_async_descriptor USART_0;
#endif

/**
 * Close peripheral interfaces
 */
void system_close_interfaces(void);

/**
 * Perform system initialization, initialize pins and clocks for
 * peripherals
 */
void system_init(void);

/**
 * Close peripheral interfaces
 */
void bootloader_close_interfaces(void);

/**
 * Perform system initialization, initialize pins and clocks for
 * peripherals
 */
void bootloader_init(void);

#endif
