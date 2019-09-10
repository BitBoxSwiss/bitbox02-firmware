
/**
 * \file
 *
 * \brief SPI related functionality implementation.
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

#include "spi_lite.h"
#include <utils_assert.h>

/**
 * \brief Initialize SPI interface
 */
int8_t SPI_1_init(void)
{

	hri_sercomspi_wait_for_sync(SERCOM0, SERCOM_SPI_SYNCBUSY_SWRST);
	if (hri_sercomspi_get_CTRLA_ENABLE_bit(SERCOM0)) {
		return ERR_DENIED;
	}

	hri_sercomspi_write_CTRLA_reg(
	    SERCOM0,
	    0 << SERCOM_SPI_CTRLA_DORD_Pos           /* Data Order: disabled */
	        | 0 << SERCOM_SPI_CTRLA_CPOL_Pos     /* Clock Polarity: disabled */
	        | 0 << SERCOM_SPI_CTRLA_CPHA_Pos     /* Clock Phase: disabled */
	        | 0 << SERCOM_SPI_CTRLA_FORM_Pos     /* Frame Format: 0 */
	        | 0 << SERCOM_SPI_CTRLA_IBON_Pos     /* Immediate Buffer Overflow Notification: disabled */
	        | 0 << SERCOM_SPI_CTRLA_RUNSTDBY_Pos /* Run In Standby: disabled */
	        | 3 << SERCOM_SPI_CTRLA_MODE_Pos);   /* Operating Mode: 3 */

	hri_sercomspi_write_CTRLA_DOPO_bf(SERCOM0, SERCOM0_TXPO);
	hri_sercomspi_write_CTRLA_DIPO_bf(SERCOM0, SERCOM0_RXPO);

	hri_sercomspi_write_CTRLB_reg(SERCOM0,
	                              1 << SERCOM_SPI_CTRLB_RXEN_Pos          /* Receiver Enable: enabled */
	                                  | 0 << SERCOM_SPI_CTRLB_MSSEN_Pos   /* Master Slave Select Enabl: disabled */
	                                  | 0 << SERCOM_SPI_CTRLB_AMODE_Pos   /* Address Mode: 0 */
	                                  | 0 << SERCOM_SPI_CTRLB_SSDE_Pos    /* Slave Select Low Detect Enable: disabled */
	                                  | 0 << SERCOM_SPI_CTRLB_PLOADEN_Pos /* Slave Data Preload Enable: disabled */
	                                  | 0);                               /* Character Size: 0 */

	hri_sercomspi_write_BAUD_reg(SERCOM0, SERCOM0_BAUD_RATE);

	// hri_sercomspi_write_DBGCTRL_reg(SERCOM0,0 << SERCOM_SPI_DBGCTRL_DBGSTOP_Pos); /* Debug Stop Mode: disabled */

	// hri_sercomspi_write_INTEN_reg(SERCOM0,0 << SERCOM_SPI_INTENSET_ERROR_Pos /* Error Interrupt Enable: disabled */
	//		 | 0 << SERCOM_SPI_INTENSET_SSL_Pos /* Slave Select Low Interrupt Enable: disabled */
	//		 | 0 << SERCOM_SPI_INTENSET_RXC_Pos /* Receive Complete Interrupt Enable: disabled */
	//		 | 0 << SERCOM_SPI_INTENSET_TXC_Pos /* Transmit Complete Interrupt Enable: disabled */
	//		 | 0 << SERCOM_SPI_INTENSET_DRE_Pos); /* Data Register Empty Interrupt Enable: disabled */

	hri_sercomspi_write_CTRLA_ENABLE_bit(SERCOM0, 1 << SERCOM_SPI_CTRLA_ENABLE_Pos); /* Enable: enabled */

	return 0;
}

/**
 * \brief Enable SPI module
 */
void SPI_1_enable(void)
{
	hri_sercomspi_set_CTRLA_ENABLE_bit(SERCOM0);
}

/**
 * \brief Disable SPI module
 */
void SPI_1_disable(void)
{
	hri_sercomspi_clear_CTRLA_ENABLE_bit(SERCOM0);
}

