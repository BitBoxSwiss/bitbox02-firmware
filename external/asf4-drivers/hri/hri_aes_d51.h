/**
 * \file
 *
 * \brief SAM AES
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
 */

#ifdef _SAMD51_AES_COMPONENT_
#ifndef _HRI_AES_D51_H_INCLUDED_
#define _HRI_AES_D51_H_INCLUDED_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <hal_atomic.h>

#if defined(ENABLE_AES_CRITICAL_SECTIONS)
#define AES_CRITICAL_SECTION_ENTER() CRITICAL_SECTION_ENTER()
#define AES_CRITICAL_SECTION_LEAVE() CRITICAL_SECTION_LEAVE()
#else
#define AES_CRITICAL_SECTION_ENTER()
#define AES_CRITICAL_SECTION_LEAVE()
#endif

typedef uint32_t hri_aes_ciplen_reg_t;
typedef uint32_t hri_aes_ctrla_reg_t;
typedef uint32_t hri_aes_ghash_reg_t;
typedef uint32_t hri_aes_hashkey_reg_t;
typedef uint32_t hri_aes_indata_reg_t;
typedef uint32_t hri_aes_intvectv_reg_t;
typedef uint32_t hri_aes_keyword_reg_t;
typedef uint32_t hri_aes_randseed_reg_t;
typedef uint8_t  hri_aes_ctrlb_reg_t;
typedef uint8_t  hri_aes_databufptr_reg_t;
typedef uint8_t  hri_aes_dbgctrl_reg_t;
typedef uint8_t  hri_aes_intenset_reg_t;
typedef uint8_t  hri_aes_intflag_reg_t;

static inline void hri_aes_set_INTEN_ENCCMP_bit(const void *const hw)
{
	((Aes *)(uintptr_t)hw)->INTENSET.reg = AES_INTENSET_ENCCMP;
}

static inline bool hri_aes_get_INTEN_ENCCMP_bit(const void *const hw)
{
	return (((Aes *)(uintptr_t)hw)->INTENSET.reg & AES_INTENSET_ENCCMP) >> AES_INTENSET_ENCCMP_Pos;
}

static inline void hri_aes_write_INTEN_ENCCMP_bit(const void *const hw, bool value)
{
	if (value == 0x0) {
		((Aes *)(uintptr_t)hw)->INTENCLR.reg = AES_INTENSET_ENCCMP;
	} else {
		((Aes *)(uintptr_t)hw)->INTENSET.reg = AES_INTENSET_ENCCMP;
	}
}

static inline void hri_aes_clear_INTEN_ENCCMP_bit(const void *const hw)
{
	((Aes *)(uintptr_t)hw)->INTENCLR.reg = AES_INTENSET_ENCCMP;
}

static inline void hri_aes_set_INTEN_GFMCMP_bit(const void *const hw)
{
	((Aes *)(uintptr_t)hw)->INTENSET.reg = AES_INTENSET_GFMCMP;
}

static inline bool hri_aes_get_INTEN_GFMCMP_bit(const void *const hw)
{
	return (((Aes *)(uintptr_t)hw)->INTENSET.reg & AES_INTENSET_GFMCMP) >> AES_INTENSET_GFMCMP_Pos;
}

static inline void hri_aes_write_INTEN_GFMCMP_bit(const void *const hw, bool value)
{
	if (value == 0x0) {
		((Aes *)(uintptr_t)hw)->INTENCLR.reg = AES_INTENSET_GFMCMP;
	} else {
		((Aes *)(uintptr_t)hw)->INTENSET.reg = AES_INTENSET_GFMCMP;
	}
}

static inline void hri_aes_clear_INTEN_GFMCMP_bit(const void *const hw)
{
	((Aes *)(uintptr_t)hw)->INTENCLR.reg = AES_INTENSET_GFMCMP;
}

static inline void hri_aes_set_INTEN_reg(const void *const hw, hri_aes_intenset_reg_t mask)
{
	((Aes *)(uintptr_t)hw)->INTENSET.reg = mask;
}

