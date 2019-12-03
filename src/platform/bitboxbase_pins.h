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

#ifndef _BITBOXBASE_PINS_H
#define _BITBOXBASE_PINS_H

#include <hal_gpio.h>

#define PIN_AUX_TX GPIO(GPIO_PORTA, 12)
#define PIN_AUX_RX GPIO(GPIO_PORTA, 13)

#define PIN_I2C_SCL GPIO(GPIO_PORTA, 22)
#define PIN_I2C_SDA GPIO(GPIO_PORTA, 23)

#define PIN_OLED_CMD GPIO(GPIO_PORTB, 17)
#define PIN_OLED_CS GPIO(GPIO_PORTA, 19)
#define PIN_OLED_ON GPIO(GPIO_PORTB, 10)
#define PIN_OLED_RES GPIO(GPIO_PORTB, 16)

#define PIN_SPI_CLK GPIO(GPIO_PORTA, 16)
#define PIN_SPI_MOSI GPIO(GPIO_PORTA, 17)
#define PIN_SPI_MISO GPIO(GPIO_PORTA, 18)

#define PIN_LED_SMALL(i) (GPIO(GPIO_PORTB, (11 + (i))))

#define PIN_BLED0_R (GPIO(GPIO_PORTA, 10))
#define PIN_BLED0_G (GPIO(GPIO_PORTA, 7))
#define PIN_BLED0_B (GPIO(GPIO_PORTA, 6))

#define PIN_BLED1_R (GPIO(GPIO_PORTA, 15))
#define PIN_BLED1_G (GPIO(GPIO_PORTA, 14))
#define PIN_BLED1_B (GPIO(GPIO_PORTA, 11))

#endif
