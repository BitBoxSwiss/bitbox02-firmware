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

#include "driver_init.h"
#include "bitbox02_pins.h"
#include "memory/memory_shared.h"
#include "util.h"
#include <compiler.h>
#include <hal_sleep.h>
#include <stdint.h>
#include <utils.h>

#define PIN_HIGH 1
#define PIN_LOW 0

struct sha_sync_descriptor HASH_ALGORITHM_0;
struct timer_descriptor TIMER_0;
struct flash_descriptor FLASH_0;
struct i2c_m_sync_desc I2C_0;
struct mci_sync_desc MCI_0;
struct rand_sync_desc RAND_0;
PPUKCL_PARAM pvPUKCLParam;
PUKCL_PARAM PUKCLParam;
struct usart_async_descriptor USART_0;

bool _is_initialized = false;

/**
 * Enables PTC peripheral, clocks and initializes PTC driver
 */
static void _ptc_clock_init(void)
{
    hri_mclk_set_APBDMASK_ADC0_bit(MCLK);
    hri_gclk_write_PCHCTRL_reg(
        GCLK, ADC0_GCLK_ID, CONF_GCLK_ADC0_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
}

/**
 * Enables PUKCC peripheral, clocks and initializes PUKCC driver
 * for hardware ECDSA
 */
static void _ecdsa_init(void)
{
    hri_mclk_set_AHBMASK_PUKCC_bit(MCLK);
}

/**
 * Enables SHA peripheral, clocks and initializes SHA driver
 * for hardware hash function
 */
static void _sha_init(void)
{
    hri_mclk_set_AHBMASK_ICM_bit(MCLK);
    hri_mclk_set_APBCMASK_ICM_bit(MCLK);
    sha_sync_init(&HASH_ALGORITHM_0, ICM);
    sha_sync_enable(&HASH_ALGORITHM_0);
}

/**
 * Enables FLASH memory access
 */
static void _flash_memory_init(void)
{
    hri_mclk_set_AHBMASK_NVMCTRL_bit(MCLK);
    flash_init(&FLASH_0, NVMCTRL);
}

/**
 * Enables Timer peripheral, clocks and initializes Timer driver
 */
static void _timer_peripheral_init(void)
{
    hri_mclk_set_APBAMASK_RTC_bit(MCLK);
    timer_init(&TIMER_0, RTC, _rtc_get_timer());
    timer_start(&TIMER_0);
}

/**
 * Set pins for SPI peripheral
 */
static void _spi_set_pins(void)
{
    // CLK
    gpio_set_pin_direction(PIN_SPI_CLK, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_SPI_CLK, PIN_LOW);
    gpio_set_pin_function(PIN_SPI_CLK, PINMUX_PA16D_SERCOM3_PAD1);
    // MOSI
    gpio_set_pin_direction(PIN_SPI_MOSI, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_SPI_MOSI, PIN_LOW);
    gpio_set_pin_function(PIN_SPI_MOSI, PINMUX_PA17D_SERCOM3_PAD0);
    // MISO
    gpio_set_pin_direction(PIN_SPI_MISO, GPIO_DIRECTION_IN);
    gpio_set_pin_pull_mode(PIN_SPI_MISO, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_SPI_MISO, PINMUX_PA18D_SERCOM3_PAD2);
}

/**
 * Initialize SPI peripheral
 */
static void _spi_init(void)
{
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SERCOM3_GCLK_ID_CORE, CONF_GCLK_SERCOM3_CORE_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SERCOM3_GCLK_ID_SLOW, CONF_GCLK_SERCOM3_SLOW_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
    hri_mclk_set_APBBMASK_SERCOM3_bit(MCLK);
    SPI_OLED_init();
    _spi_set_pins();
    SPI_OLED_enable();
}

static void _spi_mem_clock_init(void)
{
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SERCOM4_GCLK_ID_CORE, CONF_GCLK_SERCOM4_CORE_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SERCOM4_GCLK_ID_SLOW, CONF_GCLK_SERCOM4_SLOW_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));

    hri_mclk_set_APBDMASK_SERCOM4_bit(MCLK);
}

