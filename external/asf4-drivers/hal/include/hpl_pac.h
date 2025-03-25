/**
 * \file
 *
 * \brief PAC related functionality declaration.
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
#ifndef _HPL_PAC_H_INCLUDED
#define _HPL_PAC_H_INCLUDED

#include <compiler.h>
#include "hpl_irq.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \brief Enable write protect for the given hardware module
 *
 * This function enables write protect for the given hardware module.
 * For an overview of available PAC and hardware modules see datasheet.
 *
 * \param[in] module A hardware module to enable write protect for
 */
int32_t _periph_lock(const void *const module);

/**
 * \brief Disable write protect for the given hardware module
 *
 * This function disables write protect for the given hardware module.
 * For an overview of available PAC and hardware modules see datasheet.
 *
 * \param[in] module A hardware module to disable clock for
 */
int32_t _periph_unlock(const void *const module);

/**
 * \brief Get write protect state for the given hardware module
 *
 * This function get write protect state for the given hardware module.
 * For an overview of available PAC and hardware modules see datasheet.
 *
 * \param[in] module A hardware module to disable clock for
 * \param[out] state The pointer to write protect state for specified module
 */
int32_t _periph_get_lock_state(const void *const module, bool *const state);

uint32_t _pac_get_peripheral_id(const void *const module);

#ifdef __cplusplus
}
#endif

#endif /* _HPL_PAC_H_INCLUDED */
