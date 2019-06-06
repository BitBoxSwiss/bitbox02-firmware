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

#ifndef _I2C_ECC_H_
#define _I2C_ECC_H_

#ifndef TESTING

#include <stdint.h>

// Chip commands
#define I2C_ECC_CHIP_RESET 0x00
#define I2C_ECC_CHIP_SLEEP 0x01
#define I2C_ECC_CHIP_IDLE 0x02
#define I2C_ECC_CHIP_CMD 0x03

// Settings
#define I2C_ECC_ADDR 0xC0
#define I2C_ECC_TWLO 70u // tWLO min 60 us
#define I2C_ECC_TWHI 1550u // tWHI min 1500 us
#define I2C_ECC_RETRIES 25u
#define I2C_ECC_RETRY_DELAY 2u

// Status codes
#define I2C_ECC_SUCCESS 0x00 // Command successful
#define I2C_ECC_ERR_VERIFY 0x01 // Checkmac or verify error
#define I2C_ECC_ERR_PARSE 0x03 // Command parse error
#define I2C_ECC_ERR_ECC 0x05 // ECC fault
#define I2C_ECC_ERR_SELFTEST 0x07 // Self-test error
#define I2C_ECC_ERR_EXECUTION 0x0F // Command execution error
#define I2C_ECC_WAKE 0x11 // Received a proper wake token.
#define I2C_ECC_ERR_I2C 0xCC // I2C communication error
#define I2C_ECC_ERR_WATCHDOG 0xEE // Watchdog about to expire
#define I2C_ECC_ERR_CRC 0xFF // CRC or other communications error

// the following functions are not to be used directly, and serve as the
// communication backend for securechip/securechip.c.
uint8_t i2c_ecc_write(uint8_t* txdata, uint32_t txlen);
uint8_t i2c_ecc_idle(void);
uint8_t i2c_ecc_sleep(void);
uint8_t i2c_ecc_wake(void);
uint8_t i2c_ecc_read(uint8_t* rxdata, uint32_t rxlen);

#endif
#endif