static void _spi_mem_port_init(void)
{
    // CS
    gpio_set_pin_level(PIN_MEM_CS, PIN_LOW);
    gpio_set_pin_direction(PIN_MEM_CS, GPIO_DIRECTION_OUT);
    gpio_set_pin_function(PIN_MEM_CS, GPIO_PIN_FUNCTION_OFF);

    // MISO
    gpio_set_pin_direction(PIN_MEM_MISO, GPIO_DIRECTION_IN);
    gpio_set_pin_pull_mode(PIN_MEM_MISO, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_MEM_MISO, PINMUX_PA13D_SERCOM4_PAD0);

    // CLK
    gpio_set_pin_level(PIN_MEM_CLK, PIN_LOW);
    gpio_set_pin_direction(PIN_MEM_CLK, GPIO_DIRECTION_OUT);
    gpio_set_pin_function(PIN_MEM_CLK, PINMUX_PA12D_SERCOM4_PAD1);

    // MOSI
    gpio_set_pin_level(PIN_MEM_MOSI, PIN_LOW);
    gpio_set_pin_direction(PIN_MEM_MOSI, GPIO_DIRECTION_OUT);
    gpio_set_pin_function(PIN_MEM_MOSI, PINMUX_PA15D_SERCOM4_PAD3);

    // HOLD
    gpio_set_pin_level(PIN_MEM_HOLD, PIN_HIGH);
    gpio_set_pin_direction(PIN_MEM_HOLD, GPIO_DIRECTION_OUT);

    // WP
    gpio_set_pin_level(PIN_MEM_WP, PIN_HIGH);
    gpio_set_pin_direction(PIN_MEM_WP, GPIO_DIRECTION_OUT);
}

/**
 * Initialize SPI 1 peripheral
 */
static void _spi_mem_init(void)
{
    _spi_mem_clock_init();
    SPI_MEM_init();
    _spi_mem_port_init();
}

/**
 * Set pins for I2C peripheral
 */
static void _i2c_set_pins(void)
{
    // SDA
    gpio_set_pin_pull_mode(PIN_I2C_SDA, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_I2C_SDA, PINMUX_PA23D_SERCOM5_PAD0);
    // SCL
    gpio_set_pin_pull_mode(PIN_I2C_SCL, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_I2C_SCL, PINMUX_PA22D_SERCOM5_PAD1);
}

/**
 * Initialize I2C peripheral
 */
static void _i2c_init(void)
{
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SERCOM5_GCLK_ID_CORE, CONF_GCLK_SERCOM5_CORE_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SERCOM5_GCLK_ID_SLOW, CONF_GCLK_SERCOM5_SLOW_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
    hri_mclk_set_APBDMASK_SERCOM5_bit(MCLK);
    i2c_m_sync_init(&I2C_0, SERCOM5);
    _i2c_set_pins();
    i2c_m_sync_enable(&I2C_0);
}

/**
 * Set pins for SD/MMC peripheral
 */
