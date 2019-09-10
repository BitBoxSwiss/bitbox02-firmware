/**
 * \file
 *
 * \brief diskio.h
 *
 * Copyright (C) 2015 - 2017 Atmel Corporation. All rights reserved.
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
/*
 * Support and FAQ: visit <a href="http://www.atmel.com/design-support/">Atmel Support</a>
 */

#ifndef _DISKIO_DEFINED
#define _DISKIO_DEFINED

#include "stdint.h"
#include <err_codes.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Definitions of physical drive number for each drive */
#define DISKIO_ATA 0 /* Example: Map ATA harddisk to physical drive 0 */
#define DISKIO_MMC 1 /* Example: Map MMC/SD card to physical drive 1 */
#define DISKIO_USB 2 /* Example: Map USB MSD to physical drive 2 */

/* Disk Status Bits (DSTATUS) */
#define STA_NOINIT 0x01  /* Drive not initialized */
#define STA_NODISK 0x02  /* No medium in the drive */
#define STA_PROTECT 0x04 /* Write protected */

/* Generic command (defined for FatFs) */
#define CTRL_SYNC 0         /* Flush disk cache (for write functions) */
#define GET_SECTOR_COUNT 1  /* Get media size (for only f_mkfs()) */
#define GET_SECTOR_SIZE 2   /* Get sector size (for multiple sector size (_MAX_SS >= 1024)) */
#define GET_BLOCK_SIZE 3    /* Get erase block size (for only f_mkfs()) */
#define CTRL_ERASE_SECTOR 4 /* Force erased a block of sectors (for only _USE_ERASE) */

/* Generic command */
#define CTRL_POWER 5 /* Get/Set power status */
#define CTRL_LOCK 6  /* Lock/Unlock media removal */
#define CTRL_EJECT 7 /* Eject media */

/* MMC/SDC specific ioctl command */
#define MMC_GET_TYPE 10   /* Get card type */
#define MMC_GET_CSD 11    /* Get CSD */
#define MMC_GET_CID 12    /* Get CID */
#define MMC_GET_OCR 13    /* Get OCR */
#define MMC_GET_SDSTAT 14 /* Get SD status */

/* ATA/CF specific ioctl command */
#define ATA_GET_REV 20   /* Get F/W revision */
#define ATA_GET_MODEL 21 /* Get model name */
#define ATA_GET_SN 22    /* Get serial number */

/* NAND specific ioctl command */
#define NAND_FORMAT 30 /* Create physical format */

/* Status of Disk Functions */
typedef uint8_t   dstatus_t;
typedef dstatus_t DSTATUS; /* DSTATUS is referred in ff.c the original open source fatfs code  */

/* Results of Disk Functions */
typedef enum {
	RES_OK = 0, /* 0: Successful */
	RES_ERROR,  /* 1: R/W Error */
	RES_WRPRT,  /* 2: Write Protected */
	RES_NOTRDY, /* 3: Not Ready */
	RES_PARERR  /* 4: Invalid Parameter */
} dresult_t;

#ifndef SECTOR_SIZE
#define SECTOR_SIZE 512
#endif

/** Default sector size */
#define SECTOR_SIZE_DEFAULT 512

/** Supported sector size. These values are based on the LUN function:
 * mem_sector_size(). */
#define SECTOR_SIZE_512 1
#define SECTOR_SIZE_1024 2
#define SECTOR_SIZE_2048 4
#define SECTOR_SIZE_4096 8

//! Status returned by CTRL_ACCESS interfaces.
typedef enum {
	CTRL_GOOD       = 0,             //!< Success, memory ready.
	CTRL_FAIL       = CTRL_GOOD + 1, //!< An error occurred.
	CTRL_NO_PRESENT = CTRL_GOOD + 2, //!< Memory unplugged.
	CTRL_BUSY       = CTRL_GOOD + 3  //!< Memory not initialized or changed.
} ctrl_status_t;

/**
 * \brief  Return disk status.
 *
 * \param drv Physical drive number (0..).
 *
 * \return 0 or disk status in combination of DSTATUS bits
 *         (STA_NOINIT, STA_NODISK, STA_PROTECT).
 *
 */
dstatus_t disk_status(uint8_t drv);

/**
 * \brief Initialize a disk.
 *
 * \param drv Physical drive number (0..).
 *
 * \return 0 or disk status in combination of DSTATUS bits
 *         (STA_NOINIT, STA_PROTECT).
 */
dstatus_t disk_initialize(uint8_t drv);

/**
 * \brief  Read sector(s).
 *
 * \param drv Physical drive number (0..).
 * \param buff Data buffer to store read data.
 * \param sector Sector address (LBA).
 * \param count Number of sectors to read (1..255).
 *
 * \return RES_OK for success, otherwise DRESULT error code.
 */
dstatus_t disk_read(uint8_t drv, uint8_t *buff, uint32_t sector, uint8_t count);

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
 * \return RES_OK for success, otherwise DRESULT error code.
 */
dstatus_t disk_write(uint8_t drv, uint8_t const *buff, uint32_t sector, uint8_t count);

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
 * \return RES_OK for success, otherwise DRESULT error code.
 */
dstatus_t disk_ioctl(uint8_t drv, uint8_t ctrl, void *buff);

#ifdef __cplusplus
}
#endif

#endif
