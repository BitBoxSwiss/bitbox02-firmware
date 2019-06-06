/**
 * \file
 *
 * \brief Multimedia Card/ Memory Card Interface HAL.
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

#include "hal_mci_sync.h"
#include <utils_assert.h>

#ifdef __cplusplus
extern "C" {
#endif

/** MCI HAL driver version. */
#define DRIVER_VERSION 0x00000001u

/**
 *  \brief Initialize MCI low level driver.
 */
int32_t mci_sync_init(struct mci_sync_desc *mci, void *hw)
{
    ASSERT(mci && hw);
    return _mci_sync_init(&mci->device, hw);
}

/**
 *  \brief Deinitialize MCI low level driver.
 */
int32_t mci_sync_deinit(struct mci_sync_desc *mci)
{
    ASSERT(mci);
    return _mci_sync_deinit(&mci->device);
}

/**
 *  \brief Select a device and initialize it
 */
int32_t mci_sync_select_device(struct mci_sync_desc *mci, uint8_t slot, uint32_t clock, uint8_t bus_width,
                               bool high_speed)
{
    ASSERT(mci);
    return _mci_sync_select_device(&mci->device, slot, clock, bus_width, high_speed);
}

/**
 *  \brief Get the maximum bus width of a device by a selected slot
 */
uint8_t mci_sync_get_bus_width(struct mci_sync_desc *mci, uint8_t slot)
{
    ASSERT(mci);
    return _mci_sync_get_bus_width(&mci->device, slot);
}

/**
 *  \brief Get the high speed capability of the device.
 *
 */
bool mci_sync_is_high_speed_capable(struct mci_sync_desc *mci)
{
    ASSERT(mci);
    return _mci_sync_is_high_speed_capable(&mci->device);
}

/**
 *  \brief Send 74 clock cycles on the line.
 */
void mci_sync_send_clock(struct mci_sync_desc *mci)
{
    ASSERT(mci);
    _mci_sync_send_clock(&mci->device);
}

/**
 * \brief Disable the bus clock.
 */
void mci_sync_pause_clock(struct mci_sync_desc *mci)
{
    ASSERT(mci);
    _mci_sync_pause_clock(&mci->device);
}

/**
 * \brief Resume the bus clock.
 */
void mci_sync_resume_clock(struct mci_sync_desc *mci)
{
    ASSERT(mci);
    _mci_sync_resume_clock(&mci->device);
}

/**
 *  \brief Send a command on the selected slot
 */
bool mci_sync_send_cmd(struct mci_sync_desc *mci, uint32_t cmd, uint32_t arg)
{
    ASSERT(mci);
    return _mci_sync_send_cmd(&mci->device, cmd, arg);
}

/**
 *  \brief Get 32 bits response of the last command.
 */
uint32_t mci_sync_get_response(struct mci_sync_desc *mci)
{
    ASSERT(mci);
    return _mci_sync_get_response(&mci->device);
}

/**
 *  \brief Get 128 bits response of the last command.
 */
void mci_sync_get_response_128(struct mci_sync_desc *mci, uint8_t *response)
{
    ASSERT(mci && response);
    _mci_sync_get_response_128(&mci->device, response);
}

/**
 *  \brief Send an ADTC command on the selected slot
 *         An ADTC (Addressed Data Transfer Commands)
 *         command is used for read/write access.
 */
bool mci_sync_adtc_start(struct mci_sync_desc *mci, uint32_t cmd, uint32_t arg, uint16_t block_size, uint16_t nb_block,
                         bool access_block)
{
    ASSERT(mci);
    return _mci_sync_adtc_start(&mci->device, cmd, arg, block_size, nb_block, access_block);
}

/**
 *  \brief Send a command to stop an ADTC command on the selected slot.
 */
bool mci_sync_adtc_stop(struct mci_sync_desc *mci, uint32_t cmd, uint32_t arg)
{
    ASSERT(mci);
    return _mci_sync_adtc_stop(&mci->device, cmd, arg);
}

/**
 *  \brief Read a word on the line.
 */
bool mci_sync_read_word(struct mci_sync_desc *mci, uint32_t *value)
{
    ASSERT(mci && value);
    return _mci_sync_read_word(&mci->device, value);
}

/**
 *  \brief Write a word on the line
 */
bool mci_sync_write_word(struct mci_sync_desc *mci, uint32_t value)
{
    ASSERT(mci);
    return _mci_sync_write_word(&mci->device, value);
}

/**
 *  \brief Start a read blocks transfer on the line
 *  Note: The driver will use the DMA available to speed up the transfer.
 */
bool mci_sync_start_read_blocks(struct mci_sync_desc *mci, void *dst, uint16_t nb_block)
{
    ASSERT(mci && dst);
    return _mci_sync_start_read_blocks(&mci->device, dst, nb_block);
}

/**
 *  \brief Start a write blocks transfer on the line
 */
bool mci_sync_start_write_blocks(struct mci_sync_desc *mci, const void *src, uint16_t nb_block)
{
    ASSERT(mci && src);
    return _mci_sync_start_write_blocks(&mci->device, src, nb_block);
}

/**
 *  \brief Wait the end of transfer initiated by mci_start_read_blocks()
 */
bool mci_sync_wait_end_of_read_blocks(struct mci_sync_desc *mci)
{
    ASSERT(mci);
    return _mci_sync_wait_end_of_read_blocks(&mci->device);
}

/**
 *  \brief Wait the end of transfer initiated by mci_start_write_blocks()
 */
bool mci_sync_wait_end_of_write_blocks(struct mci_sync_desc *mci)
{
    ASSERT(mci);
    return _mci_sync_wait_end_of_write_blocks(&mci->device);
}

/**
 *  \brief Retrieve the current driver version
 *  \return Current driver version
 */
uint32_t mci_sync_get_version(void)
{
    return DRIVER_VERSION;
}

#ifdef __cplusplus
}
#endif
