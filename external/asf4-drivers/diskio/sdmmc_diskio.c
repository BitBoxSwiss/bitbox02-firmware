/**
 * \file
 *
 * \brief Common SDMMC Diskio implementation
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
/*
 * Support and FAQ: visit <a href="https://www.microchip.com/support/">Microchip Support</a>
 */

#include <sd_mmc.h>
#include "diskio.h"

/*
 * \brief Verify SD/MMC is ready to use.
 *
 * \param slot slot number (0..).
 *
 * \return CTRL_GOOD or card status like CTRL_NO_PRESENT, CTRL_BUSY, CTRL_FAIL.
 *
 */
static ctrl_status_t sd_mmc_test_unit_ready(uint8_t slot)
{
	switch (sd_mmc_check(slot)) {
	case SD_MMC_OK:
		if (sd_mmc_get_type(slot) & (CARD_TYPE_SD | CARD_TYPE_MMC)) {
			return CTRL_GOOD;
		} else // It is not a memory card
			return CTRL_NO_PRESENT;
	case SD_MMC_INIT_ONGOING:
		return CTRL_BUSY;

	case SD_MMC_ERR_NO_CARD:
		return CTRL_NO_PRESENT;
	default:
		return CTRL_FAIL;
	}
}

/*
 * \brief Reads the memory capacity.
 *
 * \param slot slot number (0..).
 * \param nb_sector pointer to hold the size.
 *
 * \return CTRL_GOOD or card status like CTRL_NO_PRESENT, CTRL_BUSY, CTRL_FAIL.
 *
 */
static ctrl_status_t sd_mmc_read_capacity(uint8_t slot, uint32_t *nb_sector)
{
	// Return last sector address (-1)
	*nb_sector = (sd_mmc_get_capacity(slot) * 2) - 1;
	return sd_mmc_test_unit_ready(slot);
}

/*
 * \brief Copy data from SD/MMC to RAM.
 *
 * \param slot slot number (0..).
 * \param addr address of memory location.
 * \param ram  buffer to hold the data for single block(512).
 *
 * \return CTRL_GOOD or card status like CTRL_NO_PRESENT, CTRL_FAIL
 *
 */
static ctrl_status_t sd_mmc_mem_2_ram(uint8_t slot, uint32_t addr, void *ram)
{
	switch (sd_mmc_init_read_blocks(slot, addr, 1)) {
	case SD_MMC_OK: {
		if (SD_MMC_OK != sd_mmc_start_read_blocks(ram, 1))
			return CTRL_FAIL;

		if (SD_MMC_OK != sd_mmc_wait_end_of_read_blocks(false))
			return CTRL_FAIL;
		break;
	}
	case SD_MMC_ERR_NO_CARD:
		return CTRL_NO_PRESENT;
	default:
		return CTRL_FAIL;
	}
	return CTRL_GOOD;
}

/*
 * \brief Copy data from RAM to SD/MMC.
 *
 * \param slot slot number (0..).
 * \param addr address of memory location.
 * \param ram  buffer to hold the data for single block(512).
 *
 * \return CTRL_GOOD or card status like CTRL_NO_PRESENT, CTRL_FAIL
 *
 */
static ctrl_status_t sd_mmc_ram_2_mem(uint8_t slot, uint32_t addr, const void *ram)
{
	switch (sd_mmc_init_write_blocks(slot, addr, 1)) {
	case SD_MMC_OK: {
		if (SD_MMC_OK != sd_mmc_start_write_blocks(ram, 1))
			return CTRL_FAIL;

		if (SD_MMC_OK != sd_mmc_wait_end_of_write_blocks(false))
			return CTRL_FAIL;
		break;
	}
	case SD_MMC_ERR_NO_CARD:
		return CTRL_NO_PRESENT;
	default:
		return CTRL_FAIL;
	}

	return CTRL_GOOD;
}

/**
 * \brief Initialize a disk.
 *
 * \param drv Physical drive number (0..).
 *
 * \return 0 or disk status in combination of DSTATUS bits
 *         (STA_NOINIT, STA_PROTECT).
 */
dstatus_t disk_initialize(uint8_t drv)
{
	int           i;
	ctrl_status_t mem_status;

	/* Check LUN ready (USB disk report CTRL_BUSY then CTRL_GOOD) */
	for (i = 0; i < 2; i++) {
		mem_status = sd_mmc_test_unit_ready(drv);
		if (CTRL_BUSY != mem_status) {
			break;
		}
	}
	if (mem_status != CTRL_GOOD) {
		return STA_NOINIT;
	}

	/* Check Write Protection Status */
	if (sd_mmc_is_write_protected(drv)) {
		return STA_PROTECT;
	}

	/* The memory should already be initialized */
	return 0;
}

/**
 * \brief  Return disk status.
 *
 * \param drv Physical drive number (0..).
 *
 * \return 0 or disk status in combination of DSTATUS bits
 *         (STA_NOINIT, STA_NODISK, STA_PROTECT).
 */
dstatus_t disk_status(uint8_t drv)
{
	switch (sd_mmc_test_unit_ready(drv)) {
	case CTRL_GOOD:
		return ERR_NONE;
	case CTRL_NO_PRESENT:
		return STA_NOINIT | STA_NODISK;
	default:
		return STA_NOINIT;
	}
}

