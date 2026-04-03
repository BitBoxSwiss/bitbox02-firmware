// SPDX-License-Identifier: Apache-2.0

#ifndef TESTING

    #include "i2c_ecc.h"
    #include "driver_init.h"
    #include "util.h"
    #include <string.h>

static uint8_t _i2c_ecc_transfer(uint8_t* data, uint32_t len, uint16_t flags)
{
    struct _i2c_m_msg packet;

    packet.addr = I2C_ECC_ADDR >> 1;
    packet.len = (int32_t)len;
    packet.buffer = data;
    packet.flags = flags;

    return i2c_m_sync_transfer(&I2C_0, &packet) == I2C_OK ? 0 : 1;
}

uint8_t i2c_ecc_read_one_shot(uint8_t* rxdata, uint32_t rxlen)
{
    return _i2c_ecc_transfer(rxdata, rxlen, I2C_M_SEVEN | I2C_M_RD | I2C_M_STOP);
}

uint8_t i2c_ecc_write_one_shot(uint8_t* txdata, uint32_t txlen)
{
    return _i2c_ecc_transfer(txdata, txlen, I2C_M_SEVEN | I2C_M_STOP);
}

uint8_t i2c_ecc_read(uint8_t* rxdata, uint32_t rxlen)
{
    uint8_t retries = I2C_ECC_RETRIES;
    uint8_t ret;

    do {
        ret = i2c_ecc_read_one_shot(rxdata, rxlen);
        delay_ms(I2C_ECC_RETRY_DELAY);
    } while (retries-- && ret != 0);

    return ret;
}

uint8_t i2c_ecc_write(uint8_t* txdata, uint32_t txlen)
{
    uint8_t retries = I2C_ECC_RETRIES;
    uint8_t ret;

    do {
        ret = i2c_ecc_write_one_shot(txdata, txlen);
        delay_ms(I2C_ECC_RETRY_DELAY);
    } while (retries-- && ret != 0);

    return ret;
}

uint8_t i2c_ecc_idle(void)
{
    uint8_t cmd = I2C_ECC_CHIP_IDLE;
    return i2c_ecc_write(&cmd, 1);
}

uint8_t i2c_ecc_sleep(void)
{
    uint8_t cmd = I2C_ECC_CHIP_SLEEP;
    return i2c_ecc_write(&cmd, 1);
}

uint8_t i2c_ecc_wake_one_shot(void)
{
    uint8_t buf[] = {0x00, 0x00, 0x00, 0x00};
    const uint8_t expected[] = {0x04, I2C_ECC_WAKE, 0x33, 0x43};

    // Manually set SDA level
    gpio_set_pin_direction(PIN_I2C_SDA, GPIO_DIRECTION_OUT);
    gpio_set_pin_level(PIN_I2C_SDA, 1);
    gpio_set_pin_function(PIN_I2C_SDA, GPIO_PIN_FUNCTION_OFF);

    // Hold SDA low for tWLO
    gpio_set_pin_level(PIN_I2C_SDA, 0);
    delay_us(I2C_ECC_TWLO);

    // Hold SDA high for tWHI and return SDA to I2C control.
    gpio_set_pin_level(PIN_I2C_SDA, 1);
    gpio_set_pin_direction(PIN_I2C_SDA, GPIO_DIRECTION_OFF);
    gpio_set_pin_pull_mode(PIN_I2C_SDA, GPIO_PULL_OFF);
    gpio_set_pin_function(PIN_I2C_SDA, PINMUX_PA23D_SERCOM5_PAD0);
    delay_us(I2C_ECC_TWHI);

    if (i2c_ecc_read_one_shot(buf, sizeof(buf)) != 0) {
        return I2C_ECC_ERR_I2C;
    }
    if (!MEMEQ(buf, expected, sizeof(expected))) {
        return I2C_ECC_ERR_I2C;
    }
    return I2C_ECC_WAKE;
}

uint8_t i2c_ecc_wake(void)
{
    uint8_t retries = I2C_ECC_RETRIES;
    uint8_t ret;

    do {
        ret = i2c_ecc_wake_one_shot();
        delay_ms(I2C_ECC_RETRY_DELAY);
    } while (retries-- && ret != I2C_ECC_WAKE);

    return ret;
}
#else
    #include "i2c_ecc.h"
typedef int make_iso_compilers_happy;
#endif