static void _mci_set_pins(void)
{
    // CLK
    gpio_set_pin_direction(PIN_SD_CLK, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_SD_CLK, PIN_LOW);
    gpio_set_pin_pull_mode(PIN_SD_CLK, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_SD_CLK, PINMUX_PB11I_SDHC0_SDCK);
    // CMD
    gpio_set_pin_direction(PIN_SD_CMD, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_SD_CMD, PIN_LOW);
    gpio_set_pin_pull_mode(PIN_SD_CMD, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_SD_CMD, PINMUX_PA08I_SDHC0_SDCMD);
    // DATA0
    gpio_set_pin_direction(PIN_SD_DATA0, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_SD_DATA0, PIN_LOW);
    gpio_set_pin_pull_mode(PIN_SD_DATA0, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_SD_DATA0, PINMUX_PA09I_SDHC0_SDDAT0);
    // DATA1
    gpio_set_pin_direction(PIN_SD_DATA1, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_SD_DATA1, PIN_LOW);
    gpio_set_pin_pull_mode(PIN_SD_DATA1, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_SD_DATA1, PINMUX_PA10I_SDHC0_SDDAT1);
    // DATA2
    gpio_set_pin_direction(PIN_SD_DATA2, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_SD_DATA2, PIN_LOW);
    gpio_set_pin_pull_mode(PIN_SD_DATA2, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_SD_DATA2, PINMUX_PA11I_SDHC0_SDDAT2);
    // DATA3
    gpio_set_pin_direction(PIN_SD_DATA3, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_SD_DATA3, PIN_LOW);
    gpio_set_pin_pull_mode(PIN_SD_DATA3, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_SD_DATA3, PINMUX_PB10I_SDHC0_SDDAT3);

    gpio_set_pin_direction(PIN_SD_CD, GPIO_DIRECTION_IN);
    gpio_set_pin_pull_mode(PIN_SD_CD, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_SD_CD, GPIO_PIN_FUNCTION_OFF);

    gpio_set_pin_direction(PIN_SD_PWON, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_SD_PWON, PIN_HIGH);
    gpio_set_pin_function(PIN_SD_PWON, GPIO_PIN_FUNCTION_OFF);
}

/**
 * Initialize SD/MMC peripheral
 */
static void _mci_init(void)
{
    hri_mclk_set_AHBMASK_SDHC0_bit(MCLK);
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SDHC0_GCLK_ID, CONF_GCLK_SDHC0_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SDHC0_GCLK_ID_SLOW, CONF_GCLK_SDHC0_SLOW_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
    mci_sync_init(&MCI_0, SDHC0);
    _mci_set_pins();
}

/**
 * Initialize delay driver
 */
static void _delay_driver_init(void)
{
    delay_init(SysTick);
}

/**
 * Initialize hardware random number generator
 */
static void _rand_init(void)
{
    hri_mclk_set_APBCMASK_TRNG_bit(MCLK);
    rand_sync_init(&RAND_0, TRNG);
    rand_sync_enable(&RAND_0);
}

/**
 * Set pins for USB peripheral
 */
static void _usb_set_pins(void)
{
    // DM
    gpio_set_pin_direction(PIN_USB_DM, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_USB_DM, PIN_LOW);
    gpio_set_pin_pull_mode(PIN_USB_DM, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_USB_DM, PINMUX_PA24H_USB_DM);
    // DP
    gpio_set_pin_direction(PIN_USB_DP, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_USB_DP, PIN_LOW);
    gpio_set_pin_pull_mode(PIN_USB_DP, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_USB_DP, PINMUX_PA25H_USB_DP);
}

/**
 * The USB module requires a GCLK_USB of 48 MHz ~ 0.25% clock
 * for low speed and full speed operation.
 */
#if (CONF_GCLK_USB_FREQUENCY > (48000000 + 48000000 / 400)) || \
    (CONF_GCLK_USB_FREQUENCY < (48000000 - 48000000 / 400))
#warning USB clock should be 48MHz ~ 0.25% clock, check your configuration!
#endif

/**
 * Initialize USB peripheral
 */
static void _usb_init(void)
{
    hri_gclk_write_PCHCTRL_reg(GCLK, USB_GCLK_ID, CONF_GCLK_USB_SRC | GCLK_PCHCTRL_CHEN);
    hri_mclk_set_AHBMASK_USB_bit(MCLK);
    hri_mclk_set_APBBMASK_USB_bit(MCLK);
    usb_d_init();
    _usb_set_pins();
}

