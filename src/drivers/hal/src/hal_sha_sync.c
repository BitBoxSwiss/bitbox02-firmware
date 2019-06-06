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
