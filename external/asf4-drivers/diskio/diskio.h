/**
 * \file
 *
 * \brief diskio.h
 *
 * Copyright (c) 2015-2018 Microchip Technology Inc. and its subsidiaries.
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
