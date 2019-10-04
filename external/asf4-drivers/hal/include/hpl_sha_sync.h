/**
 * \file
 *
 * \brief SHA Secure Hash Algorithm(Sync) functionality declaration.
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

#ifndef HPL_SHA_SYNC_H_INCLUDED
#define HPL_SHA_SYNC_H_INCLUDED
#include "hpl_sha.h"
#include "utils.h"

#ifdef __cplusplus
extern "C" {
#endif

struct _sha_sync_device {
	void *              hw;  /*!< Hardware module instance handler */
	struct sha_context *ctx; /*!< SHA context structure */
};

/**
 * \brief              Initialize SHA
 *
 * \param[in]  dev     The pointer to device instance
 * \param[in]  hw      The pointer to hardware instance
 *
 * \return             Initialization status.
 */
int32_t _sha_sync_init(struct _sha_sync_device *const dev, void *const hw);

/**
 * \brief              Deinitialize SHA
 *
 * \param[in]  dev     The pointer to device instance
 *
 * \return             De-initialization status.
 */
int32_t _sha_sync_deinit(struct _sha_sync_device *const dev);

/**
 * \brief              Enable SHA
 *
 * \param[in]  dev     The pointer to device instance
 *
 * \return             Enabling status.
 */
int32_t _sha_sync_enable(struct _sha_sync_device *const dev);

/**
 * \brief              Disable SHA
 *
 * \param[in]  dev     The pointer to device instance
 *
 * \return             Disabling status.
 */
int32_t _sha_sync_disable(struct _sha_sync_device *const dev);

/**
 * \brief              SHA-1 start
 *
 * \param[in]  dev     The pointer to device instance
 *
 * \return             Start status.
 */
int32_t _sha_sync_sha1_start(struct _sha_sync_device *const dev);

/**
 * \brief              SHA-256 start
 *
 * \param[in]  dev     The pointer to device instance
 *
 * \return             Start status.
 */
int32_t _sha_sync_sha256_start(struct _sha_sync_device *const dev);

/**
 * \brief              SHA-1 process
 *
 * \param[in]  dev     The pointer to device instance
 * \param[in]  input   Buffer holding the input data
 * \param[in]  length  Byte length of the input data
 *
 * \return             Process status.
 */
int32_t _sha_sync_sha1_process(struct _sha_sync_device *const dev, const uint8_t *input, uint32_t length);

/**
 * \brief              SHA-256/224 process
 *
 * \param[in]  dev     The pointer to device instance
 * \param[in]  input   Buffer holding the input data
 * \param[in]  length  Byte length of the input data
 *
 * \return             Process status.
 */
int32_t _sha_sync_sha256_process(struct _sha_sync_device *const dev, const uint8_t *input, uint32_t length);

/**
 * \brief Retrieve the current driver version
 *
 * \return Current driver version.
 */
uint32_t _sha_sync_get_version(void);

#ifdef __cplusplus
}
#endif

#endif /* HPL_SHA_SYNC_H_INCLUDED */
