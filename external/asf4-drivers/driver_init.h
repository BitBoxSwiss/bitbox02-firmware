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

#ifndef _DRIVER_INIT_H_
#define _DRIVER_INIT_H_

#include <utils.h>
#include <hal_atomic.h>
#include <hal_delay.h>
#include <hal_gpio.h>
#include <hal_init.h>
#include <hal_io.h>
#include <hal_sleep.h>
#include <hal_timer.h>
#include <spi_lite.h>
#include <hpl_rtc_base.h>
#include <hal_i2c_m_sync.h>
#include <hal_usart_async.h>
#include <hal_mci_sync.h>
#include <hal_usb_device.h>
#include <hal_sha_sync.h>
#include <hal_rand_sync.h>
#include <hal_flash.h>
#include <hal_pac.h>
#include "CryptoLib_Headers_pb.h"

#define PIN_OLED_ON  GPIO(GPIO_PORTA, 0)
#define PIN_SD_CD    GPIO(GPIO_PORTA, 1)
#define PIN_SD_PWON  GPIO(GPIO_PORTA, 2)
#define PIN_SD_CMD   GPIO(GPIO_PORTA, 8)
#define PIN_SD_DATA0 GPIO(GPIO_PORTA, 9)
#define PIN_SD_DATA1 GPIO(GPIO_PORTA, 10)
#define PIN_SD_DATA2 GPIO(GPIO_PORTA, 11)
#define PIN_SPI_CLK  GPIO(GPIO_PORTA, 16)
#define PIN_SPI_MOSI GPIO(GPIO_PORTA, 17)
#define PIN_SPI_MISO GPIO(GPIO_PORTA, 18)
#define PIN_OLED_CS  GPIO(GPIO_PORTA, 19)
#define PIN_I2C_SCL  GPIO(GPIO_PORTA, 22)
#define PIN_I2C_SDA  GPIO(GPIO_PORTA, 23)
#define PIN_USB_DM   GPIO(GPIO_PORTA, 24)
#define PIN_USB_DP   GPIO(GPIO_PORTA, 25)
#define PIN_SD_DATA3 GPIO(GPIO_PORTB, 10)
#define PIN_SD_CLK   GPIO(GPIO_PORTB, 11)
#define PIN_OLED_RES GPIO(GPIO_PORTB, 16)
#define PIN_OLED_CMD GPIO(GPIO_PORTB, 17)

#define SHA256_DIGEST_LENGTH 32

extern struct timer_descriptor TIMER_0;
extern struct i2c_m_sync_desc I2C_0;
extern struct mci_sync_desc MCI_0;
extern struct aes_sync_descriptor CRYPTOGRAPHY_0;
extern struct sha_sync_descriptor HASH_ALGORITHM_0;
extern struct flash_descriptor FLASH_0;
extern struct rand_sync_desc RAND_0;
extern PPUKCL_PARAM pvPUKCLParam;
extern PUKCL_PARAM PUKCLParam;

/**
 * Close peripheral interfaces
 */
void system_close_interfaces(void);

/**
 * Perform system initialization, initialize pins and clocks for
 * peripherals
 */
void system_init(void);

#endif
