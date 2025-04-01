/**
 * \file
 *
 * \brief SHA Secure Hash Algorithm(Sync) functionality declaration.
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