/**
 * \brief  Read sector(s).
 *
 * \param drv Physical drive number (0..).
 * \param buff Data buffer to store read data.
 * \param sector Sector address (LBA).
 * \param count Number of sectors to read (1..255).
 *
 * \return ERR_NONE for success, otherwise DRESULT error code.
 */
dstatus_t disk_read(uint8_t drv, uint8_t *buff, uint32_t sector, uint8_t count)
{
	uint8_t  uc_sector_size = SECTOR_SIZE_512;
	uint32_t i;
	uint32_t ul_last_sector_num;

	/* Check valid address */
	sd_mmc_read_capacity(drv, &ul_last_sector_num);
	if ((sector + count * uc_sector_size) > (ul_last_sector_num + 1) * uc_sector_size) {
		return ERR_INVALID_ARG;
	}

	/* Read the data */
	for (i = 0; i < count; i++) {
		if (sd_mmc_mem_2_ram(drv, sector + uc_sector_size * i, buff + uc_sector_size * SECTOR_SIZE_DEFAULT * i)
		    != CTRL_GOOD) {
			return ERR_INVALID_DATA;
		}
	}

	return ERR_NONE;
}

/**
 * \brief  Write sector(s).
 *
 * The FatFs module will issue multiple sector transfer request (count > 1) to
 * the disk I/O layer. The disk function should process the multiple sector
 * transfer properly. Do not translate it into multiple sector transfers to the
 * media, or the data read/write performance may be drastically decreased.
 *
 * \param drv Physical drive number (0..).
 * \param buff Data buffer to store read data.
 * \param sector Sector address (LBA).
 * \param count Number of sectors to read (1..255).
 *
 * \return ERR_NONE for success, otherwise DRESULT error code.
 */
dstatus_t disk_write(uint8_t drv, uint8_t const *buff, uint32_t sector, uint8_t count)
{
	uint8_t  uc_sector_size = SECTOR_SIZE_512;
	uint32_t i;
	uint32_t ul_last_sector_num;

	/* Check valid address */
	sd_mmc_read_capacity(drv, &ul_last_sector_num);
	if ((sector + count * uc_sector_size) > (ul_last_sector_num + 1) * uc_sector_size) {
		return ERR_INVALID_ARG;
	}

	/* Write the data */
	for (i = 0; i < count; i++) {
		if (sd_mmc_ram_2_mem(drv, sector + uc_sector_size * i, buff + uc_sector_size * SECTOR_SIZE_DEFAULT * i)
		    != CTRL_GOOD) {
			return ERR_INVALID_DATA;
		}
	}

	return ERR_NONE;
}

/**
 * \brief  Miscellaneous functions, which support the following commands:
 *
 * CTRL_SYNC    Make sure that the disk drive has finished pending write
 * process. When the disk I/O module has a write back cache, flush the
 * dirty sector immediately.
 * In read-only configuration, this command is not needed.
 *
 * GET_SECTOR_COUNT    Return total sectors on the drive into the DWORD variable
 * pointed by buffer.
 * This command is used only in f_mkfs function.
 *
 * GET_BLOCK_SIZE    Return erase block size of the memory array in unit
 * of sector into the DWORD variable pointed by Buffer.
 * When the erase block size is unknown or magnetic disk device, return 1.
 * This command is used only in f_mkfs function.
 *
 * GET_SECTOR_SIZE    Return sector size of the memory array.
 *
 * \param drv Physical drive number (0..).
 * \param ctrl Control code.
 * \param buff Buffer to send/receive control data.
 *
 * \return ERR_NONE for success, otherwise DRESULT error code.
 */
dstatus_t disk_ioctl(uint8_t drv, uint8_t ctrl, void *buff)
{
	dstatus_t res = ERR_INVALID_ARG;

	switch (ctrl) {
	case GET_BLOCK_SIZE:
		*(unsigned long *)buff = 1;
		res                    = ERR_NONE;
		break;

	/* Get the number of sectors on the disk */
	case GET_SECTOR_COUNT: {
		uint32_t ul_last_sector_num;

		/* Check valid address */
		sd_mmc_read_capacity(drv, &ul_last_sector_num);

		*(unsigned long *)buff = ul_last_sector_num + 1;

		res = ERR_NONE;
	} break;

	/* Get sectors on the disk (WORD) */
	case GET_SECTOR_SIZE: {
		uint8_t uc_sector_size = SECTOR_SIZE_512;

		if ((uc_sector_size != SECTOR_SIZE_512) && (uc_sector_size != SECTOR_SIZE_1024)
		    && (uc_sector_size != SECTOR_SIZE_2048) && (uc_sector_size != SECTOR_SIZE_4096)) {
			/* The sector size is not supported by the FatFS */
			return ERR_INVALID_DATA;
		}

		*(uint8_t *)buff = uc_sector_size * SECTOR_SIZE_DEFAULT;

		res = ERR_NONE;
	} break;

	/* Make sure that data has been written */
	case CTRL_SYNC:
		if (sd_mmc_test_unit_ready(drv) == CTRL_GOOD) {
			res = ERR_NONE;
		} else {
			res = ERR_NOT_READY;
		}
		break;

	default:
		res = ERR_INVALID_ARG;
	}

	return res;
}
