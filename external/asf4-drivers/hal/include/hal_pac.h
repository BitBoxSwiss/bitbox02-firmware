/**
 * \file
 *
 * \brief PAC functionality declaration.
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

#ifndef HAL_PAC_H_INCLUDED
#define HAL_PAC_H_INCLUDED

#include <hpl_pac.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup doc_driver_hal_pac
 *
 *@{
 */

/** \brief Enable write protect for the given hardware module
 *
 *  \param[in] module Pointer to the hardware module
 */
static inline int32_t periph_lock(void *const module)
{
	return _periph_lock(module);
}

/** \brief Disable write protect for the given hardware module
 *
 *  \param[in] module Pointer to the hardware module
 */
static inline int32_t periph_unlock(void *const module)
{
	return _periph_unlock(module);
}

/** \brief Get write protect state for the given hardware module
 *
 *  \param[in] module Pointer to the hardware module
 *  \param[out] state Pointer to write protect state for specified module
 */
static inline int32_t periph_get_lock_state(void *const module, bool *const state)
{
	return _periph_get_lock_state(module, state);
}

/**
 * \brief Get PAC driver version
 */
uint32_t pac_get_version(void);

#ifdef __cplusplus
}
#endif

#endif /* HAL_PAC_H_INCLUDED */
