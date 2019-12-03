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

#ifndef _BITBOX02_PINS_H_
#define _BITBOX02_PINS_H_

#include <hal_gpio.h>

#define PIN_OLED_ON GPIO(GPIO_PORTA, 0)
#define PIN_SD_CD GPIO(GPIO_PORTA, 1)
#define PIN_SD_PWON GPIO(GPIO_PORTA, 2)
#define PIN_SD_CMD GPIO(GPIO_PORTA, 8)
#define PIN_SD_DATA0 GPIO(GPIO_PORTA, 9)
#define PIN_SD_DATA1 GPIO(GPIO_PORTA, 10)
#define PIN_SD_DATA2 GPIO(GPIO_PORTA, 11)
#define PIN_SPI_CLK GPIO(GPIO_PORTA, 16)
#define PIN_SPI_MOSI GPIO(GPIO_PORTA, 17)
#define PIN_SPI_MISO GPIO(GPIO_PORTA, 18)
#define PIN_OLED_CS GPIO(GPIO_PORTA, 19)
#define PIN_I2C_SCL GPIO(GPIO_PORTA, 22)
#define PIN_I2C_SDA GPIO(GPIO_PORTA, 23)
#define PIN_USB_DM GPIO(GPIO_PORTA, 24)
#define PIN_USB_DP GPIO(GPIO_PORTA, 25)
#define PIN_SD_DATA3 GPIO(GPIO_PORTB, 10)
#define PIN_SD_CLK GPIO(GPIO_PORTB, 11)
#define PIN_OLED_RES GPIO(GPIO_PORTB, 16)
#define PIN_OLED_CMD GPIO(GPIO_PORTB, 17)

#endif