/**
 * \brief Exchange_byte in SPI module
 */
uint32_t SPI_1_exchange_data(uint32_t data)
{
	/* If settings are not applied (pending), we can not go on */
	if (hri_sercomspi_is_syncing(
	        SERCOM0, (SERCOM_SPI_SYNCBUSY_SWRST | SERCOM_SPI_SYNCBUSY_ENABLE | SERCOM_SPI_SYNCBUSY_CTRLB))) {
		return ERR_BUSY;
	}

	hri_sercomspi_write_DATA_reg(SERCOM0, data);
	while (!(hri_sercomspi_read_INTFLAG_reg(SERCOM0) & SERCOM_SPI_INTFLAG_RXC))
		;
	return hri_sercomspi_read_DATA_reg(SERCOM0);
}

void SPI_1_exchange_block(void *block, uint8_t size)
{

	uint8_t *b = (uint8_t *)block;

	while (size--) {
		hri_sercomspi_write_DATA_reg(SERCOM0, *b);
		while (!(hri_sercomspi_read_INTFLAG_reg(SERCOM0) & SERCOM_SPI_INTFLAG_RXC))
			;
		*b = hri_sercomspi_read_DATA_reg(SERCOM0);
		b++;
	}
}

void SPI_1_write_block(void *block, uint8_t size)
{

	uint8_t *b = (uint8_t *)block;
	while (size--) {
		hri_sercomspi_write_DATA_reg(SERCOM0, *b);
		while (!(hri_sercomspi_read_INTFLAG_reg(SERCOM0) & SERCOM_SPI_INTFLAG_TXC))
			;
		b++;
	}
}

void SPI_1_read_block(void *block, uint8_t size)
{

	uint8_t *b = (uint8_t *)block;
	while (size--) {
		hri_sercomspi_write_DATA_reg(SERCOM0, 0);
		while (!(hri_sercomspi_read_INTFLAG_reg(SERCOM0) & SERCOM_SPI_INTFLAG_RXC))
			;
		*b = hri_sercomspi_read_DATA_reg(SERCOM0);
		b++;
	}
}

/**
 * \brief Initialize SPI interface
 */
int8_t SPI_0_init(void)
{

	hri_sercomspi_wait_for_sync(SERCOM3, SERCOM_SPI_SYNCBUSY_SWRST);
	if (hri_sercomspi_get_CTRLA_ENABLE_bit(SERCOM3)) {
		return ERR_DENIED;
	}

	hri_sercomspi_write_CTRLA_reg(
	    SERCOM3,
	    0 << SERCOM_SPI_CTRLA_DORD_Pos           /* Data Order: disabled */
	        | 0 << SERCOM_SPI_CTRLA_CPOL_Pos     /* Clock Polarity: disabled */
	        | 0 << SERCOM_SPI_CTRLA_CPHA_Pos     /* Clock Phase: disabled */
	        | 0 << SERCOM_SPI_CTRLA_FORM_Pos     /* Frame Format: 0 */
	        | 0 << SERCOM_SPI_CTRLA_IBON_Pos     /* Immediate Buffer Overflow Notification: disabled */
	        | 0 << SERCOM_SPI_CTRLA_RUNSTDBY_Pos /* Run In Standby: disabled */
	        | 3 << SERCOM_SPI_CTRLA_MODE_Pos);   /* Operating Mode: 3 */

	hri_sercomspi_write_CTRLA_DOPO_bf(SERCOM3, SERCOM3_TXPO);
	hri_sercomspi_write_CTRLA_DIPO_bf(SERCOM3, SERCOM3_RXPO);

	hri_sercomspi_write_CTRLB_reg(SERCOM3,
	                              1 << SERCOM_SPI_CTRLB_RXEN_Pos          /* Receiver Enable: enabled */
	                                  | 0 << SERCOM_SPI_CTRLB_MSSEN_Pos   /* Master Slave Select Enabl: disabled */
	                                  | 0 << SERCOM_SPI_CTRLB_AMODE_Pos   /* Address Mode: 0 */
	                                  | 0 << SERCOM_SPI_CTRLB_SSDE_Pos    /* Slave Select Low Detect Enable: disabled */
	                                  | 0 << SERCOM_SPI_CTRLB_PLOADEN_Pos /* Slave Data Preload Enable: disabled */
	                                  | 0);                               /* Character Size: 0 */

	hri_sercomspi_write_BAUD_reg(SERCOM3, SERCOM3_BAUD_RATE);

	// hri_sercomspi_write_DBGCTRL_reg(SERCOM3,0 << SERCOM_SPI_DBGCTRL_DBGSTOP_Pos); /* Debug Stop Mode: disabled */

	// hri_sercomspi_write_INTEN_reg(SERCOM3,0 << SERCOM_SPI_INTENSET_ERROR_Pos /* Error Interrupt Enable: disabled */
	//		 | 0 << SERCOM_SPI_INTENSET_SSL_Pos /* Slave Select Low Interrupt Enable: disabled */
	//		 | 0 << SERCOM_SPI_INTENSET_RXC_Pos /* Receive Complete Interrupt Enable: disabled */
	//		 | 0 << SERCOM_SPI_INTENSET_TXC_Pos /* Transmit Complete Interrupt Enable: disabled */
	//		 | 0 << SERCOM_SPI_INTENSET_DRE_Pos); /* Data Register Empty Interrupt Enable: disabled */

	hri_sercomspi_write_CTRLA_ENABLE_bit(SERCOM3, 1 << SERCOM_SPI_CTRLA_ENABLE_Pos); /* Enable: enabled */

	return 0;
}

