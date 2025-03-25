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

#include <string.h>
#include <hal_sha_sync.h>

#define DRIVER_VERSION 0x00000001u

/**
 * \brief              Initialize SHA.
 */
int32_t sha_sync_init(struct sha_sync_descriptor *descr, void *const hw)
{
	ASSERT(descr && hw);

	return _sha_sync_init(&descr->dev, hw);
}

/**
 * \brief              Deinitialize SHA.
 */
int32_t sha_sync_deinit(struct sha_sync_descriptor *descr)
{
	ASSERT(descr);

	return _sha_sync_deinit(&descr->dev);
}

/**
 * \brief              Enable SHA
 */
int32_t sha_sync_enable(struct sha_sync_descriptor *descr)
{
	ASSERT(descr);

	return _sha_sync_enable(&descr->dev);
}

/**
 * \brief              Disable SHA
 */
int32_t sha_sync_disable(struct sha_sync_descriptor *descr)
{
	ASSERT(descr);

	return _sha_sync_disable(&descr->dev);
}

/**
 * \brief              SHA-1 start
 */
int32_t sha_sync_sha1_start(struct sha_sync_descriptor *descr, struct sha_context *ctx)
{
	ASSERT(descr);

	ctx->total_len = 0;
	memset(ctx->buffer, 0, sizeof(ctx->buffer));
	descr->dev.ctx = ctx;

	return _sha_sync_sha1_start(&descr->dev);
}

/**
 * \brief              SHA-256/224 start
 */
int32_t sha_sync_sha256_start(struct sha_sync_descriptor *descr, struct sha_context *ctx, bool is224)
{
	ASSERT(descr);

	ctx->total_len = 0;
	memset(ctx->buffer, 0, sizeof(ctx->buffer));
	ctx->is_variant = is224;
	descr->dev.ctx  = ctx;

	return _sha_sync_sha256_start(&descr->dev);
}

/**
 * \brief              SHA-1 input update
 */
int32_t sha_sync_sha1_update(struct sha_sync_descriptor *descr, const uint8_t *input, uint32_t length)
{
	uint32_t            fill;
	uint32_t            left;
	struct sha_context *ctx = descr->dev.ctx;

	ASSERT(descr && input && length);

	left = ctx->total_len & 0x3F;
	fill = 64 - left;

	ctx->total_len += length;

	if (left && length >= fill) {
		memcpy((void *)(ctx->buffer + left), input, fill);
		_sha_sync_sha1_process(&descr->dev, ctx->buffer, 64);
		input += fill;
		length -= fill;
		left = 0;
	}

	if (length >= 64) {
		_sha_sync_sha1_process(&descr->dev, input, length & 0xFFFFFFC0);
		input += (length & 0xFFFFFFC0);
		length &= 0x3F;
	}

	if (length > 0) {
		memcpy((void *)(ctx->buffer + left), input, length);
	}

	return ERR_NONE;
}

/**
 * \brief              SHA-256/224 input update
 */
int32_t sha_sync_sha256_update(struct sha_sync_descriptor *descr, const uint8_t *input, uint32_t length)
{
	uint32_t            fill;
	uint32_t            left;
	struct sha_context *ctx = descr->dev.ctx;

	ASSERT(descr && input && length);

	left = ctx->total_len & 0x3F;
	fill = 64 - left;

	ctx->total_len += length;

	if (left && length >= fill) {
		memcpy((void *)(ctx->buffer + left), input, fill);
		_sha_sync_sha256_process(&descr->dev, ctx->buffer, 64);
		input += fill;
		length -= fill;
		left = 0;
	}

	if (length >= 64) {
		_sha_sync_sha256_process(&descr->dev, input, length & 0xFFFFFFC0);
		input += (length & 0xFFFFFFC0);
		length &= 0x3F;
	}

	if (length > 0) {
		memcpy((void *)(ctx->buffer + left), input, length);
	}

	return ERR_NONE;
}

static const uint8_t sha_padding[64]
    = {0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
       0,    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

/**
 * \brief              SHA-1 finish
 */
int32_t sha_sync_sha1_finish(struct sha_sync_descriptor *descr, uint8_t output[20])
{
	uint32_t            i;
	uint32_t            last;
	uint8_t             padn;
	uint8_t             msg_len[8];
	struct sha_context *ctx       = descr->dev.ctx;
	uint64_t            total_len = ctx->total_len << 3; /* Get the length of bits */

	ASSERT(descr && output);

	for (i = 0; i < 8; i++) {
		msg_len[i] = (uint8_t)(total_len >> ((7 - i) << 3));
	}

	last = ctx->total_len & 0x3F;
	padn = (last < 56) ? (56 - last) : (120 - last);

	sha_sync_sha1_update(descr, sha_padding, padn);
	sha_sync_sha1_update(descr, msg_len, 8);

	memcpy(output, ctx->digest, 20);

	return ERR_NONE;
}

/**
 * \brief              SHA-256/224 finish
 */
int32_t sha_sync_sha256_finish(struct sha_sync_descriptor *descr, uint8_t output[32])
{
	uint32_t            i;
	uint32_t            last;
	uint8_t             padn;
	uint8_t             msg_len[8];
	struct sha_context *ctx       = descr->dev.ctx;
	uint64_t            total_len = ctx->total_len << 3; /* Get the length of bits */

	ASSERT(descr && output);

	for (i = 0; i < 8; i++) {
		msg_len[i] = (uint8_t)(total_len >> ((7 - i) << 3));
	}

	last = ctx->total_len & 0x3F;
	padn = (last < 56) ? (56 - last) : (120 - last);

	sha_sync_sha256_update(descr, sha_padding, padn);
	sha_sync_sha256_update(descr, msg_len, 8);

	if (ctx->is_variant) {
		memcpy(output, ctx->digest, 28);
	} else {
		memcpy(output, ctx->digest, 32);
	}

	return ERR_NONE;
}

/**
 * \brief              SHA-1 compute digest
 */
int32_t sha_sync_sha1_compute(struct sha_sync_descriptor *descr, struct sha_context *ctx, const uint8_t *input,
                              uint32_t length, uint8_t output[20])
{
	ASSERT(descr && input && length && output);

	sha_sync_sha1_start(descr, ctx);
	sha_sync_sha1_update(descr, input, length);
	sha_sync_sha1_finish(descr, output);

	return ERR_NONE;
}

/**
 * \brief              SHA-256/224 compute digest
 */
int32_t sha_sync_sha256_compute(struct sha_sync_descriptor *descr, struct sha_context *ctx, bool is224,
                                const uint8_t *input, uint32_t length, uint8_t output[32])
{
	ASSERT(descr && input && length && output);

	sha_sync_sha256_start(descr, ctx, is224);
	sha_sync_sha256_update(descr, input, length);
	sha_sync_sha256_finish(descr, output);

	return ERR_NONE;
}

/**
 * \brief Retrieve the current driver version
 */
uint32_t sha_sync_get_version(void)
{
	return DRIVER_VERSION;
}
