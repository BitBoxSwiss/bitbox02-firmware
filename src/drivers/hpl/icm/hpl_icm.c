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
#include <utils_assert.h>
#include <hpl_sha_sync.h>

/**
 *  ICM region descriptor in main list.
 */
struct icm_descriptor {
	/** The first byte address of the region */
	uint32_t start_addr;
	/** The configuration value of the region */
	union rcfg {
		struct {
			/** Compare Digest(true) or Write Back Digest(false) */
			uint32_t is_compare_mode : 1;
			/**
			 * The next region descriptor address loaded is the current region
			 * identifier descriptor address incremented by 0x10(false) or is ICM_DSCR(true).
			 */
			uint32_t is_wrap : 1;
			/** The current descriptor terminates the monitoring(true) or not(false) */
			uint32_t is_end_mon : 1;
			uint32_t reserved1 : 1;
			/** Enable(true) or Disable(false) Region Hash Completed Interrupt */
			uint32_t reg_hash_int_en : 1;
			/** Enable(true) or Disable(false) Digest Mismatch Interrupt */
			uint32_t dig_mis_int_en : 1;
			/** Enable(true) or Disable(false) Bus Error Interrupt */
			uint32_t bus_err_int_en : 1;
			/** Enable(true) or Disable(false) Wrap Condition Interrupt */
			uint32_t wrap_con_int_en : 1;
			/** Enable(true) or Disable(false) End Bit Condition Interrupt */
			uint32_t ebit_con_int_en : 1;
			/** Enable(true) or Disable(false) Monitoring Status Updated Condition Interrupt */
			uint32_t status_upt_con_int_en : 1;
			/** SHA processing runtime is the longest(true) one or shortest(false) one */
			uint32_t is_pro_dly : 1;
			uint32_t reserved2 : 1;
			/** SHA Algorithm */
			uint32_t algo : 3;
			uint32_t reserved3 : 9;
			/** The value of HPROT AHB signal when the ICM retrieves the memory region */
			uint32_t mem_reg_val : 6;
			uint32_t reserved4 : 2;
		} bit;
		uint32_t reg;
	} cfg;
	/**
	 * The number of blocks(512 bits) transferred from the memory to the SHA engine.
	 * Transfer size = (tran_size + 1) * 512bits
	 * The maximum size is 65536.
	 */
	uint32_t tran_size;
	/** The address of next region descriptor in secondary list */
	uint32_t *next_addr;
};

COMPILER_ALIGNED(64)
struct icm_descriptor icm_descriptor;
COMPILER_PACK_RESET()

int32_t _sha_sync_init(struct _sha_sync_device *const dev, void *const hw)
{
	dev->hw = hw;

	return ERR_NONE;
}

int32_t _sha_sync_deinit(struct _sha_sync_device *const dev)
{
	hri_icm_write_CTRL_reg(dev->hw, ICM_CTRL_SWRST);
	dev->hw = NULL;

	return ERR_NONE;
}

int32_t _sha_sync_enable(struct _sha_sync_device *const dev)
{
	(void)dev;

	return ERR_NONE;
}

int32_t _sha_sync_disable(struct _sha_sync_device *const dev)
{
	(void)dev;

	return ERR_NONE;
}

int32_t _sha_sync_sha1_start(struct _sha_sync_device *const dev)
{
	void *              hw  = dev->hw;
	struct sha_context *ctx = dev->ctx;

	hri_icm_write_CFG_reg(hw, ICM_CFG_SLBDIS | ICM_CFG_BBC(0) | ICM_CFG_UALGO(ICM_CFG_UALGO_SHA1_Val) | ICM_CFG_UIHASH);
	hri_icm_write_DSCR_reg(hw, (uint32_t)&icm_descriptor);
	hri_icm_write_HASH_reg(hw, (uint32_t)ctx->digest);

	/* SHA-1 initial value */
    //((uint32_t *)ctx->digest)[0]      = 0x01234567;
	//((uint32_t *)ctx->digest)[1]      = 0x89ABCDEF;
	//((uint32_t *)ctx->digest)[2]      = 0xFEDCBA98;
	//((uint32_t *)ctx->digest)[3]      = 0x76543210;
	//((uint32_t *)ctx->digest)[4]      = 0xF0E1D2C3;
    uint8_t init[] = {
        0x67, 0x45, 0x23, 0x01,
        0xEF, 0xCD, 0xAB, 0x89,
        0x98, 0xBA, 0xDC, 0xFE, 
        0x10, 0x32, 0x54, 0x76, 
        0xC3, 0xD2, 0xE1, 0xF0, 
    };
    memcpy(ctx->digest, init, sizeof(init));
    icm_descriptor.cfg.reg            = 0x0;
	icm_descriptor.cfg.bit.is_end_mon = true;

	return ERR_NONE;
}

