/**
 * \file
 *
 * \brief SAM MCI HPL
 *
 * Copyright (C) 2016 Atmel Corporation. All rights reserved.
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

#ifndef _HPL_MCI_H_INCLUDED
#define _HPL_MCI_H_INCLUDED

#include <compiler.h>
#include <utils.h>

#ifdef __cplusplus
extern "C" {
#endif

//! \name Flags used to define MCI parser for SD/MMC command
//! @{
//! Have response
#define MCI_RESP_PRESENT (1lu << 8)
//! 136 bit response
#define MCI_RESP_136 (1lu << 11)
//! Expect valid crc
#define MCI_RESP_CRC (1lu << 12)
//! Card may send busy
#define MCI_RESP_BUSY (1lu << 13)
//! Open drain for a braodcast command
#define MCI_CMD_OPENDRAIN (1lu << 14)
//! To signal a data write operation
#define MCI_CMD_WRITE (1lu << 15)
//! To signal a SDIO tranfer in multi byte mode
#define MCI_CMD_SDIO_BYTE (1lu << 16)
//! To signal a SDIO tranfer in block mode
#define MCI_CMD_SDIO_BLOCK (1lu << 17)
//! To signal a data transfer in stream mode
#define MCI_CMD_STREAM (1lu << 18)
//! To signal a data transfer in single block mode
#define MCI_CMD_SINGLE_BLOCK (1lu << 19)
//! To signal a data transfer in multi block mode
#define MCI_CMD_MULTI_BLOCK (1lu << 20)
//! @}

/**
 * \brief mci sync device structure
 */
struct _mci_sync_device {
    void *   hw;
    uint64_t mci_sync_trans_pos;
    uint16_t mci_sync_block_size;
    uint16_t mci_sync_nb_block;
};

/**
 *  \brief Initialize MCI low level driver.
 *
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t _mci_sync_init(struct _mci_sync_device *const mci_dev, void *const hw);

/**
 *  \brief Deinitialize MCI low level driver.
 *
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t _mci_sync_deinit(struct _mci_sync_device *const mci_dev);

/**
 *  \brief Select a device and initialize it
 *
 *  \param[in] slot    Selected slot
 *  \param[in] clock   Maximum clock to use (Hz)
 *  \param[in] bus_width  Bus width to use (1, 4 or 8)
 *  \param[in] high_speed true, to enable high speed mode
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t _mci_sync_select_device(struct _mci_sync_device *const mci_dev, uint8_t slot, uint32_t clock, uint8_t bus_width,
                                bool high_speed);

/**
 *  \brief Get the maximum bus width of a device
 *         by a selected slot
 *
 *  \param[in] slot    Selected slot
 *  \return bus width.
 */
uint8_t _mci_sync_get_bus_width(struct _mci_sync_device *const mci_dev, uint8_t slot);

/**
 *  \brief Get the high speed capability of the device.
 *
 *  \return true, if the high speed is supported.
 */
bool _mci_sync_is_high_speed_capable(struct _mci_sync_device *const mci_dev);

/**
 *  \brief Send 74 clock cycles on the line.
 *   Note: It is required after card plug and before card install.
 */
void _mci_sync_send_clock(struct _mci_sync_device *const mci_dev);

/**
 * \brief Disable the bus clock.
 */
void _mci_sync_pause_clock(struct _mci_sync_device *const mci_dev);

/**
 * \brief Resume the bus clock.
 */
void _mci_sync_resume_clock(struct _mci_sync_device *const mci_dev);

/**
 *  \brief Send a command on the selected slot
 *
 *  \param[in] cmd        Command definition
 *  \param[in] arg        Argument of the command
 *  \return true if success, otherwise false
 */
bool _mci_sync_send_cmd(struct _mci_sync_device *const mci_dev, uint32_t cmd, uint32_t arg);

/**
 *  \brief Get 32 bits response of the last command.
 *
 *  \return 32 bits response.
 */
uint32_t _mci_sync_get_response(struct _mci_sync_device *const mci_dev);

/**
 *  \brief Get 128 bits response of the last command.
 *
 *  \param[in] response   Pointer on the array to fill
 *                        with the 128 bits response.
 */
void _mci_sync_get_response_128(struct _mci_sync_device *const mci_dev, uint8_t *response);

/**
 *  \brief Send an ADTC command on the selected slot
 *         An ADTC (Addressed Data Transfer Commands)
 *         command is used for read/write access..
 *
 *  \param[in] cmd          Command definition.
 *  \param[in] arg          Argument of the command.
 *  \param[in] block_size   Block size used for the transfer.
 *  \param[in] nb_block     Total number of block for this transfer
 *  \param[in] access_block if true, the x_read_blocks() and x_write_blocks()
 *                          functions must be used after this function.
 *                          If false, the mci_read_word() and mci_write_word()
 *                          functions must be used after this function.
 *
 * \return true if success, otherwise false
 */
bool _mci_sync_adtc_start(struct _mci_sync_device *const mci_dev, uint32_t cmd, uint32_t arg, uint16_t block_size,
                          uint16_t nb_block, bool access_block);

/**
 *  \brief Send a command to stop an ADTC command on the selected slot.
 *
 * \param[in] cmd        Command definition
 * \param[in] arg        Argument of the command
 *
 * \return true if success, otherwise false
 */
bool _mci_sync_adtc_stop(struct _mci_sync_device *const mci_dev, uint32_t cmd, uint32_t arg);

/**
 *  \brief Read a word on the line.
 *
 *  \param[in] value  Pointer on a word to fill.
 *
 *  \return true if success, otherwise false
 */
bool _mci_sync_read_word(struct _mci_sync_device *const mci_dev, uint32_t *value);

/**
 *  \brief Write a word on the line
 *
 *  \param[in] value  Word to send
 *
 *  \return true if success, otherwise false
 */
bool _mci_sync_write_word(struct _mci_sync_device *const mci_dev, uint32_t value);

/**
 *  \brief Start a read blocks transfer on the line
 *  Note: The driver will use the DMA available to speed up the transfer.
 *
 *  \param[in] dst        Pointer on the buffer to fill
 *  \param[in] nb_block   Number of block to transfer
 *
 *  \return true if started, otherwise false
 */
bool _mci_sync_start_read_blocks(struct _mci_sync_device *const mci_dev, void *dst, uint16_t nb_block);

/**
 *  \brief Start a write blocks transfer on the line
 *  Note: The driver will use the DMA available to speed up the transfer.
 *
 *  \param[in] src        Pointer on the buffer to send
 *  \param[in] nb_block   Number of block to transfer
 *
 *  \return true if started, otherwise false
 */
bool _mci_sync_start_write_blocks(struct _mci_sync_device *const mci_dev, const void *src, uint16_t nb_block);

/**
 *  \brief Wait the end of transfer initiated by mci_start_read_blocks()
 *
 *  \return true if success, otherwise false
 */
bool _mci_sync_wait_end_of_read_blocks(struct _mci_sync_device *const mci_dev);

/**
 *  \brief Wait the end of transfer initiated by mci_start_write_blocks()
 *
 *  \return true if success, otherwise false
 */
bool _mci_sync_wait_end_of_write_blocks(struct _mci_sync_device *const mci_dev);

#ifdef __cplusplus
}
#endif

#endif /* _HPL_MCI_H_INCLUDED */