/**
 * \brief Enable SPI module
 */
void SPI_0_enable(void)
{
	hri_sercomspi_set_CTRLA_ENABLE_bit(SERCOM3);
}

/**
 * \brief Disable SPI module
 */
void SPI_0_disable(void)
{
	hri_sercomspi_clear_CTRLA_ENABLE_bit(SERCOM3);
}

/**
 * \brief Exchange_byte in SPI module
 */
uint32_t SPI_0_exchange_data(uint32_t data)
{
	/* If settings are not applied (pending), we can not go on */
	if (hri_sercomspi_is_syncing(
	        SERCOM3, (SERCOM_SPI_SYNCBUSY_SWRST | SERCOM_SPI_SYNCBUSY_ENABLE | SERCOM_SPI_SYNCBUSY_CTRLB))) {
		return ERR_BUSY;
	}

	hri_sercomspi_write_DATA_reg(SERCOM3, data);
	while (!(hri_sercomspi_read_INTFLAG_reg(SERCOM3) & SERCOM_SPI_INTFLAG_RXC))
		;
	return hri_sercomspi_read_DATA_reg(SERCOM3);
}

void SPI_0_exchange_block(void *block, uint8_t size)
{

	uint8_t *b = (uint8_t *)block;

	while (size--) {
		hri_sercomspi_write_DATA_reg(SERCOM3, *b);
		while (!(hri_sercomspi_read_INTFLAG_reg(SERCOM3) & SERCOM_SPI_INTFLAG_RXC))
			;
		*b = hri_sercomspi_read_DATA_reg(SERCOM3);
		b++;
	}
}

void SPI_0_write_block(void *block, uint8_t size)
{

	uint8_t *b = (uint8_t *)block;
	while (size--) {
		hri_sercomspi_write_DATA_reg(SERCOM3, *b);
		while (!(hri_sercomspi_read_INTFLAG_reg(SERCOM3) & SERCOM_SPI_INTFLAG_TXC))
			;
		b++;
	}
}

void SPI_0_read_block(void *block, uint8_t size)
{

	uint8_t *b = (uint8_t *)block;
	while (size--) {
		hri_sercomspi_write_DATA_reg(SERCOM3, 0);
		while (!(hri_sercomspi_read_INTFLAG_reg(SERCOM3) & SERCOM_SPI_INTFLAG_RXC))
			;
		*b = hri_sercomspi_read_DATA_reg(SERCOM3);
		b++;
	}
}