static inline hri_aes_intenset_reg_t hri_aes_get_INTEN_reg(const void *const hw, hri_aes_intenset_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->INTENSET.reg;
	tmp &= mask;
	return tmp;
}

static inline hri_aes_intenset_reg_t hri_aes_read_INTEN_reg(const void *const hw)
{
	return ((Aes *)(uintptr_t)hw)->INTENSET.reg;
}

static inline void hri_aes_write_INTEN_reg(const void *const hw, hri_aes_intenset_reg_t data)
{
	((Aes *)(uintptr_t)hw)->INTENSET.reg = data;
	((Aes *)(uintptr_t)hw)->INTENCLR.reg = ~data;
}

static inline void hri_aes_clear_INTEN_reg(const void *const hw, hri_aes_intenset_reg_t mask)
{
	((Aes *)(uintptr_t)hw)->INTENCLR.reg = mask;
}

static inline bool hri_aes_get_INTFLAG_ENCCMP_bit(const void *const hw)
{
	return (((Aes *)(uintptr_t)hw)->INTFLAG.reg & AES_INTFLAG_ENCCMP) >> AES_INTFLAG_ENCCMP_Pos;
}

static inline void hri_aes_clear_INTFLAG_ENCCMP_bit(const void *const hw)
{
	((Aes *)(uintptr_t)hw)->INTFLAG.reg = AES_INTFLAG_ENCCMP;
}

static inline bool hri_aes_get_INTFLAG_GFMCMP_bit(const void *const hw)
{
	return (((Aes *)(uintptr_t)hw)->INTFLAG.reg & AES_INTFLAG_GFMCMP) >> AES_INTFLAG_GFMCMP_Pos;
}

static inline void hri_aes_clear_INTFLAG_GFMCMP_bit(const void *const hw)
{
	((Aes *)(uintptr_t)hw)->INTFLAG.reg = AES_INTFLAG_GFMCMP;
}

static inline bool hri_aes_get_interrupt_ENCCMP_bit(const void *const hw)
{
	return (((Aes *)(uintptr_t)hw)->INTFLAG.reg & AES_INTFLAG_ENCCMP) >> AES_INTFLAG_ENCCMP_Pos;
}

static inline void hri_aes_clear_interrupt_ENCCMP_bit(const void *const hw)
{
	((Aes *)(uintptr_t)hw)->INTFLAG.reg = AES_INTFLAG_ENCCMP;
}

static inline bool hri_aes_get_interrupt_GFMCMP_bit(const void *const hw)
{
	return (((Aes *)(uintptr_t)hw)->INTFLAG.reg & AES_INTFLAG_GFMCMP) >> AES_INTFLAG_GFMCMP_Pos;
}

static inline void hri_aes_clear_interrupt_GFMCMP_bit(const void *const hw)
{
	((Aes *)(uintptr_t)hw)->INTFLAG.reg = AES_INTFLAG_GFMCMP;
}

static inline hri_aes_intflag_reg_t hri_aes_get_INTFLAG_reg(const void *const hw, hri_aes_intflag_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->INTFLAG.reg;
	tmp &= mask;
	return tmp;
}

static inline hri_aes_intflag_reg_t hri_aes_read_INTFLAG_reg(const void *const hw)
{
	return ((Aes *)(uintptr_t)hw)->INTFLAG.reg;
}

static inline void hri_aes_clear_INTFLAG_reg(const void *const hw, hri_aes_intflag_reg_t mask)
{
	((Aes *)(uintptr_t)hw)->INTFLAG.reg = mask;
}

