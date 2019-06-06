/**
 * \file
 *
 * \brief Multimedia Card/ Memory Card Interface related functionality declaration.
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

#ifndef _HAL_MCI_INCLUDED
#define _HAL_MCI_INCLUDED

#include <hpl_mci_sync.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup doc_driver_hal_mci_sync
 *
 *@{
 */

/**
 * \brief MCI descriptor structure
 */
struct mci_sync_desc {
    struct _mci_sync_device device;
};

/**
 *  \brief Initialize MCI low level driver.
 *
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t mci_sync_init(struct mci_sync_desc *mci, void *hw);

/**
 *  \brief Deinitialize MCI low level driver.
 *
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t mci_sync_deinit(struct mci_sync_desc *mci);

/**
 *  \brief Select a device and initialize it
 *
 *  \param[in] slot    Selected slot
 *  \param[in] clock   Maximum clock to use (Hz)
 *  \param[in] bus_width  Bus width to use (1, 4, or 8)
 *  \param[in] high_speed True, to enable high speed mode
 *  \return Operation status.
 *  \retval 0 Success.
 *  \retval <0 Error code.
 */
int32_t mci_sync_select_device(struct mci_sync_desc *mci, uint8_t slot, uint32_t clock, uint8_t bus_width,
                               bool high_speed);

/**
 *  \brief Get the maximum bus width of a device
 *         by a selected slot
 *
 *  \param[in] slot    Selected slot
 *  \return Bus width.
 */
uint8_t mci_sync_get_bus_width(struct mci_sync_desc *mci, uint8_t slot);

/**
 *  \brief Get the high-speed capability of the device.
 *
 *  \return True, if the high-speed is supported.
 */
bool mci_sync_is_high_speed_capable(struct mci_sync_desc *mci);

/**
 *  \brief Send 74 clock cycles on the line.
 *   Note: It is required after card plug and before card install.
 */
void mci_sync_send_clock(struct mci_sync_desc *mci);

/**
 * \brief Disable the bus clock.
 */
void mci_sync_pause_clock(struct mci_sync_desc *mci);

/**
 * \brief Resume the bus clock.
 */
void mci_sync_resume_clock(struct mci_sync_desc *mci);

/**
 *  \brief Send a command on the selected slot
 *
 *  \param[in] cmd    Command definition
 *  \param[in] arg    Argument of the command
 *  \return True if success, otherwise false
 */
bool mci_sync_send_cmd(struct mci_sync_desc *mci, uint32_t cmd, uint32_t arg);

/**
 *  \brief Get 32-bits response of the last command.
 *
 *  \return 32-bits response.
 */
uint32_t mci_sync_get_response(struct mci_sync_desc *mci);

/**
 *  \brief Get 128-bits response of the last command.
 *
 *  \param[in] response   Pointer on the array to fill
 *                        with the 128-bits response.
 */
void mci_sync_get_response_128(struct mci_sync_desc *mci, uint8_t *response);

/**
 *  \brief Send an ADTC command on the selected slot.
 *         An ADTC (Addressed Data Transfer Commands)
 *         command is used for read/write access.
 *
 *  \param[in] cmd          Command definition.
 *  \param[in] arg          Argument of the command.
 *  \param[in] block_size   Block size used for the transfer.
 *  \param[in] nb_block     Total number of blocks for this transfer
 *  \param[in] access_block If true, the x_read_blocks() and x_write_blocks()
 *                          functions must be used after this function.
 *                          If false, the mci_read_word() and mci_write_word()
 *                          functions must be used after this function.
 *
 * \return True if success, otherwise false
 */
bool mci_sync_adtc_start(struct mci_sync_desc *mci, uint32_t cmd, uint32_t arg, uint16_t block_size, uint16_t nb_block,
                         bool access_block);

/**
 *  \brief Send a command to stop an ADTC command on the selected slot.
 *
 * \param[in] cmd    Command definition
 * \param[in] arg    Argument of the command
 *
 * \return True if success, otherwise false
 */
bool mci_sync_adtc_stop(struct mci_sync_desc *mci, uint32_t cmd, uint32_t arg);

/**
 *  \brief Read a word on the line.
 *
 *  \param[in] value  Pointer on a word to fill.
 *
 *  \return True if success, otherwise false
 */
bool mci_sync_read_word(struct mci_sync_desc *mci, uint32_t *value);

/**
 *  \brief Write a word on the line
 *
 *  \param[in] value  Word to send
 *
 *  \return True if success, otherwise false
 */
bool mci_sync_write_word(struct mci_sync_desc *mci, uint32_t value);

/**
 *  \brief Start a read blocks transfer on the line
 *
 *  Note: The driver will use the DMA available to speed up the transfer.
 *
 *  \param[in] dst        Pointer on the buffer to fill
 *  \param[in] nb_block   Number of block to transfer
 *
 *  \return True if started, otherwise false
 */
bool mci_sync_start_read_blocks(struct mci_sync_desc *mci, void *dst, uint16_t nb_block);

/**
 *  \brief Start a write blocks transfer on the line
 *
 *  Note: The driver will use the DMA available to speed up the transfer.
 *
 *  \param[in] src        Pointer on the buffer to send
 *  \param[in] nb_block   Number of block to transfer
 *
 *  \return True if started, otherwise false
 */
bool mci_sync_start_write_blocks(struct mci_sync_desc *mci, const void *src, uint16_t nb_block);

/**
 *  \brief Wait for the end of transfer to be initiated by the mci_start_read_blocks()
 *
 *  \return True if success, otherwise false
 */
bool mci_sync_wait_end_of_read_blocks(struct mci_sync_desc *mci);

/**
 *  \brief Wait for the end of transfer to be initiated by the mci_start_write_blocks()
 *
 *  \return True if success, otherwise false
 */
bool mci_sync_wait_end_of_write_blocks(struct mci_sync_desc *mci);

/**
 *  \brief Retrieve the current driver version
 *  \return Current driver version.
 */
uint32_t mci_sync_get_version(void);

/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* _HAL_MCI_INCLUDED */
