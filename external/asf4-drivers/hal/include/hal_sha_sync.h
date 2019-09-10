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

#ifndef HAL_SHA_SYNC_H_INCLUDED
#define HAL_SHA_SYNC_H_INCLUDED

#include <hpl_sha_sync.h>
#include <utils_assert.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup doc_driver_hal_hash_sha_sync
 *
 *@{
 */

struct sha_sync_descriptor {
	struct _sha_sync_device dev; /*!< SHA HPL device descriptor */
};

/**
 * \brief              Initialize the SHA Descriptor
 *
 * \param[in] desc     The SHA descriptor to be initialized
 * \param[in] hw       The pointer to hardware instance
 *
 * \return             Initialization status.
 */
int32_t sha_sync_init(struct sha_sync_descriptor *descr, void *const hw);

/**
 * \brief              Deinitialize SHA Descriptor
 *
 * \param[in] desc     The SHA descriptor to be deinitialized
 *
 * \return             De-initialization status.
 */
int32_t sha_sync_deinit(struct sha_sync_descriptor *desc);

/**
 * \brief              Enable SHA
 *
 * \param[in] desc     SHA descriptor
 *
 * \return             Enabling status.
 */
int32_t sha_sync_enable(struct sha_sync_descriptor *desc);

/**
 * \brief              Disable SHA
 *
 * \param[in] desc     SHA descriptor
 *
 * \return             Disabling status.
 */
int32_t sha_sync_disable(struct sha_sync_descriptor *desc);

/**
 * \brief              SHA-1 start
 *
 * \param[in]  descr   SHA descriptor
 * \param[in]  ctx     SHA context structure
 *
 * \return             Start status.
 */
int32_t sha_sync_sha1_start(struct sha_sync_descriptor *descr, struct sha_context *ctx);

/**
 * \brief              SHA-256/224 start
 *
 * \param[in]  descr   SHA descriptor
 * \param[in]  ctx     SHA context structure
 * \param[in]  is224   If true, use SHA-224
 *
 * \return             Start status.
 */
int32_t sha_sync_sha256_start(struct sha_sync_descriptor *descr, struct sha_context *ctx, bool is224);

/**
 * \brief              SHA-1 input update
 *
 * \param[in]  descr   SHA descriptor
 * \param[in]  input   Buffer holding the input data
 * \param[in]  length  Byte length of the input data
 *
 * \return             Update status.
 */
int32_t sha_sync_sha1_update(struct sha_sync_descriptor *descr, const uint8_t *input, uint32_t length);

/**
 * \brief              SHA-256/224 input update
 *
 * \param[in]  descr   SHA descriptor
 * \param[in]  input   Buffer holding the input data
 * \param[in]  length  Byte length of the input data
 *
 * \return             Update status.
 */
int32_t sha_sync_sha256_update(struct sha_sync_descriptor *descr, const uint8_t *input, uint32_t length);

/**
 * \brief              SHA-1 finish
 *
 * \param[in]  descr   SHA descriptor
 * \param[out] output  SHA digest data
 *
 * \return             Finish status.
 */
int32_t sha_sync_sha1_finish(struct sha_sync_descriptor *descr, uint8_t output[20]);

/**
 * \brief              SHA-256/224 finish
 *
 * \param[in]  descr   SHA descriptor
 * \param[out] output  SHA digest data
 *
 * \return             Finish status.
 */
int32_t sha_sync_sha256_finish(struct sha_sync_descriptor *descr, uint8_t output[32]);

/**
 * \brief              SHA-1 compute digest
 *
 * \param[in]  descr   SHA descriptor
 * \param[in]  ctx     SHA context structure
 * \param[in]  input   Buffer holding the input data
 * \param[in]  length  Byte length of the input data
 * \param[out] output  SHA digest data
 *
 * \return             Compute status.
 */
int32_t sha_sync_sha1_compute(struct sha_sync_descriptor *descr, struct sha_context *ctx, const uint8_t *input,
                              uint32_t length, uint8_t output[20]);

/**
 * \brief              SHA-256/224 compute digest
 *
 * \param[in]  descr   SHA descriptor
 * \param[in]  ctx     SHA context structure
 * \param[in]  is224   If true, use SHA-224
 * \param[in]  input   Buffer holding the input data
 * \param[in]  length  Byte length of the input data
 * \param[out] output  SHA digest data
 *
 * \return             Compute status.
 */
int32_t sha_sync_sha256_compute(struct sha_sync_descriptor *descr, struct sha_context *ctx, bool is224,
                                const uint8_t *input, uint32_t length, uint8_t output[32]);

/**
 * \brief Retrieve the current driver version
 *
 * \return Current driver version.
 */
uint32_t sha_sync_get_version(void);

/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* HAL_SHA_SYNC_H_INCLUDED */