static inline void hri_aes_write_KEYWORD_reg(const void *const hw, uint8_t index, hri_aes_keyword_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->KEYWORD[index].reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_write_INTVECTV_reg(const void *const hw, uint8_t index, hri_aes_intvectv_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->INTVECTV[index].reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLA_SWRST_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_SWRST;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLA_SWRST_bit(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_SWRST) >> AES_CTRLA_SWRST_Pos;
	return (bool)tmp;
}

static inline void hri_aes_set_CTRLA_ENABLE_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_ENABLE;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLA_ENABLE_bit(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_ENABLE) >> AES_CTRLA_ENABLE_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_CTRLA_ENABLE_bit(const void *const hw, bool value)
{
	uint32_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= ~AES_CTRLA_ENABLE;
	tmp |= value << AES_CTRLA_ENABLE_Pos;
	((Aes *)(uintptr_t)hw)->CTRLA.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_ENABLE_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~AES_CTRLA_ENABLE;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_ENABLE_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= AES_CTRLA_ENABLE;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLA_CIPHER_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_CIPHER;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLA_CIPHER_bit(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_CIPHER) >> AES_CTRLA_CIPHER_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_CTRLA_CIPHER_bit(const void *const hw, bool value)
{
	uint32_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= ~AES_CTRLA_CIPHER;
	tmp |= value << AES_CTRLA_CIPHER_Pos;
	((Aes *)(uintptr_t)hw)->CTRLA.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_CIPHER_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~AES_CTRLA_CIPHER;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_CIPHER_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= AES_CTRLA_CIPHER;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLA_STARTMODE_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_STARTMODE;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLA_STARTMODE_bit(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_STARTMODE) >> AES_CTRLA_STARTMODE_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_CTRLA_STARTMODE_bit(const void *const hw, bool value)
{
	uint32_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= ~AES_CTRLA_STARTMODE;
	tmp |= value << AES_CTRLA_STARTMODE_Pos;
	((Aes *)(uintptr_t)hw)->CTRLA.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_STARTMODE_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~AES_CTRLA_STARTMODE;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_STARTMODE_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= AES_CTRLA_STARTMODE;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLA_LOD_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_LOD;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLA_LOD_bit(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_LOD) >> AES_CTRLA_LOD_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_CTRLA_LOD_bit(const void *const hw, bool value)
{
	uint32_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= ~AES_CTRLA_LOD;
	tmp |= value << AES_CTRLA_LOD_Pos;
	((Aes *)(uintptr_t)hw)->CTRLA.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_LOD_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~AES_CTRLA_LOD;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_LOD_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= AES_CTRLA_LOD;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLA_KEYGEN_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_KEYGEN;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLA_KEYGEN_bit(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_KEYGEN) >> AES_CTRLA_KEYGEN_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_CTRLA_KEYGEN_bit(const void *const hw, bool value)
{
	uint32_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= ~AES_CTRLA_KEYGEN;
	tmp |= value << AES_CTRLA_KEYGEN_Pos;
	((Aes *)(uintptr_t)hw)->CTRLA.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_KEYGEN_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~AES_CTRLA_KEYGEN;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_KEYGEN_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= AES_CTRLA_KEYGEN;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLA_XORKEY_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_XORKEY;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLA_XORKEY_bit(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_XORKEY) >> AES_CTRLA_XORKEY_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_CTRLA_XORKEY_bit(const void *const hw, bool value)
{
	uint32_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= ~AES_CTRLA_XORKEY;
	tmp |= value << AES_CTRLA_XORKEY_Pos;
	((Aes *)(uintptr_t)hw)->CTRLA.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_XORKEY_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~AES_CTRLA_XORKEY;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_XORKEY_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= AES_CTRLA_XORKEY;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLA_AESMODE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_AESMODE(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrla_reg_t hri_aes_get_CTRLA_AESMODE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_AESMODE(mask)) >> AES_CTRLA_AESMODE_Pos;
	return tmp;
}

static inline void hri_aes_write_CTRLA_AESMODE_bf(const void *const hw, hri_aes_ctrla_reg_t data)
{
	uint32_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= ~AES_CTRLA_AESMODE_Msk;
	tmp |= AES_CTRLA_AESMODE(data);
	((Aes *)(uintptr_t)hw)->CTRLA.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_AESMODE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~AES_CTRLA_AESMODE(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_AESMODE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= AES_CTRLA_AESMODE(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrla_reg_t hri_aes_read_CTRLA_AESMODE_bf(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_AESMODE_Msk) >> AES_CTRLA_AESMODE_Pos;
	return tmp;
}

static inline void hri_aes_set_CTRLA_CFBS_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_CFBS(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrla_reg_t hri_aes_get_CTRLA_CFBS_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_CFBS(mask)) >> AES_CTRLA_CFBS_Pos;
	return tmp;
}

static inline void hri_aes_write_CTRLA_CFBS_bf(const void *const hw, hri_aes_ctrla_reg_t data)
{
	uint32_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= ~AES_CTRLA_CFBS_Msk;
	tmp |= AES_CTRLA_CFBS(data);
	((Aes *)(uintptr_t)hw)->CTRLA.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_CFBS_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~AES_CTRLA_CFBS(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_CFBS_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= AES_CTRLA_CFBS(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrla_reg_t hri_aes_read_CTRLA_CFBS_bf(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_CFBS_Msk) >> AES_CTRLA_CFBS_Pos;
	return tmp;
}

static inline void hri_aes_set_CTRLA_KEYSIZE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_KEYSIZE(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrla_reg_t hri_aes_get_CTRLA_KEYSIZE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_KEYSIZE(mask)) >> AES_CTRLA_KEYSIZE_Pos;
	return tmp;
}

static inline void hri_aes_write_CTRLA_KEYSIZE_bf(const void *const hw, hri_aes_ctrla_reg_t data)
{
	uint32_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= ~AES_CTRLA_KEYSIZE_Msk;
	tmp |= AES_CTRLA_KEYSIZE(data);
	((Aes *)(uintptr_t)hw)->CTRLA.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_KEYSIZE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~AES_CTRLA_KEYSIZE(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_KEYSIZE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= AES_CTRLA_KEYSIZE(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrla_reg_t hri_aes_read_CTRLA_KEYSIZE_bf(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_KEYSIZE_Msk) >> AES_CTRLA_KEYSIZE_Pos;
	return tmp;
}

static inline void hri_aes_set_CTRLA_CTYPE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= AES_CTRLA_CTYPE(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrla_reg_t hri_aes_get_CTRLA_CTYPE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_CTYPE(mask)) >> AES_CTRLA_CTYPE_Pos;
	return tmp;
}

static inline void hri_aes_write_CTRLA_CTYPE_bf(const void *const hw, hri_aes_ctrla_reg_t data)
{
	uint32_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= ~AES_CTRLA_CTYPE_Msk;
	tmp |= AES_CTRLA_CTYPE(data);
	((Aes *)(uintptr_t)hw)->CTRLA.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_CTYPE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~AES_CTRLA_CTYPE(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_CTYPE_bf(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= AES_CTRLA_CTYPE(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrla_reg_t hri_aes_read_CTRLA_CTYPE_bf(const void *const hw)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp = (tmp & AES_CTRLA_CTYPE_Msk) >> AES_CTRLA_CTYPE_Pos;
	return tmp;
}

static inline void hri_aes_set_CTRLA_reg(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg |= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrla_reg_t hri_aes_get_CTRLA_reg(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLA.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_aes_write_CTRLA_reg(const void *const hw, hri_aes_ctrla_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLA_reg(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg &= ~mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLA_reg(const void *const hw, hri_aes_ctrla_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLA.reg ^= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrla_reg_t hri_aes_read_CTRLA_reg(const void *const hw)
{
	return ((Aes *)(uintptr_t)hw)->CTRLA.reg;
}

static inline void hri_aes_set_CTRLB_START_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg |= AES_CTRLB_START;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLB_START_bit(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLB.reg;
	tmp = (tmp & AES_CTRLB_START) >> AES_CTRLB_START_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_CTRLB_START_bit(const void *const hw, bool value)
{
	uint8_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLB.reg;
	tmp &= ~AES_CTRLB_START;
	tmp |= value << AES_CTRLB_START_Pos;
	((Aes *)(uintptr_t)hw)->CTRLB.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLB_START_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg &= ~AES_CTRLB_START;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLB_START_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg ^= AES_CTRLB_START;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLB_NEWMSG_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg |= AES_CTRLB_NEWMSG;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLB_NEWMSG_bit(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLB.reg;
	tmp = (tmp & AES_CTRLB_NEWMSG) >> AES_CTRLB_NEWMSG_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_CTRLB_NEWMSG_bit(const void *const hw, bool value)
{
	uint8_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLB.reg;
	tmp &= ~AES_CTRLB_NEWMSG;
	tmp |= value << AES_CTRLB_NEWMSG_Pos;
	((Aes *)(uintptr_t)hw)->CTRLB.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLB_NEWMSG_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg &= ~AES_CTRLB_NEWMSG;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLB_NEWMSG_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg ^= AES_CTRLB_NEWMSG;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLB_EOM_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg |= AES_CTRLB_EOM;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLB_EOM_bit(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLB.reg;
	tmp = (tmp & AES_CTRLB_EOM) >> AES_CTRLB_EOM_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_CTRLB_EOM_bit(const void *const hw, bool value)
{
	uint8_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLB.reg;
	tmp &= ~AES_CTRLB_EOM;
	tmp |= value << AES_CTRLB_EOM_Pos;
	((Aes *)(uintptr_t)hw)->CTRLB.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLB_EOM_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg &= ~AES_CTRLB_EOM;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLB_EOM_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg ^= AES_CTRLB_EOM;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLB_GFMUL_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg |= AES_CTRLB_GFMUL;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_CTRLB_GFMUL_bit(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLB.reg;
	tmp = (tmp & AES_CTRLB_GFMUL) >> AES_CTRLB_GFMUL_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_CTRLB_GFMUL_bit(const void *const hw, bool value)
{
	uint8_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->CTRLB.reg;
	tmp &= ~AES_CTRLB_GFMUL;
	tmp |= value << AES_CTRLB_GFMUL_Pos;
	((Aes *)(uintptr_t)hw)->CTRLB.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLB_GFMUL_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg &= ~AES_CTRLB_GFMUL;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLB_GFMUL_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg ^= AES_CTRLB_GFMUL;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_CTRLB_reg(const void *const hw, hri_aes_ctrlb_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg |= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrlb_reg_t hri_aes_get_CTRLB_reg(const void *const hw, hri_aes_ctrlb_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CTRLB.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_aes_write_CTRLB_reg(const void *const hw, hri_aes_ctrlb_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CTRLB_reg(const void *const hw, hri_aes_ctrlb_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg &= ~mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CTRLB_reg(const void *const hw, hri_aes_ctrlb_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CTRLB.reg ^= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ctrlb_reg_t hri_aes_read_CTRLB_reg(const void *const hw)
{
	return ((Aes *)(uintptr_t)hw)->CTRLB.reg;
}

static inline void hri_aes_set_DATABUFPTR_INDATAPTR_bf(const void *const hw, hri_aes_databufptr_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DATABUFPTR.reg |= AES_DATABUFPTR_INDATAPTR(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_databufptr_reg_t hri_aes_get_DATABUFPTR_INDATAPTR_bf(const void *const        hw,
                                                                           hri_aes_databufptr_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->DATABUFPTR.reg;
	tmp = (tmp & AES_DATABUFPTR_INDATAPTR(mask)) >> AES_DATABUFPTR_INDATAPTR_Pos;
	return tmp;
}

static inline void hri_aes_write_DATABUFPTR_INDATAPTR_bf(const void *const hw, hri_aes_databufptr_reg_t data)
{
	uint8_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->DATABUFPTR.reg;
	tmp &= ~AES_DATABUFPTR_INDATAPTR_Msk;
	tmp |= AES_DATABUFPTR_INDATAPTR(data);
	((Aes *)(uintptr_t)hw)->DATABUFPTR.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_DATABUFPTR_INDATAPTR_bf(const void *const hw, hri_aes_databufptr_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DATABUFPTR.reg &= ~AES_DATABUFPTR_INDATAPTR(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_DATABUFPTR_INDATAPTR_bf(const void *const hw, hri_aes_databufptr_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DATABUFPTR.reg ^= AES_DATABUFPTR_INDATAPTR(mask);
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_databufptr_reg_t hri_aes_read_DATABUFPTR_INDATAPTR_bf(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->DATABUFPTR.reg;
	tmp = (tmp & AES_DATABUFPTR_INDATAPTR_Msk) >> AES_DATABUFPTR_INDATAPTR_Pos;
	return tmp;
}

static inline void hri_aes_set_DATABUFPTR_reg(const void *const hw, hri_aes_databufptr_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DATABUFPTR.reg |= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_databufptr_reg_t hri_aes_get_DATABUFPTR_reg(const void *const hw, hri_aes_databufptr_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->DATABUFPTR.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_aes_write_DATABUFPTR_reg(const void *const hw, hri_aes_databufptr_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DATABUFPTR.reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_DATABUFPTR_reg(const void *const hw, hri_aes_databufptr_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DATABUFPTR.reg &= ~mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_DATABUFPTR_reg(const void *const hw, hri_aes_databufptr_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DATABUFPTR.reg ^= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_databufptr_reg_t hri_aes_read_DATABUFPTR_reg(const void *const hw)
{
	return ((Aes *)(uintptr_t)hw)->DATABUFPTR.reg;
}

static inline void hri_aes_set_DBGCTRL_DBGRUN_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DBGCTRL.reg |= AES_DBGCTRL_DBGRUN;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_aes_get_DBGCTRL_DBGRUN_bit(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->DBGCTRL.reg;
	tmp = (tmp & AES_DBGCTRL_DBGRUN) >> AES_DBGCTRL_DBGRUN_Pos;
	return (bool)tmp;
}

static inline void hri_aes_write_DBGCTRL_DBGRUN_bit(const void *const hw, bool value)
{
	uint8_t tmp;
	AES_CRITICAL_SECTION_ENTER();
	tmp = ((Aes *)(uintptr_t)hw)->DBGCTRL.reg;
	tmp &= ~AES_DBGCTRL_DBGRUN;
	tmp |= value << AES_DBGCTRL_DBGRUN_Pos;
	((Aes *)(uintptr_t)hw)->DBGCTRL.reg = tmp;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_DBGCTRL_DBGRUN_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DBGCTRL.reg &= ~AES_DBGCTRL_DBGRUN;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_DBGCTRL_DBGRUN_bit(const void *const hw)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DBGCTRL.reg ^= AES_DBGCTRL_DBGRUN;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_set_DBGCTRL_reg(const void *const hw, hri_aes_dbgctrl_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DBGCTRL.reg |= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_dbgctrl_reg_t hri_aes_get_DBGCTRL_reg(const void *const hw, hri_aes_dbgctrl_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->DBGCTRL.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_aes_write_DBGCTRL_reg(const void *const hw, hri_aes_dbgctrl_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DBGCTRL.reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_DBGCTRL_reg(const void *const hw, hri_aes_dbgctrl_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DBGCTRL.reg &= ~mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_DBGCTRL_reg(const void *const hw, hri_aes_dbgctrl_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->DBGCTRL.reg ^= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_dbgctrl_reg_t hri_aes_read_DBGCTRL_reg(const void *const hw)
{
	return ((Aes *)(uintptr_t)hw)->DBGCTRL.reg;
}

static inline void hri_aes_set_INDATA_reg(const void *const hw, hri_aes_indata_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->INDATA.reg |= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_indata_reg_t hri_aes_get_INDATA_reg(const void *const hw, hri_aes_indata_reg_t mask)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->INDATA.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_aes_write_INDATA_reg(const void *const hw, hri_aes_indata_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->INDATA.reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_INDATA_reg(const void *const hw, hri_aes_indata_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->INDATA.reg &= ~mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_INDATA_reg(const void *const hw, hri_aes_indata_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->INDATA.reg ^= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_indata_reg_t hri_aes_read_INDATA_reg(const void *const hw)
{
	return ((Aes *)(uintptr_t)hw)->INDATA.reg;
}

static inline void hri_aes_set_HASHKEY_reg(const void *const hw, uint8_t index, hri_aes_hashkey_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->HASHKEY[index].reg |= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_hashkey_reg_t hri_aes_get_HASHKEY_reg(const void *const hw, uint8_t index,
                                                            hri_aes_hashkey_reg_t mask)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->HASHKEY[index].reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_aes_write_HASHKEY_reg(const void *const hw, uint8_t index, hri_aes_hashkey_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->HASHKEY[index].reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_HASHKEY_reg(const void *const hw, uint8_t index, hri_aes_hashkey_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->HASHKEY[index].reg &= ~mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_HASHKEY_reg(const void *const hw, uint8_t index, hri_aes_hashkey_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->HASHKEY[index].reg ^= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_hashkey_reg_t hri_aes_read_HASHKEY_reg(const void *const hw, uint8_t index)
{
	return ((Aes *)(uintptr_t)hw)->HASHKEY[index].reg;
}

static inline void hri_aes_set_GHASH_reg(const void *const hw, uint8_t index, hri_aes_ghash_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->GHASH[index].reg |= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ghash_reg_t hri_aes_get_GHASH_reg(const void *const hw, uint8_t index, hri_aes_ghash_reg_t mask)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->GHASH[index].reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_aes_write_GHASH_reg(const void *const hw, uint8_t index, hri_aes_ghash_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->GHASH[index].reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_GHASH_reg(const void *const hw, uint8_t index, hri_aes_ghash_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->GHASH[index].reg &= ~mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_GHASH_reg(const void *const hw, uint8_t index, hri_aes_ghash_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->GHASH[index].reg ^= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ghash_reg_t hri_aes_read_GHASH_reg(const void *const hw, uint8_t index)
{
	return ((Aes *)(uintptr_t)hw)->GHASH[index].reg;
}

static inline void hri_aes_set_CIPLEN_reg(const void *const hw, hri_aes_ciplen_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CIPLEN.reg |= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ciplen_reg_t hri_aes_get_CIPLEN_reg(const void *const hw, hri_aes_ciplen_reg_t mask)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->CIPLEN.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_aes_write_CIPLEN_reg(const void *const hw, hri_aes_ciplen_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CIPLEN.reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_CIPLEN_reg(const void *const hw, hri_aes_ciplen_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CIPLEN.reg &= ~mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_CIPLEN_reg(const void *const hw, hri_aes_ciplen_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->CIPLEN.reg ^= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_ciplen_reg_t hri_aes_read_CIPLEN_reg(const void *const hw)
{
	return ((Aes *)(uintptr_t)hw)->CIPLEN.reg;
}

static inline void hri_aes_set_RANDSEED_reg(const void *const hw, hri_aes_randseed_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->RANDSEED.reg |= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_randseed_reg_t hri_aes_get_RANDSEED_reg(const void *const hw, hri_aes_randseed_reg_t mask)
{
	uint32_t tmp;
	tmp = ((Aes *)(uintptr_t)hw)->RANDSEED.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_aes_write_RANDSEED_reg(const void *const hw, hri_aes_randseed_reg_t data)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->RANDSEED.reg = data;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_clear_RANDSEED_reg(const void *const hw, hri_aes_randseed_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->RANDSEED.reg &= ~mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline void hri_aes_toggle_RANDSEED_reg(const void *const hw, hri_aes_randseed_reg_t mask)
{
	AES_CRITICAL_SECTION_ENTER();
	((Aes *)(uintptr_t)hw)->RANDSEED.reg ^= mask;
	AES_CRITICAL_SECTION_LEAVE();
}

static inline hri_aes_randseed_reg_t hri_aes_read_RANDSEED_reg(const void *const hw)
{
	return ((Aes *)(uintptr_t)hw)->RANDSEED.reg;
}

#ifdef __cplusplus
}
#endif

#endif /* _HRI_AES_D51_H_INCLUDED */
#endif /* _SAMD51_AES_COMPONENT_ */