int32_t _sha_sync_sha256_start(struct _sha_sync_device *const dev)
{
	void *              hw  = dev->hw;
	struct sha_context *ctx = dev->ctx;

	hri_icm_write_CFG_reg(hw,
	                      ICM_CFG_SLBDIS | ICM_CFG_BBC(0) | ICM_CFG_UALGO(ICM_CFG_UALGO_SHA256_Val) | ICM_CFG_UIHASH);
	hri_icm_write_DSCR_reg(hw, (uint32_t)&icm_descriptor);
	hri_icm_write_HASH_reg(hw, (uint32_t)ctx->digest);

	icm_descriptor.cfg.reg            = 0x0;
	icm_descriptor.cfg.bit.is_end_mon = true;
	if (dev->ctx->is_variant) {
		/* SHA-224 initial value */
		//((uint32_t *)ctx->digest)[0] = 0xD89E05C1;
		//((uint32_t *)ctx->digest)[1] = 0x07D57C36;
		//((uint32_t *)ctx->digest)[2] = 0x17DD7030;
		//((uint32_t *)ctx->digest)[3] = 0x39590EF7;
		//((uint32_t *)ctx->digest)[4] = 0x310BC0FF;
		//((uint32_t *)ctx->digest)[5] = 0x11155868;
		//((uint32_t *)ctx->digest)[6] = 0xA78FF964;
		//((uint32_t *)ctx->digest)[7] = 0xA44FFABE;
        uint8_t init[] = {
            0xC1, 0x05, 0x9E, 0xD8,
            0x36, 0x7C, 0xD5, 0x07,
            0x30, 0x70, 0xDD, 0x17, 
            0xF7, 0x0E, 0x59, 0x39, 
            0xFF, 0xC0, 0x0B, 0x31, 
            0x68, 0x58, 0x15, 0x11, 
            0x64, 0xF9, 0x8F, 0xA7, 
            0xBE, 0xFA, 0x4F, 0xA4, 
        };
	    memcpy(ctx->digest, init, sizeof(init));
	} else {
		/* SHA-256 initial value */
	    /*
        ((uint32_t *)ctx->digest)[0] = 0x67E6096A;
		((uint32_t *)ctx->digest)[1] = 0x85AE67BB;
		((uint32_t *)ctx->digest)[2] = 0x72F36E3C;
		((uint32_t *)ctx->digest)[3] = 0x3AF54FA5;
		((uint32_t *)ctx->digest)[4] = 0x7F520E51;
		((uint32_t *)ctx->digest)[5] = 0x8C68059B;
		((uint32_t *)ctx->digest)[6] = 0xABD9831F;
		((uint32_t *)ctx->digest)[7] = 0x19CDE05B;
        */
        uint8_t init[] = {
            0x6A, 0x09, 0xE6, 0x67,
            0xBB, 0x67, 0xAE, 0x85,
            0x3C, 0x6E, 0xF3, 0x72,
            0xA5, 0x4F, 0xF5, 0x3A,
            0x51, 0x0E, 0x52, 0x7F,
            0x9B, 0x05, 0x68, 0x8C,
            0x1F, 0x83, 0xD9, 0xAB,
            0x5B, 0xE0, 0xCD, 0x19,
        };
	    memcpy(ctx->digest, init, sizeof(init));
    }

	return ERR_NONE;
}

int32_t _sha_sync_sha1_process(struct _sha_sync_device *const dev, const uint8_t *input, uint32_t length)
{
	uint32_t            index;
	void *              hw  = dev->hw;
	struct sha_context *ctx = dev->ctx;

	ASSERT(length >= 64 && !(length & 0x3F));

	icm_descriptor.start_addr = (uint32_t)input;
	/* Transfer size = (tran_size + 1) * 512bits */
	icm_descriptor.tran_size = (length >> 6) - 1;

	for (index = 0; index < 5; index++) {
		hri_icm_write_UIHVAL_reg(hw, index, ((uint32_t *)ctx->digest)[index]);
	}

	hri_icm_write_CTRL_reg(hw, ICM_CTRL_ENABLE);
	while (!(hri_icm_read_ISR_reg(hw) & ICM_ISR_RHC(1))) {
	}

	return ERR_NONE;
}

int32_t _sha_sync_sha256_process(struct _sha_sync_device *const dev, const uint8_t *input, uint32_t length)
{
	uint32_t            index;
	void *              hw  = dev->hw;
	struct sha_context *ctx = dev->ctx;

	ASSERT(length >= 64 && !(length & 0x3F));

	icm_descriptor.start_addr = (uint32_t)input;
	/* Transfer size = (tran_size + 1) * 512bits */
	icm_descriptor.tran_size = (length >> 6) - 1;

	for (index = 0; index < 8; index++) {
		hri_icm_write_UIHVAL_reg(hw, index, ((uint32_t *)ctx->digest)[index]);
	}

	hri_icm_write_CTRL_reg(hw, ICM_CTRL_ENABLE);
	while (!(hri_icm_read_ISR_reg(hw) & ICM_ISR_RHC(1))) {
	}

	return ERR_NONE;
}
