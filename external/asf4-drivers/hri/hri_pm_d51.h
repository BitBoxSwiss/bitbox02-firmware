/**
 * \file
 *
 * \brief SAM PM
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

#ifdef _SAMD51_PM_COMPONENT_
#ifndef _HRI_PM_D51_H_INCLUDED_
#define _HRI_PM_D51_H_INCLUDED_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <hal_atomic.h>

#if defined(ENABLE_PM_CRITICAL_SECTIONS)
#define PM_CRITICAL_SECTION_ENTER() CRITICAL_SECTION_ENTER()
#define PM_CRITICAL_SECTION_LEAVE() CRITICAL_SECTION_LEAVE()
#else
#define PM_CRITICAL_SECTION_ENTER()
#define PM_CRITICAL_SECTION_LEAVE()
#endif

typedef uint8_t hri_pm_bkupcfg_reg_t;
typedef uint8_t hri_pm_ctrla_reg_t;
typedef uint8_t hri_pm_hibcfg_reg_t;
typedef uint8_t hri_pm_intenset_reg_t;
typedef uint8_t hri_pm_intflag_reg_t;
typedef uint8_t hri_pm_pwsakdly_reg_t;
typedef uint8_t hri_pm_sleepcfg_reg_t;
typedef uint8_t hri_pm_stdbycfg_reg_t;

static inline bool hri_pm_get_INTFLAG_SLEEPRDY_bit(const void *const hw)
{
	return (((Pm *)hw)->INTFLAG.reg & PM_INTFLAG_SLEEPRDY) >> PM_INTFLAG_SLEEPRDY_Pos;
}

static inline void hri_pm_clear_INTFLAG_SLEEPRDY_bit(const void *const hw)
{
	((Pm *)hw)->INTFLAG.reg = PM_INTFLAG_SLEEPRDY;
}

static inline bool hri_pm_get_interrupt_SLEEPRDY_bit(const void *const hw)
{
	return (((Pm *)hw)->INTFLAG.reg & PM_INTFLAG_SLEEPRDY) >> PM_INTFLAG_SLEEPRDY_Pos;
}

static inline void hri_pm_clear_interrupt_SLEEPRDY_bit(const void *const hw)
{
	((Pm *)hw)->INTFLAG.reg = PM_INTFLAG_SLEEPRDY;
}

static inline hri_pm_intflag_reg_t hri_pm_get_INTFLAG_reg(const void *const hw, hri_pm_intflag_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->INTFLAG.reg;
	tmp &= mask;
	return tmp;
}

static inline hri_pm_intflag_reg_t hri_pm_read_INTFLAG_reg(const void *const hw)
{
	return ((Pm *)hw)->INTFLAG.reg;
}

static inline void hri_pm_clear_INTFLAG_reg(const void *const hw, hri_pm_intflag_reg_t mask)
{
	((Pm *)hw)->INTFLAG.reg = mask;
}

static inline void hri_pm_set_INTEN_SLEEPRDY_bit(const void *const hw)
{
	((Pm *)hw)->INTENSET.reg = PM_INTENSET_SLEEPRDY;
}

static inline bool hri_pm_get_INTEN_SLEEPRDY_bit(const void *const hw)
{
	return (((Pm *)hw)->INTENSET.reg & PM_INTENSET_SLEEPRDY) >> PM_INTENSET_SLEEPRDY_Pos;
}

static inline void hri_pm_write_INTEN_SLEEPRDY_bit(const void *const hw, bool value)
{
	if (value == 0x0) {
		((Pm *)hw)->INTENCLR.reg = PM_INTENSET_SLEEPRDY;
	} else {
		((Pm *)hw)->INTENSET.reg = PM_INTENSET_SLEEPRDY;
	}
}

static inline void hri_pm_clear_INTEN_SLEEPRDY_bit(const void *const hw)
{
	((Pm *)hw)->INTENCLR.reg = PM_INTENSET_SLEEPRDY;
}

static inline void hri_pm_set_INTEN_reg(const void *const hw, hri_pm_intenset_reg_t mask)
{
	((Pm *)hw)->INTENSET.reg = mask;
}

static inline hri_pm_intenset_reg_t hri_pm_get_INTEN_reg(const void *const hw, hri_pm_intenset_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->INTENSET.reg;
	tmp &= mask;
	return tmp;
}

static inline hri_pm_intenset_reg_t hri_pm_read_INTEN_reg(const void *const hw)
{
	return ((Pm *)hw)->INTENSET.reg;
}

static inline void hri_pm_write_INTEN_reg(const void *const hw, hri_pm_intenset_reg_t data)
{
	((Pm *)hw)->INTENSET.reg = data;
	((Pm *)hw)->INTENCLR.reg = ~data;
}

static inline void hri_pm_clear_INTEN_reg(const void *const hw, hri_pm_intenset_reg_t mask)
{
	((Pm *)hw)->INTENCLR.reg = mask;
}

static inline void hri_pm_set_CTRLA_IORET_bit(const void *const hw)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->CTRLA.reg |= PM_CTRLA_IORET;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_pm_get_CTRLA_IORET_bit(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->CTRLA.reg;
	tmp = (tmp & PM_CTRLA_IORET) >> PM_CTRLA_IORET_Pos;
	return (bool)tmp;
}

static inline void hri_pm_write_CTRLA_IORET_bit(const void *const hw, bool value)
{
	uint8_t tmp;
	PM_CRITICAL_SECTION_ENTER();
	tmp = ((Pm *)hw)->CTRLA.reg;
	tmp &= ~PM_CTRLA_IORET;
	tmp |= value << PM_CTRLA_IORET_Pos;
	((Pm *)hw)->CTRLA.reg = tmp;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_CTRLA_IORET_bit(const void *const hw)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->CTRLA.reg &= ~PM_CTRLA_IORET;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_CTRLA_IORET_bit(const void *const hw)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->CTRLA.reg ^= PM_CTRLA_IORET;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_set_CTRLA_reg(const void *const hw, hri_pm_ctrla_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->CTRLA.reg |= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_ctrla_reg_t hri_pm_get_CTRLA_reg(const void *const hw, hri_pm_ctrla_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->CTRLA.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_pm_write_CTRLA_reg(const void *const hw, hri_pm_ctrla_reg_t data)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->CTRLA.reg = data;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_CTRLA_reg(const void *const hw, hri_pm_ctrla_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->CTRLA.reg &= ~mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_CTRLA_reg(const void *const hw, hri_pm_ctrla_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->CTRLA.reg ^= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_ctrla_reg_t hri_pm_read_CTRLA_reg(const void *const hw)
{
	return ((Pm *)hw)->CTRLA.reg;
}

static inline void hri_pm_set_SLEEPCFG_SLEEPMODE_bf(const void *const hw, hri_pm_sleepcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->SLEEPCFG.reg |= PM_SLEEPCFG_SLEEPMODE(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_sleepcfg_reg_t hri_pm_get_SLEEPCFG_SLEEPMODE_bf(const void *const hw, hri_pm_sleepcfg_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->SLEEPCFG.reg;
	tmp = (tmp & PM_SLEEPCFG_SLEEPMODE(mask)) >> PM_SLEEPCFG_SLEEPMODE_Pos;
	return tmp;
}

static inline void hri_pm_write_SLEEPCFG_SLEEPMODE_bf(const void *const hw, hri_pm_sleepcfg_reg_t data)
{
	uint8_t tmp;
	PM_CRITICAL_SECTION_ENTER();
	tmp = ((Pm *)hw)->SLEEPCFG.reg;
	tmp &= ~PM_SLEEPCFG_SLEEPMODE_Msk;
	tmp |= PM_SLEEPCFG_SLEEPMODE(data);
	((Pm *)hw)->SLEEPCFG.reg = tmp;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_SLEEPCFG_SLEEPMODE_bf(const void *const hw, hri_pm_sleepcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->SLEEPCFG.reg &= ~PM_SLEEPCFG_SLEEPMODE(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_SLEEPCFG_SLEEPMODE_bf(const void *const hw, hri_pm_sleepcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->SLEEPCFG.reg ^= PM_SLEEPCFG_SLEEPMODE(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_sleepcfg_reg_t hri_pm_read_SLEEPCFG_SLEEPMODE_bf(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->SLEEPCFG.reg;
	tmp = (tmp & PM_SLEEPCFG_SLEEPMODE_Msk) >> PM_SLEEPCFG_SLEEPMODE_Pos;
	return tmp;
}

static inline void hri_pm_set_SLEEPCFG_reg(const void *const hw, hri_pm_sleepcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->SLEEPCFG.reg |= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_sleepcfg_reg_t hri_pm_get_SLEEPCFG_reg(const void *const hw, hri_pm_sleepcfg_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->SLEEPCFG.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_pm_write_SLEEPCFG_reg(const void *const hw, hri_pm_sleepcfg_reg_t data)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->SLEEPCFG.reg = data;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_SLEEPCFG_reg(const void *const hw, hri_pm_sleepcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->SLEEPCFG.reg &= ~mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_SLEEPCFG_reg(const void *const hw, hri_pm_sleepcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->SLEEPCFG.reg ^= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_sleepcfg_reg_t hri_pm_read_SLEEPCFG_reg(const void *const hw)
{
	return ((Pm *)hw)->SLEEPCFG.reg;
}

static inline void hri_pm_set_STDBYCFG_RAMCFG_bf(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->STDBYCFG.reg |= PM_STDBYCFG_RAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_stdbycfg_reg_t hri_pm_get_STDBYCFG_RAMCFG_bf(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->STDBYCFG.reg;
	tmp = (tmp & PM_STDBYCFG_RAMCFG(mask)) >> PM_STDBYCFG_RAMCFG_Pos;
	return tmp;
}

static inline void hri_pm_write_STDBYCFG_RAMCFG_bf(const void *const hw, hri_pm_stdbycfg_reg_t data)
{
	uint8_t tmp;
	PM_CRITICAL_SECTION_ENTER();
	tmp = ((Pm *)hw)->STDBYCFG.reg;
	tmp &= ~PM_STDBYCFG_RAMCFG_Msk;
	tmp |= PM_STDBYCFG_RAMCFG(data);
	((Pm *)hw)->STDBYCFG.reg = tmp;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_STDBYCFG_RAMCFG_bf(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->STDBYCFG.reg &= ~PM_STDBYCFG_RAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_STDBYCFG_RAMCFG_bf(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->STDBYCFG.reg ^= PM_STDBYCFG_RAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_stdbycfg_reg_t hri_pm_read_STDBYCFG_RAMCFG_bf(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->STDBYCFG.reg;
	tmp = (tmp & PM_STDBYCFG_RAMCFG_Msk) >> PM_STDBYCFG_RAMCFG_Pos;
	return tmp;
}

static inline void hri_pm_set_STDBYCFG_FASTWKUP_bf(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->STDBYCFG.reg |= PM_STDBYCFG_FASTWKUP(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_stdbycfg_reg_t hri_pm_get_STDBYCFG_FASTWKUP_bf(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->STDBYCFG.reg;
	tmp = (tmp & PM_STDBYCFG_FASTWKUP(mask)) >> PM_STDBYCFG_FASTWKUP_Pos;
	return tmp;
}

static inline void hri_pm_write_STDBYCFG_FASTWKUP_bf(const void *const hw, hri_pm_stdbycfg_reg_t data)
{
	uint8_t tmp;
	PM_CRITICAL_SECTION_ENTER();
	tmp = ((Pm *)hw)->STDBYCFG.reg;
	tmp &= ~PM_STDBYCFG_FASTWKUP_Msk;
	tmp |= PM_STDBYCFG_FASTWKUP(data);
	((Pm *)hw)->STDBYCFG.reg = tmp;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_STDBYCFG_FASTWKUP_bf(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->STDBYCFG.reg &= ~PM_STDBYCFG_FASTWKUP(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_STDBYCFG_FASTWKUP_bf(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->STDBYCFG.reg ^= PM_STDBYCFG_FASTWKUP(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_stdbycfg_reg_t hri_pm_read_STDBYCFG_FASTWKUP_bf(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->STDBYCFG.reg;
	tmp = (tmp & PM_STDBYCFG_FASTWKUP_Msk) >> PM_STDBYCFG_FASTWKUP_Pos;
	return tmp;
}

static inline void hri_pm_set_STDBYCFG_reg(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->STDBYCFG.reg |= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_stdbycfg_reg_t hri_pm_get_STDBYCFG_reg(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->STDBYCFG.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_pm_write_STDBYCFG_reg(const void *const hw, hri_pm_stdbycfg_reg_t data)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->STDBYCFG.reg = data;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_STDBYCFG_reg(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->STDBYCFG.reg &= ~mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_STDBYCFG_reg(const void *const hw, hri_pm_stdbycfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->STDBYCFG.reg ^= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_stdbycfg_reg_t hri_pm_read_STDBYCFG_reg(const void *const hw)
{
	return ((Pm *)hw)->STDBYCFG.reg;
}

static inline void hri_pm_set_HIBCFG_RAMCFG_bf(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->HIBCFG.reg |= PM_HIBCFG_RAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_hibcfg_reg_t hri_pm_get_HIBCFG_RAMCFG_bf(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->HIBCFG.reg;
	tmp = (tmp & PM_HIBCFG_RAMCFG(mask)) >> PM_HIBCFG_RAMCFG_Pos;
	return tmp;
}

static inline void hri_pm_write_HIBCFG_RAMCFG_bf(const void *const hw, hri_pm_hibcfg_reg_t data)
{
	uint8_t tmp;
	PM_CRITICAL_SECTION_ENTER();
	tmp = ((Pm *)hw)->HIBCFG.reg;
	tmp &= ~PM_HIBCFG_RAMCFG_Msk;
	tmp |= PM_HIBCFG_RAMCFG(data);
	((Pm *)hw)->HIBCFG.reg = tmp;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_HIBCFG_RAMCFG_bf(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->HIBCFG.reg &= ~PM_HIBCFG_RAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_HIBCFG_RAMCFG_bf(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->HIBCFG.reg ^= PM_HIBCFG_RAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_hibcfg_reg_t hri_pm_read_HIBCFG_RAMCFG_bf(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->HIBCFG.reg;
	tmp = (tmp & PM_HIBCFG_RAMCFG_Msk) >> PM_HIBCFG_RAMCFG_Pos;
	return tmp;
}

static inline void hri_pm_set_HIBCFG_BRAMCFG_bf(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->HIBCFG.reg |= PM_HIBCFG_BRAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_hibcfg_reg_t hri_pm_get_HIBCFG_BRAMCFG_bf(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->HIBCFG.reg;
	tmp = (tmp & PM_HIBCFG_BRAMCFG(mask)) >> PM_HIBCFG_BRAMCFG_Pos;
	return tmp;
}

static inline void hri_pm_write_HIBCFG_BRAMCFG_bf(const void *const hw, hri_pm_hibcfg_reg_t data)
{
	uint8_t tmp;
	PM_CRITICAL_SECTION_ENTER();
	tmp = ((Pm *)hw)->HIBCFG.reg;
	tmp &= ~PM_HIBCFG_BRAMCFG_Msk;
	tmp |= PM_HIBCFG_BRAMCFG(data);
	((Pm *)hw)->HIBCFG.reg = tmp;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_HIBCFG_BRAMCFG_bf(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->HIBCFG.reg &= ~PM_HIBCFG_BRAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_HIBCFG_BRAMCFG_bf(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->HIBCFG.reg ^= PM_HIBCFG_BRAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_hibcfg_reg_t hri_pm_read_HIBCFG_BRAMCFG_bf(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->HIBCFG.reg;
	tmp = (tmp & PM_HIBCFG_BRAMCFG_Msk) >> PM_HIBCFG_BRAMCFG_Pos;
	return tmp;
}

static inline void hri_pm_set_HIBCFG_reg(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->HIBCFG.reg |= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_hibcfg_reg_t hri_pm_get_HIBCFG_reg(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->HIBCFG.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_pm_write_HIBCFG_reg(const void *const hw, hri_pm_hibcfg_reg_t data)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->HIBCFG.reg = data;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_HIBCFG_reg(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->HIBCFG.reg &= ~mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_HIBCFG_reg(const void *const hw, hri_pm_hibcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->HIBCFG.reg ^= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_hibcfg_reg_t hri_pm_read_HIBCFG_reg(const void *const hw)
{
	return ((Pm *)hw)->HIBCFG.reg;
}

static inline void hri_pm_set_BKUPCFG_BRAMCFG_bf(const void *const hw, hri_pm_bkupcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->BKUPCFG.reg |= PM_BKUPCFG_BRAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_bkupcfg_reg_t hri_pm_get_BKUPCFG_BRAMCFG_bf(const void *const hw, hri_pm_bkupcfg_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->BKUPCFG.reg;
	tmp = (tmp & PM_BKUPCFG_BRAMCFG(mask)) >> PM_BKUPCFG_BRAMCFG_Pos;
	return tmp;
}

static inline void hri_pm_write_BKUPCFG_BRAMCFG_bf(const void *const hw, hri_pm_bkupcfg_reg_t data)
{
	uint8_t tmp;
	PM_CRITICAL_SECTION_ENTER();
	tmp = ((Pm *)hw)->BKUPCFG.reg;
	tmp &= ~PM_BKUPCFG_BRAMCFG_Msk;
	tmp |= PM_BKUPCFG_BRAMCFG(data);
	((Pm *)hw)->BKUPCFG.reg = tmp;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_BKUPCFG_BRAMCFG_bf(const void *const hw, hri_pm_bkupcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->BKUPCFG.reg &= ~PM_BKUPCFG_BRAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_BKUPCFG_BRAMCFG_bf(const void *const hw, hri_pm_bkupcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->BKUPCFG.reg ^= PM_BKUPCFG_BRAMCFG(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_bkupcfg_reg_t hri_pm_read_BKUPCFG_BRAMCFG_bf(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->BKUPCFG.reg;
	tmp = (tmp & PM_BKUPCFG_BRAMCFG_Msk) >> PM_BKUPCFG_BRAMCFG_Pos;
	return tmp;
}

static inline void hri_pm_set_BKUPCFG_reg(const void *const hw, hri_pm_bkupcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->BKUPCFG.reg |= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_bkupcfg_reg_t hri_pm_get_BKUPCFG_reg(const void *const hw, hri_pm_bkupcfg_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->BKUPCFG.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_pm_write_BKUPCFG_reg(const void *const hw, hri_pm_bkupcfg_reg_t data)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->BKUPCFG.reg = data;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_BKUPCFG_reg(const void *const hw, hri_pm_bkupcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->BKUPCFG.reg &= ~mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_BKUPCFG_reg(const void *const hw, hri_pm_bkupcfg_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->BKUPCFG.reg ^= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_bkupcfg_reg_t hri_pm_read_BKUPCFG_reg(const void *const hw)
{
	return ((Pm *)hw)->BKUPCFG.reg;
}

static inline void hri_pm_set_PWSAKDLY_IGNACK_bit(const void *const hw)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->PWSAKDLY.reg |= PM_PWSAKDLY_IGNACK;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline bool hri_pm_get_PWSAKDLY_IGNACK_bit(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->PWSAKDLY.reg;
	tmp = (tmp & PM_PWSAKDLY_IGNACK) >> PM_PWSAKDLY_IGNACK_Pos;
	return (bool)tmp;
}

static inline void hri_pm_write_PWSAKDLY_IGNACK_bit(const void *const hw, bool value)
{
	uint8_t tmp;
	PM_CRITICAL_SECTION_ENTER();
	tmp = ((Pm *)hw)->PWSAKDLY.reg;
	tmp &= ~PM_PWSAKDLY_IGNACK;
	tmp |= value << PM_PWSAKDLY_IGNACK_Pos;
	((Pm *)hw)->PWSAKDLY.reg = tmp;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_PWSAKDLY_IGNACK_bit(const void *const hw)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->PWSAKDLY.reg &= ~PM_PWSAKDLY_IGNACK;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_PWSAKDLY_IGNACK_bit(const void *const hw)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->PWSAKDLY.reg ^= PM_PWSAKDLY_IGNACK;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_set_PWSAKDLY_DLYVAL_bf(const void *const hw, hri_pm_pwsakdly_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->PWSAKDLY.reg |= PM_PWSAKDLY_DLYVAL(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_pwsakdly_reg_t hri_pm_get_PWSAKDLY_DLYVAL_bf(const void *const hw, hri_pm_pwsakdly_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->PWSAKDLY.reg;
	tmp = (tmp & PM_PWSAKDLY_DLYVAL(mask)) >> PM_PWSAKDLY_DLYVAL_Pos;
	return tmp;
}

static inline void hri_pm_write_PWSAKDLY_DLYVAL_bf(const void *const hw, hri_pm_pwsakdly_reg_t data)
{
	uint8_t tmp;
	PM_CRITICAL_SECTION_ENTER();
	tmp = ((Pm *)hw)->PWSAKDLY.reg;
	tmp &= ~PM_PWSAKDLY_DLYVAL_Msk;
	tmp |= PM_PWSAKDLY_DLYVAL(data);
	((Pm *)hw)->PWSAKDLY.reg = tmp;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_PWSAKDLY_DLYVAL_bf(const void *const hw, hri_pm_pwsakdly_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->PWSAKDLY.reg &= ~PM_PWSAKDLY_DLYVAL(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_PWSAKDLY_DLYVAL_bf(const void *const hw, hri_pm_pwsakdly_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->PWSAKDLY.reg ^= PM_PWSAKDLY_DLYVAL(mask);
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_pwsakdly_reg_t hri_pm_read_PWSAKDLY_DLYVAL_bf(const void *const hw)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->PWSAKDLY.reg;
	tmp = (tmp & PM_PWSAKDLY_DLYVAL_Msk) >> PM_PWSAKDLY_DLYVAL_Pos;
	return tmp;
}

static inline void hri_pm_set_PWSAKDLY_reg(const void *const hw, hri_pm_pwsakdly_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->PWSAKDLY.reg |= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_pwsakdly_reg_t hri_pm_get_PWSAKDLY_reg(const void *const hw, hri_pm_pwsakdly_reg_t mask)
{
	uint8_t tmp;
	tmp = ((Pm *)hw)->PWSAKDLY.reg;
	tmp &= mask;
	return tmp;
}

static inline void hri_pm_write_PWSAKDLY_reg(const void *const hw, hri_pm_pwsakdly_reg_t data)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->PWSAKDLY.reg = data;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_clear_PWSAKDLY_reg(const void *const hw, hri_pm_pwsakdly_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->PWSAKDLY.reg &= ~mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline void hri_pm_toggle_PWSAKDLY_reg(const void *const hw, hri_pm_pwsakdly_reg_t mask)
{
	PM_CRITICAL_SECTION_ENTER();
	((Pm *)hw)->PWSAKDLY.reg ^= mask;
	PM_CRITICAL_SECTION_LEAVE();
}

static inline hri_pm_pwsakdly_reg_t hri_pm_read_PWSAKDLY_reg(const void *const hw)
{
	return ((Pm *)hw)->PWSAKDLY.reg;
}

#ifdef __cplusplus
}
#endif

#endif /* _HRI_PM_D51_H_INCLUDED */
#endif /* _SAMD51_PM_COMPONENT_ */
