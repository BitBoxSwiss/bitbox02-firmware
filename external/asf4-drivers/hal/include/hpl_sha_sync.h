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