static void _oled_set_pins(void)
{
    gpio_set_pin_direction(PIN_OLED_CS, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_OLED_CS, PIN_HIGH);
    gpio_set_pin_function(PIN_OLED_CS, GPIO_PIN_FUNCTION_OFF);

    gpio_set_pin_direction(PIN_OLED_ON, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_OLED_ON, PIN_LOW);
    gpio_set_pin_function(PIN_OLED_ON, GPIO_PIN_FUNCTION_OFF);

    gpio_set_pin_direction(PIN_OLED_RES, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_OLED_RES, PIN_HIGH);
    gpio_set_pin_function(PIN_OLED_RES, GPIO_PIN_FUNCTION_OFF);

    gpio_set_pin_direction(PIN_OLED_CMD, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_OLED_CMD, PIN_HIGH);
    gpio_set_pin_function(PIN_OLED_CMD, GPIO_PIN_FUNCTION_OFF);
}

static uint8_t USART_0_buffer[USART_0_BUFFER_SIZE];

static void _uart_init(void)
{
    // Clock init
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SERCOM0_GCLK_ID_CORE, CONF_GCLK_SERCOM0_CORE_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));
    hri_gclk_write_PCHCTRL_reg(
        GCLK, SERCOM0_GCLK_ID_SLOW, CONF_GCLK_SERCOM0_SLOW_SRC | (1 << GCLK_PCHCTRL_CHEN_Pos));

    hri_mclk_set_APBAMASK_SERCOM0_bit(MCLK);

    usart_async_init(&USART_0, SERCOM0, USART_0_buffer, USART_0_BUFFER_SIZE, (void*)NULL);

    // Port init
    gpio_set_pin_function(PIN_UART_TX, PINMUX_PA04D_SERCOM0_PAD0);
    gpio_set_pin_function(PIN_UART_RX, PINMUX_PA05D_SERCOM0_PAD1);
    gpio_set_pin_function(PIN_UART_RTS, PINMUX_PA06D_SERCOM0_PAD2);
    gpio_set_pin_function(PIN_UART_CTS, PINMUX_PA07D_SERCOM0_PAD3);
}

void system_init(void)
{
    _oled_set_pins();
    _ptc_clock_init();

    _timer_peripheral_init();
    _delay_driver_init();

    // OLED
    _spi_init();
    // ATECC608A
    _i2c_init();
    // uSD
    _mci_init();

    // Hardware crypto
    _ecdsa_init();
    _sha_init();
    _rand_init();
    // Flash
    _flash_memory_init();
    // USB
    _usb_init();

    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        // External MX25 flash memory
        _spi_mem_init();
        // DA14531
        _uart_init();
    }

    _is_initialized = true;
}

void bootloader_init(void)
{
    _oled_set_pins();
    _ptc_clock_init();

#ifdef BOOTLOADER_DEVDEVICE
    // Only needed for qtouch, which is only needed in the devdevice bootloader.
    _timer_peripheral_init();
#endif
    _delay_driver_init();

    // OLED
    _spi_init();

    // Hardware crypto
    _ecdsa_init();
    _sha_init();
    _rand_init();
    // Flash
    _flash_memory_init();
    // USB
    _usb_init();

    if (memory_get_platform() == MEMORY_PLATFORM_BITBOX02_PLUS) {
        // External MX25 flash memory
        _spi_mem_init();
        // DA14531
        _uart_init();
    }

    _is_initialized = true;
}

void system_close_interfaces(void)
{
    if (!_is_initialized) {
        return;
    }
    // uSD
    mci_sync_deinit(&MCI_0);
    // ATECC608A
    i2c_m_sync_deinit(&I2C_0);
    // OLED interface bus
    // Display remains on last screen
    SPI_OLED_disable();
    // Flash
    flash_deinit(&FLASH_0);
    // USB
    usb_d_deinit();
    // Hardware crypto
    sha_sync_deinit(&HASH_ALGORITHM_0);
    rand_sync_deinit(&RAND_0);
}

void bootloader_close_interfaces(void)
{
    if (!_is_initialized) {
        return;
    }
    // OLED interface bus
    // Display remains on last screen
    SPI_OLED_disable();
    // Flash
    flash_deinit(&FLASH_0);
    // USB
    usb_d_deinit();
    // Hardware crypto
    sha_sync_deinit(&HASH_ALGORITHM_0);
    rand_sync_deinit(&RAND_0);
}
