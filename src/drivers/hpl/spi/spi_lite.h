
/**
 * \file
 *
 * \brief SPI related functionality declaration.
 *
 * Copyright (C) 2017 Atmel Corporation. All rights reserved.
 *
 * \asf_license_start
 *
 * \page License
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * 3. The name of Atmel may not be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * 4. This software may only be redistributed and used in connection with an
 *    Atmel microcontroller product.
 *
 * THIS SOFTWARE IS PROVIDED BY ATMEL "AS IS" AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT ARE
 * EXPRESSLY AND SPECIFICALLY DISCLAIMED. IN NO EVENT SHALL ATMEL BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
 * STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 * ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * \asf_license_stop
 *
 */

#ifndef _SPI_LITE_H_INCLUDED
#define _SPI_LITE_H_INCLUDED

#include <compiler.h>
#include <peripheral_clk_config.h>

/**
 * \addtogroup spi SPI driver
 *
 * \section spi_rev Revision History
 * - v0.0.0.1 Initial Commit
 *
 *@{
 */

#ifdef __cplusplus
extern "C" {
#endif

// Calculate baud register value from requested baudrate value
#ifndef SERCOM0_BAUD_RATE
#define SERCOM0_BAUD_RATE ((float)CONF_GCLK_SERCOM0_CORE_FREQUENCY / (float)(2 * 10000000)) - 1
#endif

#ifndef SERCOM0_RXPO
#define SERCOM0_RXPO 2
#endif

#ifndef SERCOM0_TXPO
#define SERCOM0_TXPO 0
#endif

/**
 * \brief Initialize usart interface
 *
 * \return Initialization status.
 */
int8_t SPI_1_init(void);

/**
 * \brief Enable SPI module
 */
void SPI_1_enable(void);

/**
 * \brief Disable SPI module
 */
void SPI_1_disable(void);

/**
 * \brief Exchange byte in SPI module
 */
uint32_t SPI_1_exchange_data(uint32_t data);

/**
 * \brief Exchange block in SPI module
 */
void SPI_1_exchange_block(void *block, uint8_t size);

/**
 * \brief Write block in SPI module
 */
void SPI_1_write_block(void *block, uint8_t size);

/**
 * \brief Read block in SPI module
 */
void SPI_1_read_block(void *block, uint8_t size);

// Calculate baud register value from requested baudrate value
#ifndef SERCOM3_BAUD_RATE
#define SERCOM3_BAUD_RATE ((float)CONF_GCLK_SERCOM3_CORE_FREQUENCY / (float)(2 * 3000000)) - 1
#endif

#ifndef SERCOM3_RXPO
#define SERCOM3_RXPO 2
#endif

#ifndef SERCOM3_TXPO
#define SERCOM3_TXPO 0
#endif

/**
 * \brief Initialize usart interface
 *
 * \return Initialization status.
 */
int8_t SPI_0_init(void);

/**
 * \brief Enable SPI module
 */
void SPI_0_enable(void);

/**
 * \brief Disable SPI module
 */
void SPI_0_disable(void);

/**
 * \brief Exchange byte in SPI module
 */
uint32_t SPI_0_exchange_data(uint32_t data);

/**
 * \brief Exchange block in SPI module
 */
void SPI_0_exchange_block(void *block, uint8_t size);

/**
 * \brief Write block in SPI module
 */
void SPI_0_write_block(void *block, uint8_t size);

/**
 * \brief Read block in SPI module
 */
void SPI_0_read_block(void *block, uint8_t size);

#ifdef __cplusplus
}
#endif

#endif /* _SPI_LITE_H_INCLUDED */
