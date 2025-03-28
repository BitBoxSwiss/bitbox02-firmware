
/**
 * \file
 *
 * \brief SAM Peripheral Access Controller
 *
 * Copyright (c) 2015-2018 Microchip Technology Inc. and its subsidiaries.
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

#include <compiler.h>
#include <utils_assert.h>
#include <hpl_pac.h>

uint32_t _pac_get_peripheral_id(const void *const module)
{
	uint32_t peripheral = 10;

	if (((uint32_t)module & (uint32_t)HPB1_ADDR) == (uint32_t)HPB1_ADDR) {
		peripheral = 13;
	}

	peripheral = (((uint32_t)module & 0x0F000000) >> 24) * 32 + (((uint32_t)module & 0x000fff00) >> peripheral);

	return peripheral;
}

/**
 * \brief Enable write protect for the given hardware module
 */
int32_t _periph_lock(const void *const module)
{
	ASSERT((((uint32_t)module) > (uint32_t)HPB0_ADDR));

	uint32_t peripheral;
	int32_t  timeout = 1000;
	bool     stat;

	peripheral = _pac_get_peripheral_id(module);

	hri_pac_write_WRCTRL_reg(PAC, PAC_WRCTRL_PERID(peripheral) | PAC_WRCTRL_KEY_SET);

	do {
		_periph_get_lock_state(module, &stat);
	} while (!stat && timeout--);

	if (timeout < 0) {
		return ERR_TIMEOUT;
	}

	return ERR_NONE;
}

/**
 * \brief Disable write protect for the given hardware module
 */
int32_t _periph_unlock(const void *const module)
{
	ASSERT((((uint32_t)module) > (uint32_t)HPB0_ADDR));

	uint32_t peripheral;
	int32_t  timeout = 1000;
	bool     stat;

	peripheral = _pac_get_peripheral_id(module);

	hri_pac_write_WRCTRL_reg(PAC, PAC_WRCTRL_PERID(peripheral) | PAC_WRCTRL_KEY_CLR);

	do {
		_periph_get_lock_state(module, &stat);
	} while (stat && timeout--);

	if (timeout < 0) {
		return ERR_TIMEOUT;
	}

	return ERR_NONE;
}

/**
 * \brief Get write protect for the given hardware module
 */
int32_t _periph_get_lock_state(const void *const module, bool *const state)
{
	ASSERT((((uint32_t)module) > (uint32_t)HPB0_ADDR));

	uint32_t peripheral;

	peripheral = _pac_get_peripheral_id(module) & 0x1F;

	if (((uint32_t)module) < (uint32_t)HPB1_ADDR) {
		*state = hri_pac_get_STATUSA_reg(PAC, 1 << peripheral);
	} else if (((uint32_t)module) < (uint32_t)HPB2_ADDR) {
		*state = hri_pac_get_STATUSB_reg(PAC, 1 << peripheral);
	} else if (((uint32_t)module) < (uint32_t)HPB3_ADDR) {
		*state = hri_pac_get_STATUSC_reg(PAC, 1 << peripheral);
	} else {
		*state = hri_pac_get_STATUSD_reg(PAC, 1 << peripheral);
	}

	return ERR_NONE;
}
