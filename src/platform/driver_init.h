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

// THIS IS A GENERATED FILE, MODIFY AS LITTLE AS POSSIBLE

#ifndef _DRIVER_INIT_H_
#define _DRIVER_INIT_H_

#include "CryptoLib_Headers_pb.h"
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
#include <utils.h>

#include "platform_config.h"

#if PLATFORM_BITBOX02 == 1
#include <bitbox02_pins.h>
#elif PLATFORM_BITBOXBASE == 1
#include <bitboxbase_pins.h>
#endif

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
