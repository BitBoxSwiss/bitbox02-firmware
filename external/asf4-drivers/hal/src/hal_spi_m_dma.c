/**
 * \file
 *
 * \brief I/O SPI DMA related functionality implementation.
 *
 * Copyright (c) 2016-2018 Microchip Technology Inc. and its subsidiaries.
 *
 * \asf_license_start
 *
 * \page License
 *
 * Subject to your compliance with these terms, you may use Microchip
 * software and any derivatives exclusively with Microchip products.
 * It is your responsibility to comply with third party license terms applicable
 * to your use of third party software (including open source software) that
 * may accompany Microchip software.
 *
 * THIS SOFTWARE IS SUPPLIED BY MICROCHIP "AS IS". NO WARRANTIES,
 * WHETHER EXPRESS, IMPLIED OR STATUTORY, APPLY TO THIS SOFTWARE,
 * INCLUDING ANY IMPLIED WARRANTIES OF NON-INFRINGEMENT, MERCHANTABILITY,
 * AND FITNESS FOR A PARTICULAR PURPOSE. IN NO EVENT WILL MICROCHIP BE
 * LIABLE FOR ANY INDIRECT, SPECIAL, PUNITIVE, INCIDENTAL OR CONSEQUENTIAL
 * LOSS, DAMAGE, COST OR EXPENSE OF ANY KIND WHATSOEVER RELATED TO THE
 * SOFTWARE, HOWEVER CAUSED, EVEN IF MICROCHIP HAS BEEN ADVISED OF THE
 * POSSIBILITY OR THE DAMAGES ARE FORESEEABLE.  TO THE FULLEST EXTENT
 * ALLOWED BY LAW, MICROCHIP'S TOTAL LIABILITY ON ALL CLAIMS IN ANY WAY
 * RELATED TO THIS SOFTWARE WILL NOT EXCEED THE AMOUNT OF FEES, IF ANY,
 * THAT YOU HAVE PAID DIRECTLY TO MICROCHIP FOR THIS SOFTWARE.
 *
 * \asf_license_stop
 *
 */

#include "hal_atomic.h"
#include "hal_spi_m_dma.h"
#include <utils_assert.h>
#include <utils.h>

/**
 * \brief Driver version
 */
#define SPI_DRIVER_VERSION 0x00000001u

static int32_t _spi_m_dma_io_write(struct io_descriptor *const io, const uint8_t *const buf, const uint16_t length);
static int32_t _spi_m_dma_io_read(struct io_descriptor *const io, uint8_t *const buf, const uint16_t length);

/**
 *  \brief Initialize the SPI HAL instance function pointer for HPL APIs.
 */
void spi_m_dma_set_func_ptr(struct spi_m_dma_descriptor *spi, void *const func)
{
	ASSERT(spi);
	spi->func = (struct _spi_m_dma_hpl_interface *)func;
}

int32_t spi_m_dma_init(struct spi_m_dma_descriptor *spi, void *const hw)
{
	int32_t rc = 0;
	ASSERT(spi && hw);
	spi->dev.prvt = (void *)hw;
	rc            = _spi_m_dma_init(&spi->dev, hw);

	if (rc) {
		return rc;
	}

	spi->io.read  = _spi_m_dma_io_read;
	spi->io.write = _spi_m_dma_io_write;

	return ERR_NONE;
}

void spi_m_dma_deinit(struct spi_m_dma_descriptor *spi)
{
	ASSERT(spi);
	_spi_m_dma_deinit(&spi->dev);
}

void spi_m_dma_enable(struct spi_m_dma_descriptor *spi)
{
	ASSERT(spi);
	_spi_m_dma_enable(&spi->dev);
}

void spi_m_dma_disable(struct spi_m_dma_descriptor *spi)
{
	ASSERT(spi);
	_spi_m_dma_disable(&spi->dev);
}

int32_t spi_m_dma_set_baudrate(struct spi_m_dma_descriptor *spi, const uint32_t baud_val)
{
	ASSERT(spi);
	return _spi_m_dma_set_baudrate(&spi->dev, baud_val);
}

int32_t spi_m_dma_set_mode(struct spi_m_dma_descriptor *spi, const enum spi_transfer_mode mode)
{
	ASSERT(spi);
	return _spi_m_dma_set_mode(&spi->dev, mode);
}

int32_t spi_m_dma_set_char_size(struct spi_m_dma_descriptor *spi, const enum spi_char_size char_size)
{
	ASSERT(spi);
	return _spi_m_dma_set_char_size(&spi->dev, char_size);
}

int32_t spi_m_dma_set_data_order(struct spi_m_dma_descriptor *spi, const enum spi_data_order dord)
{
	ASSERT(spi);
	return _spi_m_dma_set_data_order(&spi->dev, dord);
}

/** \brief Do SPI read in background
 *
 *  It never blocks and return quickly, user check status or set callback to
 *  know when data is ready to process.
 *
 *  \param[in, out] spi Pointer to the HAL SPI instance.
 *  \param[out] p_buf Pointer to the buffer to store read data.
 *  \param[in] size Size of the data in number of characters.
 *  \return ERR_NONE on success, or an error code on failure.
 *  \retval ERR_NONE Success, transfer started.
 *  \retval ERR_BUSY Busy.
 */
static int32_t _spi_m_dma_io_read(struct io_descriptor *io, uint8_t *const buf, const uint16_t length)
{
	ASSERT(io);

	struct spi_m_dma_descriptor *spi = CONTAINER_OF(io, struct spi_m_dma_descriptor, io);
	return _spi_m_dma_transfer(&spi->dev, NULL, buf, length);
}

/** \brief Do SPI data write in background
 *
 *  The data read back is discarded.
 *
 *  It never blocks and return quickly, user check status or set callback to
 *  know when data is sent.
 *
 *  \param[in, out] spi Pointer to the HAL SPI instance.
 *  \param[in] p_buf Pointer to the buffer to store data to write.
 *  \param[in] size Size of the data in number of characters.
 *
 *  \return ERR_NONE on success, or an error code on failure.
 *  \retval ERR_NONE Success, transfer started.
 *  \retval ERR_BUSY Busy.
 */
static int32_t _spi_m_dma_io_write(struct io_descriptor *io, const uint8_t *const buf, const uint16_t length)
{
	ASSERT(io);

	struct spi_m_dma_descriptor *spi = CONTAINER_OF(io, struct spi_m_dma_descriptor, io);
	return _spi_m_dma_transfer(&spi->dev, buf, NULL, length);
}

int32_t spi_m_dma_transfer(struct spi_m_dma_descriptor *spi, uint8_t const *txbuf, uint8_t *const rxbuf,
                           const uint16_t length)
{
	ASSERT(spi);
	return _spi_m_dma_transfer(&spi->dev, txbuf, rxbuf, length);
}

void spi_m_dma_register_callback(struct spi_m_dma_descriptor *spi, const enum spi_m_dma_cb_type type,
                                 spi_m_dma_cb_t func)
{
	ASSERT(spi);
	_spi_m_dma_register_callback(&spi->dev, (enum _spi_dma_dev_cb_type)type, func);
}

int32_t spi_m_dma_get_io_descriptor(struct spi_m_dma_descriptor *const spi, struct io_descriptor **io)
{
	ASSERT(spi && io);
	*io = &spi->io;

	return 0;
}

uint32_t spi_m_dma_get_version(void)
{
	return SPI_DRIVER_VERSION;
}
